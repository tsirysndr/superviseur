use std::{
    collections::HashMap,
    io::{self, BufRead},
    process::Command,
    sync::{Arc, Mutex},
    thread,
};

use async_trait::async_trait;
use futures_util::Stream;
use hcl::format;
use owo_colors::OwoColorize;
use shiplift::{
    tty::{self, TtyChunk},
    ContainerOptions, Docker, LogsOptions,
};
use tokio::sync::mpsc;
use tokio_stream::StreamExt;

use crate::{
    superviseur::{
        core::ProcessEvent,
        drivers::DriverPlugin,
        logs::{self, LogEngine},
    },
    types::{
        configuration::{DriverConfig, Service},
        process::Process,
    },
};

#[derive(Clone)]
pub struct Driver {
    docker: Docker,
    project: String,
    service: Service,
    processes: Arc<Mutex<Vec<(Process, String)>>>,
    childs: Arc<Mutex<HashMap<String, i32>>>,
    event_tx: mpsc::UnboundedSender<ProcessEvent>,
    log_engine: logs::LogEngine,
    config: Option<DriverConfig>,
}

impl Default for Driver {
    fn default() -> Self {
        let (event_tx, _) = mpsc::unbounded_channel();
        Self {
            docker: Docker::new(),
            project: "".to_string(),
            service: Service::default(),
            processes: Arc::new(Mutex::new(Vec::new())),
            childs: Arc::new(Mutex::new(HashMap::new())),
            event_tx,
            log_engine: logs::LogEngine::new(),
            config: None,
        }
    }
}

impl Driver {
    pub fn new(
        project: String,
        service: &Service,
        processes: Arc<Mutex<Vec<(Process, String)>>>,
        event_tx: mpsc::UnboundedSender<ProcessEvent>,
        childs: Arc<Mutex<HashMap<String, i32>>>,
        log_engine: LogEngine,
    ) -> Self {
        let config = service
            .r#use
            .as_ref()
            .unwrap()
            .into_iter()
            .find(|(driver, _)| *driver == "docker")
            .map(|(_, x)| x)
            .unwrap();
        Self {
            docker: Docker::new(),
            project,
            service: service.clone(),
            processes,
            childs,
            event_tx,
            log_engine,
            config: Some(config.clone()),
        }
    }
}

#[async_trait]
impl DriverPlugin for Driver {
    async fn start(&self, project: String) -> Result<(), anyhow::Error> {
        let container_name = format!("{}_{}", project, self.service.name);
        let container = self.docker.containers().get(&container_name);
        match container.inspect().await {
            Ok(_) => {
                println!(
                    "Container {} already exists",
                    &container_name.bright_green()
                );
            }
            Err(_) => {
                println!(
                    "Container {} does not exists",
                    &container_name.bright_green()
                );
                self.build(project.clone()).await?;
                let builder = &self
                    .config
                    .as_ref()
                    .unwrap()
                    .image
                    .as_ref()
                    .unwrap_or(&container_name);
                let options = match self.service.port {
                    Some(port) => ContainerOptions::builder(builder)
                        .name(&container_name)
                        .expose(port, "tcp", port)
                        .build(),
                    None => ContainerOptions::builder(builder)
                        .name(&container_name)
                        .build(),
                };
                self.docker.containers().create(&options).await.unwrap();
                println!("Container {} built", &container_name.bright_green());
            }
        }
        let container = self.docker.containers().get(&container_name);
        container.start().await?;
        let id = container.inspect().await?.id;

        let docker = self.docker.clone();
        let prefix = format!("{} |", self.service.name);
        let prefix = format!("{}", prefix.cyan());
        thread::spawn(move || {
            let logs_stream = docker
                .containers()
                .get(&id)
                .logs(&LogsOptions::builder().stdout(true).stderr(true).build());
            let rt = tokio::runtime::Runtime::new().unwrap();
            rt.block_on(stream_result(logs_stream, &prefix));
        });
        Ok(())
    }

    async fn stop(&self, project: String) -> Result<(), anyhow::Error> {
        todo!()
    }

    async fn restart(&self, project: String) -> Result<(), anyhow::Error> {
        todo!()
    }

    async fn status(&self) -> Result<(), anyhow::Error> {
        todo!()
    }

    async fn logs(&self) -> Result<(), anyhow::Error> {
        todo!()
    }

    async fn exec(&self) -> Result<(), anyhow::Error> {
        todo!()
    }

    async fn build(&self, project: String) -> Result<(), anyhow::Error> {
        if self.config.as_ref().unwrap().image.is_some() {
            println!(
                "Skipping {} build, using image from config",
                self.service.name.bright_green()
            );
            return Ok(());
        }
        println!(
            "Building {}",
            self.service.name.bright_green().bold().underline()
        );

        let image_name = format!("{}_{}", project, self.service.name);
        let mut child = Command::new("docker")
            .arg("build")
            .arg("-t")
            .arg(&image_name)
            .arg(&self.service.working_dir)
            .env("DOCKER_BUILDKIT", "1")
            .stdout(std::process::Stdio::piped())
            .stderr(std::process::Stdio::piped())
            .spawn()?;
        if let Some(stdout) = child.stdout.take() {
            let reader = io::BufReader::new(stdout);

            for line in reader.lines() {
                println!("{:?}", line?);
            }
        }
        if let Some(stderr) = child.stderr.take() {
            let reader = io::BufReader::new(stderr);

            for line in reader.lines() {
                println!("{:?}", line?);
            }
        }

        let status = child.wait()?;
        if !status.success() {
            println!(
                "Error while building {}",
                self.service.name.bright_red().bold().underline()
            );
            return Err(anyhow::anyhow!("Error while building"));
        }
        println!(
            "Successfully built {}",
            self.service.name.bright_green().bold().underline()
        );

        Ok(())
    }
}

async fn stream_result(
    mut stream: impl Stream<Item = Result<tty::TtyChunk, shiplift::Error>> + Unpin,
    prefix: &str,
) {
    while let Some(result) = stream.next().await {
        match result {
            Ok(chunk) => {
                print_chunk(chunk, prefix);
            }
            Err(e) => {
                println!("Error: {}", e);
            }
        }
    }
}

fn print_chunk(chunk: TtyChunk, prefix: &str) {
    match chunk {
        TtyChunk::StdOut(bytes) => println!("{} {}", prefix, std::str::from_utf8(&bytes).unwrap()),
        TtyChunk::StdErr(bytes) => eprintln!("{} {}", prefix, std::str::from_utf8(&bytes).unwrap()),
        TtyChunk::StdIn(_) => unreachable!(),
    }
}