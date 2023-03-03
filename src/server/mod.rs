use std::net::SocketAddr;

use anyhow::Error;
use owo_colors::OwoColorize;
use tonic::transport::Server;

use crate::{
    api::superviseur::v1alpha1::{
        control_service_server::ControlServiceServer, logging_service_server::LoggingServiceServer,
    },
    server::{control::Control, logging::Logging},
};

pub mod control;
pub mod logging;

pub async fn exec(port: u16) -> Result<(), Error> {
    let addr: SocketAddr = format!("0.0.0.0:{}", port).parse().unwrap();
    println!("Listening on {}", addr.cyan());
    Server::builder()
        .accept_http1(true)
        .add_service(tonic_web::enable(LoggingServiceServer::new(
            Logging::default(),
        )))
        .add_service(tonic_web::enable(ControlServiceServer::new(
            Control::default(),
        )))
        .serve(addr)
        .await?;
    Ok(())
}
