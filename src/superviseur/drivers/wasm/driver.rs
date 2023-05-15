use std::{
    collections::HashMap,
    io::{BufRead, Write},
    process::{ChildStderr, ChildStdout},
    sync::{Arc, Mutex},
    thread,
};

use anyhow::Error;
use async_trait::async_trait;
use nix::{
    sys::signal::{self, Signal},
    unistd::Pid,
};
use owo_colors::OwoColorize;
use tokio::sync::mpsc;

use crate::{
    graphql::{
        schema::objects::subscriptions::{LogStream, TailLogStream},
        simple_broker::SimpleBroker,
    },
    superviseur::{
        core::ProcessEvent,
        drivers::DriverPlugin,
        logs::{self, Log, LogEngine},
    },
    types::{
        configuration::{DriverConfig, Service},
        process::Process,
    },
};

use super::macros::check_wasm_runtime;

#[derive(Clone)]
pub struct Driver {
    project: String,
    service: Service,
    processes: Arc<Mutex<Vec<(Process, String)>>>,
    childs: Arc<Mutex<HashMap<String, i32>>>,
    event_tx: mpsc::UnboundedSender<ProcessEvent>,
    log_engine: Arc<Mutex<logs::LogEngine>>,
    cfg: DriverConfig,
}

impl Driver {
    pub fn new(
        project: String,
        service: &Service,
        processes: Arc<Mutex<Vec<(Process, String)>>>,
        event_tx: mpsc::UnboundedSender<ProcessEvent>,
        childs: Arc<Mutex<HashMap<String, i32>>>,
        log_engine: Arc<Mutex<LogEngine>>,
    ) -> Self {
        let cfg = service
            .r#use
            .as_ref()
            .unwrap()
            .into_iter()
            .find(|(driver, _)| *driver == "wasm")
            .map(|(_, x)| x)
            .unwrap();
        Self {
            project,
            service: service.clone(),
            processes,
            childs,
            event_tx,
            log_engine,
            cfg: cfg.clone(),
        }
    }

    pub fn verify_runtime(&self) -> Result<(), Error> {
        if let Some(runtime) = &self.cfg.runtime {
            if !check_wasm_runtime!(runtime, "lunatic")
                && !check_wasm_runtime!(runtime, "wasmtime")
                && !check_wasm_runtime!(runtime, "wasmer")
                && !check_wasm_runtime!(runtime, "spiderlightning")
                && !check_wasm_runtime!(runtime, "spin")
                && !check_wasm_runtime!(runtime, "wasmedge")
            {
                return Err(anyhow::anyhow!(
                    "Runtime {} is not supported",
                    runtime
                        .iter()
                        .map(|(key, _)| key.clone())
                        .next()
                        .unwrap_or_default()
                ));
            }

            if check_wasm_runtime!(runtime, "lunatic") {
                std::process::Command::new("lunatic")
                    .arg("--version")
                    .spawn()
                    .map_err(|_| anyhow::anyhow!("Lunatic runtime not installed, see https://github.com/lunatic-solutions/lunatic/releases"))?;
            }

            if check_wasm_runtime!(runtime, "wasmtime") {
                std::process::Command::new("wasmtime")
                    .arg("--version")
                    .spawn()
                    .map_err(|_| {
                        anyhow::anyhow!("Wasmtime runtime not installed, see https://wasmtime.dev/")
                    })?;
            }

            if check_wasm_runtime!(runtime, "wasmer") {
                std::process::Command::new("wasmer")
                    .arg("--version")
                    .spawn()
                    .map_err(|_| {
                        anyhow::anyhow!("Wasmer runtime not installed, see https://wasmer.io/")
                    })?;
            }

            if check_wasm_runtime!(runtime, "spiderlightning") {
                std::process::Command::new("slight")
                    .arg("--version")
                    .spawn()
                    .map_err(|_| {
                        anyhow::anyhow!("Spiderlightning runtime not installed, see https://github.com/deislabs/spiderlightning")
                    })?;
            }

            if check_wasm_runtime!(runtime, "spin") {
                std::process::Command::new("spin")
                    .arg("--version")
                    .spawn()
                    .map_err(|_| {
                        anyhow::anyhow!("Spin runtime not installed, see https://developer.fermyon.com/spin/install")
                    })?;
            }

            if check_wasm_runtime!(runtime, "wasmedge") {
                std::process::Command::new("wasmedge")
                    .arg("--version")
                    .spawn()
                    .map_err(|_| {
                        anyhow::anyhow!("Wasmedge runtime not installed, see https://wasmedge.org/book/en/quick_start/install.html")
                    })?;
            }

            return Ok(());
        }

        std::process::Command::new("wasmer")
            .arg("--version")
            .spawn()
            .map_err(|_| anyhow::anyhow!("Wasmer runtime not installed, see https://wasmer.io/"))?;

        Ok(())
    }

    pub fn command(&self) -> String {
        let cmd = format!("wasmer run {}", self.service.command);
        if let Some(runtime) = &self.cfg.runtime {
            if check_wasm_runtime!(runtime, "lunatic") {
                return format!("lunatic run {}", cmd);
            }

            if check_wasm_runtime!(runtime, "wasmtime") {
                return format!("wasmtime {}", cmd);
            }

            if check_wasm_runtime!(runtime, "wasmer") {
                return format!("wasmer run {}", cmd);
            }

            if check_wasm_runtime!(runtime, "spiderlightning") {
                return format!("slight -c slightfile.toml {}", cmd);
            }

            if check_wasm_runtime!(runtime, "spin") {
                return format!("spin up");
            }

            if check_wasm_runtime!(runtime, "wasmedge") {
                return format!("wasmedge {}", cmd);
            }
        }
        cmd
    }

    pub fn write_logs(&self, stdout: ChildStdout, stderr: ChildStderr) {
        let cloned_service = self.service.clone();
        let log_engine = self.log_engine.clone();
        let project = self.project.clone();

        thread::spawn(move || {
            let service = cloned_service;
            let id = service.id.unwrap_or("-".to_string());
            // write stdout to file
            let mut log_file = std::fs::File::create(service.stdout).unwrap();

            let stdout = std::io::BufReader::new(stdout);
            for line in stdout.lines() {
                let line = line.unwrap();
                let line = format!("{}\n", line);

                let log = Log {
                    project: project.clone(),
                    service: service.name.clone(),
                    line: line.clone(),
                    output: String::from("stdout"),
                    date: tantivy::DateTime::from_timestamp_secs(chrono::Local::now().timestamp()),
                };
                let log_engine = log_engine.lock().unwrap();
                match log_engine.insert(&log) {
                    Ok(_) => {}
                    Err(e) => {
                        println!("Error while inserting log: {}", e);
                    }
                }

                SimpleBroker::publish(TailLogStream {
                    id: id.clone(),
                    line: line.clone(),
                });
                SimpleBroker::publish(LogStream {
                    id: id.clone(),
                    line: line.clone(),
                });
                let service_name = format!("{} | ", service.name);
                print!("{} {}", service_name.cyan(), line);
                log_file.write_all(line.as_bytes()).unwrap();
            }

            // write stderr to file
            let mut err_file = std::fs::File::create(service.stderr).unwrap();
            let stderr = std::io::BufReader::new(stderr);
            for line in stderr.lines() {
                let line = line.unwrap();

                let log = Log {
                    project: project.clone(),
                    service: service.name.clone(),
                    line: line.clone(),
                    output: String::from("stderr"),
                    date: tantivy::DateTime::from_timestamp_secs(chrono::Local::now().timestamp()),
                };
                let log_engine = log_engine.lock().unwrap();
                match log_engine.insert(&log) {
                    Ok(_) => {}
                    Err(e) => {
                        println!("Error while inserting log: {}", e);
                    }
                }

                err_file.write_all(line.as_bytes()).unwrap();
            }
        });
    }
}

#[async_trait]
impl DriverPlugin for Driver {
    async fn start(&self, project: String) -> Result<(), Error> {
        self.verify_runtime()?;
        let command = &self.command();
        let envs = self.service.env.clone();
        let working_dir = self.service.working_dir.clone();
        println!("command: {}", command);
        let mut child = std::process::Command::new("sh")
            .arg("-c")
            .arg(command)
            .current_dir(working_dir)
            .envs(envs)
            .stdout(std::process::Stdio::piped())
            .stderr(std::process::Stdio::piped())
            .spawn()
            .unwrap();

        let mut processes = self.processes.lock().unwrap();
        let mut process = &mut processes
            .iter_mut()
            .find(|(p, key)| p.name == self.service.name && key == &project)
            .unwrap()
            .0;
        process.pid = Some(child.id());
        process.up_time = Some(chrono::Utc::now());
        let service_key = format!("{}-{}", project, self.service.name);
        self.childs
            .lock()
            .unwrap()
            .insert(service_key, process.pid.unwrap() as i32);

        self.event_tx
            .send(ProcessEvent::Started(
                self.service.name.clone(),
                project.clone(),
            ))
            .unwrap();

        let stdout = child.stdout.take().unwrap();
        let stderr = child.stderr.take().unwrap();
        self.write_logs(stdout, stderr);

        Ok(())
    }

    async fn stop(&self, project: String) -> Result<(), Error> {
        if let Some(stop_command) = self.service.stop_command.clone() {
            let envs = self.service.env.clone();
            let working_dir = self.service.working_dir.clone();

            let mut child = std::process::Command::new("sh")
                .arg("-c")
                .arg(stop_command)
                .current_dir(working_dir)
                .envs(envs)
                .stdout(std::process::Stdio::piped())
                .stderr(std::process::Stdio::piped())
                .spawn()
                .unwrap();
            child.wait().unwrap();

            let mut childs = self.childs.lock().unwrap();
            let service_key = format!("{}-{}", project.clone(), self.service.name.clone());
            childs.remove(&service_key);

            self.event_tx
                .send(ProcessEvent::Stopped(
                    self.service.name.clone(),
                    project.clone(),
                ))
                .unwrap();

            return Ok(());
        }

        let mut childs = self.childs.lock().unwrap();
        let service_key = format!("{}-{}", project.clone(), self.service.name.clone());

        match childs.get(&service_key) {
            Some(pid) => {
                println!(
                    "Stopping service {} (pid: {})",
                    self.service.name.clone(),
                    pid
                );
                signal::kill(Pid::from_raw(*pid), Signal::SIGTERM)?;
                childs.remove(&service_key);

                self.event_tx
                    .send(ProcessEvent::Stopped(
                        self.service.name.clone(),
                        project.clone(),
                    ))
                    .unwrap();

                println!("Service {} stopped", self.service.name);
                Ok(())
            }
            None => {
                println!("Service {} is not running", self.service.name);
                Ok(())
            }
        }
    }

    async fn restart(&self, project: String) -> Result<(), Error> {
        self.stop(project.clone()).await?;
        self.start(project.clone()).await?;
        Ok(())
    }

    async fn status(&self) -> Result<(), Error> {
        Ok(())
    }

    async fn logs(&self) -> Result<(), Error> {
        Ok(())
    }

    async fn exec(&self) -> Result<(), Error> {
        Ok(())
    }

    async fn build(&self, project: String) -> Result<(), Error> {
        if let Some(build) = self.service.build.clone() {
            let envs = self.service.env.clone();
            let working_dir = self.service.working_dir.clone();

            let build_command = build.command.clone();
            println!("build_command: {}", build_command);
            let mut child = std::process::Command::new("sh")
                .arg("-c")
                .arg(build_command)
                .current_dir(working_dir)
                .envs(envs)
                .stdout(std::process::Stdio::piped())
                .stderr(std::process::Stdio::piped())
                .spawn()?;
            let stdout = child.stdout.take().unwrap();
            let stderr = child.stderr.take().unwrap();
            self.write_logs(stdout, stderr);

            child.wait()?;

            self.event_tx.send(ProcessEvent::Built(
                self.service.name.clone(),
                project.clone(),
            ))?;
        }
        Ok(())
    }
}
