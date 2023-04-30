use crate::{client::Client, service::Service};

pub struct Project {
    pub client: Client,
    pub name: String,
    pub services: Vec<Service>,
}

impl Project {
    pub fn new(client: Client) -> Project {
        Project {
            client,
            name: String::new(),
            services: Vec::new(),
        }
    }

    pub fn with_name(mut self, name: &str) -> Self {
        self.name = name.to_string();
        self
    }

    pub fn add_service(mut self, service: Service) -> Self {
        self.services.push(service);
        self
    }

    pub fn start(self, service_id: Option<&str>) {}

    pub fn stop(self, service_id: Option<&str>) {}

    pub fn restart(self, service_id: Option<&str>) {}

    pub fn status(self, service_id: Option<&str>) {}

    pub fn list(self) {}

    pub fn logs(self, service_id: &str) {}
}
