pub mod cmd;
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
                types::process::Process {
                    name: self.name,
                    pid: Some(self.pid),
                    command: self.command,
                    up_time: self.up_time.parse().ok(),
                    ..Default::default()
                }
            }
        }

        impl From<types::process::Process> for Process {
            fn from(process: types::process::Process) -> Self {
                Self {
                    name: process.name,
                    pid: process.pid.unwrap(),
                    command: process.command,
                    up_time: process.up_time.map(|t| t.to_rfc3339()).unwrap_or_default(),
                    ..Default::default()
                }
            }
        }
    }
}
