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
        processes: Arc<Mutex<Vec<(Process, String)>>>,
    ) -> Self {
        thread::spawn(move || {
            let internal = SuperviseurInternal::new(cmd_rx, processes);
            futures::executor::block_on(internal);
        });
        Self {}
    }
}

#[derive(Debug)]
pub enum SuperviseurCommand {
    Load(Service, String),
    Start(Service, String),
    Stop(Service, String),
    Restart(Service, String),
    Status(String),
}

struct SuperviseurInternal {
    commands: Arc<Mutex<mpsc::UnboundedReceiver<SuperviseurCommand>>>,
    processes: Arc<Mutex<Vec<(Process, String)>>>,
    childs: Arc<Mutex<HashMap<String, i32>>>,
}

impl SuperviseurInternal {
    pub fn new(
        commands: Arc<Mutex<mpsc::UnboundedReceiver<SuperviseurCommand>>>,
        processes: Arc<Mutex<Vec<(Process, String)>>>,
    ) -> Self {
        Self {
            commands,
            processes,
            childs: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    fn handle_load(&self, service: Service, project: String) -> Result<(), Error> {
        let mut processes = self.processes.lock().unwrap();

        // skip if already loaded
        if processes
            .iter()
            .any(|(p, key)| p.name == service.name && key == &project)
        {
            return Ok(());
        }

        processes.push((
            Process {
                name: service.name,
                command: service.command,
                pid: None,
                uid: None,
                gid: None,
                working_dir: service.working_dir,
                state: State::Stopped,
                cpu: None,
                mem: None,
                time: None,
                port: None,
                env: service.env,
            },
            project,
        ));
        Ok(())
    }

    fn handle_start(&mut self, service: Service, project: String) -> Result<(), Error> {
        let child = std::process::Command::new("sh")
            .arg("-c")
            .arg(&service.command)
            .current_dir(service.working_dir)
            .envs(service.env)
            .stdout(std::process::Stdio::piped())
            .stderr(std::process::Stdio::piped())
            .spawn()
            .unwrap();

        let mut processes = self.processes.lock().unwrap();
        let mut process = &mut processes
            .iter_mut()
            .find(|(p, key)| p.name == service.name && key == &project)
            .unwrap()
            .0;
        process.pid = Some(child.id());
        process.state = State::Running;
        let service_key = format!("{}-{}", project, service.name);
        self.childs
            .lock()
            .unwrap()
            .insert(service_key, child.id() as i32);

        thread::spawn(move || {
            // write stdout to file
            let mut log_file = std::fs::File::create(service.stdout).unwrap();
            let stdout = child.stdout.unwrap();
            let stdout = std::io::BufReader::new(stdout);
            for line in stdout.lines() {
                let line = line.unwrap();
                let line = format!("{}\n", line);
                log_file.write_all(line.as_bytes()).unwrap();
            }

            // write stderr to file
            let mut err_file = std::fs::File::create(service.stderr).unwrap();
            let stderr = child.stderr.unwrap();
            let stderr = std::io::BufReader::new(stderr);
            for line in stderr.lines() {
                let line = line.unwrap();
                err_file.write_all(line.as_bytes()).unwrap();
            }
        });

        Ok(())
    }

    fn handle_stop(&self, service: Service, project: String) -> Result<(), Error> {
        let mut childs = self.childs.lock().unwrap();
        let service_key = format!("{}-{}", project, service.name);
        let pid = childs.get(&service_key).unwrap();
        signal::kill(Pid::from_raw(*pid), Signal::SIGTERM)?;
        childs.remove(&service_key);

        // update process state
        let mut processes = self.processes.lock().unwrap();
        let mut process = &mut processes
            .iter_mut()
            .find(|(p, key)| p.name == service.name && key == &project)
            .unwrap()
            .0;
        process.state = State::Stopped;

        Ok(())
    }

    fn handle_restart(&self, service: Service, project: String) -> Result<(), Error> {
        todo!()
    }

    fn handle_status(&self, name: String) -> Result<(), Error> {
        todo!()
    }

    fn handle_command(&mut self, cmd: SuperviseurCommand) -> Result<(), Error> {
        match cmd {
            SuperviseurCommand::Load(service, project) => self.handle_load(service, project),
            SuperviseurCommand::Start(service, project) => self.handle_start(service, project),
            SuperviseurCommand::Stop(service, project) => self.handle_stop(service, project),
            SuperviseurCommand::Restart(service, project) => self.handle_restart(service, project),
            SuperviseurCommand::Status(name) => self.handle_status(name),
        }
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

            thread::sleep(Duration::from_millis(500));
        }
    }
}
