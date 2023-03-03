use serde::{Deserialize, Serialize};
use std::path::PathBuf;

pub struct Configuration {
    pub path: PathBuf,
    pub data: ConfigurationData,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Service {
    pub name: String,
    pub r#type: String,
    pub command: String,
    pub working_dir: String,
    pub description: Option<String>,
    pub depends_on: Vec<String>,
    pub env: Vec<String>,
    pub autostart: bool,
    pub namespace: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ConfigurationData {
    pub services: Vec<Service>,
    pub port: u16,
}
