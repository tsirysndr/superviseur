use crate::api::superviseur::v1alpha1::{
    control_service_client::ControlServiceClient, LoadConfigRequest, StartRequest,
};
use anyhow::Error;

pub async fn execute_start(name: Option<&str>) -> Result<(), Error> {
    let current_dir = std::env::current_dir()?;
    let config = std::fs::read_to_string(current_dir.join("Superfile.hcl"))?;
    let mut client = ControlServiceClient::connect("http://127.0.0.1:5476").await?;

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
