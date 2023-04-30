use async_graphql::{Context, Object, ID};

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
        &self
    }

    async fn start(&self) -> bool {
        true
    }
}
