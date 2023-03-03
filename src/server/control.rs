use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use tokio::sync::mpsc;
use tonic::{Request, Response};

use crate::{
    api::{
        objects::v1alpha1::Service,
        superviseur::v1alpha1::{
            control_service_server::ControlService, ListRequest, ListResponse, LoadConfigRequest,
            LoadConfigResponse, RestartRequest, RestartResponse, StartRequest, StartResponse,
            StatusRequest, StatusResponse, StopRequest, StopResponse,
        },
    },
    superviseur::{Superviseur, SuperviseurCommand},
    types::{configuration::ConfigurationData, process::Process},
};

pub struct Control {
    cmd_tx: mpsc::UnboundedSender<SuperviseurCommand>,
    superviseur: Superviseur,
    processes: Arc<Mutex<Vec<(Process, String)>>>,
    config_map: Arc<Mutex<HashMap<String, ConfigurationData>>>,
}

impl Control {
    pub fn new(
        cmd_tx: mpsc::UnboundedSender<SuperviseurCommand>,
        superviseur: Superviseur,
        processes: Arc<Mutex<Vec<(Process, String)>>>,
        config_map: Arc<Mutex<HashMap<String, ConfigurationData>>>,
    ) -> Self {
        Self {
            cmd_tx,
            superviseur,
            processes,
            config_map,
        }
    }
}

#[tonic::async_trait]
impl ControlService for Control {
    async fn load_config(
        &self,
        request: Request<LoadConfigRequest>,
    ) -> Result<Response<LoadConfigResponse>, tonic::Status> {
        let request = request.into_inner();
        let config = request.config;
        let path = request.file_path;
        let config: ConfigurationData =
            hcl::from_str(&config).map_err(|e| tonic::Status::internal(e.to_string()))?;
        self.config_map
            .lock()
            .unwrap()
            .insert(path.clone(), config.clone());

        for service in config.services {
            self.cmd_tx
                .send(SuperviseurCommand::Load(service, config.project.clone()))
                .unwrap();
        }

        Ok(Response::new(LoadConfigResponse { success: true }))
    }
    async fn start(
        &self,
        request: Request<StartRequest>,
    ) -> Result<Response<StartResponse>, tonic::Status> {
        let request = request.into_inner();
        let path = request.config_file_path;
        let name = request.name;
        let config_map = self.config_map.lock().unwrap();

        if !config_map.contains_key(&path) {
            return Err(tonic::Status::not_found("Config file not found"));
        }

        let config = config_map.get(&path).unwrap();
        let service = config
            .services
            .iter()
            .find(|s| s.name == name)
            .ok_or_else(|| tonic::Status::not_found("Service not found"))?;

        self.cmd_tx
            .send(SuperviseurCommand::Start(
                service.clone(),
                config.project.clone(),
            ))
            .unwrap();

        Ok(Response::new(StartResponse { success: true }))
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
        request: Request<ListRequest>,
    ) -> Result<Response<ListResponse>, tonic::Status> {
        let request = request.into_inner();
        let _project = request.project;
        let processes = self.processes.lock().unwrap();
        Ok(Response::new(ListResponse {
            services: processes
                .iter()
                .map(|(p, _)| Service {
                    name: p.name.clone(),
                    status: p.state.to_string(),
                    ..Default::default()
                })
                .collect::<Vec<Service>>(),
        }))
    }
}
