use anyhow::Error;
use superviseur_server::api::superviseur::v1alpha1::{project_service_client::ProjectServiceClient, OpenProjectRequest};
use superviseur_types::UNIX_SOCKET_PATH;
use tokio::net::UnixStream;
use tonic::transport::{ Endpoint, Uri};
use tower::service_fn;

pub async fn execute_code(name: &str) -> Result<(), Error> {
  let channel = Endpoint::try_from("http://[::]:50051")?
  .connect_with_connector(service_fn(move |_: Uri| UnixStream::connect(UNIX_SOCKET_PATH)))
      .await
      .map_err(|_| 
          Error::msg(format!("Cannot connect to the Superviseur daemon at unix:{}. Is the superviseur daemon running?", UNIX_SOCKET_PATH)))?;

  let mut client = ProjectServiceClient::new(channel);
  let request = tonic::Request::new(OpenProjectRequest {
      id: name.to_string(),
  });
  let response = client.open_project(request).await?;
  let mut stream = response.into_inner();

    while let Some(message) = stream.message().await? {
        println!("{}", message.line);
        if message.line.starts_with("Open this link in your browser") {
          let link = message.line.replace("Open this link in your browser ", "");
            open::that(&link)?;
        }
    }

  Ok(())
}
