use anyhow::Error;
use tabled::{Style, Table};

use crate::{
    api::superviseur::v1alpha1::{
        control_service_client::ControlServiceClient, ListRequest, LoadConfigRequest,
    },
    types::service::Service,
};

pub async fn execute_list() -> Result<(), Error> {
    let current_dir = std::env::current_dir()?;
    let config = std::fs::read_to_string(current_dir.join("Superfile.hcl"))?;
    let mut client = ControlServiceClient::connect("http://127.0.0.1:5476").await?;

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
