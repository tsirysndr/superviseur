use async_graphql::Object;

use super::service::Service;

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
pub struct ServiceStarted {
    pub payload: Service,
}

#[Object]
impl ServiceStarted {
    async fn payload(&self) -> &Service {
        &self.payload
    }
}

#[derive(Default, Clone)]
pub struct ServiceStopped {
    pub payload: Service,
}

#[Object]
impl ServiceStopped {
    async fn payload(&self) -> &Service {
        &self.payload
    }
}

#[derive(Default, Clone)]
pub struct ServiceRestarted {
    pub payload: Service,
}

#[Object]
impl ServiceRestarted {
    async fn payload(&self) -> &Service {
        &self.payload
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
