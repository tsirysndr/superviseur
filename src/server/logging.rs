use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader, Read},
    sync::{Arc, Mutex},
};

use crate::{
    api::superviseur::v1alpha1::{
        logging_service_server::LoggingService, LogRequest, LogResponse, TailRequest, TailResponse,
    },
    superviseur::core::Superviseur,
    types::{configuration::ConfigurationData, process::Process},
};
use tokio_stream::wrappers::ReceiverStream;
use tonic::{Request, Response, Status};

pub struct Logging {
    superviseur: Superviseur,
    processes: Arc<Mutex<Vec<(Process, String)>>>,
    config_map: Arc<Mutex<HashMap<String, ConfigurationData>>>,
}

impl Logging {
    pub fn new(
        superviseur: Superviseur,
        processes: Arc<Mutex<Vec<(Process, String)>>>,
        config_map: Arc<Mutex<HashMap<String, ConfigurationData>>>,
    ) -> Self {
        Self {
            superviseur,
            processes,
            config_map,
        }
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
}
