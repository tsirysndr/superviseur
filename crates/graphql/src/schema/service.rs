use std::sync::{Arc, Mutex};

use async_graphql::{Context, Error, Object, ID};
use superviseur_provider::kv::kv::Provider;
use superviseur_types::{self as types};

use crate::macros::project_exists;

use super::objects::service::Service;

#[derive(Clone, Default)]
pub struct ServiceQuery;

#[Object]
impl ServiceQuery {
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
}

#[derive(Clone, Default)]
pub struct ServiceMutation;

#[Object]
impl ServiceMutation {
    async fn create_service(
        &self,
        ctx: &Context<'_>,
        project_id: ID,
        name: String,
        from: Option<String>,
    ) -> Result<Service, Error> {
        todo!("create_service")
    }

    async fn delete_service(&self, ctx: &Context<'_>, id: ID) -> Result<Service, Error> {
        todo!("delete_service")
    }
}
