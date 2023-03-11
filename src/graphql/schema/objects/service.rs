use async_graphql::Object;

#[derive(Default, Clone)]
pub struct Service {
    pub name: String,
    pub command: String,
    pub description: String,
    pub namespace: String,
    pub r#type: String,
    pub status: String,
    pub depends_on: Vec<String>,
    pub env: Vec<String>,
    pub auto_restart: bool,
}

#[Object]
impl Service {
    async fn name(&self) -> &str {
        &self.name
    }

    async fn command(&self) -> &str {
        &self.command
    }

    async fn description(&self) -> &str {
        &self.description
    }

    async fn namespace(&self) -> &str {
        &self.namespace
    }

    async fn r#type(&self) -> &str {
        &self.r#type
    }

    async fn status(&self) -> &str {
        &self.status
    }

    async fn depends_on(&self) -> &Vec<String> {
        &self.depends_on
    }

    async fn env(&self) -> &Vec<String> {
        &self.env
    }

    async fn auto_restart(&self) -> bool {
        self.auto_restart
    }
}
