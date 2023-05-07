use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use tokio::sync::mpsc;

use crate::{
    api::objects::v1alpha1::Project as ProjectProto,
    api::objects::v1alpha1::Service as ServiceProto,
    api::superviseur::v1alpha1::{
        project_service_server::ProjectService, GetProjectRequest, GetProjectResponse,
        ListProjectsRequest, ListProjectsResponse,
    },
    superviseur::core::SuperviseurCommand,
    types::{configuration::ConfigurationData, process::Process},
};

pub struct Project {
    cmd_tx: mpsc::UnboundedSender<SuperviseurCommand>,
    processes: Arc<Mutex<Vec<(Process, String)>>>,
    config_map: Arc<Mutex<HashMap<String, ConfigurationData>>>,
    project_map: Arc<Mutex<HashMap<String, String>>>,
}

impl Project {
    pub fn new(
        cmd_tx: mpsc::UnboundedSender<SuperviseurCommand>,
        processes: Arc<Mutex<Vec<(Process, String)>>>,
        config_map: Arc<Mutex<HashMap<String, ConfigurationData>>>,
        project_map: Arc<Mutex<HashMap<String, String>>>,
    ) -> Self {
        Self {
            cmd_tx,
            processes,
            config_map,
            project_map,
        }
    }
}

#[tonic::async_trait]
impl ProjectService for Project {
    async fn list_projects(
        &self,
        _request: tonic::Request<ListProjectsRequest>,
    ) -> Result<tonic::Response<ListProjectsResponse>, tonic::Status> {
        let config_map = self.config_map.lock().unwrap();

        let projects = config_map
            .iter()
            .map(|(id, config)| ProjectProto {
                id: id.clone(),
                name: config.project.clone(),
                context: config
                    .context
                    .as_ref()
                    .map(|x| x.to_string())
                    .unwrap_or_default(),
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
        let config_map = self.config_map.lock().unwrap();
        let project = config_map.get(&id).map(|x| ProjectProto {
            id: id.clone(),
            name: x.project.clone(),
            context: x
                .context
                .as_ref()
                .map(|x| x.to_string())
                .unwrap_or_default(),
            services: x
                .services
                .iter()
                .map(|(_, service)| ServiceProto::from(service.clone()))
                .collect(),
            ..Default::default()
        });
        Ok(tonic::Response::new(GetProjectResponse { project }))
    }
}
