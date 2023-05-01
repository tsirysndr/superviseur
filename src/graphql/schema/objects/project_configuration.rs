use std::{
    collections::HashMap,
    path::Path,
    sync::{Arc, Mutex},
    thread::sleep,
    time::Duration,
};

use async_graphql::{Context, Error, Object, ID};
use names::Generator;
use tokio::sync::mpsc;

use crate::{
    graphql::simple_broker::SimpleBroker,
    superviseur::core::SuperviseurCommand,
    types::configuration::{ConfigurationData, Service},
    util::{convert_dir_path_to_absolute_path, read_lines},
};

use super::{service_configuration::ServiceConfiguration, subscriptions::AllServicesStarted};

#[derive(Default, Clone, Debug)]
pub struct ProjectConfiguration {
    pub id: ID,
    pub name: String,
    pub description: Option<String>,
    pub context: String,
}

#[Object]
impl ProjectConfiguration {
    async fn id(&self) -> &ID {
        &self.id
    }

    async fn name(&self) -> &str {
        &self.name
    }

    async fn description(&self) -> Option<&str> {
        self.description.as_deref()
    }

    async fn with_service(
        &self,
        ctx: &Context<'_>,
        service: ServiceConfiguration,
    ) -> Result<&ProjectConfiguration, Error> {
        let working_directory = convert_dir_path_to_absolute_path(
            &service.working_directory.unwrap_or("./".to_string()),
            &self.context,
        )?;
        if !Path::new(&working_directory).exists() {
            return Err(Error::new(format!(
                "Working directory {} does not exist",
                working_directory
            )));
        }

        let config_map = ctx
            .data::<Arc<Mutex<HashMap<String, ConfigurationData>>>>()
            .unwrap();

        let mut config_map = config_map.lock().unwrap();
        let config = config_map.get_mut(&self.id.to_string()).unwrap();

        let mut generator = Generator::default();
        let service_id = generator.next().unwrap();
        let env = match service.env {
            Some(env) => env
                .iter()
                .map(|e| {
                    let mut split = e.split('=');
                    let key = split.next().unwrap().to_string();
                    let value = split.next().unwrap().to_string();
                    (key, value)
                })
                .collect::<HashMap<String, String>>(),
            None => HashMap::new(),
        };

        let service = Service {
            id: Some(service_id.clone()),
            name: service.name,
            description: service.description,
            r#type: service.r#type.unwrap_or("exec".to_string()),
            command: service.command,
            working_dir: working_directory,
            stdout: service
                .log_file
                .unwrap_or(format!("/tmp/stdout-{}.log", service_id.clone())),
            stderr: service
                .stderr_file
                .unwrap_or(format!("/tmp/stderr-{}.log", service_id)),
            autorestart: service.auto_restart.unwrap_or(false),
            autostart: service.auto_start.unwrap_or(false),
            env,
            ..Default::default()
        };
        config.services.push(service);

        Ok(&self)
    }

    async fn stdout(&self, ctx: &Context<'_>) -> Result<Vec<String>, Error> {
        let cmd_tx = ctx
            .data::<mpsc::UnboundedSender<SuperviseurCommand>>()
            .unwrap();

        let config_map = ctx
            .data::<Arc<Mutex<HashMap<String, ConfigurationData>>>>()
            .unwrap();

        let mut config_map = config_map.lock().unwrap();
        let config = config_map.get(&self.id.to_string()).unwrap();

        cmd_tx
            .send(SuperviseurCommand::LoadConfig(
                config.clone(),
                config.project.clone(),
            ))
            .unwrap();

        let config = config_map.get_mut(&self.id.to_string()).unwrap();

        let services = config.services.clone();
        let mut services = services.into_iter();

        // convert services dependencies to ids
        for service in &mut config.services {
            let mut dependencies = vec![];
            for dependency in &service.depends_on {
                match services.find(|s| s.name == *dependency) {
                    Some(service) => {
                        dependencies.push(service.id.clone().unwrap());
                    }
                    None => {
                        return Err(Error::new(format!("Service {} not found", dependency)));
                    }
                }
            }
            service.dependencies = dependencies;
        }

        for service in services.into_iter() {
            cmd_tx.send(SuperviseurCommand::Load(service, config.project.clone()))?;
        }

        cmd_tx.send(SuperviseurCommand::StartAll(config.project.clone()))?;

        futures::executor::block_on_stream(SimpleBroker::<AllServicesStarted>::subscribe()).next();

        let mut stdout = vec![];
        for service in &config.services {
            // loop while the file does not exist
            while !Path::new(&service.stdout).exists() {
                sleep(Duration::from_secs(2));
            }
            let lines = read_lines(&service.stdout).unwrap();
            stdout.extend(lines);
        }
        Ok(stdout)
    }
}
