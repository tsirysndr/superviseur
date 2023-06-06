use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
    time::Duration,
};

use superviseur_code::start_code_tunnel;
use superviseur_provider::kv::kv::Provider;
use superviseur_types::{command::SuperviseurCommand, process::Process};
use tokio::{sync::mpsc, time::sleep};
use tokio_stream::wrappers::ReceiverStream;
use tonic::{Response, Status};

use crate::{
    api::objects::v1alpha1::Project as ProjectProto,
    api::superviseur::v1alpha1::{
        project_service_server::ProjectService, GetProjectRequest, GetProjectResponse,
        ListProjectsRequest, ListProjectsResponse,
    },
    api::{
        objects::v1alpha1::Service as ServiceProto,
        superviseur::v1alpha1::{OpenProjectRequest, OpenProjectResponse},
    },
};

pub struct Project {
    cmd_tx: mpsc::UnboundedSender<SuperviseurCommand>,
    processes: Arc<Mutex<Vec<(Process, String)>>>,
    provider: Arc<Provider>,
    project_map: Arc<Mutex<HashMap<String, String>>>,
}

impl Project {
    pub fn new(
        cmd_tx: mpsc::UnboundedSender<SuperviseurCommand>,
        processes: Arc<Mutex<Vec<(Process, String)>>>,
        provider: Arc<Provider>,
        project_map: Arc<Mutex<HashMap<String, String>>>,
    ) -> Self {
        Self {
            cmd_tx,
            processes,
            provider,
            project_map,
        }
    }
}

#[tonic::async_trait]
impl ProjectService for Project {
    type OpenProjectStream = ReceiverStream<Result<OpenProjectResponse, Status>>;

    async fn list_projects(
        &self,
        _request: tonic::Request<ListProjectsRequest>,
    ) -> Result<tonic::Response<ListProjectsResponse>, tonic::Status> {
        let projects = self
            .provider
            .all_projects()
            .map_err(|e| tonic::Status::internal(e.to_string()))?;

        let projects = projects
            .into_iter()
            .map(|(id, name, context)| ProjectProto {
                id,
                name,
                context,
                ..Default::default()
            })
            .collect();

        Ok(tonic::Response::new(ListProjectsResponse { projects }))
    }

    async fn get_project(
        &self,
        request: tonic::Request<GetProjectRequest>,
    ) -> Result<tonic::Response<GetProjectResponse>, tonic::Status> {
        let request = request.into_inner();
        let id = request.id;
        let (name, context) = self
            .provider
            .project(&id)
            .map_err(|e| tonic::Status::internal(e.to_string()))?;
        let config = self
            .provider
            .build_configuration(&id)
            .map_err(|e| tonic::Status::internal(e.to_string()))?;
        let project = Some(ProjectProto {
            id: id.clone(),
            name,
            context,
            services: config
                .services
                .iter()
                .map(|(_, service)| ServiceProto::from(service.clone()))
                .collect(),
            ..Default::default()
        });
        Ok(tonic::Response::new(GetProjectResponse { project }))
    }

    async fn open_project(
        &self,
        request: tonic::Request<OpenProjectRequest>,
    ) -> Result<tonic::Response<Self::OpenProjectStream>, tonic::Status> {
        let request = request.into_inner();
        let id = request.id;
        let (_, context) = self
            .provider
            .project(&id)
            .map_err(|e| tonic::Status::internal(e.to_string()))?;

        let (tx, rx) = tokio::sync::mpsc::channel(1024);
        let (sender, mut receiver) = tokio::sync::mpsc::unbounded_channel();

        tokio::spawn(async move {
            loop {
                let tx = tx.clone();
                match receiver.recv().await {
                    Some(line) => {
                        let response = OpenProjectResponse { line };
                        tx.send(Ok(response)).await.unwrap();
                    }
                    None => {
                        break;
                    }
                }
            }
        });

        sleep(Duration::from_secs(1)).await;

        start_code_tunnel(sender, &context).map_err(|e| tonic::Status::internal(e.to_string()))?;

        Ok(Response::new(ReceiverStream::new(rx)))
    }
}
