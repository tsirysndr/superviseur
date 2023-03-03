use std::{
    pin::Pin,
    sync::{Arc, Mutex},
    task::{Context, Poll},
    thread,
    time::Duration,
};

use anyhow::Error;
use futures::Future;
use tokio::sync::mpsc;

#[derive(Clone)]
pub struct Superviseur {}

impl Superviseur {
    pub fn new(cmd_rx: Arc<Mutex<mpsc::UnboundedReceiver<SuperviseurCommand>>>) -> Self {
        thread::spawn(move || {
            let internal = SuperviseurInternal::new(cmd_rx);
            futures::executor::block_on(internal);
        });
        Self {}
    }
}

#[derive(Debug)]
pub enum SuperviseurCommand {
    Start(String),
    Stop(String),
    Restart(String),
    Status(String),
}

struct SuperviseurInternal {
    commands: Arc<Mutex<mpsc::UnboundedReceiver<SuperviseurCommand>>>,
}

impl SuperviseurInternal {
    pub fn new(commands: Arc<Mutex<mpsc::UnboundedReceiver<SuperviseurCommand>>>) -> Self {
        Self { commands }
    }

    fn handle_start(&self, name: String) -> Result<(), Error> {
        todo!()
    }

    fn handle_stop(&self, name: String) -> Result<(), Error> {
        todo!()
    }

    fn handle_restart(&self, name: String) -> Result<(), Error> {
        todo!()
    }

    fn handle_status(&self, name: String) -> Result<(), Error> {
        todo!()
    }

    fn handle_command(&self, cmd: SuperviseurCommand) -> Result<(), Error> {
        match cmd {
            SuperviseurCommand::Start(name) => self.handle_start(name),
            SuperviseurCommand::Stop(name) => self.handle_stop(name),
            SuperviseurCommand::Restart(name) => self.handle_restart(name),
            SuperviseurCommand::Status(name) => self.handle_status(name),
        }
    }
}

impl Future for SuperviseurInternal {
    type Output = ();

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
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
