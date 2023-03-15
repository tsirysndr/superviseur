use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
    thread,
};

use tokio::sync::mpsc;
use tonic::{Request, Response};

use crate::{
    api::superviseur::v1alpha1::{
        core_service_server::CoreService, GetVersionRequest, GetVersionResponse,
        StartWebDashboardRequest, StartWebDashboardResponse,
    },
    superviseur::{Superviseur, SuperviseurCommand},
    types::{configuration::ConfigurationData, process::Process},
    webui::start_webui,
};

pub struct Core {
    cmd_tx: mpsc::UnboundedSender<SuperviseurCommand>,
    superviseur: Superviseur,
    processes: Arc<Mutex<Vec<(Process, String)>>>,
    config_map: Arc<Mutex<HashMap<String, ConfigurationData>>>,
}

impl Core {
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
impl CoreService for Core {
    async fn get_version(
        &self,
        _request: Request<GetVersionRequest>,
    ) -> Result<Response<GetVersionResponse>, tonic::Status> {
        Ok(Response::new(GetVersionResponse {
            version: env!("CARGO_PKG_VERSION").to_string(),
        }))
    }

    async fn start_web_dashboard(
        &self,
        request: Request<StartWebDashboardRequest>,
    ) -> Result<Response<StartWebDashboardResponse>, tonic::Status> {
        let request = request.into_inner();
        let path = request.config_file_path;

        let cmd_tx = self.cmd_tx.clone();
        let superviseur = self.superviseur.clone();
        let processes = self.processes.clone();
        let config_map = self.config_map.clone();

        thread::spawn(move || {
            let rt = actix_web::rt::Runtime::new().unwrap();
            rt.block_on(async move {
                start_webui(path, cmd_tx, superviseur, processes, config_map).await
            })
            .map_err(|e| eprintln!("Error: {}", e))
            .unwrap_or_default();
        });

        let ip = local_ip_addr::get_local_ip_address()
            .map_err(|e| tonic::Status::internal(e.to_string()))?;
        let port = 5478;
        Ok(Response::new(StartWebDashboardResponse {
            url: format!("http://{}:{}", ip, port),
        }))
    }
}
