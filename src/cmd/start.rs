use crate::{
    api::superviseur::v1alpha1::{
        control_service_client::ControlServiceClient, LoadConfigRequest, StartRequest, EventsRequest, logging_service_client::LoggingServiceClient,
    },
    types::{UNIX_SOCKET_PATH, events::{SERVICE_STARTING, SERVICE_BUILDING, SERVICE_CRASHED, SERVICE_ERROR, SERVICE_RESTARTING, SERVICE_STOPPING, SERVICE_LOGS, SERVICE_SETUP_ENV, ALL_SERVICES_STOPPED, ALL_SERVICES_BUILT, ALL_SERVICES_RESTARTED}}, config::verify_if_config_file_is_present,
};
use anyhow::Error;
use tokio::net::UnixStream;
use tonic::transport::{ Endpoint, Uri};
use tower::service_fn;
use owo_colors::OwoColorize;

use super::event_handler;

pub async fn execute_start(name: Option<&str>, build: bool) -> Result<(), Error> {
    let (config, config_format) = verify_if_config_file_is_present()?;
    let current_dir = std::env::current_dir()?;
    let channel = Endpoint::try_from("http://[::]:50051")?
    .connect_with_connector(service_fn(move |_: Uri| UnixStream::connect( UNIX_SOCKET_PATH)))
        .await
        .map_err(|_| 
            Error::msg(format!("Cannot connect to the Superviseur daemon at unix:{}. Is the superviseur daemon running?", UNIX_SOCKET_PATH)))?;
    
    // let mut client = ControlServiceClient::connect("http://127.0.0.1:5476").await?;
    let mut client = ControlServiceClient::new(channel.clone());

    let config_file_path = current_dir.to_string_lossy().to_string();
    let file_path = config_file_path.clone();

    let request = tonic::Request::new(LoadConfigRequest {
        config,
        file_path,
        config_format
    });

    client.load_config(request).await?;

    let name = name.unwrap_or_default();
    let name = name.to_string();
    let mut client = LoggingServiceClient::new(channel.clone());

    let service = name.clone();
    
    let handle = event_handler!(client, service, config_file_path);

    let mut client = ControlServiceClient::new(channel.clone());

    let request = tonic::Request::new(StartRequest {
        name,
        config_file_path: current_dir.to_str().unwrap().to_string(),
        build
    });

    client.start(request).await?;

    handle.await??;

    Ok(())
}
