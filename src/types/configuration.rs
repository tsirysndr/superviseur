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

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Service {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    pub name: String,
    pub r#type: String, // docker, podman, exec, wasm
    pub command: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stop_command: Option<String>,
    pub working_dir: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub watch_dir: Option<String>,
    pub description: Option<String>,
    pub depends_on: Vec<String>,
    #[serde(skip_serializing, skip_deserializing)]
    pub dependencies: Vec<String>,
    pub env: HashMap<String, String>,
    pub autostart: bool,
    pub autorestart: bool,
    pub namespace: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub port: Option<u16>,
    pub stdout: String,
    pub stderr: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wait_for: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flox: Option<Flox>,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct ConfigurationData {
    pub project: String,
    pub services: Vec<Service>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Flox {
    pub environment: String,
}
