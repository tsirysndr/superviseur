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
use tokio::sync::mpsc;

use crate::{
    graphql::{
        schema::{
            self,
            objects::subscriptions::{
                AllServicesBuilt, AllServicesRestarted, AllServicesStarted, AllServicesStopped,
                ServiceBuilding, ServiceBuilt, ServiceRestarted, ServiceStarted, ServiceStarting,
                ServiceStopped, ServiceStopping,
            },
        },
        simple_broker::SimpleBroker,
    },
    types::{
        configuration::{ConfigurationData, Service},
        process::{Process, State},
    },
};

use super::{
    dependencies::{DependencyGraph, GraphCommand},
    logs::LogEngine,
    watch::WatchForChanges,
};

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
        log_engine: LogEngine,
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
                log_engine,
            );
            futures::executor::block_on(internal);
        });
        Self {}
    }
}

#[derive(Debug)]
pub enum SuperviseurCommand {
    Load(Service, String),
    Start(Service, String, bool),
    Stop(Service, String),
    Restart(Service, String),
    Build(Service, String),
    LoadConfig(ConfigurationData, String),
    WatchForChanges(String, Service, String),
    StartDependency(Service, String),
    StartAll(String, bool),
    StopAll(String),
    RestartAll(String),
    BuildAll(String),
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
    Building(String, String),
    Built(String, String),
    AllBuilt(String),
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
    log_engine: LogEngine,
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
        log_engine: LogEngine,
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
            log_engine,
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
        let (cmd_tx, cmd_rx) = mpsc::unbounded_channel();
        let mut graph = DependencyGraph::new(
            project.clone(),
            cfg.context.unwrap(),
            cmd_tx,
            Arc::new(Mutex::new(cmd_rx)),
        );
        for (key, mut service) in cfg.services.clone().into_iter() {
            service.name = key.clone();
            services.insert(
                graph.add_vertex(
                    &service,
                    self.processes.clone(),
                    self.childs.clone(),
                    self.event_tx.clone(),
                    self.log_engine.clone(),
                ),
                service.clone(),
            );
            graph
                .cmd_tx
                .send(GraphCommand::AddVertex(
                    service,
                    self.processes.clone(),
                    self.childs.clone(),
                    self.event_tx.clone(),
                    self.log_engine.clone(),
                ))
                .unwrap();
        }

        // Add edges to the graph
        for (service_name, service) in cfg.services.iter() {
            for dep in service.depends_on.iter() {
                let from = services
                    .iter()
                    .find(|(_, s)| s.name == *service_name)
                    .map(|(from, _)| *from)
                    .ok_or(anyhow!("service not found"))?;
                services
                    .iter()
                    .find(|(_, s)| s.name == *dep)
                    .map(|(id, _)| *id)
                    .map(|to| {
                        graph.add_edge(from, to);
                        graph.cmd_tx.send(GraphCommand::AddEdge(from, to)).unwrap();
                    });
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
            process.port = service.port;
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

    fn handle_start(
        &mut self,
        service: Service,
        project: String,
        build: bool,
    ) -> Result<(), Error> {
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
        graph
            .cmd_tx
            .send(GraphCommand::StartService(service, build))
            .unwrap();
        Ok(())
    }

    fn handle_stop(&self, service: Service, project: String) -> Result<(), Error> {
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
        // let mut visited = vec![false; graph.size()];
        // graph.stop_service(&service, &mut visited);
        graph
            .cmd_tx
            .send(GraphCommand::StopService(service))
            .unwrap();
        Ok(())
    }

    fn handle_restart(&mut self, service: Service, project: String) -> Result<(), Error> {
        self.handle_stop(service.clone(), project.clone())?;
        self.handle_start(service.clone(), project.clone(), false)?;
        Ok(())
    }

    fn handle_build(&mut self, service: Service, project: String) -> Result<(), Error> {
        self.event_tx
            .send(ProcessEvent::Building(
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
        graph
            .cmd_tx
            .send(GraphCommand::BuildService(service))
            .unwrap();
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

    fn handle_start_all(&mut self, project: String, build: bool) -> Result<(), Error> {
        let service_graph = self.service_graph.lock().unwrap();
        let graph = service_graph
            .clone()
            .into_iter()
            .filter(|(_, key)| *key == project)
            .map(|(s, _)| s)
            .next()
            .ok_or(anyhow::anyhow!("Project {} not found", project))?;
        graph
            .cmd_tx
            .send(GraphCommand::StartServices(build))
            .unwrap();
        let services = self.get_project_services(&project)?;
        SimpleBroker::publish(AllServicesStarted { payload: services });
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
        graph.cmd_tx.send(GraphCommand::StopServices).unwrap();
        Ok(())
    }

    fn handle_restart_all(&mut self, project: String) -> Result<(), Error> {
        self.handle_stop_all(project.clone())?;
        self.handle_start_all(project, false)?;
        Ok(())
    }

    fn handle_build_all(&mut self, project: String) -> Result<(), Error> {
        let service_graph = self.service_graph.lock().unwrap();
        let graph = service_graph
            .clone()
            .into_iter()
            .filter(|(_, key)| *key == project)
            .map(|(s, _)| s)
            .next()
            .ok_or(anyhow::anyhow!("Project {} not found", project))?;
        graph.cmd_tx.send(GraphCommand::BuildServices).unwrap();
        Ok(())
    }

    fn handle_command(&mut self, cmd: SuperviseurCommand) -> Result<(), Error> {
        match cmd {
            SuperviseurCommand::Load(service, project) => self.handle_load(service, project),
            SuperviseurCommand::Start(service, project, build) => {
                self.handle_start(service, project, build)
            }
            SuperviseurCommand::Stop(service, project) => self.handle_stop(service, project),
            SuperviseurCommand::Restart(service, project) => self.handle_restart(service, project),
            SuperviseurCommand::LoadConfig(config, project) => {
                self.handle_load_config(config, project)
            }
            SuperviseurCommand::WatchForChanges(dir, service, project) => {
                self.handle_watch_for_changes(dir, service, project)
            }
            SuperviseurCommand::StartDependency(_, _) => Ok(()),
            SuperviseurCommand::StartAll(project, build) => self.handle_start_all(project, build),
            SuperviseurCommand::StopAll(project) => self.handle_stop_all(project),
            SuperviseurCommand::RestartAll(project) => self.handle_restart_all(project),
            SuperviseurCommand::Build(service, project) => self.handle_build(service, project),
            SuperviseurCommand::BuildAll(project) => self.handle_build_all(project),
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
            ProcessEvent::Building(service_name, project) => {
                let mut process = &mut processes
                    .iter_mut()
                    .find(|(p, key)| p.name == service_name && key == &project)
                    .unwrap()
                    .0;
                process.state = State::Building;
                // call SimpleBroker::publish
                let service = self.get_service(&service_name, &project)?;
                let mut service = schema::objects::service::Service::from(&service);
                service.status = String::from("BUILDING");
                SimpleBroker::publish(ServiceBuilding {
                    payload: service.clone(),
                    process: process.into(),
                });
            }
            ProcessEvent::Built(service_name, project) => {
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
                SimpleBroker::publish(ServiceBuilt {
                    payload: service.clone(),
                    process: process.into(),
                });
            }
            ProcessEvent::AllBuilt(project) => {
                // call SimpleBroker::publish
                let services = self.get_project_services(&project)?;
                SimpleBroker::publish(AllServicesBuilt { payload: services });
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
        let (_, service) = config
            .services
            .iter()
            .find(|(_, s)| s.name == service_name)
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
            .map(|(_, s)| schema::objects::service::Service::from(s))
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
