use std::{time::Duration, thread};

use actix_web::http::Uri;
use anyhow::Error;
use owo_colors::OwoColorize;
use tokio::net::UnixStream;
use tonic::transport::Endpoint;
use tower::service_fn;

use superviseur_server::{api::superviseur::v1alpha1::{core_service_client::CoreServiceClient,  control_service_client::ControlServiceClient, LoadConfigRequest, StartWebDashboardRequest}};
use superviseur_types::{BANNER, UNIX_SOCKET_PATH};
use crate::config::verify_if_config_file_is_present;

pub async fn execute_ui() -> Result<(), Error> {
    let mut config_file_path = "";
    let current_dir = std::env::current_dir()?;    
    let channel = Endpoint::try_from("http://[::]:50051")?
        .connect_with_connector(service_fn(move |_: Uri| UnixStream::connect(UNIX_SOCKET_PATH)))
                .await
                .map_err(|_| 
                    Error::msg(format!("Cannot connect to the Superviseur daemon at unix:{}. Is the superviseur daemon running?", UNIX_SOCKET_PATH)))?;

    if let Ok((config, config_format)) = verify_if_config_file_is_present() {
        let mut client = ControlServiceClient::new(channel.clone());
    
        let request = tonic::Request::new(LoadConfigRequest {
            config,
            file_path: current_dir.to_str().unwrap().to_string(),
            config_format
        });
        client.load_config(request).await?;
        config_file_path = current_dir.to_str().unwrap();
    }

    let mut client = CoreServiceClient::new(channel);
    let request = tonic::Request::new(StartWebDashboardRequest {
        config_file_path: config_file_path.to_string(),
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

}
