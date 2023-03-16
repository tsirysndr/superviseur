use std::{
    collections::HashMap,
    io::{BufRead, Write},
    pin::Pin,
    sync::{Arc, Mutex},
    task::{Context, Poll},
    thread,
    time::Duration,
};

use anyhow::Error;
use futures::Future;
use nix::{
    sys::signal::{self, Signal},
    unistd::Pid,
};
use tokio::sync::mpsc;

use crate::types::{
    configuration::Service,
    process::{Process, State},
};

#[derive(Clone)]
pub struct Superviseur {}

impl Superviseur {
    pub fn new(
        cmd_rx: Arc<Mutex<mpsc::UnboundedReceiver<SuperviseurCommand>>>,
        cmd_tx: mpsc::UnboundedSender<SuperviseurCommand>,
        processes: Arc<Mutex<Vec<(Process, String)>>>,
    ) -> Self {
        thread::spawn(move || {
            let internal = SuperviseurInternal::new(cmd_rx, cmd_tx, processes);
            futures::executor::block_on(internal);
        });
        Self {}
    }
}

#[derive(Debug)]
pub enum SuperviseurCommand {
    Load(Service, String),
    Start(Service, String, Vec<Service>),
    Stop(Service, String),
    Restart(Service, String, Vec<Service>),
}

#[derive(Debug)]
pub enum ProcessEvent {
    Started(String, String),
    Stopped(String, String),
    Restarted(String, String),
}

struct SuperviseurInternal {
    commands: Arc<Mutex<mpsc::UnboundedReceiver<SuperviseurCommand>>>,
    events: mpsc::UnboundedReceiver<ProcessEvent>,
    event_tx: mpsc::UnboundedSender<ProcessEvent>,
    cmd_tx: mpsc::UnboundedSender<SuperviseurCommand>,
    processes: Arc<Mutex<Vec<(Process, String)>>>,
    childs: Arc<Mutex<HashMap<String, i32>>>,
}

impl SuperviseurInternal {
    pub fn new(
        commands: Arc<Mutex<mpsc::UnboundedReceiver<SuperviseurCommand>>>,
        cmd_tx: mpsc::UnboundedSender<SuperviseurCommand>,
        processes: Arc<Mutex<Vec<(Process, String)>>>,
    ) -> Self {
        let (event_tx, events) = mpsc::unbounded_channel();
        Self {
            commands,
            events,
            event_tx,
            cmd_tx,
            processes,
            childs: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    fn handle_load(&self, service: Service, project: String) -> Result<(), Error> {
        let mut processes = self.processes.lock().unwrap();

        // update and skip if already loaded
        if let Some(process) = processes
            .iter_mut()
            .find(|(p, key)| p.name == service.name && key == &project)
            .map(|(p, _)| p)
        {
            process.service_id = service.id.unwrap_or("-".to_string());
            process.name = service.name;
            process.command = service.command;
            process.description = service.description;
            process.working_dir = service.working_dir;
            process.env = service.env;
            process.project = project.clone();
            process.r#type = service.r#type;
            process.auto_restart = service.autorestart;
            process.stdout = service.stdout;
            process.stderr = service.stderr;
            return Ok(());
        }

        processes.push((
            Process {
                service_id: service.id.unwrap_or("-".to_string()),
                name: service.name,
                command: service.command,
                description: service.description,
                pid: None,
                uid: None,
                gid: None,
                working_dir: service.working_dir,
                state: State::Stopped,
                cpu: None,
                mem: None,
                up_time: None,
                port: None,
                env: service.env,
                project: project.clone(),
                r#type: service.r#type,
                auto_restart: service.autorestart,
                stdout: service.stdout,
                stderr: service.stderr,
            },
            project,
        ));
        Ok(())
    }

    fn handle_start(
        &mut self,
        service: Service,
        project: String,
        services: Vec<Service>,
        force: bool,
    ) -> Result<(), Error> {
        // start recursively if service depends on other services
        let dependencies = service.depends_on.clone();
        let dependencies = dependencies
            .into_iter()
            .filter(|d| d != &service.name)
            .collect::<Vec<String>>();
        if dependencies.len() > 0 {
            for dependency in dependencies.into_iter() {
                match services.iter().find(|s| s.name == dependency) {
                    Some(s) => {
                        self.handle_start(s.clone(), project.clone(), services.clone(), force)?;
                        thread::sleep(Duration::from_millis(100));
                    }
                    None => {
                        return Err(anyhow::anyhow!("Service {} not found", dependency));
                    }
                }
            }
        }

        // skip if already started
        let mut processes = self.processes.lock().unwrap();
        if let Some(process) = processes
            .iter()
            .find(|(p, key)| p.name == service.name && key == &project)
            .map(|(p, _)| p)
        {
            if process.state == State::Running && !force {
                return Ok(());
            }
        }

        let envs = service.env.clone();
        let working_dir = service.working_dir.clone();
        let mut child = std::process::Command::new("sh")
            .arg("-c")
            .arg(&service.command)
            .current_dir(working_dir)
            .envs(envs)
            .stdout(std::process::Stdio::piped())
            .stderr(std::process::Stdio::piped())
            .spawn()
            .unwrap();

        let mut process = &mut processes
            .iter_mut()
            .find(|(p, key)| p.name == service.name && key == &project)
            .unwrap()
            .0;
        process.pid = Some(child.id());
        self.event_tx
            .send(ProcessEvent::Started(service.name.clone(), project.clone()))
            .unwrap();

        process.up_time = Some(chrono::Utc::now());
        let service_key = format!("{}-{}", project, service.name);
        self.childs
            .lock()
            .unwrap()
            .insert(service_key, child.id() as i32);

        let stdout = child.stdout.take().unwrap();
        let stderr = child.stderr.take().unwrap();

        let cloned_service = service.clone();

        thread::spawn(move || {
            let service = cloned_service;
            // write stdout to file
            let mut log_file = std::fs::File::create(service.stdout).unwrap();

            let stdout = std::io::BufReader::new(stdout);
            for line in stdout.lines() {
                let line = line.unwrap();
                let line = format!("{}\n", line);
                log_file.write_all(line.as_bytes()).unwrap();
            }

            // write stderr to file
            let mut err_file = std::fs::File::create(service.stderr).unwrap();
            let stderr = std::io::BufReader::new(stderr);
            for line in stderr.lines() {
                let line = line.unwrap();
                err_file.write_all(line.as_bytes()).unwrap();
            }
        });

        let cmd_tx = self.cmd_tx.clone();
        let event_tx = self.event_tx.clone();
        thread::spawn(move || {
            let _status = child.wait().unwrap();
            // println!("child exited with status: {}", status);
            if service.autorestart {
                cmd_tx
                    .send(SuperviseurCommand::Start(
                        service.clone(),
                        project.clone(),
                        services.clone(),
                    ))
                    .unwrap();

                event_tx
                    .send(ProcessEvent::Restarted(service.name, project))
                    .unwrap();
                return;
            }
            event_tx
                .send(ProcessEvent::Stopped(service.name, project))
                .unwrap();
        });

        Ok(())
    }

    fn handle_stop(&self, service: Service, project: String) -> Result<(), Error> {
        let mut childs = self.childs.lock().unwrap();
        let service_key = format!("{}-{}", project, service.name);
        match childs.get(&service_key) {
            Some(pid) => {
                signal::kill(Pid::from_raw(*pid), Signal::SIGTERM)?;
                childs.remove(&service_key);

                self.event_tx
                    .send(ProcessEvent::Stopped(service.name, project))
                    .unwrap();
                Ok(())
            }
            None => Ok(()),
        }
    }

    fn handle_restart(
        &mut self,
        service: Service,
        project: String,
        services: Vec<Service>,
    ) -> Result<(), Error> {
        self.handle_stop(service.clone(), project.clone())?;
        self.handle_start(service, project, services, true)
    }

    fn handle_command(&mut self, cmd: SuperviseurCommand) -> Result<(), Error> {
        match cmd {
            SuperviseurCommand::Load(service, project) => self.handle_load(service, project),
            SuperviseurCommand::Start(service, project, services) => {
                self.handle_start(service, project, services, false)
            }
            SuperviseurCommand::Stop(service, project) => self.handle_stop(service, project),
            SuperviseurCommand::Restart(service, project, services) => {
                self.handle_restart(service, project, services)
            }
        }
    }

    fn handle_event(&mut self, event: ProcessEvent) -> Result<(), Error> {
        let mut processes = self.processes.lock().unwrap();
        match event {
            ProcessEvent::Started(service_name, project) => {
                let mut process = &mut processes
                    .iter_mut()
                    .find(|(p, key)| p.name == service_name && key == &project)
                    .unwrap()
                    .0;
                process.state = State::Running;
            }
            ProcessEvent::Stopped(service_name, project) => {
                let mut process = &mut processes
                    .iter_mut()
                    .find(|(p, key)| p.name == service_name && key == &project)
                    .unwrap()
                    .0;
                process.state = State::Stopped;
            }
            ProcessEvent::Restarted(service_name, project) => {
                let mut process = &mut processes
                    .iter_mut()
                    .find(|(p, key)| p.name == service_name && key == &project)
                    .unwrap()
                    .0;
                process.state = State::Running;
            }
        }
        Ok(())
    }
}

impl Future for SuperviseurInternal {
    type Output = ();

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        loop {
            let cmd = match self.commands.lock().unwrap().poll_recv(cx) {
                Poll::Ready(Some(cmd)) => Some(cmd),
                Poll::Ready(None) => return Poll::Ready(()), // client has disconnected - shut down.
                _ => None,
            };

            if let Some(cmd) = cmd {
                if let Err(e) = self.handle_command(cmd) {
                    println!("{:?}", e);
                }
            }

            let event = match self.events.poll_recv(cx) {
                Poll::Ready(Some(event)) => Some(event),
                Poll::Ready(None) => return Poll::Ready(()), // client has disconnected - shut down.
                _ => None,
            };

            if let Some(event) = event {
                if let Err(e) = self.handle_event(event) {
                    println!("{:?}", e);
                }
            }

            thread::sleep(Duration::from_millis(500));
        }
    }
}
