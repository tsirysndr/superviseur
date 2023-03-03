use std::{
    net::SocketAddr,
    sync::{Arc, Mutex},
};

use anyhow::Error;
use owo_colors::OwoColorize;
use tonic::transport::Server;

use crate::{
    api::superviseur::v1alpha1::{
        control_service_server::ControlServiceServer, logging_service_server::LoggingServiceServer,
    },
    server::{control::Control, logging::Logging},
    superviseur::Superviseur,
};

pub mod control;
pub mod logging;

pub async fn exec(port: u16) -> Result<(), Error> {
    let addr: SocketAddr = format!("0.0.0.0:{}", port).parse().unwrap();
    println!("Listening on {}", addr.cyan());
    let (_cmd_tx, cmd_rx) = tokio::sync::mpsc::unbounded_channel();
    let cmd_rx = Arc::new(Mutex::new(cmd_rx));
    let superviseur = Superviseur::new(cmd_rx);
    Server::builder()
        .accept_http1(true)
        .add_service(tonic_web::enable(LoggingServiceServer::new(Logging::new(
            superviseur.clone(),
        ))))
        .add_service(tonic_web::enable(ControlServiceServer::new(Control::new(
            superviseur,
        ))))
        .serve(addr)
        .await?;
    Ok(())
}
