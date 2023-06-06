use async_graphql::Object;

use super::{process::Process, service::Service};

#[derive(Default, Clone)]
pub struct TailLogStream {
    pub id: String,
    pub line: String,
}

#[Object]
impl TailLogStream {
    async fn line(&self) -> &str {
        &self.line
    }
}

#[derive(Default, Clone)]
pub struct LogStream {
    pub id: String,
    pub line: String,
}

#[Object]
impl LogStream {
    async fn line(&self) -> &str {
        &self.line
    }
}

#[derive(Default, Clone)]
pub struct ServiceStarting {
    pub payload: Service,
    pub process: Process,
}

#[Object]
impl ServiceStarting {
    async fn payload(&self) -> &Service {
        &self.payload
    }

    async fn process(&self) -> &Process {
        &self.process
    }
}

#[derive(Default, Clone)]
pub struct ServiceStopping {
    pub payload: Service,
    pub process: Process,
}

#[Object]
impl ServiceStopping {
    async fn payload(&self) -> &Service {
        &self.payload
    }

    async fn process(&self) -> &Process {
        &self.process
    }
}

#[derive(Default, Clone)]
pub struct ServiceStarted {
    pub payload: Service,
    pub process: Process,
}

#[Object]
impl ServiceStarted {
    async fn payload(&self) -> &Service {
        &self.payload
    }

    async fn process(&self) -> &Process {
        &self.process
    }
}

#[derive(Default, Clone)]
pub struct ServiceStopped {
    pub payload: Service,
    pub process: Process,
}

#[Object]
impl ServiceStopped {
    async fn payload(&self) -> &Service {
        &self.payload
    }

    async fn process(&self) -> &Process {
        &self.process
    }
}

#[derive(Default, Clone)]
pub struct ServiceRestarted {
    pub payload: Service,
    pub process: Process,
}

#[Object]
impl ServiceRestarted {
    async fn payload(&self) -> &Service {
        &self.payload
    }

    async fn process(&self) -> &Process {
        &self.process
    }
}

#[derive(Default, Clone)]
pub struct AllServicesStarted {
    pub payload: Vec<Service>,
}

#[Object]
impl AllServicesStarted {
    async fn payload(&self) -> &Vec<Service> {
        &self.payload
    }
}

#[derive(Default, Clone)]
pub struct AllServicesStopped {
    pub payload: Vec<Service>,
}

#[Object]
impl AllServicesStopped {
    async fn payload(&self) -> &Vec<Service> {
        &self.payload
    }
}

#[derive(Default, Clone)]
pub struct AllServicesRestarted {
    pub payload: Vec<Service>,
}

#[Object]
impl AllServicesRestarted {
    async fn payload(&self) -> &Vec<Service> {
        &self.payload
    }
}

#[derive(Default, Clone)]
pub struct ServiceBuilding {
    pub payload: Service,
    pub process: Process,
}

#[Object]
impl ServiceBuilding {
    async fn payload(&self) -> &Service {
        &self.payload
    }
}

#[derive(Default, Clone)]
pub struct ServiceBuilt {
    pub payload: Service,
    pub process: Process,
}

#[Object]
impl ServiceBuilt {
    async fn payload(&self) -> &Service {
        &self.payload
    }
}

#[derive(Default, Clone)]
pub struct AllServicesBuilt {
    pub payload: Vec<Service>,
}

#[Object]
impl AllServicesBuilt {
    async fn payload(&self) -> &Vec<Service> {
        &self.payload
    }
}

#[derive(Default, Clone)]
pub struct ProjectOpened {
    pub id: String,
    pub line: String,
}

#[Object]
impl ProjectOpened {
    async fn line(&self) -> &str {
        &self.line
    }
}
