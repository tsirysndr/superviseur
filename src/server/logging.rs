use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader, Read},
    sync::{Arc, Mutex},
};

use crate::{
    api::superviseur::v1alpha1::{
        logging_service_server::LoggingService, EventsRequest, EventsResponse, LogDetails,
        LogRequest, LogResponse, SearchRequest, SearchResponse, TailRequest, TailResponse,
    },
    superviseur::{core::Superviseur, logs::LogEngine},
    types::{
        configuration::ConfigurationData,
        events::{
            SuperviseurEvent, ALL_SERVICES_BUILT, ALL_SERVICES_RESTARTED, ALL_SERVICES_STARTED,
            ALL_SERVICES_STOPPED, SERVICE_BUILDING, SERVICE_BUILT, SERVICE_CRASHED, SERVICE_ERROR,
            SERVICE_LOGS, SERVICE_RESTARTED, SERVICE_RESTARTING, SERVICE_SETUP_ENV,
            SERVICE_STARTED, SERVICE_STARTING, SERVICE_STOPPED, SERVICE_STOPPING,
        },
        process::Process,
    },
};
use anyhow::Error;
use chrono::{TimeZone, Utc};
use tokio_stream::wrappers::ReceiverStream;
use tonic::{Request, Response, Status};

use super::return_event;

pub struct Logging {
    superviseur: Superviseur,
    processes: Arc<Mutex<Vec<(Process, String)>>>,
    config_map: Arc<Mutex<HashMap<String, ConfigurationData>>>,
    project_map: Arc<Mutex<HashMap<String, String>>>,
    log_engine: Arc<Mutex<LogEngine>>,
    superviseur_events_rx:
        Arc<tokio::sync::Mutex<tokio::sync::mpsc::UnboundedReceiver<SuperviseurEvent>>>,
}

impl Logging {
    pub fn new(
        superviseur: Superviseur,
        processes: Arc<Mutex<Vec<(Process, String)>>>,
        config_map: Arc<Mutex<HashMap<String, ConfigurationData>>>,
        project_map: Arc<Mutex<HashMap<String, String>>>,
        log_engine: Arc<Mutex<LogEngine>>,
        superviseur_events_rx: Arc<
            tokio::sync::Mutex<tokio::sync::mpsc::UnboundedReceiver<SuperviseurEvent>>,
        >,
    ) -> Self {
        Self {
            superviseur,
            processes,
            config_map,
            project_map,
            log_engine,
            superviseur_events_rx,
        }
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
impl LoggingService for Logging {
    type TailStream = ReceiverStream<Result<TailResponse, Status>>;
    type LogStream = ReceiverStream<Result<LogResponse, Status>>;
    type EventsStream = ReceiverStream<Result<EventsResponse, Status>>;

    async fn log(&self, request: Request<LogRequest>) -> Result<Response<Self::LogStream>, Status> {
        let request = request.into_inner();
        let path = request.config_file_path;
        let name = request.service;
        let project_id = self
            .get_project_id(path.clone())
            .map_err(|e| tonic::Status::not_found(e.to_string()))?;
        let config_map = self.config_map.lock().unwrap();

        if !config_map.contains_key(&project_id) {
            return Err(tonic::Status::not_found("Config file not found"));
        }

        let config = config_map.get(&project_id).unwrap();

        let (_, service) = config
            .services
            .iter()
            .find(|(_, s)| s.name == name)
            .ok_or_else(|| tonic::Status::not_found("Service not found"))?;

        let log_file =
            File::open(&service.stdout).map_err(|e| tonic::Status::internal(e.to_string()))?;

        let (tx, rx) = tokio::sync::mpsc::channel(1024);

        let reader = BufReader::new(log_file);

        for line in reader.lines() {
            let line = line.map_err(|e| tonic::Status::internal(e.to_string()))?;
            let tx = tx.clone();
            tokio::spawn(async move {
                tx.send(Ok(LogResponse {
                    line: line.to_string(),
                }))
                .await
                .map_err(|e| tonic::Status::internal(e.to_string()))
            });
        }

        if request.follow {
            let tx = tx.clone();
            let stdout = service.stdout.clone();
            tokio::spawn(async move {
                let mut file = File::open(&stdout).unwrap();
                let mut buf = [0; 1024];
                loop {
                    let n = file.read(&mut buf).unwrap();
                    if n == 0 {
                        tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
                        continue;
                    }
                    let line = String::from_utf8_lossy(&buf[0..n]);
                    let line = line.trim_end_matches('\n');
                    tx.send(Ok(LogResponse {
                        line: line.to_string(),
                    }))
                    .await
                    .unwrap_or_default();
                }
            });
        }

        Ok(Response::new(ReceiverStream::new(rx)))
    }

    async fn tail(
        &self,
        request: Request<TailRequest>,
    ) -> Result<Response<Self::TailStream>, Status> {
        let request = request.into_inner();
        let path = request.config_file_path;
        let name = request.service;
        let project_id = self
            .get_project_id(path.clone())
            .map_err(|e| tonic::Status::not_found(e.to_string()))?;
        let config_map = self.config_map.lock().unwrap();

        if !config_map.contains_key(&project_id) {
            return Err(tonic::Status::not_found("Config file not found"));
        }

        let config = config_map.get(&project_id).unwrap();

        let (_, service) = config
            .services
            .iter()
            .find(|(_, s)| s.name == name)
            .ok_or_else(|| tonic::Status::not_found("Service not found"))?;

        let log_file =
            File::open(&service.stdout).map_err(|e| tonic::Status::internal(e.to_string()))?;

        let (tx, rx) = tokio::sync::mpsc::channel(1024);

        let reader = BufReader::new(log_file);
        let num_lines = request.lines as usize;

        // Read the last `num_lines` lines of the file
        let mut lines: Vec<String> = reader
            .lines()
            .map(|line| line.unwrap())
            .collect::<Vec<String>>();

        let tail_lines = lines.split_off(lines.len().saturating_sub(num_lines));

        for line in tail_lines {
            let tx = tx.clone();
            tokio::spawn(async move {
                tx.send(Ok(TailResponse {
                    line: line.to_string(),
                }))
                .await
                .map_err(|e| tonic::Status::internal(e.to_string()))
            });
        }

        if request.follow {
            let tx = tx.clone();
            let stdout = service.stdout.clone();
            tokio::spawn(async move {
                let mut file = File::open(&stdout).unwrap();
                let mut buf = [0; 1024];
                loop {
                    let n = file.read(&mut buf).unwrap();
                    if n == 0 {
                        tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
                        continue;
                    }
                    let line = String::from_utf8_lossy(&buf[0..n]);
                    let line = line.trim_end_matches('\n');
                    tx.send(Ok(TailResponse {
                        line: line.to_string(),
                    }))
                    .await
                    .unwrap();
                }
            });
        }

        Ok(Response::new(ReceiverStream::new(rx)))
    }

    async fn search(
        &self,
        request: Request<SearchRequest>,
    ) -> Result<Response<SearchResponse>, tonic::Status> {
        let request = request.into_inner();
        let service = request.service;
        let term = request.term;
        let path = request.config_file_path;

        let project_id = self
            .get_project_id(path.clone())
            .map_err(|e| tonic::Status::not_found(e.to_string()))?;
        let config_map = self.config_map.lock().unwrap();

        if !config_map.contains_key(&project_id) {
            return Err(tonic::Status::not_found("Config file not found"));
        }

        let config = config_map.get(&project_id).unwrap();

        let query = format!("{} AND {} AND {}", config.project, service, term);
        let log_engine = self.log_engine.lock().unwrap();
        let result = log_engine.search_in_service(&query).map_err(|e| {
            tonic::Status::internal(format!("Error searching in service: {}", e.to_string()))
        })?;
        let response = SearchResponse {
            log_details: result
                .iter()
                .map(|r| {
                    let timestamp = r.date.into_timestamp_secs();
                    let date = Utc.timestamp_opt(timestamp, 0).unwrap();
                    LogDetails {
                        line: r.line.clone(),
                        project: r.project.clone(),
                        service: r.service.clone(),
                        date: date.to_rfc3339(),
                        output: r.output.clone(),
                    }
                })
                .collect(),
        };
        Ok(Response::new(response))
    }

    async fn events(
        &self,
        request: Request<EventsRequest>,
    ) -> Result<Response<Self::EventsStream>, Status> {
        let (tx, rx) = tokio::sync::mpsc::channel(1024);
        let events_rx = Arc::clone(&self.superviseur_events_rx);
        let request = request.into_inner();
        let path = request.config_file_path;
        let service_name = request.service;

        let project_id = self
            .get_project_id(path.clone())
            .map_err(|e| tonic::Status::not_found(e.to_string()))?;
        let config_map = self.config_map.lock().unwrap();

        if !config_map.contains_key(&project_id) {
            return Err(tonic::Status::not_found("Config file not found"));
        }

        tokio::spawn(async move {
            loop {
                let mut events_rx = events_rx.lock().await;
                let tx = tx.clone();

                match events_rx.recv().await {
                    Some(SuperviseurEvent::Starting(project, service)) => {
                        return_event!(
                            tx,
                            service_name,
                            SERVICE_STARTING,
                            project,
                            service,
                            "".into()
                        )
                    }

                    Some(SuperviseurEvent::Stopping(project, service)) => {
                        return_event!(
                            tx,
                            service_name,
                            SERVICE_STOPPING,
                            project,
                            service,
                            "".into()
                        )
                    }

                    Some(SuperviseurEvent::Restarting(project, service)) => {
                        return_event!(
                            tx,
                            service_name,
                            SERVICE_RESTARTING,
                            project,
                            service,
                            "".into()
                        )
                    }

                    Some(SuperviseurEvent::Building(project, service)) => {
                        return_event!(
                            tx,
                            service_name,
                            SERVICE_BUILDING,
                            project,
                            service,
                            "".into()
                        )
                    }

                    Some(SuperviseurEvent::SetupEnv(project, service, output)) => return_event!(
                        tx,
                        service_name,
                        SERVICE_SETUP_ENV,
                        project,
                        service,
                        output
                    ),

                    Some(SuperviseurEvent::Started(project, service)) => {
                        return_event!(
                            tx,
                            service_name,
                            SERVICE_STARTED,
                            project,
                            service,
                            "".into()
                        )
                    }
                    Some(SuperviseurEvent::Stopped(project, service)) => {
                        return_event!(
                            tx,
                            service_name,
                            SERVICE_STOPPED,
                            project,
                            service,
                            "".into()
                        )
                    }

                    Some(SuperviseurEvent::Restarted(project, service)) => {
                        return_event!(
                            tx,
                            service_name,
                            SERVICE_RESTARTED,
                            project,
                            service,
                            "".into()
                        )
                    }

                    Some(SuperviseurEvent::Built(project, service)) => {
                        return_event!(tx, service_name, SERVICE_BUILT, project, service, "".into())
                    }

                    Some(SuperviseurEvent::Logs(project, service, output)) => {
                        return_event!(tx, service_name, SERVICE_LOGS, project, service, output)
                    }

                    Some(SuperviseurEvent::Error(project, service, output)) => {
                        return_event!(tx, service_name, SERVICE_ERROR, project, service, output)
                    }

                    Some(SuperviseurEvent::Crashed(project, service)) => {
                        return_event!(
                            tx,
                            service_name,
                            SERVICE_CRASHED,
                            project,
                            service,
                            "".into()
                        )
                    }

                    Some(SuperviseurEvent::AllServicesStarted(project)) => {
                        tx.send(Ok(EventsResponse {
                            event: ALL_SERVICES_STARTED.to_string(),
                            project,
                            date: chrono::Utc::now().to_rfc3339(),
                            ..Default::default()
                        }))
                        .await
                        .unwrap();
                    }

                    Some(SuperviseurEvent::AllServicesStopped(project)) => {
                        tx.send(Ok(EventsResponse {
                            event: ALL_SERVICES_STOPPED.to_string(),
                            project,
                            date: chrono::Utc::now().to_rfc3339(),
                            ..Default::default()
                        }))
                        .await
                        .unwrap();
                    }

                    Some(SuperviseurEvent::AllServicesRestarted(project)) => {
                        tx.send(Ok(EventsResponse {
                            event: ALL_SERVICES_RESTARTED.to_string(),
                            project,
                            date: chrono::Utc::now().to_rfc3339(),
                            ..Default::default()
                        }))
                        .await
                        .unwrap();
                    }

                    Some(SuperviseurEvent::AllServicesBuilt(project)) => {
                        tx.send(Ok(EventsResponse {
                            event: ALL_SERVICES_BUILT.to_string(),
                            project,
                            date: chrono::Utc::now().to_rfc3339(),
                            ..Default::default()
                        }))
                        .await
                        .unwrap();
                    }

                    None => {
                        break;
                    }
                }
            }
        });

        Ok(Response::new(ReceiverStream::new(rx)))
    }
}
