use anyhow::Error;
use chrono::{DateTime, Utc};
use owo_colors::OwoColorize;
use tokio::net::UnixStream;
use tonic::transport::{ Endpoint, Uri};
use tower::service_fn;

use crate::{
    api::superviseur::v1alpha1::{
        control_service_client::ControlServiceClient, LoadConfigRequest, StatusRequest,
    },
    types::{process::format_duration, UNIX_SOCKET_PATH},
};

pub async fn execute_status(name: &str) -> Result<(), Error> {
    let current_dir = std::env::current_dir()?;
    let config = std::fs::read_to_string(current_dir.join("Superfile.hcl"))?;
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

    let request = tonic::Request::new(StatusRequest {
        name: name.to_string(),
        config_file_path: current_dir.to_str().unwrap().to_string(),
    });

    let response = client.status(request).await?;
    let response = response.into_inner();

    let process = response.process.unwrap();

    let status_dot = match process.state.as_str() {
        "Running" => format!("{}", "●".bright_green()),
        "Stopped" => format!("{}", "●".bright_red()),
        _ => format!("{}", "●".bright_yellow()),
    };
    match process.description.len() {
        0 => println!("{} {}", status_dot, process.name),
        _ => println!("{} {} - {}", status_dot, process.name, process.description),
    }

    let status = match process.state.as_str() {
        "Running" => {
            let uptime: DateTime<Utc> = process.up_time.parse().unwrap();
            let since = format_duration(Utc::now() - uptime);
            format!(
                "{} since {}; {}",
                "Running".bright_green(),
                uptime.to_string(),
                since
            )
        }
        _ => format!("{}", process.state),
    };
    println!("{:>15} {}", "Active:", status);

    if process.state == "Running" {
        println!("{:>15} {}", "PID:", process.pid);
    }

    println!("{:>15} {}", "Command:", process.command);
    println!("{:>15} {}", "Directory:", process.working_directory);
    println!("{:>15} {}", "Log:", process.log_file);
    println!("{:>15} {}", "Stderr:", process.stderr_file);
    println!("{:>15} {}", "AutoRestart:", process.auto_restart);
    println!("{:>15} {}", "Type:", process.r#type);
    println!("{:>15} {}", "Envs:", process.env.join(", "));

    Ok(())
}
