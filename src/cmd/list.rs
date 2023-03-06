use anyhow::Error;
use tabled::{Style, Table};
use tokio::net::UnixStream;
use tonic::transport::{ Endpoint, Uri};
use tower::service_fn;

use crate::{
    api::superviseur::v1alpha1::{
        control_service_client::ControlServiceClient, ListRequest, LoadConfigRequest,
    },
    types::{service::Service, UNIX_SOCKET_PATH, SUPERFILE}, config::verify_if_config_file_is_present,
};

pub async fn execute_list() -> Result<(), Error> {
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

    let request = tonic::Request::new(ListRequest {
        config_file_path: current_dir.to_str().unwrap().to_string(),
    });

    let response = client.list(request).await?;
    let response = response.into_inner();
    let services: Vec<Service> = response.services.into_iter().map(Into::into).collect();
    println!("{}", Table::new(&services).with(Style::blank()));
    Ok(())
}
