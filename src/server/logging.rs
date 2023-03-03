use crate::api::superviseur::v1alpha1::{
    logging_service_server::LoggingService, LogRequest, LogResponse, TailRequest, TailResponse,
};
use tokio_stream::wrappers::ReceiverStream;
use tonic::{Request, Response, Status};

#[derive(Default)]
pub struct Logging {}

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
