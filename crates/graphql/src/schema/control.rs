use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
    thread,
    time::Duration,
};

use async_graphql::{async_stream::stream, Context, Error, Object, Subscription, ID};
use futures_util::Stream;
use superviseur_provider::kv::kv::Provider;
use tokio::sync::mpsc;
use tokio_stream::StreamExt;

use crate::{
    macros::{project_exists, send_event, send_event_alt},
    schema::objects::subscriptions::ServiceStarted,
    simple_broker::SimpleBroker,
};
use superviseur_types::{self as types, command::SuperviseurCommand, process::State};

use super::objects::{
    process::Process,
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
}

#[derive(Default, Clone)]
pub struct ControlMutation;

#[Object]
impl ControlMutation {
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
