use std::{collections::HashMap, path::Path, sync::Arc, thread::sleep, time::Duration};
use async_graphql::{Context, Error, Object, ID};
use names::Generator;
use superviseur_provider::kv::kv::Provider;
use superviseur_types::{command::SuperviseurCommand, configuration::Service};
use tokio::sync::mpsc;
use crate::simple_broker::SimpleBroker;
use superviseur_util::{convert_dir_path_to_absolute_path, read_lines};
use superviseur_macros::default_stdout;

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

        let provider = ctx.data::<Arc<Provider>>().unwrap();

        let mut config = provider
            .build_configuration(&self.id.to_string())
            .map_err(|e| Error::new(e.to_string()))?;

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
            stdout: service.log_file,
            stderr: service.stderr_file,
            autorestart: service.auto_restart,
            autostart: service.auto_start,
            env,
            depends_on: service.depends_on.unwrap_or(vec![]),
            port: service.port,
            ..Default::default()
        };
        config.services.insert(service.name.clone(), service);

        provider
            .save_configuration(&self.id.to_string(), config)
            .map_err(|e| Error::new(e.to_string()))?;

        Ok(&self)
    }

    async fn stdout(&self, ctx: &Context<'_>) -> Result<Vec<String>, Error> {
        let cmd_tx = ctx
            .data::<mpsc::UnboundedSender<SuperviseurCommand>>()
            .unwrap();

        let provider = ctx.data::<Arc<Provider>>().unwrap();

        let mut config = provider
            .build_configuration(&self.id.to_string())
            .map_err(|e| Error::new(e.to_string()))?;

        cmd_tx
            .send(SuperviseurCommand::LoadConfig(
                config.clone(),
                config.project.clone(),
            ))
            .unwrap();

        let services = config.services.clone();
        let mut services = services.into_iter();

        // convert services dependencies to ids
        for (_, service) in &mut config.services {
            let mut dependencies = vec![];
            for dependency in &service.depends_on {
                match services.find(|(name, _)| *name == *dependency) {
                    Some((_, service)) => {
                        dependencies.push(service.id.clone().unwrap());
                    }
                    None => {
                        return Err(Error::new(format!("Service {} not found", dependency)));
                    }
                }
            }
            service.dependencies = dependencies;
        }

        for (_, service) in services.into_iter() {
            cmd_tx.send(SuperviseurCommand::Load(service, config.project.clone()))?;
        }

        cmd_tx.send(SuperviseurCommand::StartAll(config.project.clone(), true))?;

        futures::executor::block_on_stream(SimpleBroker::<AllServicesStarted>::subscribe()).next();

        let mut stdout = vec![];
        for (_, service) in &config.services {
            // loop while the file does not exist
            let stdout_file = &service
                .clone()
                .stdout
                .unwrap_or(default_stdout!(config.project, service.name));
            while !Path::new(stdout_file).exists() {
                sleep(Duration::from_secs(2));
            }
            let lines = read_lines(&stdout_file).unwrap();
            stdout.extend(lines);
        }

        provider
            .save_configuration(&self.id.to_string(), config)
            .map_err(|e| Error::new(e.to_string()))?;

        Ok(stdout)
    }
}
