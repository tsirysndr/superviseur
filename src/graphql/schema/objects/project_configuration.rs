use std::{
    collections::HashMap,
    path::Path,
    sync::{Arc, Mutex},
};

use async_graphql::{Context, Object, ID};
use names::Generator;

use crate::types::configuration::{ConfigurationData, Service};

use super::service_configuration::ServiceConfiguration;

#[derive(Default, Clone, Debug)]
pub struct ProjectConfiguration {
    pub id: ID,
    pub name: String,
    pub description: Option<String>,
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
    ) -> &ProjectConfiguration {
        if !Path::new(&service.working_directory).exists() {
            return self;
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
            working_dir: service.working_directory,
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

        &self
    }

    async fn stdout(&self) -> Vec<String> {
        vec![]
    }
}
