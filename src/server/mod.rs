use std::{
    collections::HashMap,
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
    types::process::Process,
};

pub mod control;
pub mod logging;

const BANNER: &str = r#"
         _____                             _                     
        / ___/__  ______  ___  ______   __(_)_______  __  _______
        \__ \/ / / / __ \/ _ \/ ___/ | / / / ___/ _ \/ / / / ___/
       ___/ / /_/ / /_/ /  __/ /   | |/ / (__  )  __/ /_/ / /    
      /____/\__,_/ .___/\___/_/    |___/_/____/\___/\__,_/_/     
                /_/                                              
      "#;

pub async fn exec(port: u16) -> Result<(), Error> {
    let addr: SocketAddr = format!("0.0.0.0:{}", port).parse().unwrap();
    println!("{}", BANNER.bright_purple());
    println!("Listening on {}", addr.cyan());

    let config_map = Arc::new(Mutex::new(HashMap::new()));
    let (cmd_tx, cmd_rx) = tokio::sync::mpsc::unbounded_channel();
    let processes = Arc::new(Mutex::new(vec![] as Vec<(Process, String)>));
    let cmd_rx = Arc::new(Mutex::new(cmd_rx));

    let superviseur = Superviseur::new(cmd_rx, cmd_tx.clone(), processes.clone());

    Server::builder()
        .accept_http1(true)
        .add_service(tonic_web::enable(LoggingServiceServer::new(Logging::new(
            superviseur.clone(),
            processes.clone(),
            config_map.clone(),
        ))))
        .add_service(tonic_web::enable(ControlServiceServer::new(Control::new(
            cmd_tx,
            superviseur,
            processes,
            config_map,
        ))))
        .serve(addr)
        .await?;
    Ok(())
}
