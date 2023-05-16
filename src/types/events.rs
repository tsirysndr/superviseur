#[derive(Debug)]
pub enum SuperviseurEvent {
    Starting(String, String),
    Building(String, String),
    Stopping(String, String),
    Restarting(String, String),
    SetupEnv(String, String, String),
    Started(String, String),
    Stopped(String, String),
    Restarted(String, String),
    Crashed(String, String),
    AllServicesStarted(String),
    AllServicesStopped(String),
    AllServicesRestarted(String),
    AllServicesBuilt(String),
    Built(String, String),
    Logs(String, String, String),
    Error(String, String, String),
}

pub const SERVICE_STARTING: &str = "service_starting";
pub const SERVICE_BUILDING: &str = "service_building";
pub const SERVICE_STOPPING: &str = "service_stopping";
pub const SERVICE_RESTARTING: &str = "service_restarting";
pub const SERVICE_SETUP_ENV: &str = "service_setup_env";
pub const SERVICE_STARTED: &str = "service_started";
pub const SERVICE_STOPPED: &str = "service_stopped";
pub const SERVICE_RESTARTED: &str = "service_restarted";
pub const SERVICE_CRASHED: &str = "service_crashed";
pub const SERVICE_BUILT: &str = "service_built";
pub const SERVICE_LOGS: &str = "service_logs";
pub const SERVICE_ERROR: &str = "service_error";
pub const ALL_SERVICES_STARTED: &str = "all_services_started";
pub const ALL_SERVICES_STOPPED: &str = "all_services_stopped";
pub const ALL_SERVICES_RESTARTED: &str = "all_services_restarted";
pub const ALL_SERVICES_BUILT: &str = "all_services_built";
