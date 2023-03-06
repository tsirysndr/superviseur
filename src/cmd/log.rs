use anyhow::Error;
use tokio::net::UnixStream;
use tonic::transport::{ Endpoint, Uri};
use tower::service_fn;

use crate::{
    api::superviseur::v1alpha1::{
        control_service_client::ControlServiceClient, logging_service_client::LoggingServiceClient,
        LoadConfigRequest, LogRequest,
    },
    types::UNIX_SOCKET_PATH,
};

pub async fn execute_log(name: &str, follow: bool) -> Result<(), Error> {
    let current_dir = std::env::current_dir()?;
    let config = std::fs::read_to_string(current_dir.join("Superfile.hcl"))?;
    let channel = Endpoint::try_from("http://[::]:50051")?
    .connect_with_connector(service_fn(move |_: Uri| UnixStream::connect( UNIX_SOCKET_PATH)))
        .await
        .map_err(|_| 
            Error::msg(format!("Cannot connect to the Superviseur daemon at unix:{}. Is the superviseur daemon running?", UNIX_SOCKET_PATH)))?;

    // let mut client = ControlServiceClient::connect("http://127.0.0.1:5476").await?;
    let mut client = ControlServiceClient::new(channel.clone());

    let request = tonic::Request::new(LoadConfigRequest {
        config,
        file_path: current_dir.to_str().unwrap().to_string(),
    });

    client.load_config(request).await?;

    // let mut client = LoggingServiceClient::connect("http://127.0.0.1:5476").await?;
    let mut client = LoggingServiceClient::new(channel);

    let request = tonic::Request::new(LogRequest {
        service: name.to_string(),
        config_file_path: current_dir.to_str().unwrap().to_string(),
        follow,
    });

    let response = client.log(request).await?;
    let mut stream = response.into_inner();

    while let Some(message) = stream.message().await? {
        println!("{}", message.line);
    }

    Ok(())
}
