use std::{
    collections::HashMap,
    net::SocketAddr,
    path::PathBuf,
    sync::{Arc, Mutex},
};

use crate::{
    api::superviseur::v1alpha1::{
        control_service_server::ControlServiceServer, core_service_server::CoreServiceServer,
        logging_service_server::LoggingServiceServer,
    },
    server::{control::Control, logging::Logging},
    superviseur::Superviseur,
    types::{process::Process, BANNER, UNIX_SOCKET_PATH},
};
use anyhow::Error;
use owo_colors::OwoColorize;
use tokio::net::UnixListener;
use tokio_stream::wrappers::UnixListenerStream;
use tonic::transport::Server;

pub mod control;
pub mod core;
pub mod logging;

pub async fn exec(port: u16, serve: bool) -> Result<(), Error> {
    let addr: SocketAddr = format!("0.0.0.0:{}", port).parse().unwrap();
    println!("{}", BANNER.bright_purple());
    println!(
        "Listening on {}{} 🚀",
        "unix:".cyan(),
        UNIX_SOCKET_PATH.cyan()
    );
    if serve {
        println!("Listening on {} 🚀", addr.cyan());
    }

    let config_map = Arc::new(Mutex::new(HashMap::new()));
    let (cmd_tx, cmd_rx) = tokio::sync::mpsc::unbounded_channel();
    let processes = Arc::new(Mutex::new(vec![] as Vec<(Process, String)>));
    let cmd_rx = Arc::new(Mutex::new(cmd_rx));

    let superviseur = Superviseur::new(cmd_rx, cmd_tx.clone(), processes.clone());

    let cloned_cmd_tx = cmd_tx.clone();
    let cloned_superviseur = superviseur.clone();
    let cloned_processes = processes.clone();
    let cloned_config_map = config_map.clone();

    // create a one-shot channel to wait for the server to start
    let (tx, rx) = tokio::sync::oneshot::channel::<bool>();

    tokio::spawn(async move {
        let socket_path = PathBuf::from(UNIX_SOCKET_PATH);

        if socket_path.exists() {
            std::fs::remove_file(&socket_path).unwrap();
        }

        let listener = UnixListener::bind(&socket_path).unwrap();

        Server::builder()
            .accept_http1(true)
            .add_service(tonic_web::enable(LoggingServiceServer::new(Logging::new(
                cloned_superviseur.clone(),
                cloned_processes.clone(),
                cloned_config_map.clone(),
            ))))
            .add_service(tonic_web::enable(ControlServiceServer::new(Control::new(
                cloned_cmd_tx.clone(),
                cloned_superviseur.clone(),
                cloned_processes.clone(),
                cloned_config_map.clone(),
            ))))
            .add_service(tonic_web::enable(CoreServiceServer::new(core::Core::new(
                cloned_cmd_tx,
                cloned_superviseur,
                cloned_processes,
                cloned_config_map,
            ))))
            .serve_with_incoming(UnixListenerStream::new(listener))
            .await
            .unwrap();
        tx.send(true).unwrap();
    });

    if serve {
        Server::builder()
            .accept_http1(true)
            .add_service(tonic_web::enable(LoggingServiceServer::new(Logging::new(
                superviseur.clone(),
                processes.clone(),
                config_map.clone(),
            ))))
            .add_service(tonic_web::enable(ControlServiceServer::new(Control::new(
                cmd_tx.clone(),
                superviseur.clone(),
                processes.clone(),
                config_map.clone(),
            ))))
            .add_service(tonic_web::enable(CoreServiceServer::new(core::Core::new(
                cmd_tx,
                superviseur,
                processes,
                config_map,
            ))))
            .serve(addr)
            .await?;
    }

    // wait for the server to start
    rx.await.unwrap();

    Ok(())
}
