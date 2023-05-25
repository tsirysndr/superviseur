use std::{
    collections::HashMap,
    io::Write,
    sync::{Arc, Mutex},
    thread,
    time::Duration,
};

use async_graphql::{async_stream::stream, Context, Error, Object, Subscription, ID};
use futures_util::Stream;
use indexmap::IndexMap;
use names::Generator;
use superviseur_code::start_code_tunnel;
use superviseur_provider::kv::kv::Provider;
use tokio::sync::mpsc;
use tokio_stream::StreamExt;
use types::PROJECTS_DIR;

use crate::{
    macros::{project_exists, send_event, send_event_alt},
    schema::objects::subscriptions::ServiceStarted,
    simple_broker::SimpleBroker,
};
use superviseur_types::{
    self as types, command::SuperviseurCommand, configuration::ConfigurationData, process::State,
};

use super::objects::{
    process::Process,
    project::Project,
    project_configuration::ProjectConfiguration,
    service::Service,
    subscriptions::{
        AllServicesRestarted, AllServicesStarted, AllServicesStopped, ProjectOpened,
        ServiceRestarted, ServiceStarting, ServiceStopped, ServiceStopping,
    },
};

#[derive(Default, Clone)]
pub struct ControlQuery;

#[Object]
impl ControlQuery {
    async fn status(&self, ctx: &Context<'_>, id: ID) -> Result<Process, Error> {
        let processes = ctx
            .data::<Arc<Mutex<Vec<(types::process::Process, String)>>>>()
            .unwrap();

        let processes = processes.lock().unwrap();

        let process = processes
            .iter()
            .find(|(p, _)| p.service_id.clone() == id.to_string())
            .map(|(p, _)| p.clone());

        match process {
            Some(p) => Ok(Process::from(p)),
            None => Err(Error::new("Process not found")),
        }
    }

    async fn projects(&self, ctx: &Context<'_>) -> Result<Vec<Project>, Error> {
        let project_map = ctx.data::<Arc<Mutex<HashMap<String, String>>>>().unwrap();
        let provider = ctx.data::<Arc<Provider>>().unwrap();

        let project_map = project_map.lock().unwrap();

        let projects = provider
            .all_projects()
            .map_err(|e| Error::new(e.to_string()))?;
        let projects = projects
            .iter()
            .map(|(id, name, _)| {
                let config_path = match project_map.clone().into_iter().find(|(_, v)| v == id) {
                    Some((k, _)) => Some(k),
                    None => None,
                };

                Project {
                    id: ID(id.clone()),
                    name: name.clone(),
                    config_path,
                }
            })
            .collect();
        Ok(projects)
    }

    async fn services(&self, ctx: &Context<'_>, project_id: ID) -> Result<Vec<Service>, Error> {
        let processes = ctx
            .data::<Arc<Mutex<Vec<(types::process::Process, String)>>>>()
            .unwrap();
        let provider = ctx.data::<Arc<Provider>>().unwrap();

        let processes = processes.lock().unwrap();

        let project_id = project_id.to_string();

        project_exists!(provider, project_id);

        let config = provider.build_configuration(&project_id)?;

        let services = config.services.clone();
        let mut services = services
            .iter()
            .map(|(_, x)| Service::from(x))
            .collect::<Vec<Service>>();

        for service in services.iter_mut() {
            let process = processes
                .iter()
                .find(|(p, _)| p.name == service.name)
                .map(|(p, _)| p);
            if let Some(process) = process {
                service.status = process.state.to_string().to_uppercase();
            } else {
                service.status = "stopped".to_string();
            }
        }

        Ok(services)
    }

    async fn project(&self, ctx: &Context<'_>, id: ID) -> Result<Project, Error> {
        let project_map = ctx.data::<Arc<Mutex<HashMap<String, String>>>>().unwrap();
        let provider = ctx.data::<Arc<Provider>>().unwrap();

        let project_map = project_map.lock().unwrap();

        let project_id = id.to_string();

        project_exists!(provider, project_id);

        let config = provider.build_configuration(&project_id)?;

        let config_path = match project_map
            .clone()
            .into_iter()
            .find(|(_, v)| v == &project_id)
        {
            Some((k, _)) => Some(k),
            None => None,
        };

        Ok(Project {
            id: ID(project_id),
            name: config.project.clone(),
            config_path,
        })
    }

    async fn processes(&self, ctx: &Context<'_>) -> Result<Vec<Process>, Error> {
        let processes = ctx
            .data::<Arc<Mutex<Vec<(types::process::Process, String)>>>>()
            .unwrap();

        let processes = processes.lock().unwrap();
        Ok(processes
            .iter()
            .filter(|(p, _)| p.state != State::Stopped)
            .map(|(p, _)| Process::from(p.clone()))
            .collect())
    }

    async fn service(&self, ctx: &Context<'_>, id: ID, project_id: ID) -> Result<Service, Error> {
        let project_id = project_id.to_string();
        let processes = ctx
            .data::<Arc<Mutex<Vec<(types::process::Process, String)>>>>()
            .unwrap();
        let provider = ctx.data::<Arc<Provider>>().unwrap();

        project_exists!(provider, project_id);

        let config = provider.build_configuration(&project_id)?;

        let processes = processes.lock().unwrap();

        match processes
            .iter()
            .find(|(p, _)| p.service_id.clone() == id.to_string())
        {
            Some((process, _)) => {
                let (_, service) = config
                    .services
                    .iter()
                    .find(|(_, s)| s.id == Some(id.to_string()))
                    .ok_or(Error::new("Service not found"))?;

                Ok(Service {
                    status: process.state.to_string(),
                    ..Service::from(service)
                })
            }
            None => {
                let (_, service) = config
                    .services
                    .iter()
                    .find(|(_, s)| s.id == Some(id.to_string()))
                    .ok_or(Error::new("Service not found"))?;

                Ok(Service {
                    status: "stopped".to_string(),
                    ..Service::from(service)
                })
            }
        }
    }
}

#[derive(Default, Clone)]
pub struct ControlMutation;

#[Object]
impl ControlMutation {
    async fn new_project(
        &self,
        ctx: &Context<'_>,
        name: String,
        context: Option<String>,
    ) -> Result<ProjectConfiguration, Error> {
        let provider = ctx.data::<Arc<Provider>>().unwrap();

        let mut generator = Generator::default();
        let id = generator.next().unwrap();

        let project_dir = match context.clone() {
            Some(c) => Some(c),
            None => {
                let dir = format!(
                    "{}/{}/{}",
                    dirs::home_dir().unwrap().to_str().unwrap(),
                    PROJECTS_DIR,
                    id
                );
                std::fs::create_dir_all(&dir).unwrap();
                Some(dir)
            }
        };

        let config = ConfigurationData {
            project: name.clone(),
            services: IndexMap::new(),
            context: project_dir.clone(),
            network_settings: None,
            volume_settings: None,
        };

        if context.is_none() {
            let mut config_file =
                std::fs::File::create(format!("{}/Superfile.hcl", project_dir.clone().unwrap()))?;
            let config = hcl::to_string(&config)?;
            config_file.write_all(config.as_bytes())?;
        }

        let projects = provider
            .all_projects()
            .map_err(|e| Error::new(e.to_string()))?;
        if projects.into_iter().any(|(_, _, ctx)| Some(ctx) == context) {
            return Err(Error::new("Project already exists with this context"));
        }

        provider.save_configuration(&id, config)?;

        return Ok(ProjectConfiguration {
            id: ID(id),
            name,
            context: project_dir.unwrap(),
            ..Default::default()
        });
    }

    async fn delete_project(&self, ctx: &Context<'_>, id: ID) -> Result<Project, Error> {
        let project_id = id.to_string();
        let cmd_tx = ctx
            .data::<mpsc::UnboundedSender<SuperviseurCommand>>()
            .unwrap();
        let provider = ctx.data::<Arc<Provider>>().unwrap();
        let project_map = ctx.data::<Arc<Mutex<HashMap<String, String>>>>().unwrap();

        project_exists!(provider, project_id);

        let config = provider
            .build_configuration(&project_id)
            .map_err(|e| Error::new(e.to_string()))?;

        send_event!(
            config.project,
            config.services,
            cmd_tx,
            Stop,
            AllServicesStopped
        );

        let project_map = project_map.lock().unwrap();
        let config_path = match project_map
            .clone()
            .into_iter()
            .find(|(_, v)| v == &project_id)
        {
            Some((k, _)) => Some(k),
            None => None,
        };

        provider.delete_configuration(&project_id)?;

        Ok(Project {
            id: ID(project_id),
            config_path,
            ..Default::default()
        })
    }

    async fn open_project(&self, ctx: &Context<'_>, id: ID) -> Result<Project, Error> {
        let project_id = id.to_string();
        let provider = ctx.data::<Arc<Provider>>().unwrap();
        let project_map = ctx.data::<Arc<Mutex<HashMap<String, String>>>>().unwrap();

        project_exists!(provider, project_id);

        let (_, context) = provider.project(&project_id)?;

        let config = provider
            .build_configuration(&project_id)
            .map_err(|e| Error::new(e.to_string()))?;

        let project_map = project_map.lock().unwrap();
        let config_path = match project_map
            .clone()
            .into_iter()
            .find(|(_, v)| v == &project_id)
        {
            Some((k, _)) => Some(k),
            None => None,
        };

        let (sender, mut receiver) = tokio::sync::mpsc::unbounded_channel();
        let id = project_id.clone();

        tokio::spawn(async move {
            loop {
                match receiver.recv().await {
                    Some(line) => {
                        println!("{}", line);
                        SimpleBroker::publish(ProjectOpened {
                            id: id.clone(),
                            line,
                        });
                    }
                    None => {
                        break;
                    }
                }
            }
        });

        start_code_tunnel(sender, &context)?;

        Ok(Project {
            id: ID(project_id),
            config_path,
            name: config.project,
            ..Default::default()
        })
    }

    async fn start(
        &self,
        ctx: &Context<'_>,
        id: Option<ID>,
        project_id: ID,
    ) -> Result<Process, Error> {
        let project_id = project_id.to_string();
        let cmd_tx = ctx
            .data::<mpsc::UnboundedSender<SuperviseurCommand>>()
            .unwrap();
        let processes = ctx
            .data::<Arc<Mutex<Vec<(types::process::Process, String)>>>>()
            .unwrap();
        let provider = ctx.data::<Arc<Provider>>().unwrap();

        project_exists!(provider, project_id);

        let config = provider
            .build_configuration(&project_id)
            .map_err(|e| Error::new(e.to_string()))?;

        if id.is_none() {
            send_event_alt!(
                config.project,
                config.services,
                cmd_tx,
                Start,
                AllServicesStarted
            );

            return Ok(Process {
                ..Default::default()
            });
        }

        let (_, service) = config
            .services
            .iter()
            .find(|(_, s)| s.id == id.as_ref().map(|x| x.to_string()))
            .ok_or(Error::new("Service not found"))?;

        cmd_tx.send(SuperviseurCommand::Start(
            service.clone(),
            config.project.clone(),
            true,
        ))?;

        thread::sleep(Duration::from_secs(1));
        let processes = processes.lock().unwrap();
        let (process, _) = processes
            .iter()
            .find(|(p, _)| Some(p.service_id.clone()) == service.id)
            .ok_or(Error::new("Process not found"))?;

        Ok(Process::from(process.clone()))
    }

    async fn stop(
        &self,
        ctx: &Context<'_>,
        id: Option<ID>,
        project_id: ID,
    ) -> Result<Process, Error> {
        let project_id = project_id.to_string();
        let cmd_tx = ctx
            .data::<mpsc::UnboundedSender<SuperviseurCommand>>()
            .unwrap();
        let processes = ctx
            .data::<Arc<Mutex<Vec<(types::process::Process, String)>>>>()
            .unwrap();
        let provider = ctx.data::<Arc<Provider>>().unwrap();

        project_exists!(provider, project_id);

        let config = provider
            .build_configuration(&project_id)
            .map_err(|e| Error::new(e.to_string()))?;

        if id.is_none() {
            send_event!(
                config.project,
                config.services,
                cmd_tx,
                Stop,
                AllServicesStopped
            );
            return Ok(Process {
                ..Default::default()
            });
        }

        let (_, service) = config
            .services
            .iter()
            .find(|(_, s)| s.id == id.as_ref().map(|x| x.to_string()))
            .ok_or(Error::new("Service not found"))?;

        cmd_tx.send(SuperviseurCommand::Stop(
            service.clone(),
            config.project.clone(),
        ))?;

        thread::sleep(Duration::from_secs(1));
        let processes = processes.lock().unwrap();
        let (process, _) = processes
            .iter()
            .find(|(p, _)| Some(p.service_id.clone()) == service.id)
            .ok_or(Error::new("Process not found"))?;

        Ok(Process::from(process.clone()))
    }

    async fn restart(
        &self,
        ctx: &Context<'_>,
        id: Option<ID>,
        project_id: ID,
    ) -> Result<Process, Error> {
        let project_id = project_id.to_string();
        let cmd_tx = ctx
            .data::<mpsc::UnboundedSender<SuperviseurCommand>>()
            .unwrap();
        let processes = ctx
            .data::<Arc<Mutex<Vec<(types::process::Process, String)>>>>()
            .unwrap();
        let provider = ctx.data::<Arc<Provider>>().unwrap();
        let _project_map = ctx.data::<Arc<Mutex<HashMap<String, String>>>>().unwrap();

        project_exists!(provider, project_id);

        let config = provider
            .build_configuration(&project_id)
            .map_err(|e| Error::new(e.to_string()))?;

        if id.is_none() {
            send_event!(
                config.project,
                config.services,
                cmd_tx,
                Restart,
                AllServicesRestarted
            );
            return Ok(Process {
                ..Default::default()
            });
        }

        let (_, service) = config
            .services
            .iter()
            .find(|(_, s)| s.id == id.as_ref().map(|x| x.to_string()))
            .ok_or(Error::new("Service not found"))?;

        cmd_tx.send(SuperviseurCommand::Restart(
            service.clone(),
            config.project.clone(),
        ))?;

        thread::sleep(Duration::from_secs(1));
        let processes = processes.lock().unwrap();
        let (process, _) = processes
            .iter()
            .find(|(p, _)| Some(p.service_id.clone()) == service.id)
            .ok_or(Error::new("Process not found"))?;

        Ok(Process::from(process.clone()))
    }

    async fn create_env_var(
        &self,
        ctx: &Context<'_>,
        id: ID,
        name: String,
        value: String,
        project_id: ID,
    ) -> Result<Service, Error> {
        let project_id = project_id.to_string();
        let processes = ctx
            .data::<Arc<Mutex<Vec<(types::process::Process, String)>>>>()
            .unwrap();
        let provider = ctx.data::<Arc<Provider>>().unwrap();
        let _project_map = ctx.data::<Arc<Mutex<HashMap<String, String>>>>().unwrap();

        project_exists!(provider, project_id);

        let mut config = provider
            .build_configuration(&project_id)
            .map_err(|e| Error::new(e.to_string()))?;

        let (_, service) = config
            .services
            .iter_mut()
            .find(|(_, s)| s.id == Some(id.to_string()))
            .ok_or(Error::new("Service not found"))?;

        service.env.insert(name, value);
        let processes = processes.lock().unwrap();
        let (process, _) = processes
            .iter()
            .find(|(p, _)| Some(p.service_id.clone()) == service.id)
            .ok_or(Error::new("Process not found"))?;
        let service = service.clone();
        provider.save_configuration(&project_id, config)?;

        Ok(Service {
            status: process.state.to_string(),
            ..Service::from(service)
        })
    }

    async fn delete_env_var(
        &self,
        ctx: &Context<'_>,
        id: ID,
        name: String,
        project_id: ID,
    ) -> Result<Service, Error> {
        let project_id = project_id.to_string();
        let processes = ctx
            .data::<Arc<Mutex<Vec<(types::process::Process, String)>>>>()
            .unwrap();
        let provider = ctx.data::<Arc<Provider>>().unwrap();

        project_exists!(provider, project_id);

        let mut config = provider
            .build_configuration(&project_id)
            .map_err(|e| Error::new(e.to_string()))?;

        let (_, service) = config
            .services
            .iter_mut()
            .find(|(_, s)| s.id == Some(id.to_string()))
            .ok_or(Error::new("Service not found"))?;

        service.env.remove(&name);

        let processes = processes.lock().unwrap();
        let (process, _) = processes
            .iter()
            .find(|(p, _)| Some(p.service_id.clone()) == service.id)
            .ok_or(Error::new("Process not found"))?;
        let service = service.clone();

        provider
            .save_configuration(&project_id, config)
            .map_err(|e| Error::new(e.to_string()))?;

        Ok(Service {
            status: process.state.to_string(),
            ..Service::from(service)
        })
    }

    async fn update_env_var(
        &self,
        ctx: &Context<'_>,
        id: ID,
        name: String,
        value: String,
        project_id: ID,
    ) -> Result<Service, Error> {
        let project_id = project_id.to_string();
        let processes = ctx
            .data::<Arc<Mutex<Vec<(types::process::Process, String)>>>>()
            .unwrap();
        let provider = ctx.data::<Arc<Provider>>().unwrap();

        project_exists!(provider, project_id);

        let mut config = provider
            .build_configuration(&project_id)
            .map_err(|e| Error::new(e.to_string()))?;

        let (_, service) = config
            .services
            .iter_mut()
            .find(|(_, s)| s.id == Some(id.to_string()))
            .ok_or(Error::new("Service not found"))?;

        service.env.remove(&name);
        service.env.insert(name, value);
        let processes = processes.lock().unwrap();
        let (process, _) = processes
            .iter()
            .find(|(p, _)| Some(p.service_id.clone()) == service.id)
            .ok_or(Error::new("Process not found"))?;

        let service = service.clone();

        provider
            .save_configuration(&project_id, config)
            .map_err(|e| Error::new(e.to_string()))?;

        Ok(Service {
            status: process.state.to_string(),
            ..Service::from(service)
        })
    }
}

#[derive(Default, Clone)]
pub struct ControlSubscription;

#[Subscription]
impl ControlSubscription {
    async fn on_starting(&self, _ctx: &Context<'_>) -> impl Stream<Item = ServiceStarting> {
        SimpleBroker::<ServiceStarting>::subscribe()
    }

    async fn on_stopping(&self, _ctx: &Context<'_>) -> impl Stream<Item = ServiceStopping> {
        SimpleBroker::<ServiceStopping>::subscribe()
    }

    async fn on_start(&self, _ctx: &Context<'_>) -> impl Stream<Item = ServiceStarted> {
        SimpleBroker::<ServiceStarted>::subscribe()
    }

    async fn on_stop(&self, _ctx: &Context<'_>) -> impl Stream<Item = ServiceStopped> {
        SimpleBroker::<ServiceStopped>::subscribe()
    }

    async fn on_restart(&self, _ctx: &Context<'_>) -> impl Stream<Item = ServiceRestarted> {
        SimpleBroker::<ServiceRestarted>::subscribe()
    }

    async fn on_start_all(&self, _ctx: &Context<'_>) -> impl Stream<Item = AllServicesStarted> {
        SimpleBroker::<AllServicesStarted>::subscribe()
    }

    async fn on_stop_all(&self, _ctx: &Context<'_>) -> impl Stream<Item = AllServicesStopped> {
        SimpleBroker::<AllServicesStopped>::subscribe()
    }

    async fn on_restart_all(&self, _ctx: &Context<'_>) -> impl Stream<Item = AllServicesRestarted> {
        SimpleBroker::<AllServicesRestarted>::subscribe()
    }

    async fn on_open_project(
        &self,
        _ctx: &Context<'_>,
        id: ID,
    ) -> Result<impl Stream<Item = ProjectOpened>, Error> {
        Ok(stream! {
            while let Some(project) = SimpleBroker::<ProjectOpened>::subscribe().next().await {
                if ID(project.id.clone()) == id {
                    yield project;
                }
            }
        })
    }
}
