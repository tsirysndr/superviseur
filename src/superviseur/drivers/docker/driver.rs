use std::{
    collections::HashMap,
    io::{self, BufRead, Write},
    process::Command,
    sync::{Arc, Mutex},
    thread,
};

use async_trait::async_trait;
use futures_util::Stream;
use owo_colors::OwoColorize;
use shiplift::{
    rep::ContainerDetails,
    tty::{self, TtyChunk},
    ContainerConnectionOptions, ContainerOptions, Docker, LogsOptions, NetworkCreateOptions,
    PullOptions,
};
use tokio::sync::mpsc;
use tokio_stream::StreamExt;

use crate::{
    graphql::{
        schema::objects::subscriptions::{LogStream, TailLogStream},
        simple_broker::SimpleBroker,
    },
    superviseur::{
        core::ProcessEvent,
        drivers::DriverPlugin,
        logs::{self, Log, LogEngine},
    },
    types::{
        configuration::{DriverConfig, Service},
        events::SuperviseurEvent,
        process::{Process, State},
    },
    util::convert_dir_path_to_absolute_path,
};

#[derive(Clone)]
pub struct Driver {
    docker: Docker,
    project: String,
    context: String,
    service: Service,
    processes: Arc<Mutex<Vec<(Process, String)>>>,
    childs: Arc<Mutex<HashMap<String, i32>>>,
    event_tx: mpsc::UnboundedSender<ProcessEvent>,
    log_engine: Arc<Mutex<logs::LogEngine>>,
    config: Option<DriverConfig>,
    superviseur_event: mpsc::UnboundedSender<SuperviseurEvent>,
}

impl Default for Driver {
    fn default() -> Self {
        let (event_tx, _) = mpsc::unbounded_channel();
        let (superviseur_event, _) = mpsc::unbounded_channel();
        Self {
            docker: Docker::new(),
            project: "".to_string(),
            context: "".to_string(),
            service: Service::default(),
            processes: Arc::new(Mutex::new(Vec::new())),
            childs: Arc::new(Mutex::new(HashMap::new())),
            event_tx,
            log_engine: Arc::new(Mutex::new(logs::LogEngine::new())),
            config: None,
            superviseur_event,
        }
    }
}

impl Driver {
    pub fn new(
        project: String,
        context: String,
        service: &Service,
        processes: Arc<Mutex<Vec<(Process, String)>>>,
        event_tx: mpsc::UnboundedSender<ProcessEvent>,
        childs: Arc<Mutex<HashMap<String, i32>>>,
        log_engine: Arc<Mutex<LogEngine>>,
        superviseur_event: mpsc::UnboundedSender<SuperviseurEvent>,
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
            context,
            service: service.clone(),
            processes,
            childs,
            event_tx,
            log_engine,
            config: Some(config.clone()),
            superviseur_event,
        }
    }

    pub fn verify_docker(&self) -> Result<(), anyhow::Error> {
        let mut cmd = Command::new("docker");
        cmd.arg("--version");
        let output = cmd.output()?;
        if !output.status.success() {
            return Err(anyhow::anyhow!("Docker is not installed on your system"));
        }
        Ok(())
    }

    async fn setup_container_network(&self, container_name: &str) -> anyhow::Result<()> {
        match &self.config {
            Some(cfg) => {
                let container = self.docker.containers().get(container_name);
                let networks = self.docker.networks().list(&Default::default()).await?;
                for network_name in cfg.networks.clone().unwrap_or(Vec::new()) {
                    if let Some(network) = networks.iter().find(|x| x.name == network_name) {
                        self.docker
                            .networks()
                            .get(&network.id)
                            .connect(
                                &ContainerConnectionOptions::builder(container.id())
                                    .aliases(vec![&self.service.name])
                                    .build(),
                            )
                            .await?;
                        continue;
                    }
                    let network = self
                        .docker
                        .networks()
                        .create(&NetworkCreateOptions::builder(&network_name).build())
                        .await?;
                    self.docker
                        .networks()
                        .get(&network.id)
                        .connect(
                            &ContainerConnectionOptions::builder(container.id())
                                .aliases(vec![&self.service.name])
                                .build(),
                        )
                        .await?;
                }
                if cfg.networks.clone().unwrap_or(Vec::new()).len() == 0 {
                    // create a network
                    let project_hash = format!("{:x}", md5::compute(&self.context));
                    let network_name = format!("{}_{}", self.project, project_hash);
                    // verify if network exists
                    match self.docker.networks().get(&network_name).inspect().await {
                        Ok(network) => {
                            // network exists
                            self.docker
                                .networks()
                                .get(&network.id)
                                .connect(
                                    &ContainerConnectionOptions::builder(container.id())
                                        .aliases(vec![&self.service.name])
                                        .build(),
                                )
                                .await?;
                        }
                        Err(_) => {
                            // network does not exist
                            let network = self
                                .docker
                                .networks()
                                .create(&NetworkCreateOptions::builder(&network_name).build())
                                .await?;
                            self.docker
                                .networks()
                                .get(&network.id)
                                .connect(
                                    &ContainerConnectionOptions::builder(container.id())
                                        .aliases(vec![&self.service.name])
                                        .build(),
                                )
                                .await?;
                        }
                    }
                }
            }
            None => {}
        };
        Ok(())
    }

    fn remove_container(&self, container_details: ContainerDetails) -> anyhow::Result<()> {
        let id = container_details.id;
        Command::new("docker")
            .arg("container")
            .arg("stop")
            .arg(&id)
            .stdout(std::process::Stdio::piped())
            .stderr(std::process::Stdio::piped())
            .spawn()?
            .wait()?;

        Command::new("docker")
            .arg("container")
            .arg("rm")
            .arg(&id)
            .stdout(std::process::Stdio::piped())
            .stderr(std::process::Stdio::piped())
            .spawn()?
            .wait()?;
        Ok(())
    }

    async fn build_image(&self, project: String) -> anyhow::Result<()> {
        if let Some(img) = &self.config.as_ref().unwrap().image {
            println!(
                "-> Skipping {} build, using image from config",
                self.service.name.bright_green()
            );
            let mut stream = self
                .docker
                .images()
                .pull(&PullOptions::builder().image(img).build());

            while let Some(pull_result) = stream.next().await {
                match pull_result {
                    Ok(output) => {
                        print!("\r");
                        print!(
                            "{} {} {}",
                            output["id"], output["status"], output["progress"]
                        )
                    }
                    Err(e) => eprintln!("Error: {}", e),
                }
            }

            println!("");

            return Ok(());
        }
        println!(
            "-> Building {}",
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

    fn format_volumes(&self, volumes: Vec<String>) -> anyhow::Result<Vec<String>> {
        let mut result = vec![];
        for volume in volumes.clone() {
            if !volume.starts_with(".") && !volume.starts_with("/") {
                result.push(volume);
                continue;
            }
            if let Some(host_dir) = volume.split(":").next() {
                let host_dir = convert_dir_path_to_absolute_path(host_dir, &self.context)?;
                let container_dir = volume.split(":").last().unwrap();
                result.push(format!("{}:{}", host_dir, container_dir))
            }
        }
        Ok(result)
    }

    async fn build_container(&self, project: String, container_name: String) -> anyhow::Result<()> {
        let builder = &self
            .config
            .as_ref()
            .unwrap()
            .image
            .as_ref()
            .unwrap_or(&container_name);

        let volumes = self.format_volumes(match &self.config {
            Some(cfg) => cfg.volumes.clone().unwrap_or(Vec::new()),
            None => Vec::new(),
        })?;

        let options = match self.service.port {
            Some(port) => ContainerOptions::builder(builder)
                .name(&container_name)
                .env(
                    self.service
                        .env
                        .iter()
                        .map(|(key, value)| format!("{}={}", key, value))
                        .collect::<Vec<String>>(),
                )
                .expose(port, "tcp", port)
                .volumes(volumes.iter().map(|x| x.as_str()).collect())
                .build(),
            None => ContainerOptions::builder(builder)
                .name(&container_name)
                .env(
                    self.service
                        .env
                        .iter()
                        .map(|(key, value)| format!("{}={}", key, value))
                        .collect::<Vec<String>>(),
                )
                .volumes(volumes.iter().map(|x| x.as_str()).collect())
                .build(),
        };
        self.docker.containers().create(&options).await?;
        self.setup_container_network(&container_name).await?;
        println!("Container {} built", container_name.bright_green());
        Ok(())
    }
}

#[async_trait]
impl DriverPlugin for Driver {
    async fn start(&self, project: String) -> Result<(), anyhow::Error> {
        self.verify_docker()?;
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
            }
        }

        let container = self.docker.containers().get(&container_name);
        container.start().await?;
        let pid = container.inspect().await?.state.pid as u32;

        let mut processes = self.processes.lock().unwrap();
        let mut process = &mut processes
            .iter_mut()
            .find(|(p, key)| p.name == self.service.name && key == &project)
            .unwrap()
            .0;
        process.up_time = Some(chrono::Utc::now());
        process.state = State::Running;
        process.pid = Some(pid);
        drop(process);
        drop(processes);
        drop(container);

        self.event_tx
            .send(ProcessEvent::Started(
                self.service.name.clone(),
                project.clone(),
            ))
            .unwrap();
        self.superviseur_event.send(SuperviseurEvent::Started(
            project.clone(),
            self.service.name.clone(),
        ))?;

        let docker = self.docker.clone();
        let service = self.service.clone();
        let log_engine = self.log_engine.clone();
        let project = self.project.clone();
        let superviseur_event = self.superviseur_event.clone();
        thread::spawn(move || {
            let rt = tokio::runtime::Runtime::new().unwrap();
            let container = docker.containers().get(&container_name);
            let id = rt.block_on(container.inspect()).unwrap().id;
            let logs_stream = docker.containers().get(&id).logs(
                &LogsOptions::builder()
                    .stdout(true)
                    .stderr(true)
                    .follow(true)
                    .timestamps(true)
                    .build(),
            );
            rt.block_on(write_logs(
                service,
                log_engine,
                project,
                superviseur_event,
                logs_stream,
            ));
        });
        Ok(())
    }

    async fn stop(&self, project: String) -> Result<(), anyhow::Error> {
        let container_name = format!("{}_{}", project, self.service.name);
        let container = self.docker.containers().get(&container_name);
        match container.inspect().await {
            Ok(_) => {
                println!("Stopping container {}", &container_name.bright_green());
                container.stop(None).await?;
                self.event_tx
                    .send(ProcessEvent::Stopped(
                        self.service.name.clone(),
                        project.clone(),
                    ))
                    .unwrap();
                self.superviseur_event
                    .send(SuperviseurEvent::Stopped(
                        project.clone(),
                        self.service.name.clone(),
                    ))
                    .unwrap();
            }
            Err(_) => {
                println!(
                    "Container {} does not exists",
                    &container_name.bright_green()
                );
            }
        }
        Ok(())
    }

    async fn restart(&self, project: String) -> Result<(), anyhow::Error> {
        let container_name = format!("{}_{}", project, self.service.name);
        let container = self.docker.containers().get(&container_name);
        match container.inspect().await {
            Ok(_) => {
                println!("Restarting container {}", &container_name.bright_green());
                container.restart(None).await?;
                self.superviseur_event.send(SuperviseurEvent::Restarted(
                    project,
                    self.service.name.clone(),
                ))?;
            }
            Err(_) => {
                println!(
                    "Container {} does not exists",
                    &container_name.bright_green()
                );
            }
        }
        Ok(())
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
        self.build_image(project.clone()).await?;

        let container_name = format!("{}_{}", project, self.service.name);
        let container = self.docker.containers().get(&container_name);
        if let Ok(container_details) = container.inspect().await {
            self.remove_container(container_details)?;
        }
        self.build_container(project.clone(), container_name)
            .await?;
        self.superviseur_event
            .send(SuperviseurEvent::Built(project, self.service.name.clone()))?;
        Ok(())
    }
}

pub async fn write_logs(
    service: Service,
    log_engine: Arc<Mutex<LogEngine>>,
    project: String,
    superviseur_event: mpsc::UnboundedSender<SuperviseurEvent>,
    mut stream: impl Stream<Item = Result<tty::TtyChunk, shiplift::Error>> + Unpin,
) {
    let cloned_service = service.clone();

    let service = cloned_service;
    let id = service.id.unwrap_or("-".to_string());
    let mut log_file = std::fs::File::create(&service.stdout).unwrap();
    let mut err_file = std::fs::File::create(&service.stderr).unwrap();

    while let Some(result) = stream.next().await {
        match result {
            Ok(chunk) => match chunk {
                TtyChunk::StdOut(bytes) => {
                    let line = String::from_utf8(bytes).unwrap();

                    let date = line.split(" ").take(1).collect::<String>();
                    let mut line = line.split_whitespace();
                    line.next();
                    let line = line.collect::<Vec<&str>>().join(" ");
                    let line = format!("{}\n", line);

                    let log = Log {
                        project: project.clone(),
                        service: service.name.clone(),
                        line: line.clone(),
                        output: String::from("stdout"),
                        date: tantivy::DateTime::from_timestamp_secs(
                            chrono::DateTime::parse_from_rfc3339(&date)
                                .unwrap()
                                .timestamp(),
                        ),
                    };

                    superviseur_event
                        .send(SuperviseurEvent::Logs(
                            project.clone(),
                            service.name.clone(),
                            line.clone(),
                        ))
                        .unwrap();

                    let log_engine = log_engine.lock().unwrap();
                    match log_engine.insert(&log) {
                        Ok(_) => {}
                        Err(e) => {
                            println!("Error while inserting log: {}", e);
                        }
                    }

                    SimpleBroker::publish(TailLogStream {
                        id: id.clone(),
                        line: line.clone(),
                    });
                    SimpleBroker::publish(LogStream {
                        id: id.clone(),
                        line: line.clone(),
                    });
                    let service_name = format!("{} | ", service.name);
                    print!("{} {}", service_name.cyan(), line.clone());
                    log_file.write_all(line.as_bytes()).unwrap();
                }
                TtyChunk::StdErr(bytes) => {
                    let line = String::from_utf8(bytes).unwrap();

                    let date = line.split(" ").take(1).collect::<String>();
                    let mut line = line.split_whitespace();
                    line.next();
                    let line = line.collect::<Vec<&str>>().join(" ");
                    let line = format!("{}\n", line);

                    let log = Log {
                        project: project.clone(),
                        service: service.name.clone(),
                        line: line.clone(),
                        output: String::from("stderr"),
                        date: tantivy::DateTime::from_timestamp_secs(
                            chrono::DateTime::parse_from_rfc3339(&date)
                                .unwrap()
                                .timestamp(),
                        ),
                    };
                    let log_engine = log_engine.lock().unwrap();
                    match log_engine.insert(&log) {
                        Ok(_) => {}
                        Err(e) => {
                            println!("Error while inserting log: {}", e);
                        }
                    }
                    err_file.write_all(line.as_bytes()).unwrap();
                }
                TtyChunk::StdIn(_) => unreachable!(),
            },
            Err(e) => {
                println!("Error: {}", e);
            }
        }
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
