use std::{
    path::Path,
    pin::Pin,
    task::{Context, Poll},
    thread,
    time::Duration,
};

use futures_util::Future;
use notify::{Config, Error, Event, PollWatcher, RecommendedWatcher, Watcher, WatcherKind};
use tokio::sync::mpsc;

use superviseur_types::{command::SuperviseurCommand, configuration::Service};

pub struct WatchForChanges {}

impl WatchForChanges {
    pub fn new(
        dir: String,
        superviseur_tx: mpsc::UnboundedSender<SuperviseurCommand>,
        service: Service,
        project: String,
    ) -> Self {
        thread::spawn(move || {
            let internal = WatchForChangesInternal::new(&dir, superviseur_tx, service, project);
            futures::executor::block_on(internal);
        });
        Self {}
    }
}

struct WatchForChangesInternal {
    cmd_rx: mpsc::UnboundedReceiver<notify::Result<notify::Event>>,
    superviseur_tx: mpsc::UnboundedSender<SuperviseurCommand>,
    service: Service,
    project: String,
    watcher: Box<dyn Watcher>,
}

impl WatchForChangesInternal {
    pub fn new(
        dir: &str,
        superviseur_tx: mpsc::UnboundedSender<SuperviseurCommand>,
        service: Service,
        project: String,
    ) -> Self {
        let (cmd_tx, cmd_rx) = mpsc::unbounded_channel();
        let mut watcher: Box<dyn Watcher> =
            if RecommendedWatcher::kind() == WatcherKind::PollWatcher {
                // custom config for PollWatcher kind
                // you
                let config = Config::default().with_poll_interval(Duration::from_secs(1));
                Box::new(
                    PollWatcher::new(
                        move |result: Result<Event, Error>| {
                            cmd_tx.send(result).unwrap();
                        },
                        config,
                    )
                    .unwrap(),
                )
            } else {
                // use default config for everything else
                Box::new(
                    RecommendedWatcher::new(
                        move |result: Result<Event, Error>| {
                            cmd_tx.send(result).unwrap();
                        },
                        Config::default(),
                    )
                    .unwrap(),
                )
            };
        watcher
            .watch(Path::new(dir), notify::RecursiveMode::Recursive)
            .unwrap();
        Self {
            cmd_rx,
            service,
            project,
            superviseur_tx,
            watcher,
        }
    }
}

impl Future for WatchForChangesInternal {
    type Output = ();

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        loop {
            let event = match self.cmd_rx.poll_recv(cx) {
                Poll::Ready(Some(event)) => Some(event),
                Poll::Ready(None) => return Poll::Ready(()), // client has disconnected - shut down.
                _ => None,
            };

            if event.is_none() {
                return Poll::Pending;
            }

            match event {
                Some(Ok(_)) => {
                    self.superviseur_tx
                        .send(SuperviseurCommand::Restart(
                            self.service.clone(),
                            self.project.clone(),
                        ))
                        .unwrap();
                }
                Some(Err(e)) => println!("watch error: {:?}", e),
                None => {}
            }
        }
    }
}
