use std::{
    pin::Pin,
    sync::{Arc, Mutex},
    task::{Context, Poll},
    thread,
};

use futures::Future;
use tokio::sync::mpsc;

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
}

impl Future for SuperviseurInternal {
    type Output = ();

    fn poll(self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Self::Output> {
        Poll::Ready(())
    }
}
