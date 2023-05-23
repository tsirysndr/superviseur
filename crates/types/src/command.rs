use crate::configuration::{ConfigurationData, Service};

#[derive(Debug)]
pub enum SuperviseurCommand {
    Load(Service, String),
    Start(Service, String, bool),
    Stop(Service, String),
    Restart(Service, String),
    Build(Service, String),
    LoadConfig(ConfigurationData, String),
    WatchForChanges(String, Service, String),
    StartDependency(Service, String),
    StartAll(String, bool),
    StopAll(String),
    RestartAll(String),
    BuildAll(String),
}
