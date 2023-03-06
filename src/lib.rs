pub mod cmd;
pub mod config;
pub mod server;
pub mod superviseur;
pub mod types;
pub mod api {
    #[path = ""]
    pub mod superviseur {
        #[path = "superviseur.v1alpha1.rs"]
        pub mod v1alpha1;
    }
    #[path = ""]
    pub mod objects {
        use std::collections::HashMap;

        use crate::types;

        use self::v1alpha1::{Process, Service};

        #[path = "objects.v1alpha1.rs"]
        pub mod v1alpha1;

        impl Into<types::service::Service> for Service {
            fn into(self) -> types::service::Service {
                types::service::Service {
                    name: self.name,
                    status: self.status,
                    depends_on: self.depends_on,
                    command: self.command,
                    r#type: self.r#type,
                    ..Default::default()
                }
            }
        }

        impl From<types::configuration::Service> for Service {
            fn from(service: types::configuration::Service) -> Self {
                Self {
                    name: service.name,
                    depends_on: service.depends_on,
                    command: service.command,
                    r#type: service.r#type,
                    ..Default::default()
                }
            }
        }

        impl Into<types::process::Process> for Process {
            fn into(self) -> types::process::Process {
                let mut env: HashMap<String, String> = HashMap::new();
                self.env.iter().for_each(|e| {
                    let mut split = e.split('=');
                    if let Some(key) = split.next() {
                        if let Some(value) = split.next() {
                            env.insert(key.to_string(), value.to_string());
                        }
                    }
                });
                types::process::Process {
                    name: self.name,
                    pid: Some(self.pid),
                    command: self.command,
                    up_time: self.up_time.parse().ok(),
                    state: self.state.parse().unwrap_or_default(),
                    description: Some(self.description),
                    working_dir: self.working_directory,
                    project: self.project,
                    r#type: self.r#type,
                    stdout: self.log_file,
                    stderr: self.stderr_file,
                    auto_restart: self.auto_restart,
                    env,
                    ..Default::default()
                }
            }
        }

        impl From<types::process::Process> for Process {
            fn from(process: types::process::Process) -> Self {
                let mut env = vec![];
                process.env.iter().for_each(|(k, v)| {
                    env.push(format!("{}={}", k, v));
                });
                Self {
                    name: process.name,
                    pid: process.pid.unwrap_or_default(),
                    command: process.command,
                    up_time: process.up_time.map(|t| t.to_rfc3339()).unwrap_or_default(),
                    state: process.state.to_string(),
                    description: process.description.unwrap_or_default(),
                    working_directory: process.working_dir,
                    project: process.project,
                    r#type: process.r#type,
                    log_file: process.stdout,
                    stderr_file: process.stderr,
                    auto_restart: process.auto_restart,
                    env,
                    ..Default::default()
                }
            }
        }
    }
}
