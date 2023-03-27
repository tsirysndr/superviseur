use std::{
    collections::HashMap,
    pin::Pin,
    sync::{Arc, Mutex},
    task::{Context, Poll},
    thread,
    time::Duration,
};

use anyhow::{anyhow, Error, Ok};
use futures::Future;
use nix::{
    sys::signal::{self, Signal},
    unistd::Pid,
};
use tokio::sync::mpsc;

use crate::{
    graphql::{
        schema::{
            self,
            objects::subscriptions::{
                AllServicesRestarted, AllServicesStarted, AllServicesStopped, ServiceRestarted,
                ServiceStarted, ServiceStarting, ServiceStopped, ServiceStopping,
            },
        },
        simple_broker::SimpleBroker,
    },
    types::{
        configuration::{ConfigurationData, Service},
        process::{Process, State},
    },
};

use super::{dependencies::DependencyGraph, wait::wait_for_service, watch::WatchForChanges};

#[derive(Clone)]
pub struct Superviseur {}

impl Superviseur {
    pub fn new(
        cmd_rx: Arc<Mutex<mpsc::UnboundedReceiver<SuperviseurCommand>>>,
        cmd_tx: mpsc::UnboundedSender<SuperviseurCommand>,
        event_tx: mpsc::UnboundedSender<ProcessEvent>,
        events: mpsc::UnboundedReceiver<ProcessEvent>,
        processes: Arc<Mutex<Vec<(Process, String)>>>,
        config_map: Arc<Mutex<HashMap<String, ConfigurationData>>>,
        service_graph: Arc<Mutex<Vec<(DependencyGraph, String)>>>,
        service_map: Arc<Mutex<Vec<(HashMap<usize, Service>, String)>>>,
    ) -> Self {
        let childs = Arc::new(Mutex::new(HashMap::new()));
        thread::spawn(move || {
            let internal = SuperviseurInternal::new(
                cmd_rx,
                cmd_tx,
                event_tx,
                events,
                processes,
                childs,
                config_map,
                service_graph,
                service_map,
            );
            futures::executor::block_on(internal);
        });
        Self {}
    }
}

#[derive(Debug)]
pub enum SuperviseurCommand {
    Load(Service, String),
    Start(Service, String),
    Stop(Service, String),
    Restart(Service, String),
    LoadConfig(ConfigurationData, String),
    WatchForChanges(String, Service, String),
    StartDependency(Service, String),
    StartAll(String),
    StopAll(String),
    RestartAll(String),
}

#[derive(Debug)]
pub enum ProcessEvent {
    Starting(String, String),
    Stopping(String, String),
    Started(String, String),
    Stopped(String, String),
    Restarted(String, String),
    AllStarted(String),
    AllStopped(String),
    AllRestarted(String),
}

struct SuperviseurInternal {
    commands: Arc<Mutex<mpsc::UnboundedReceiver<SuperviseurCommand>>>,
    cmd_tx: mpsc::UnboundedSender<SuperviseurCommand>,
    events: mpsc::UnboundedReceiver<ProcessEvent>,
    event_tx: mpsc::UnboundedSender<ProcessEvent>,
    processes: Arc<Mutex<Vec<(Process, String)>>>,
    childs: Arc<Mutex<HashMap<String, i32>>>,
    config_map: Arc<Mutex<Vec<(ConfigurationData, String)>>>,
    service_graph: Arc<Mutex<Vec<(DependencyGraph, String)>>>,
    service_map: Arc<Mutex<Vec<(HashMap<usize, Service>, String)>>>,
}

impl SuperviseurInternal {
    pub fn new(
        commands: Arc<Mutex<mpsc::UnboundedReceiver<SuperviseurCommand>>>,
        cmd_tx: mpsc::UnboundedSender<SuperviseurCommand>,
        event_tx: mpsc::UnboundedSender<ProcessEvent>,
        events: mpsc::UnboundedReceiver<ProcessEvent>,
        processes: Arc<Mutex<Vec<(Process, String)>>>,
        childs: Arc<Mutex<HashMap<String, i32>>>,
        config_map: Arc<Mutex<HashMap<String, ConfigurationData>>>,
        service_graph: Arc<Mutex<Vec<(DependencyGraph, String)>>>,
        service_map: Arc<Mutex<Vec<(HashMap<usize, Service>, String)>>>,
    ) -> Self {
        let config_map = Arc::new(Mutex::new(
            config_map
                .lock()
                .unwrap()
                .iter()
                .map(|(_, v)| (v.clone(), v.project.clone()))
                .collect(),
        ));
        Self {
            commands,
            events,
            event_tx,
            cmd_tx,
            processes,
            childs,
            config_map,
            service_graph,
            service_map,
        }
    }

    pub fn handle_load_config(
        &mut self,
        cfg: ConfigurationData,
        project: String,
    ) -> Result<(), Error> {
        let mut config_map = self.config_map.lock().unwrap();
        config_map.retain(|(_, key)| *key != project);
        config_map.push((cfg.clone(), project.clone()));

        let mut services = HashMap::new();
        let mut graph = DependencyGraph::new(project.clone());
        for service in cfg.services.iter() {
            services.insert(
                graph.add_vertex(
                    service,
                    self.processes.clone(),
                    self.childs.clone(),
                    self.event_tx.clone(),
                ),
                service.clone(),
            );
        }

        // Add edges to the graph
        for service in cfg.services.iter() {
            for dep in service.depends_on.iter() {
                let from = services
                    .iter()
                    .find(|(_, s)| s.name == service.name)
                    .map(|(from, _)| *from)
                    .ok_or(anyhow!("service not found"))?;
                services
                    .iter()
                    .find(|(_, s)| s.name == *dep)
                    .map(|(id, _)| *id)
                    .map(|to| graph.add_edge(from, to));
            }
        }

        let mut service_graph = self.service_graph.lock().unwrap();
        service_graph.retain(|(_, key)| *key != project);
        service_graph.push((graph, project.clone()));

        let mut service_map = self.service_map.lock().unwrap();
        service_map.retain(|(_, key)| *key != project);
        service_map.push((services, project.clone()));

        Ok(())
    }

    fn handle_load(&self, service: Service, project: String) -> Result<(), Error> {
        let mut processes = self.processes.lock().unwrap();

        // update and skip if already loaded
        if let Some(process) = processes
            .iter_mut()
            .find(|(p, key)| p.name == service.name && key == &project)
            .map(|(p, _)| p)
        {
            process.service_id = service.id.unwrap_or("-".to_string());
            process.name = service.name;
            process.command = service.command;
            process.description = service.description;
            process.working_dir = service.working_dir;
            process.env = service.env;
            process.project = project.clone();
            process.r#type = service.r#type;
            process.auto_restart = service.autorestart;
            process.stdout = service.stdout;
            process.stderr = service.stderr;
            return Ok(());
        }

        processes.push((
            Process {
                service_id: service.id.unwrap_or("-".to_string()),
                name: service.name,
                command: service.command,
                description: service.description,
                pid: None,
                uid: None,
                gid: None,
                working_dir: service.working_dir,
                state: State::Stopped,
                cpu: None,
                mem: None,
                up_time: None,
                port: None,
                env: service.env,
                project: project.clone(),
                r#type: service.r#type,
                auto_restart: service.autorestart,
                stdout: service.stdout,
                stderr: service.stderr,
            },
            project,
        ));
        Ok(())
    }

    fn handle_start(&mut self, service: Service, project: String) -> Result<(), Error> {
        /*
        println!("starting {}", service.clone().name);
          self.event_tx
              .send(ProcessEvent::Starting(
                  service.name.clone(),
                  project.clone(),
              ))
              .unwrap();

          self.start_dependencies(service.clone(), project.clone())?;
          self.wait_for_service_deps(service.clone(), project.clone())?;

          // skip if already started
          let mut processes = self.processes.lock().unwrap();
          if let Some(process) = processes
              .iter()
              .find(|(p, key)| p.name == service.name && key == &project)
              .map(|(p, _)| p)
          {
              if process.state == State::Running || process.state == State::Starting {
                  return Ok(());
              }
          }

          let envs = service.env.clone();
          let working_dir = service.working_dir.clone();

          let mut child = match service.clone().flox {
              Some(flox) => {
                  // verify if flox is installed
                  std::process::Command::new("sh")
                      .arg("-c")
                      .arg("flox --version")
                      .stdout(std::process::Stdio::piped())
                      .stderr(std::process::Stdio::piped())
                      .spawn()
                      .expect("flox is not installed, see https://floxdev.com/docs/");

                  let command = format!(
                      "flox print-dev-env -A {}",
                      flox.environment.replace(".#", "")
                  );
                  let mut child = std::process::Command::new("sh")
                      .arg("-c")
                      .arg(command)
                      .stdout(std::process::Stdio::piped())
                      .stderr(std::process::Stdio::piped())
                      .spawn()
                      .unwrap();
                  child.wait().unwrap();

                  let command = format!(
                      "flox activate -e {} -- {}",
                      flox.environment, &service.command
                  );
                  println!("command: {}", command);
                  std::process::Command::new("sh")
                      .arg("-c")
                      .arg(command)
                      .current_dir(working_dir)
                      .envs(envs)
                      .stdout(std::process::Stdio::piped())
                      .stderr(std::process::Stdio::piped())
                      .spawn()
                      .unwrap()
              }
              None => std::process::Command::new("sh")
                  .arg("-c")
                  .arg(&service.command)
                  .current_dir(working_dir)
                  .envs(envs)
                  .stdout(std::process::Stdio::piped())
                  .stderr(std::process::Stdio::piped())
                  .spawn()
                  .unwrap(),
          };

          let mut process = &mut processes
              .iter_mut()
              .find(|(p, key)| p.name == service.name && key == &project)
              .unwrap()
              .0;
          process.pid = Some(child.id());
          self.event_tx
              .send(ProcessEvent::Started(service.name.clone(), project.clone()))
              .unwrap();
          println!("started {}", service.clone().name);

          process.up_time = Some(chrono::Utc::now());
          let service_key = format!("{}-{}", project, service.name);
          self.childs
              .lock()
              .unwrap()
              .insert(service_key, process.pid.unwrap() as i32);

          let stdout = child.stdout.take().unwrap();
          let stderr = child.stderr.take().unwrap();

          let cloned_service = service.clone();

          thread::spawn(move || {
              let service = cloned_service;
              let id = service.id.unwrap_or("-".to_string());
              // write stdout to file
              let mut log_file = std::fs::File::create(service.stdout).unwrap();

              let stdout = std::io::BufReader::new(stdout);
              for line in stdout.lines() {
                  let line = line.unwrap();
                  let line = format!("{}\n", line);
                  SimpleBroker::publish(TailLogStream {
                      id: id.clone(),
                      line: line.clone(),
                  });
                  SimpleBroker::publish(LogStream {
                      id: id.clone(),
                      line: line.clone(),
                  });
                  log_file.write_all(line.as_bytes()).unwrap();
              }

              // write stderr to file
              let mut err_file = std::fs::File::create(service.stderr).unwrap();
              let stderr = std::io::BufReader::new(stderr);
              for line in stderr.lines() {
                  let line = line.unwrap();
                  err_file.write_all(line.as_bytes()).unwrap();
              }
          });

          let cmd_tx = self.cmd_tx.clone();
          let event_tx = self.event_tx.clone();
          thread::spawn(move || {
              let _status = child.wait().unwrap();
              // println!("child exited with status: {}", status);
              if service.autorestart {
                  cmd_tx
                      .send(SuperviseurCommand::Start(service.clone(), project.clone()))
                      .unwrap();

                  event_tx
                      .send(ProcessEvent::Restarted(service.name, project))
                      .unwrap();
                  return;
              }
              event_tx
                  .send(ProcessEvent::Stopped(service.name, project))
                  .unwrap();
          });
         */

        self.event_tx
            .send(ProcessEvent::Starting(
                service.name.clone(),
                project.clone(),
            ))
            .unwrap();

        let service_graph = self.service_graph.lock().unwrap();
        let graph = service_graph
            .clone()
            .into_iter()
            .filter(|(_, key)| *key == project)
            .map(|(s, _)| s)
            .next()
            .ok_or(anyhow::anyhow!("Project {} not found", project))?;
        let mut visited = vec![false; graph.size()];
        graph.start_service(&service, &mut visited);
        Ok(())
    }

    fn handle_stop(&self, service: Service, project: String) -> Result<(), Error> {
        /*
        self.event_tx
             .send(ProcessEvent::Stopping(
                 service.name.clone(),
                 project.clone(),
             ))
             .unwrap();
         println!(
             "service: {} | stop_command: {:?}",
             service.name, service.stop_command
         );
         if let Some(stop_command) = service.stop_command.clone() {
             let envs = service.env.clone();
             let working_dir = service.working_dir.clone();

             match service.clone().flox {
                 Some(flox) => {
                     let stop_command =
                         format!("flox activate -e {} -- {}", flox.environment, stop_command);
                     let mut child = std::process::Command::new("sh")
                         .arg("-c")
                         .arg(stop_command)
                         .current_dir(working_dir)
                         .envs(envs)
                         .stdout(std::process::Stdio::piped())
                         .stderr(std::process::Stdio::piped())
                         .spawn()
                         .unwrap();
                     child.wait().unwrap();
                 }
                 None => {
                     let mut child = std::process::Command::new("sh")
                         .arg("-c")
                         .arg(stop_command)
                         .current_dir(working_dir)
                         .envs(envs)
                         .stdout(std::process::Stdio::piped())
                         .stderr(std::process::Stdio::piped())
                         .spawn()
                         .unwrap();
                     child.wait().unwrap();
                 }
             };
             let mut childs = self.childs.lock().unwrap();
             let service_key = format!("{}-{}", project.clone(), service.name.clone());
             childs.remove(&service_key);

             self.event_tx
                 .send(ProcessEvent::Stopped(service.name.clone(), project.clone()))
                 .unwrap();
             if restart {
                 self.cmd_tx
                     .send(SuperviseurCommand::Start(service, project))
                     .unwrap();
             }
             return Ok(());
         }
         let mut childs = self.childs.lock().unwrap();
         let service_key = format!("{}-{}", project.clone(), service.name.clone());
         match childs.get(&service_key) {
             Some(pid) => {
                 println!("Stopping service {} (pid: {})", service.name.clone(), pid);
                 signal::kill(Pid::from_raw(*pid), Signal::SIGTERM)?;
                 childs.remove(&service_key);

                 self.event_tx
                     .send(ProcessEvent::Stopped(service.name.clone(), project.clone()))
                     .unwrap();
                 if restart {
                     self.cmd_tx
                         .send(SuperviseurCommand::Start(service.clone(), project))
                         .unwrap();
                 }
                 println!("Service {} stopped", service.name);
                 Ok(())
             }
             None => Ok(()),
         }
         */
        self.event_tx
            .send(ProcessEvent::Stopping(
                service.name.clone(),
                project.clone(),
            ))
            .unwrap();

        let service_graph = self.service_graph.lock().unwrap();
        let graph = service_graph
            .clone()
            .into_iter()
            .filter(|(_, key)| *key == project)
            .map(|(s, _)| s)
            .next()
            .ok_or(anyhow::anyhow!("Project {} not found", project))?;
        let mut visited = vec![false; graph.size()];
        graph.stop_service(&service, &mut visited);
        Ok(())
    }

    fn handle_restart(&mut self, service: Service, project: String) -> Result<(), Error> {
        // self.handle_stop(service.clone(), project.clone(), true)
        self.handle_stop(service.clone(), project.clone())?;
        self.handle_start(service.clone(), project.clone())?;
        Ok(())
    }

    fn handle_watch_for_changes(
        &mut self,
        dir: String,
        service: Service,
        project: String,
    ) -> Result<(), Error> {
        let superviseur_tx = self.cmd_tx.clone();
        thread::spawn(move || {
            let _watcher = WatchForChanges::new(dir, superviseur_tx, service, project.clone());
            loop {
                thread::sleep(Duration::from_secs(5));
            }
        });
        Ok(())
    }

    fn handle_start_all(&mut self, project: String) -> Result<(), Error> {
        let service_graph = self.service_graph.lock().unwrap();
        let graph = service_graph
            .clone()
            .into_iter()
            .filter(|(_, key)| *key == project)
            .map(|(s, _)| s)
            .next()
            .ok_or(anyhow::anyhow!("Project {} not found", project))?;
        graph.start_services();
        Ok(())
    }

    fn handle_stop_all(&mut self, project: String) -> Result<(), Error> {
        let service_graph = self.service_graph.lock().unwrap();
        let graph = service_graph
            .clone()
            .into_iter()
            .filter(|(_, key)| *key == project)
            .map(|(s, _)| s)
            .next()
            .ok_or(anyhow::anyhow!("Project {} not found", project))?;
        graph.stop_services();
        Ok(())
    }

    fn handle_restart_all(&mut self, project: String) -> Result<(), Error> {
        self.handle_stop_all(project.clone())?;
        self.handle_start_all(project)?;
        Ok(())
    }

    fn handle_command(&mut self, cmd: SuperviseurCommand) -> Result<(), Error> {
        match cmd {
            SuperviseurCommand::Load(service, project) => self.handle_load(service, project),
            SuperviseurCommand::Start(service, project) => self.handle_start(service, project),
            SuperviseurCommand::Stop(service, project) => self.handle_stop(service, project),
            SuperviseurCommand::Restart(service, project) => self.handle_restart(service, project),
            SuperviseurCommand::LoadConfig(config, project) => {
                self.handle_load_config(config, project)
            }
            SuperviseurCommand::WatchForChanges(dir, service, project) => {
                self.handle_watch_for_changes(dir, service, project)
            }
            SuperviseurCommand::StartDependency(_, _) => Ok(()),
            SuperviseurCommand::StartAll(project) => self.handle_start_all(project),
            SuperviseurCommand::StopAll(project) => self.handle_stop_all(project),
            SuperviseurCommand::RestartAll(project) => self.handle_restart_all(project),
        }
    }

    fn handle_event(&mut self, event: ProcessEvent) -> Result<(), Error> {
        let mut processes = self.processes.lock().unwrap();
        match event {
            ProcessEvent::Started(service_name, project) => {
                let mut process = &mut processes
                    .iter_mut()
                    .find(|(p, key)| p.name == service_name && key == &project)
                    .unwrap()
                    .0;
                process.state = State::Running;

                // call SimpleBroker::publish
                let service = self.get_service(&service_name, &project)?;
                let mut service = schema::objects::service::Service::from(&service);
                service.status = String::from("RUNNING");
                SimpleBroker::publish(ServiceStarted {
                    payload: service.clone(),
                    process: process.into(),
                });
            }
            ProcessEvent::Stopped(service_name, project) => {
                let mut process = &mut processes
                    .iter_mut()
                    .find(|(p, key)| p.name == service_name && key == &project)
                    .unwrap()
                    .0;
                process.state = State::Stopped;

                // call SimpleBroker::publish
                let service = self.get_service(&service_name, &project)?;
                let mut service = schema::objects::service::Service::from(&service);
                service.status = String::from("STOPPED");
                SimpleBroker::publish(ServiceStopped {
                    payload: service.clone(),
                    process: process.into(),
                });
            }
            ProcessEvent::Restarted(service_name, project) => {
                let mut process = &mut processes
                    .iter_mut()
                    .find(|(p, key)| p.name == service_name && key == &project)
                    .unwrap()
                    .0;
                process.state = State::Running;

                // call SimpleBroker::publish
                let service = self.get_service(&service_name, &project)?;
                let mut service = schema::objects::service::Service::from(&service);
                service.status = String::from("RUNNING");
                SimpleBroker::publish(ServiceRestarted {
                    payload: service.clone(),
                    process: process.into(),
                });
            }
            ProcessEvent::AllStarted(project) => {
                // call SimpleBroker::publish
                let services = self.get_project_services(&project)?;
                SimpleBroker::publish(AllServicesStarted { payload: services });
            }
            ProcessEvent::AllRestarted(project) => {
                // call SimpleBroker::publish
                let services = self.get_project_services(&project)?;
                SimpleBroker::publish(AllServicesRestarted { payload: services });
            }
            ProcessEvent::AllStopped(project) => {
                // call SimpleBroker::publish
                let services = self.get_project_services(&project)?;
                SimpleBroker::publish(AllServicesStopped { payload: services });
            }
            ProcessEvent::Starting(service_name, project) => {
                let mut process = &mut processes
                    .iter_mut()
                    .find(|(p, key)| p.name == service_name && key == &project)
                    .unwrap()
                    .0;
                process.state = State::Starting;
                // call SimpleBroker::publish
                let service = self.get_service(&service_name, &project)?;
                let mut service = schema::objects::service::Service::from(&service);
                service.status = String::from("STARTING");
                SimpleBroker::publish(ServiceStarting {
                    payload: service.clone(),
                    process: process.into(),
                });
            }
            ProcessEvent::Stopping(service_name, project) => {
                let mut process = &mut processes
                    .iter_mut()
                    .find(|(p, key)| p.name == service_name && key == &project)
                    .unwrap()
                    .0;
                process.state = State::Stopping;
                // call SimpleBroker::publish
                let service = self.get_service(&service_name, &project)?;
                let mut service = schema::objects::service::Service::from(&service);
                service.status = String::from("STOPPING");
                SimpleBroker::publish(ServiceStopping {
                    payload: service.clone(),
                    process: process.into(),
                });
            }
        }
        Ok(())
    }

    fn get_service(&self, service_name: &str, project: &str) -> Result<Service, Error> {
        let config_map = self.config_map.lock().unwrap();
        let config = config_map
            .iter()
            .find(|(_, k)| k == &project)
            .map(|(c, _)| c)
            .ok_or(anyhow::anyhow!("Config not found"))?;
        let service = config
            .services
            .iter()
            .find(|s| s.name == service_name)
            .ok_or(anyhow::anyhow!("Service not found"))?;
        Ok(service.clone())
    }

    fn get_project_services(
        &self,
        project: &str,
    ) -> Result<Vec<schema::objects::service::Service>, Error> {
        let config_map = self.config_map.lock().unwrap();
        let config = config_map
            .iter()
            .find(|(_, k)| k == &project)
            .map(|(c, _)| c)
            .ok_or(anyhow::anyhow!("Config not found"))?;
        let services = config
            .services
            .iter()
            .map(|s| schema::objects::service::Service::from(s))
            .collect();
        Ok(services)
    }
}

impl Future for SuperviseurInternal {
    type Output = ();

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        loop {
            let cmd = match self.commands.lock().unwrap().poll_recv(cx) {
                Poll::Ready(Some(cmd)) => Some(cmd),
                Poll::Ready(None) => return Poll::Ready(()), // client has disconnected - shut down.
                _ => None,
            };

            if let Some(cmd) = cmd {
                if let Err(e) = self.handle_command(cmd) {
                    println!("{:?}", e);
                }
            }

            let event = match self.events.poll_recv(cx) {
                Poll::Ready(Some(event)) => Some(event),
                Poll::Ready(None) => return Poll::Ready(()), // client has disconnected - shut down.
                _ => None,
            };

            if let Some(event) = event {
                if let Err(e) = self.handle_event(event) {
                    println!("{:?}", e);
                }
            }

            thread::sleep(Duration::from_millis(500));
        }
    }
}
