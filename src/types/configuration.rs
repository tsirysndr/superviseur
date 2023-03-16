use serde::{Deserialize, Serialize};
use std::{collections::HashMap, path::PathBuf};

pub enum ConfigFormat {
    TOML,
    HCL,
}

pub struct Configuration {
    pub path: PathBuf,
    pub data: ConfigurationData,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Service {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    pub name: String,
    pub r#type: String, // docker, podman, exec, wasm
    pub command: String,
    pub working_dir: String,
    pub description: Option<String>,
    pub depends_on: Vec<String>,
    #[serde(skip_serializing, skip_deserializing)]
    pub dependencies: Vec<String>,
    pub env: HashMap<String, String>,
    pub autostart: bool,
    pub autorestart: bool,
    pub namespace: Option<String>,
    pub port: u16,
    pub stdout: String,
    pub stderr: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ConfigurationData {
    pub project: String,
    pub services: Vec<Service>,
}
