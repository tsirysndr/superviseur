use superviseur_client::client::connect;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let project = connect().project("obese-ants").await?;
    let logs = project.logs("happy-poison").await?;
    println!("{:#?}", logs);
    Ok(())
}
