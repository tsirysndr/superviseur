use async_graphql::{Object, ID};

#[derive(Default, Clone)]
pub struct Service {
    pub id: ID,
    pub name: String,
    pub command: String,
    pub description: String,
    pub namespace: String,
    pub r#type: String,
    pub status: String,
    pub depends_on: Vec<String>,
    pub env: Vec<String>,
    pub auto_restart: bool,
    pub working_directory: String,
    pub log_file: String,
    pub stderr_file: String,
    pub port: i32,
}

#[Object]
impl Service {
    async fn id(&self) -> &str {
        &self.id
    }

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

    async fn working_directory(&self) -> &str {
        &self.working_directory
    }

    async fn log_file(&self) -> &str {
        &self.log_file
    }

    async fn stderr_file(&self) -> &str {
        &self.stderr_file
    }

    async fn port(&self) -> i32 {
        self.port
    }
}
