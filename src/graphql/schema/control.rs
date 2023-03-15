use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
    thread,
    time::Duration,
};

use async_graphql::{Context, Error, Object, ID};
use tokio::sync::mpsc;

use crate::{
    superviseur::SuperviseurCommand,
    types::{self, configuration::ConfigurationData},
};

use super::objects::{process::Process, service::Service};

#[derive(Default, Clone)]
pub struct ControlQuery;

#[Object]
impl ControlQuery {
    async fn status(&self, ctx: &Context<'_>, id: ID) -> Process {
        let config_file_path = ctx.data::<String>().unwrap();
        let cmd_tx = ctx.data::<mpsc::UnboundedSender<SuperviseurCommand>>();
        let processes = ctx.data::<Arc<Mutex<Vec<(Process, String)>>>>().unwrap();
        let config_map = ctx
            .data::<Arc<Mutex<HashMap<String, ConfigurationData>>>>()
            .unwrap();
        Process {
            ..Default::default()
        }
    }

    async fn services(&self, ctx: &Context<'_>) -> Vec<Service> {
        let config_file_path = ctx.data::<String>().unwrap();
        let cmd_tx = ctx.data::<mpsc::UnboundedSender<SuperviseurCommand>>();
        let processes = ctx.data::<Arc<Mutex<Vec<(Process, String)>>>>().unwrap();
        let config_map = ctx
            .data::<Arc<Mutex<HashMap<String, ConfigurationData>>>>()
            .unwrap();
        vec![]
    }

    async fn processes(&self, ctx: &Context<'_>) -> Vec<Process> {
        let config_file_path = ctx.data::<String>().unwrap();
        let cmd_tx = ctx.data::<mpsc::UnboundedSender<SuperviseurCommand>>();
        let processes = ctx.data::<Arc<Mutex<Vec<(Process, String)>>>>().unwrap();
        let config_map = ctx
            .data::<Arc<Mutex<HashMap<String, ConfigurationData>>>>()
            .unwrap();
        vec![]
    }

    async fn service(&self, ctx: &Context<'_>, id: ID) -> Service {
        let config_file_path = ctx.data::<String>().unwrap();
        let cmd_tx = ctx.data::<mpsc::UnboundedSender<SuperviseurCommand>>();
        let processes = ctx.data::<Arc<Mutex<Vec<(Process, String)>>>>().unwrap();
        let config_map = ctx
            .data::<Arc<Mutex<HashMap<String, ConfigurationData>>>>()
            .unwrap();
        Service {
            ..Default::default()
        }
    }
}

#[derive(Default, Clone)]
pub struct ControlMutation;

#[Object]
impl ControlMutation {
    async fn start(&self, ctx: &Context<'_>, id: Option<ID>) -> Result<Process, Error> {
        let config_file_path = ctx.data::<String>().unwrap();
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

        if !config_map.contains_key(config_file_path.as_str()) {
            return Err(Error::new("Configuration file not found"));
        }

        let config = config_map.get(config_file_path.as_str()).unwrap();

        if id.is_none() {
            for service in &config.services {
                cmd_tx
                    .send(SuperviseurCommand::Start(
                        service.clone(),
                        config.project.clone(),
                    ))
                    .unwrap();
            }
            return Ok(Process {
                ..Default::default()
            });
        }

        let service = config
            .services
            .iter()
            .find(|s| s.id == id.as_ref().map(|x| x.to_string()))
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

        Ok(Process {
            name: process.name.clone(),
            description: process.description.clone(),
            pid: process.pid,
            ppid: None,
            command: process.command.clone(),
            state: process.state.to_string(),
            ..Default::default()
        })
    }

    async fn stop(&self, ctx: &Context<'_>, id: Option<ID>) -> Result<Process, Error> {
        let config_file_path = ctx.data::<String>().unwrap();
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

        if !config_map.contains_key(config_file_path.as_str()) {
            return Err(Error::new("Configuration file not found"));
        }

        let config = config_map.get(config_file_path.as_str()).unwrap();

        if id.is_none() {
            for service in &config.services {
                cmd_tx
                    .send(SuperviseurCommand::Stop(
                        service.clone(),
                        config.project.clone(),
                    ))
                    .unwrap();
            }
            return Ok(Process {
                ..Default::default()
            });
        }

        let service = config
            .services
            .iter()
            .find(|s| s.id == id.as_ref().map(|x| x.to_string()))
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

        Ok(Process {
            name: process.name.clone(),
            description: process.description.clone(),
            pid: process.pid,
            ppid: None,
            command: process.command.clone(),
            state: process.state.to_string(),
            ..Default::default()
        })
    }

    async fn restart(&self, ctx: &Context<'_>, id: Option<ID>) -> Result<Process, Error> {
        let config_file_path = ctx.data::<String>().unwrap();
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

        if !config_map.contains_key(config_file_path.as_str()) {
            return Err(Error::new("Configuration file not found"));
        }

        let config = config_map.get(config_file_path.as_str()).unwrap();

        if id.is_none() {
            for service in &config.services {
                cmd_tx
                    .send(SuperviseurCommand::Restart(
                        service.clone(),
                        config.project.clone(),
                    ))
                    .unwrap();
            }
            return Ok(Process {
                ..Default::default()
            });
        }

        let service = config
            .services
            .iter()
            .find(|s| s.id == id.as_ref().map(|x| x.to_string()))
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

        Ok(Process {
            name: process.name.clone(),
            description: process.description.clone(),
            pid: process.pid,
            ppid: None,
            command: process.command.clone(),
            state: process.state.to_string(),
            ..Default::default()
        })
    }

    async fn create_env_var(
        &self,
        ctx: &Context<'_>,
        id: ID,
        name: String,
        value: String,
    ) -> Service {
        let config_file_path = ctx.data::<String>().unwrap();
        let cmd_tx = ctx
            .data::<mpsc::UnboundedSender<SuperviseurCommand>>()
            .unwrap();
        let processes = ctx
            .data::<Arc<Mutex<Vec<(types::process::Process, String)>>>>()
            .unwrap();
        let config_map = ctx
            .data::<Arc<Mutex<HashMap<String, ConfigurationData>>>>()
            .unwrap();
        Service {
            ..Default::default()
        }
    }

    async fn delete_env_var(&self, ctx: &Context<'_>, id: ID, name: String) -> Service {
        let config_file_path = ctx.data::<String>().unwrap();
        let cmd_tx = ctx
            .data::<mpsc::UnboundedSender<SuperviseurCommand>>()
            .unwrap();
        let processes = ctx
            .data::<Arc<Mutex<Vec<(types::process::Process, String)>>>>()
            .unwrap();
        let config_map = ctx
            .data::<Arc<Mutex<HashMap<String, ConfigurationData>>>>()
            .unwrap();
        Service {
            ..Default::default()
        }
    }

    async fn update_env_var(
        &self,
        ctx: &Context<'_>,
        id: ID,
        name: String,
        value: String,
    ) -> Service {
        let config_file_path = ctx.data::<String>().unwrap();
        let cmd_tx = ctx
            .data::<mpsc::UnboundedSender<SuperviseurCommand>>()
            .unwrap();
        let processes = ctx
            .data::<Arc<Mutex<Vec<(types::process::Process, String)>>>>()
            .unwrap();
        let config_map = ctx
            .data::<Arc<Mutex<HashMap<String, ConfigurationData>>>>()
            .unwrap();
        Service {
            ..Default::default()
        }
    }
}
