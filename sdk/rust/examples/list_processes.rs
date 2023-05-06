use superviseur_client::client::connect;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let project = connect().project("obese-ants").await?;
    let processes = project.processes().await?;
    println!("{:#?}", processes);
    Ok(())
}
