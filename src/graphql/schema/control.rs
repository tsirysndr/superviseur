use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
    thread,
    time::Duration,
};

use async_graphql::{Context, Error, Object, Subscription, ID};
use futures_util::Stream;
use indexmap::IndexMap;
use names::Generator;
use tokio::sync::mpsc;

use crate::{
    graphql::{schema::objects::subscriptions::ServiceStarted, simple_broker::SimpleBroker},
    superviseur::core::SuperviseurCommand,
    types::{self, configuration::ConfigurationData, process::State},
};

use super::objects::{
    process::Process,
    project::Project,
    project_configuration::ProjectConfiguration,
    service::Service,
    subscriptions::{
        AllServicesRestarted, AllServicesStarted, AllServicesStopped, ServiceRestarted,
        ServiceStarting, ServiceStopped, ServiceStopping,
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
        let config_map = ctx
            .data::<Arc<Mutex<HashMap<String, ConfigurationData>>>>()
            .unwrap();

        let project_map = project_map.lock().unwrap();
        let config_map = config_map.lock().unwrap();

        let projects = config_map
            .iter()
            .map(|(id, config)| {
                let config_path = match project_map.clone().into_iter().find(|(_, v)| v == id) {
                    Some((k, _)) => Some(k),
                    None => None,
                };

                Project {
                    id: ID(id.clone()),
                    name: config.project.clone(),
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
        let config_map = ctx
            .data::<Arc<Mutex<HashMap<String, ConfigurationData>>>>()
            .unwrap();

        let processes = processes.lock().unwrap();

        let config_map = config_map.lock().unwrap();
        let project_id = project_id.to_string();

        if !config_map.contains_key(&project_id) {
            return Err(Error::new("Configuration file not found"));
        }

        let config = config_map.get(&project_id).unwrap();

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
        let config_map = ctx
            .data::<Arc<Mutex<HashMap<String, ConfigurationData>>>>()
            .unwrap();

        let project_map = project_map.lock().unwrap();
        let config_map = config_map.lock().unwrap();

        let project_id = id.to_string();

        match config_map.get(&project_id) {
            Some(config) => {
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
            None => Err(Error::new("Configuration file not found")),
        }
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
        let config_map = ctx
            .data::<Arc<Mutex<HashMap<String, ConfigurationData>>>>()
            .unwrap();

        let config_map = config_map.lock().unwrap();

        if !config_map.contains_key(&project_id) {
            return Err(Error::new("Configuration file not found"));
        }

        let config = config_map.get(&project_id).unwrap();

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
        context: String,
    ) -> Result<ProjectConfiguration, Error> {
        let config_map = ctx
            .data::<Arc<Mutex<HashMap<String, ConfigurationData>>>>()
            .unwrap();

        let mut generator = Generator::default();
        let id = generator.next().unwrap();
        let config = ConfigurationData {
            project: name.clone(),
            services: IndexMap::new(),
            context: Some(context.clone()),
            network_settings: None,
            volume_settings: None,
        };

        let mut config_map = config_map.lock().unwrap();
        // check if project already exists by verifying if the context is already used
        if config_map
            .values()
            .any(|c| c.context == Some(context.clone()))
        {
            return Err(Error::new("Project already exists with this context"));
        }

        config_map.insert(id.clone(), config.clone());
        drop(config_map);

        return Ok(ProjectConfiguration {
            id: ID(id),
            name,
            context,
            ..Default::default()
        });
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
        let config_map = ctx
            .data::<Arc<Mutex<HashMap<String, ConfigurationData>>>>()
            .unwrap();

        let config_map = config_map.lock().unwrap();

        if !config_map.contains_key(&project_id) {
            return Err(Error::new("Configuration file not found"));
        }

        let config = config_map.get(&project_id).unwrap();

        if id.is_none() {
            for (_, service) in &config.services {
                cmd_tx
                    .send(SuperviseurCommand::Start(
                        service.clone(),
                        config.project.clone(),
                    ))
                    .unwrap();
            }

            let services = config.services.clone();
            let services = services
                .iter()
                .map(|(_, x)| Service::from(x))
                .collect::<Vec<Service>>();
            SimpleBroker::publish(AllServicesStarted { payload: services });

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
        let config_map = ctx
            .data::<Arc<Mutex<HashMap<String, ConfigurationData>>>>()
            .unwrap();

        let config_map = config_map.lock().unwrap();

        if !config_map.contains_key(&project_id) {
            return Err(Error::new("Configuration file not found"));
        }

        let config = config_map.get(&project_id).unwrap();

        if id.is_none() {
            for (_, service) in &config.services {
                cmd_tx
                    .send(SuperviseurCommand::Stop(
                        service.clone(),
                        config.project.clone(),
                    ))
                    .unwrap();
            }
            let services = config.services.clone();
            let services = services
                .iter()
                .map(|(_, x)| Service::from(x))
                .collect::<Vec<Service>>();
            SimpleBroker::publish(AllServicesStopped { payload: services });
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
        let config_map = ctx
            .data::<Arc<Mutex<HashMap<String, ConfigurationData>>>>()
            .unwrap();
        let project_map = ctx.data::<Arc<Mutex<HashMap<String, String>>>>().unwrap();

        let config_map = config_map.lock().unwrap();

        if !config_map.contains_key(&project_id) {
            return Err(Error::new("Configuration file not found"));
        }

        let config = config_map.get(&project_id).unwrap();

        if id.is_none() {
            for (_, service) in &config.services {
                cmd_tx
                    .send(SuperviseurCommand::Restart(
                        service.clone(),
                        config.project.clone(),
                    ))
                    .unwrap();
            }

            let services = config.services.clone();
            let services = services
                .iter()
                .map(|(_, x)| Service::from(x))
                .collect::<Vec<Service>>();
            SimpleBroker::publish(AllServicesStarted { payload: services });

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
        let config_map = ctx
            .data::<Arc<Mutex<HashMap<String, ConfigurationData>>>>()
            .unwrap();
        let project_map = ctx.data::<Arc<Mutex<HashMap<String, String>>>>().unwrap();

        let mut config_map = config_map.lock().unwrap();

        if !config_map.contains_key(&project_id) {
            return Err(Error::new("Configuration file not found"));
        }

        let config = config_map.get_mut(&project_id).unwrap();

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
        let config_map = ctx
            .data::<Arc<Mutex<HashMap<String, ConfigurationData>>>>()
            .unwrap();

        let mut config_map = config_map.lock().unwrap();

        if !config_map.contains_key(&project_id) {
            return Err(Error::new("Configuration file not found"));
        }

        let config = config_map.get_mut(&project_id).unwrap();

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
        let config_map = ctx
            .data::<Arc<Mutex<HashMap<String, ConfigurationData>>>>()
            .unwrap();

        let mut config_map = config_map.lock().unwrap();

        if !config_map.contains_key(&project_id) {
            return Err(Error::new("Configuration file not found"));
        }

        let config = config_map.get_mut(&project_id).unwrap();

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
}
