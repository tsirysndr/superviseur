use async_graphql::InputObject;

#[derive(InputObject)]
pub struct ServiceConfiguration {
    pub name: String,
    pub description: Option<String>,
    pub command: String,
    pub working_directory: Option<String>,
    pub r#type: Option<String>,
    pub log_file: Option<String>,
    pub stderr_file: Option<String>,
    pub auto_restart: Option<bool>,
    pub auto_start: Option<bool>,
    pub env: Option<Vec<String>>,
}
