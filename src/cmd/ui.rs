use std::{time::Duration, thread};

use actix_web::http::Uri;
use anyhow::Error;
use owo_colors::OwoColorize;
use tokio::net::UnixStream;
use tonic::transport::Endpoint;
use tower::service_fn;

use crate::{types::{BANNER, UNIX_SOCKET_PATH, SUPERFILE},  config::verify_if_config_file_is_present, api::superviseur::v1alpha1::{core_service_client::CoreServiceClient,  control_service_client::ControlServiceClient, LoadConfigRequest, StartWebDashboardRequest}};

pub async fn execute_ui() -> Result<(), Error> {
    verify_if_config_file_is_present()?;
    let current_dir = std::env::current_dir()?;
    let config = std::fs::read_to_string(current_dir.join(SUPERFILE))?;
    let channel = Endpoint::try_from("http://[::]:50051")?
    .connect_with_connector(service_fn(move |_: Uri| UnixStream::connect(UNIX_SOCKET_PATH)))
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

    let mut client = CoreServiceClient::new(channel);
    let request = tonic::Request::new(StartWebDashboardRequest {
        config_file_path: current_dir.to_str().unwrap().to_string(),
    });
    let response = client.start_web_dashboard(request).await?;
    let response = response.into_inner();

    println!("{}", BANNER.bright_purple());
    println!(
        "Starting dashboard ui on {} 🚀",
        response.url.cyan()
    );

    thread::spawn(move || {
        thread::sleep(Duration::from_secs(2));
        open::that(response.url).unwrap();
    });


    loop {
        tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
    }

    Ok(())
}
