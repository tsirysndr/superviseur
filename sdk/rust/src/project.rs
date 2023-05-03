use crate::{client::Client, query::build_nested_with_service_query, service::Service};

#[derive(Default)]
pub struct Project {
    pub client: Client,
    pub name: String,
    pub context: String,
    pub services: Vec<Service>,
}

impl Project {
    pub fn new(client: Client) -> Project {
        Project {
            client,
            name: String::new(),
            context: String::new(),
            services: Vec::new(),
        }
    }

    pub fn with_context(mut self, context: &str) -> Self {
        self.context = context.to_string();
        self
    }

    pub fn with_name(mut self, name: &str) -> Self {
        self.name = name.to_string();
        self
    }

    pub fn with_service(mut self, service: Service) -> Self {
        self.services.push(service);
        self
    }

    pub fn start(self, service_id: &str) {}

    pub fn stop(self, service_id: &str) {}

    pub fn restart(self, service_id: &str) {}

    pub fn status(self, service_id: &str) {}

    pub fn start_all(self) {}

    pub fn stop_all(self) {}

    pub fn restart_all(self) {}

    pub fn services(self) {}

    pub fn processes(self) {}

    pub fn logs(self, service_id: &str) {}

    pub fn stdout(self) {
        build_nested_with_service_query(self.services);
    }

    pub fn add_env_var(self, service_id: &str, name: &str, value: &str) {}

    pub fn remove_env_var(self, service_id: &str, name: &str) {}

    pub fn update_env_var(self, service_id: &str, name: &str, value: &str) {}
}
