use std::{
    collections::HashMap,
    io::{BufRead, Write},
    process::{ChildStderr, ChildStdout},
    sync::{Arc, Mutex},
    thread,
};

use anyhow::Error;
use async_trait::async_trait;
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
        logs::{self, Log},
        macros::wait_child_process_in_background,
    },
    types::{configuration::Service, events::SuperviseurEvent, process::Process},
};

use nix::{
    sys::signal::{self, Signal},
    unistd::Pid,
};

#[derive(Clone)]
pub struct Driver {
    project: String,
    service: Service,
    processes: Arc<Mutex<Vec<(Process, String)>>>,
    childs: Arc<Mutex<HashMap<String, i32>>>,
    event_tx: mpsc::UnboundedSender<ProcessEvent>,
    log_engine: Arc<Mutex<logs::LogEngine>>,
    superviseur_event: mpsc::UnboundedSender<SuperviseurEvent>,
}

impl Default for Driver {
    fn default() -> Self {
        let (event_tx, _) = mpsc::unbounded_channel();
        let (superviseur_event, _) = mpsc::unbounded_channel();
        Self {
            project: "".to_string(),
            service: Service::default(),
            processes: Arc::new(Mutex::new(Vec::new())),
            childs: Arc::new(Mutex::new(HashMap::new())),
            event_tx,
            log_engine: Arc::new(Mutex::new(logs::LogEngine::new())),
            superviseur_event,
        }
    }
}

impl Driver {
    pub fn new(
        project: String,
        service: &Service,
        processes: Arc<Mutex<Vec<(Process, String)>>>,
        event_tx: mpsc::UnboundedSender<ProcessEvent>,
        childs: Arc<Mutex<HashMap<String, i32>>>,
        log_engine: Arc<Mutex<logs::LogEngine>>,
        superviseur_event: mpsc::UnboundedSender<SuperviseurEvent>,
    ) -> Self {
        Self {
            project,
            service: service.clone(),
            processes,
            childs,
            event_tx,
            log_engine,
            superviseur_event,
        }
    }

    pub fn write_logs(&self, stdout: ChildStdout, stderr: ChildStderr) {
        let cloned_service = self.service.clone();
        let log_engine = self.log_engine.clone();
        let project = self.project.clone();
        let superviseur_event = self.superviseur_event.clone();

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

                superviseur_event
                    .send(SuperviseurEvent::Logs(
                        project.clone(),
                        service.name.clone(),
                        line.clone(),
                    ))
                    .unwrap();

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
        let command = &self.service.command;
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

        let event_tx = self.event_tx.clone();
        let superviseur_event_tx = self.superviseur_event.clone();
        let service_name = self.service.name.clone();

        wait_child_process_in_background!(
            child,
            event_tx,
            service_name,
            project,
            superviseur_event_tx
        );

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
                .spawn()?;

            child.wait()?;

            let mut childs = self.childs.lock().unwrap();
            let service_key = format!("{}-{}", project.clone(), self.service.name.clone());
            childs.remove(&service_key);

            self.event_tx.send(ProcessEvent::Stopped(
                self.service.name.clone(),
                project.clone(),
            ))?;

            self.superviseur_event.send(SuperviseurEvent::Stopped(
                project.clone(),
                self.service.name.clone(),
            ))?;

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

        self.superviseur_event.send(SuperviseurEvent::Restarted(
            project,
            self.service.name.clone(),
        ))?;

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

            self.superviseur_event
                .send(SuperviseurEvent::Built(project, self.service.name.clone()))?;
        }
        Ok(())
    }
}
