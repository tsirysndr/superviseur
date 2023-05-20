use std::{
    collections::HashMap,
    io::{self, BufRead, Write},
    process::Command,
    sync::{Arc, Mutex},
};

use async_trait::async_trait;
use bollard::{
    container::{
        CreateContainerOptions, ListContainersOptions, LogOutput, LogsOptions,
        RemoveContainerOptions, RestartContainerOptions, StartContainerOptions,
        StopContainerOptions,
    },
    image::ListImagesOptions,
    network::{ConnectNetworkOptions, CreateNetworkOptions, ListNetworksOptions},
    service::EndpointSettings,
    Docker,
};
use owo_colors::OwoColorize;
use tokio::sync::mpsc;
use tokio_stream::StreamExt;

use crate::{
    default_stdout,
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

use super::{connect_network, create_network, hashmap};

#[derive(Clone)]
pub struct Driver {
    docker: Option<Docker>,
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
            docker: None,
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
    ) -> Result<Self, anyhow::Error> {
        let config = service
            .r#use
            .as_ref()
            .unwrap()
            .into_iter()
            .find(|(driver, _)| *driver == "docker")
            .map(|(_, x)| x)
            .unwrap();
        Ok(Self {
            docker: Some(connect()?),
            project,
            context,
            service: service.clone(),
            processes,
            childs,
            event_tx,
            log_engine,
            config: Some(config.clone()),
            superviseur_event,
        })
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

    pub async fn container_exists(&self, container_name: &str) -> Result<bool, anyhow::Error> {
        let docker = self.clone().docker.unwrap();
        let mut filters = HashMap::new();
        filters.insert(String::from("name"), vec![String::from(container_name)]);
        let options = Some(ListContainersOptions::<String> {
            all: true,
            filters,
            ..Default::default()
        });
        let containers = docker.list_containers(options).await?;
        Ok(!containers.is_empty())
    }

    async fn setup_container_network(&self, container_name: &str) -> anyhow::Result<()> {
        let docker = self.clone().docker.unwrap();
        match &self.config {
            Some(cfg) => {
                let mut filters = HashMap::new();
                filters.insert(String::from("name"), vec![String::from(container_name)]);
                let options = Some(ListContainersOptions::<String> {
                    all: true,
                    filters,
                    ..Default::default()
                });
                let containers = docker.list_containers(options).await?;
                let container = containers.first().unwrap().clone();
                let networks = docker
                    .list_networks(None::<ListNetworksOptions<String>>)
                    .await?;
                for network_name in cfg.networks.clone().unwrap_or(Vec::new()) {
                    if let Some(_) = networks
                        .iter()
                        .find(|x| x.name == Some(network_name.clone()))
                    {
                        connect_network!(self.service.name, &network_name, container.id, docker);
                        continue;
                    }

                    create_network!(docker, network_name);
                    connect_network!(self.service.name, &network_name, container.id, docker);
                }

                if cfg.networks.clone().unwrap_or(Vec::new()).len() == 0 {
                    // create a network
                    let project_hash = format!("{:x}", md5::compute(&self.context));
                    let network_name = format!("{}_{}", self.project, project_hash);
                    // verify if network exists
                    let mut filters = HashMap::new();
                    filters.insert(String::from("name"), vec![network_name.clone()]);
                    let networks = docker
                        .list_networks(Some(ListNetworksOptions {
                            filters,
                            ..Default::default()
                        }))
                        .await?;

                    match networks.first() {
                        Some(_) => {
                            // network exists
                            connect_network!(
                                self.service.name,
                                &network_name,
                                container.id,
                                docker
                            );
                        }
                        None => {
                            // network does not exist
                            create_network!(docker, network_name);
                            connect_network!(
                                self.service.name,
                                &network_name,
                                container.id,
                                docker
                            );
                        }
                    }
                }
            }
            None => {}
        };
        Ok(())
    }

    async fn remove_container(&self, name: &str) -> anyhow::Result<()> {
        let options = Some(StopContainerOptions { t: 30 });
        let docker = self.clone().docker.unwrap();
        docker.stop_container(name, options).await?;

        let options = Some(RemoveContainerOptions {
            force: true,
            ..Default::default()
        });
        docker.remove_container(name, options).await?;

        Ok(())
    }

    async fn build_image(&self, project: String) -> anyhow::Result<()> {
        if let Some(img) = &self.config.as_ref().unwrap().image {
            println!(
                "-> Skipping {} build, using image from config",
                self.service.name.bright_green()
            );

            let docker = self.clone().docker.unwrap();
            let mut filters = HashMap::new();
            filters.insert(
                String::from("name"),
                vec![img.clone().split(":").collect::<Vec<_>>()[0].to_string()],
            );
            let options = Some(ListImagesOptions::<String> {
                all: true,
                filters,
                ..Default::default()
            });
            let images = docker.list_images(options).await?;
            if !images.is_empty() {
                println!("-> Image {} already exists", img.bright_green().bold());
                return Ok(());
            }

            let mut child = Command::new("docker")
                .arg("pull")
                .arg(img)
                .stdout(std::process::Stdio::piped())
                .stderr(std::process::Stdio::piped())
                .spawn()?;

            if let Some(stdout) = child.stdout.take() {
                let reader = io::BufReader::new(stdout);

                for line in reader.lines() {
                    println!("{}", line?);
                }
            }

            if let Some(stderr) = child.stderr.take() {
                let reader = io::BufReader::new(stderr);

                for line in reader.lines() {
                    println!("{}", line?);
                }
            }

            if child.wait()?.success() {
                return Ok(());
            }

            return Err(anyhow::anyhow!("Failed to pull image"));
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

    async fn build_container(
        &self,
        _project: String,
        container_name: String,
    ) -> anyhow::Result<()> {
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

        let docker = self.clone().docker.unwrap();

        let options = Some(CreateContainerOptions {
            name: container_name.clone(),
            ..Default::default()
        });

        let config = match self.service.port {
            Some(port) => bollard::container::Config {
                image: Some(builder.to_string()),
                exposed_ports: Some(hashmap! { format!("{}/tcp", port) => hashmap! {} }),
                env: Some(
                    self.service
                        .env
                        .iter()
                        .map(|(key, value)| format!("{}={}", key, value))
                        .collect::<Vec<String>>(),
                ),
                volumes: Some(HashMap::from_iter(
                    volumes
                        .iter()
                        .map(|volume| (volume.clone(), HashMap::new())),
                )),
                ..Default::default()
            },
            None => bollard::container::Config {
                image: Some(builder.to_string()),
                ..Default::default()
            },
        };

        docker.create_container(options, config).await?;

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
        let docker = self.clone().docker.unwrap();

        match self.container_exists(&container_name).await {
            Ok(true) => {
                println!(
                    "Container {} already exists",
                    &container_name.bright_green()
                );
            }
            Ok(false) => {
                println!(
                    "Container {} does not exists",
                    &container_name.bright_green()
                );
                self.build(project.clone()).await?;
            }
            Err(_) => {
                println!(
                    "Container {} does not exists",
                    &container_name.bright_green()
                );
                self.build(project.clone()).await?;
            }
        }

        docker
            .start_container(&container_name, None::<StartContainerOptions<String>>)
            .await?;

        let container = docker.inspect_container(&container_name, None).await?;
        let pid = container.state.unwrap_or_default().pid.unwrap_or_default() as u32;

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

        let service = self.service.clone();
        let log_engine = self.log_engine.clone();
        let project = self.project.clone();
        let superviseur_event = self.superviseur_event.clone();

        tokio::spawn(async move {
            let options = Some(LogsOptions::<String> {
                stdout: true,
                stderr: true,
                timestamps: true,
                follow: true,
                ..Default::default()
            });
            let stream = docker.logs(&container_name, options);
            write_logs(service, log_engine, project, superviseur_event, stream).await;
            Ok::<(), anyhow::Error>(())
        });

        Ok(())
    }

    async fn stop(&self, project: String) -> Result<(), anyhow::Error> {
        let container_name = format!("{}_{}", project, self.service.name);
        let docker = self.clone().docker.unwrap();
        match self.container_exists(&container_name).await {
            Ok(true) => {
                println!("Stopping container {}", &container_name.bright_green());
                docker
                    .stop_container(&container_name, None::<StopContainerOptions>)
                    .await?;
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
            Ok(false) => {
                println!(
                    "Container {} does not exists",
                    &container_name.bright_green()
                );
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
        let docker = self.clone().docker.unwrap();
        match docker
            .restart_container(&container_name, None::<RestartContainerOptions>)
            .await
        {
            Ok(_) => {
                println!("Restarting container {}", &container_name.bright_green());
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
        let docker = self.clone().docker.unwrap();
        if let Ok(container_details) = docker.inspect_container(&container_name, None).await {
            if let Some(id) = container_details.id {
                self.remove_container(&id).await?;
            }
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
    stream: impl futures_util::Stream<Item = std::result::Result<LogOutput, bollard::errors::Error>>,
) {
    let cloned_service = service.clone();

    let service = cloned_service;
    let id = service.id.unwrap_or("-".to_string());
    let mut log_file = std::fs::File::create(
        &service
            .stdout
            .unwrap_or(default_stdout!(project, service.name)),
    )
    .unwrap();

    let mut stream = Box::pin(stream);

    while let Some(result) = stream.next().await {
        match result {
            Ok(bytes) => {
                let line = bytes.to_string();

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
            Err(e) => {
                println!("Error: {}", e);
            }
        }
    }
}

pub fn connect() -> Result<Docker, anyhow::Error> {
    // check if running on macos
    if cfg!(target_os = "macos") {
        // set DOCKER_HOST env variable if not set
        if std::env::var("DOCKER_HOST").is_err() {
            let home = std::env::var("HOME").unwrap();
            std::env::set_var(
                "DOCKER_HOST",
                format!("unix://{}/.docker/run/docker.sock", home),
            );
        }
    }
    Ok(Docker::connect_with_local_defaults()?)
}
