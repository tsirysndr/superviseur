use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader, Read},
    sync::{Arc, Mutex},
};

use crate::{
    api::superviseur::v1alpha1::{
        logging_service_server::LoggingService, LogDetails, LogRequest, LogResponse, SearchRequest,
        SearchResponse, TailRequest, TailResponse,
    },
    superviseur::{core::Superviseur, logs::LogEngine},
    types::{configuration::ConfigurationData, process::Process},
};
use anyhow::Error;
use chrono::{TimeZone, Utc};
use tokio_stream::wrappers::ReceiverStream;
use tonic::{Request, Response, Status};

pub struct Logging {
    superviseur: Superviseur,
    processes: Arc<Mutex<Vec<(Process, String)>>>,
    config_map: Arc<Mutex<HashMap<String, ConfigurationData>>>,
    project_map: Arc<Mutex<HashMap<String, String>>>,
    log_engine: Arc<Mutex<LogEngine>>,
}

impl Logging {
    pub fn new(
        superviseur: Superviseur,
        processes: Arc<Mutex<Vec<(Process, String)>>>,
        config_map: Arc<Mutex<HashMap<String, ConfigurationData>>>,
        project_map: Arc<Mutex<HashMap<String, String>>>,
        log_engine: Arc<Mutex<LogEngine>>,
    ) -> Self {
        Self {
            superviseur,
            processes,
            config_map,
            project_map,
            log_engine,
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
}
