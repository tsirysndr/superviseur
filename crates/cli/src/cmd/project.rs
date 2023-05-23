use anyhow::Error;
use colored_json::ToColoredJson;
use tokio::net::UnixStream;
use tonic::transport::{ Endpoint, Uri};
use tower::service_fn;

use superviseur_server::{api::superviseur::v1alpha1::{project_service_client::ProjectServiceClient, GetProjectRequest, ListProjectsRequest}};
use  superviseur_types::UNIX_SOCKET_PATH;

pub async fn execute_get_project(project_id: &str)-> Result<(), Error> {
    let channel = Endpoint::try_from("http://[::]:50051")?
    .connect_with_connector(service_fn(move |_: Uri| UnixStream::connect(UNIX_SOCKET_PATH)))
        .await
        .map_err(|_| 
            Error::msg(format!("Cannot connect to the Superviseur daemon at unix:{}. Is the superviseur daemon running?", UNIX_SOCKET_PATH)))?;

    let mut client = ProjectServiceClient::new(channel);
    let request = tonic::Request::new(GetProjectRequest {
        id: project_id.to_string(),
    });
    let response = client.get_project(request).await?;
    let response = response.into_inner();
    println!("{}", serde_json::to_string_pretty(&response)?.to_colored_json_auto()?);
    Ok(())
}

pub async fn execute_list_projects() -> Result<(), Error> {
    let channel = Endpoint::try_from("http://[::]:50051")?
    .connect_with_connector(service_fn(move |_: Uri| UnixStream::connect(UNIX_SOCKET_PATH)))
        .await
        .map_err(|_| 
            Error::msg(format!("Cannot connect to the Superviseur daemon at unix:{}. Is the superviseur daemon running?", UNIX_SOCKET_PATH)))?;

    let mut client = ProjectServiceClient::new(channel);
    let request = tonic::Request::new(ListProjectsRequest {
        filter: "".to_string(),
    });
    let response = client.list_projects(request).await?;
    let response = response.into_inner();
    println!("{}", serde_json::to_string_pretty(&response)?.to_colored_json_auto()?);
    Ok(())
}
