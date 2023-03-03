use tonic::{Request, Response};

use crate::{
    api::superviseur::v1alpha1::{
        control_service_server::ControlService, ListRequest, ListResponse, RestartRequest,
        RestartResponse, StartRequest, StartResponse, StatusRequest, StatusResponse, StopRequest,
        StopResponse,
    },
    superviseur::Superviseur,
};

pub struct Control {
    superviseur: Superviseur,
}

impl Control {
    pub fn new(superviseur: Superviseur) -> Self {
        Self { superviseur }
    }
}

#[tonic::async_trait]
impl ControlService for Control {
    async fn start(
        &self,
        _request: Request<StartRequest>,
    ) -> Result<Response<StartResponse>, tonic::Status> {
        unimplemented!()
    }

    async fn stop(
        &self,
        _request: Request<StopRequest>,
    ) -> Result<Response<StopResponse>, tonic::Status> {
        unimplemented!()
    }

    async fn restart(
        &self,
        _request: Request<RestartRequest>,
    ) -> Result<Response<RestartResponse>, tonic::Status> {
        unimplemented!()
    }

    async fn status(
        &self,
        _request: Request<StatusRequest>,
    ) -> Result<Response<StatusResponse>, tonic::Status> {
        unimplemented!()
    }

    async fn list(
        &self,
        _request: Request<ListRequest>,
    ) -> Result<Response<ListResponse>, tonic::Status> {
        unimplemented!()
    }
}
