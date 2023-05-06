use superviseur_client::client::connect;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let projects = connect().projects().await?;
    println!(
        "{:#?}",
        projects
            .into_iter()
            .map(|p| (p.name, p.id))
            .collect::<Vec<(String, String)>>()
    );
    Ok(())
}
