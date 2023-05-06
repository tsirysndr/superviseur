use crate::graphql::query::create_env_var::CreateEnvVarCreateEnvVar;
use crate::graphql::query::delete_env_var::DeleteEnvVarDeleteEnvVar;
use crate::graphql::query::services_query::ServicesQueryServices;
use crate::graphql::query::update_env_var::UpdateEnvVarUpdateEnvVar;

#[derive(Debug, Default)]
pub struct Service {
    pub id: String,
    pub name: String,
    pub command: String,
    pub status: String,
    pub env: Vec<String>,
}

impl From<ServicesQueryServices> for Service {
    fn from(service: ServicesQueryServices) -> Self {
        Service {
            id: service.id,
            name: service.name,
            command: service.command,
            status: service.status,
            ..Default::default()
        }
    }
}

impl From<CreateEnvVarCreateEnvVar> for Service {
    fn from(service: CreateEnvVarCreateEnvVar) -> Self {
        Service {
            id: service.id,
            env: service.env,
            ..Default::default()
        }
    }
}

impl From<UpdateEnvVarUpdateEnvVar> for Service {
    fn from(service: UpdateEnvVarUpdateEnvVar) -> Self {
        Service {
            id: service.id,
            env: service.env,
            ..Default::default()
        }
    }
}

impl From<DeleteEnvVarDeleteEnvVar> for Service {
    fn from(service: DeleteEnvVarDeleteEnvVar) -> Self {
        Service {
            id: service.id,
            env: service.env,
            ..Default::default()
        }
    }
}
