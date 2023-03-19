use async_graphql::Object;

use crate::types;

#[derive(Default, Clone)]
pub struct Process {
    pub name: String,
    pub description: Option<String>,
    pub pid: Option<u32>,
    pub ppid: Option<u32>,
    pub command: String,
    pub working_directory: String,
    pub project: String,
    pub r#type: String,
    pub log_file: String,
    pub stderr_file: String,
    pub auto_restart: bool,
    pub env: Vec<String>,
    pub state: String,
    pub up_time: String,
}

#[Object]
impl Process {
    async fn name(&self) -> &str {
        &self.name
    }

    async fn description(&self) -> Option<&str> {
        self.description.as_deref()
    }

    async fn pid(&self) -> Option<u32> {
        self.pid
    }

    async fn ppid(&self) -> Option<u32> {
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

    async fn state(&self) -> &str {
        &self.state
    }

    async fn up_time(&self) -> &str {
        &self.up_time
    }
}

impl From<types::process::Process> for Process {
    fn from(process: types::process::Process) -> Self {
        Self {
            name: process.name.clone(),
            description: process.description.clone(),
            pid: process.pid,
            command: process.command.clone(),
            state: process.state.to_string(),
            r#type: process.r#type.to_string(),
            stderr_file: process.stderr.clone(),
            log_file: process.stdout.clone(),
            up_time: process.up_time.map(|t| t.to_rfc3339()).unwrap_or_default(),
            working_directory: process.working_dir.clone(),
            ..Default::default()
        }
    }
}
