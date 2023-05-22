use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
    thread,
    time::Duration,
};

use anyhow::Error;
use indexmap::IndexMap;
use names::Generator;
use tokio::sync::mpsc;
use tonic::{Request, Response};

use crate::{
    api::{
        objects::v1alpha1::Service,
        superviseur::v1alpha1::{
            control_service_server::ControlService, BuildRequest, BuildResponse, ListRequest,
            ListResponse, ListRunningProcessesRequest, ListRunningProcessesResponse,
            LoadConfigRequest, LoadConfigResponse, RestartRequest, RestartResponse, StartRequest,
            StartResponse, StatusRequest, StatusResponse, StopRequest, StopResponse,
        },
    },
    default_stderr, default_stdout,
    server::macros::{get_project_configuration, save_project_configuration},
    superviseur::{
        core::{ProcessEvent, Superviseur, SuperviseurCommand},
        drivers::setup_drivers,
        provider::{self, kv::kv::Provider},
    },
    types::{
        self, configuration,
        configuration::ConfigurationData,
        process::{Process, State},
    },
    util::convert_dir_path_to_absolute_path,
};

use super::macros::project_exists;

pub struct Control {
    cmd_tx: mpsc::UnboundedSender<SuperviseurCommand>,
    event_tx: mpsc::UnboundedSender<ProcessEvent>,
    superviseur: Superviseur,
    processes: Arc<Mutex<Vec<(Process, String)>>>,
    provider: Arc<Provider>,
    project_map: Arc<Mutex<HashMap<String, String>>>,
}

impl Control {
    pub fn new(
        cmd_tx: mpsc::UnboundedSender<SuperviseurCommand>,
        event_tx: mpsc::UnboundedSender<ProcessEvent>,
        superviseur: Superviseur,
        processes: Arc<Mutex<Vec<(Process, String)>>>,
        provider: Arc<Provider>,
        project_map: Arc<Mutex<HashMap<String, String>>>,
    ) -> Self {
        Self {
            cmd_tx,
            event_tx,
            superviseur,
            processes,
            provider,
            project_map,
        }
    }

    pub fn insert_config_and_get_project_id(
        &self,
        path: String,
        config: ConfigurationData,
    ) -> Result<(String, bool), Error> {
        let mut generator = Generator::default();
        let mut project_map = self.project_map.lock().unwrap();

        let mut is_new = false;
        // check if path is not already loaded
        if !project_map.contains_key(&path) {
            // generate a new id for the project
            let id = generator.next().unwrap();
            project_map.insert(path.clone(), id.clone());
            self.provider.save_configuration(&id, config.clone())?;
            is_new = true;
            setup_drivers(config);
        }
        Ok((project_map.get(&path).map(|x| x.clone()).unwrap(), is_new))
    }

    pub fn get_project_id(&self, path: String) -> Result<String, Error> {
        let project_map = self.project_map.lock().unwrap();
        project_map
            .get(&path)
            .map(|x| x.clone())
            .ok_or_else(|| anyhow::anyhow!("The project with path {} is not loaded", path))
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
        let cfg_format = request.config_format;
        let mut config: ConfigurationData = match cfg_format.as_str() {
            "hcl" => hcl::from_str(&config).map_err(|e| tonic::Status::internal(e.to_string()))?,
            "toml" => {
                toml::from_str(&config).map_err(|e| tonic::Status::internal(e.to_string()))?
            }
            &_ => {
                return Err(tonic::Status::internal(format!(
                    "The config format {} is not supported",
                    cfg_format
                )))
            }
        };

        // set the name of the services
        config.services = config
            .services
            .iter()
            .map(|(name, service)| {
                let mut service = service.clone();
                service.name = name.clone();
                (name.clone(), service)
            })
            .collect();

        // get directory of the config file
        config.context = Some(path.clone());

        let (project_id, is_new_config) = self
            .insert_config_and_get_project_id(path.clone(), config.clone())
            .map_err(|e| {
                tonic::Status::internal(format!("Error while loading config: {}", e.to_string()))
            })?;
        let mut generator = Generator::default();

        // check if the config is already loaded
        if !is_new_config {
            let old_config = self
                .provider
                .build_configuration(&project_id)
                .map_err(|e| tonic::Status::internal(e.to_string()))?;
            // reuse the id of the services
            for (service_name, service) in &mut config.services {
                match old_config
                    .services
                    .iter()
                    .find(|(_, s)| s.name == *service_name)
                {
                    Some((_, old_service)) => {
                        service.id = old_service.id.clone();
                        service.working_dir = convert_dir_path_to_absolute_path(
                            service.working_dir.as_str(),
                            path.as_str(),
                        )
                        .map(|x| x.to_string())
                        .map_err(|e| tonic::Status::internal(e.to_string()))?;
                        // rewacth the directory if watch_dir changed
                        match &service.watch_dir {
                            Some(watch_dir) => {
                                if old_service.watch_dir != Some(watch_dir.clone()) {
                                    self.cmd_tx
                                        .send(SuperviseurCommand::WatchForChanges(
                                            watch_dir.clone(),
                                            service.clone(),
                                            config.project.clone(),
                                        ))
                                        .unwrap();
                                }
                            }
                            None => {}
                        }
                    }
                    None => {
                        service.id = Some(generator.next().unwrap());
                    }
                };
            }
            self.cmd_tx
                .send(SuperviseurCommand::LoadConfig(
                    config.clone(),
                    config.project.clone(),
                ))
                .unwrap();
            // update the config
            save_project_configuration!(self, project_id, config);
        } else {
            config.services = config
                .services
                .into_iter()
                .map(|(key, mut service)| {
                    service.id = Some(generator.next().unwrap());
                    service.working_dir = convert_dir_path_to_absolute_path(
                        service.working_dir.as_str(),
                        path.as_str(),
                    )
                    .map(|x| x.to_string())?;
                    Ok((key, service))
                })
                .collect::<Result<IndexMap<String, configuration::Service>, Error>>()
                .map_err(|e| tonic::Status::internal(e.to_string()))?;

            // update the config
            save_project_configuration!(self, project_id, config);

            let services = config.services.clone();
            let project = config.project.clone();

            for (_, service) in services.into_iter() {
                match &service.watch_dir {
                    Some(watch_dir) => {
                        self.cmd_tx
                            .send(SuperviseurCommand::WatchForChanges(
                                watch_dir.clone(),
                                service.clone(),
                                project.clone(),
                            ))
                            .unwrap();
                    }
                    None => {}
                }
            }

            self.cmd_tx
                .send(SuperviseurCommand::LoadConfig(
                    config.clone(),
                    config.project.clone(),
                ))
                .unwrap();
        }

        let mut config = get_project_configuration!(self, project_id);

        let services = config.services.clone();
        let mut services = services.into_iter();

        // convert services dependencies to ids
        for (_, service) in &mut config.services {
            let mut dependencies = vec![];
            for dependency in &service.depends_on {
                match services.find(|(name, _)| *name == *dependency) {
                    Some((_, service)) => {
                        dependencies.push(service.id.clone().unwrap());
                    }
                    None => {
                        return Err(tonic::Status::not_found(format!(
                            "Service {} not found",
                            dependency
                        )));
                    }
                }
            }
            service.dependencies = dependencies;
        }

        let services = config.services.clone();

        for (_, service) in services.into_iter() {
            self.cmd_tx
                .send(SuperviseurCommand::Load(service, config.project.clone()))
                .map_err(|e| tonic::Status::internal(e.to_string()))?;
        }

        thread::sleep(Duration::from_millis(500));

        Ok(Response::new(LoadConfigResponse { success: true }))
    }

    async fn start(
        &self,
        request: Request<StartRequest>,
    ) -> Result<Response<StartResponse>, tonic::Status> {
        let request = request.into_inner();
        let path = request.config_file_path;
        let name = request.name;
        let build = request.build;
        let project_id = self
            .get_project_id(path.clone())
            .map_err(|e| tonic::Status::not_found(e.to_string()))?;

        project_exists!(self, project_id);

        let config = get_project_configuration!(self, project_id);

        if name.len() > 0 {
            let (_, service) = config
                .services
                .iter()
                .find(|(_, s)| s.name == name)
                .ok_or_else(|| tonic::Status::not_found("Service not found"))?;

            self.cmd_tx
                .send(SuperviseurCommand::Start(
                    service.clone(),
                    config.project.clone(),
                    build,
                ))
                .map_err(|e| tonic::Status::internal(e.to_string()))?;
            return Ok(Response::new(StartResponse { success: true }));
        }

        self.cmd_tx
            .send(SuperviseurCommand::StartAll(config.project.clone(), build))
            .map_err(|e| tonic::Status::internal(e.to_string()))?;

        Ok(Response::new(StartResponse { success: true }))
    }

    async fn stop(
        &self,
        request: Request<StopRequest>,
    ) -> Result<Response<StopResponse>, tonic::Status> {
        let request = request.into_inner();
        let path = request.config_file_path;
        let name = request.name;
        let project_id = self
            .get_project_id(path.clone())
            .map_err(|e| tonic::Status::not_found(e.to_string()))?;

        project_exists!(self, project_id);

        let config = get_project_configuration!(self, project_id);

        if name.len() > 0 {
            let (_, service) = config
                .services
                .iter()
                .find(|(_, s)| s.name == name)
                .ok_or_else(|| tonic::Status::not_found("Service not found"))?;

            self.cmd_tx
                .send(SuperviseurCommand::Stop(
                    service.clone(),
                    config.project.clone(),
                ))
                .unwrap();
            return Ok(Response::new(StopResponse { success: true }));
        }

        self.cmd_tx
            .send(SuperviseurCommand::StopAll(config.project.clone()))
            .unwrap();

        Ok(Response::new(StopResponse { success: true }))
    }

    async fn restart(
        &self,
        request: Request<RestartRequest>,
    ) -> Result<Response<RestartResponse>, tonic::Status> {
        let request = request.into_inner();
        let path = request.config_file_path;
        let name = request.name;
        let project_id = self
            .get_project_id(path.clone())
            .map_err(|e| tonic::Status::not_found(e.to_string()))?;

        project_exists!(self, project_id);

        let config = get_project_configuration!(self, project_id);

        if name.len() > 0 {
            let (_, service) = config
                .services
                .iter()
                .find(|(_, s)| s.name == name)
                .ok_or_else(|| tonic::Status::not_found("Service not found"))?;

            self.cmd_tx
                .send(SuperviseurCommand::Restart(
                    service.clone(),
                    config.project.clone(),
                ))
                .map_err(|e| tonic::Status::internal(e.to_string()))?;
            return Ok(Response::new(RestartResponse { success: true }));
        }

        self.cmd_tx
            .send(SuperviseurCommand::RestartAll(config.project.clone()))
            .map_err(|e| tonic::Status::internal(e.to_string()))?;

        Ok(Response::new(RestartResponse { success: true }))
    }

    async fn status(
        &self,
        request: Request<StatusRequest>,
    ) -> Result<Response<StatusResponse>, tonic::Status> {
        let request = request.into_inner();
        let path = request.config_file_path;
        let name = request.name;
        let project_id = self
            .get_project_id(path.clone())
            .map_err(|e| tonic::Status::not_found(e.to_string()))?;

        project_exists!(self, project_id);

        let config = get_project_configuration!(self, project_id);

        let (_, service) = config
            .services
            .iter()
            .find(|(_, s)| s.name == name)
            .ok_or_else(|| tonic::Status::not_found("Service not found"))?;

        let processes = self.processes.lock().unwrap();
        let process = processes
            .iter()
            .find(|(p, _)| p.name == name && p.project == config.project)
            .map(|(p, _)| p.clone())
            .unwrap_or(Process {
                name: name.clone(),
                project: config.project.clone(),
                r#type: service.r#type.clone(),
                state: types::process::State::Stopped,
                command: service.command.clone(),
                description: service.description.clone(),
                working_dir: service.working_dir.clone(),
                env: service.env.clone(),
                auto_restart: service.autorestart.unwrap_or(false),
                stdout: service
                    .stdout
                    .clone()
                    .unwrap_or(default_stdout!(config.project, service.name)),
                stderr: service
                    .stderr
                    .clone()
                    .unwrap_or(default_stderr!(config.project, service.name)),
                port: service.port,
                ..Default::default()
            });
        Ok(Response::new(StatusResponse {
            process: Some(process.into()),
        }))
    }

    async fn list(
        &self,
        request: Request<ListRequest>,
    ) -> Result<Response<ListResponse>, tonic::Status> {
        let request = request.into_inner();
        let path = request.config_file_path;
        let project_id = self
            .get_project_id(path.clone())
            .map_err(|e| tonic::Status::not_found(e.to_string()))?;

        project_exists!(self, project_id);

        let config = get_project_configuration!(self, project_id);
        let services = config.services.clone();
        let mut list_response = ListResponse {
            services: services
                .into_iter()
                .map(|(_, x)| Service::from(x))
                .collect(),
        };

        let processes = self.processes.lock().unwrap();
        for service in list_response.services.iter_mut() {
            let process = processes
                .iter()
                .find(|(p, _)| p.name == service.name)
                .map(|(p, _)| p);
            if let Some(process) = process {
                service.status = process.state.to_string().to_uppercase();
            } else {
                service.status = "STOPPED".to_string();
            }
        }

        Ok(Response::new(list_response))
    }

    async fn list_running_processes(
        &self,
        _request: Request<ListRunningProcessesRequest>,
    ) -> Result<Response<ListRunningProcessesResponse>, tonic::Status> {
        let processes = self.processes.lock().unwrap();
        let list_response = ListRunningProcessesResponse {
            processes: processes
                .iter()
                .filter(|(p, _)| p.state == State::Running)
                .map(|(p, _)| Into::into(p.clone()))
                .collect(),
        };
        Ok(Response::new(list_response))
    }

    async fn build(
        &self,
        request: Request<BuildRequest>,
    ) -> Result<Response<BuildResponse>, tonic::Status> {
        let request = request.into_inner();
        let path = request.config_file_path;
        let name = request.name;
        let project_id = self
            .get_project_id(path.clone())
            .map_err(|e| tonic::Status::not_found(e.to_string()))?;

        project_exists!(self, project_id);

        let config = get_project_configuration!(self, project_id);

        if name.len() > 0 {
            let (_, service) = config
                .services
                .iter()
                .find(|(_, s)| s.name == name)
                .ok_or_else(|| tonic::Status::not_found("Service not found"))?;

            service
                .build
                .as_ref()
                .ok_or_else(|| tonic::Status::invalid_argument("Service has no build command"))?;

            self.cmd_tx
                .send(SuperviseurCommand::Build(
                    service.clone(),
                    config.project.clone(),
                ))
                .map_err(|e| tonic::Status::internal(e.to_string()))?;
            return Ok(Response::new(BuildResponse { success: true }));
        }

        self.cmd_tx
            .send(SuperviseurCommand::BuildAll(config.project.clone()))
            .map_err(|e| tonic::Status::internal(e.to_string()))?;
        Ok(Response::new(BuildResponse { success: true }))
    }
}
