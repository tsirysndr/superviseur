use anyhow::Error;

use crate::api::superviseur::v1alpha1::{
    control_service_client::ControlServiceClient, logging_service_client::LoggingServiceClient,
    LoadConfigRequest, TailRequest,
};

pub async fn execute_tail(name: &str, follow: bool, lines: usize) -> Result<(), Error> {
    let current_dir = std::env::current_dir()?;
    let config = std::fs::read_to_string(current_dir.join("Superfile.hcl"))?;
    let mut client = ControlServiceClient::connect("http://127.0.0.1:5476").await?;

    let request = tonic::Request::new(LoadConfigRequest {
        config,
        file_path: current_dir.to_str().unwrap().to_string(),
    });

    client.load_config(request).await?;

    let mut client = LoggingServiceClient::connect("http://127.0.0.1:5476").await?;

    let request = tonic::Request::new(TailRequest {
        service: name.to_string(),
        config_file_path: current_dir.to_str().unwrap().to_string(),
        follow,
        lines: lines as u32,
    });

    let response = client.tail(request).await?;
    let mut stream = response.into_inner();

    while let Some(message) = stream.message().await? {
        println!("{}", message.line);
    }

    Ok(())
}
