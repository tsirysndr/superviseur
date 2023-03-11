use async_graphql::Object;

#[derive(Default, Clone)]
pub struct Process {
    pub name: String,
    pub description: String,
    pub pid: i32,
    pub ppid: i32,
    pub command: String,
    pub working_directory: String,
    pub project: String,
    pub r#type: String,
    pub log_file: String,
    pub stderr_file: String,
    pub auto_restart: bool,
    pub env: Vec<String>,
}

#[Object]
impl Process {
    async fn name(&self) -> &str {
        &self.name
    }

    async fn description(&self) -> &str {
        &self.description
    }

    async fn pid(&self) -> i32 {
        self.pid
    }

    async fn ppid(&self) -> i32 {
        self.ppid
    }

    async fn command(&self) -> &str {
        &self.command
    }

    async fn working_directory(&self) -> &str {
        &self.working_directory
    }

    async fn project(&self) -> &str {
        &self.project
    }

    async fn r#type(&self) -> &str {
        &self.r#type
    }

    async fn log_file(&self) -> &str {
        &self.log_file
    }

    async fn stderr_file(&self) -> &str {
        &self.stderr_file
    }

    async fn auto_restart(&self) -> bool {
        self.auto_restart
    }

    async fn env(&self) -> &Vec<String> {
        &self.env
    }
}
