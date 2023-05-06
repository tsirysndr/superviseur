use async_graphql::InputObject;

#[derive(InputObject)]
pub struct ServiceConfiguration {
    pub name: String,
    pub description: Option<String>,
    pub command: String,
    pub depends_on: Option<Vec<String>>,
    pub working_directory: Option<String>,
    pub r#type: Option<String>,
    pub log_file: Option<String>,
    pub stderr_file: Option<String>,
    pub auto_restart: Option<bool>,
    pub auto_start: Option<bool>,
    pub env: Option<Vec<String>>,
    pub enable_docker: Option<bool>,
    pub enable_flox: Option<bool>,
    pub enable_nix: Option<bool>,
    pub flox_enviroment: Option<String>,
    pub port: Option<u32>,
}
