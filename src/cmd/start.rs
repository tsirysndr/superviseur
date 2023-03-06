use crate::{
    api::superviseur::v1alpha1::{
        control_service_client::ControlServiceClient, LoadConfigRequest, StartRequest,
    },
    types::{UNIX_SOCKET_PATH, SUPERFILE}, config::verify_if_config_file_is_present,
};
use anyhow::Error;
use tokio::net::UnixStream;
use tonic::transport::{ Endpoint, Uri};
use tower::service_fn;

pub async fn execute_start(name: Option<&str>) -> Result<(), Error> {
    verify_if_config_file_is_present()?;
    let current_dir = std::env::current_dir()?;
    let config = std::fs::read_to_string(current_dir.join(SUPERFILE))?;
    let channel = Endpoint::try_from("http://[::]:50051")?
    .connect_with_connector(service_fn(move |_: Uri| UnixStream::connect( UNIX_SOCKET_PATH)))
        .await
        .map_err(|_| 
            Error::msg(format!("Cannot connect to the Superviseur daemon at unix:{}. Is the superviseur daemon running?", UNIX_SOCKET_PATH)))?;

    // let mut client = ControlServiceClient::connect("http://127.0.0.1:5476").await?;
    let mut client = ControlServiceClient::new(channel);

    let request = tonic::Request::new(LoadConfigRequest {
        config,
        file_path: current_dir.to_str().unwrap().to_string(),
    });

    client.load_config(request).await?;

    let name = name.unwrap_or_default().to_string();

    let request = tonic::Request::new(StartRequest {
        name,
        config_file_path: current_dir.to_str().unwrap().to_string(),
    });

    client.start(request).await?;

    Ok(())
}
