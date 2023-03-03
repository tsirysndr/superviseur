use crate::{
    api::superviseur::v1alpha1::{
        logging_service_server::LoggingService, LogRequest, LogResponse, TailRequest, TailResponse,
    },
    superviseur::Superviseur,
};
use tokio_stream::wrappers::ReceiverStream;
use tonic::{Request, Response, Status};

pub struct Logging {
    superviseur: Superviseur,
}

impl Logging {
    pub fn new(superviseur: Superviseur) -> Self {
        Self { superviseur }
    }
}

#[tonic::async_trait]
impl LoggingService for Logging {
    async fn log(&self, _request: Request<LogRequest>) -> Result<Response<LogResponse>, Status> {
        unimplemented!()
    }

    type TailStream = ReceiverStream<Result<TailResponse, Status>>;

    async fn tail(
        &self,
        _request: Request<TailRequest>,
    ) -> Result<Response<Self::TailStream>, Status> {
        unimplemented!()
    }
}
