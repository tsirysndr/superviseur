use superviseur_client::{client::connect, service::new_service};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let deno_fresh = new_service()
        .with_name("deno-fresh")
        .with_command("./dev.ts")
        .with_env("PORT", "8000");

    connect()
        .new_project("deno-example")
        .with_context("/Users/tsirysandratraina/Documents/GitHub/superviseur/examples/deno-fresh")
        .with_service(deno_fresh)
        .stdout()
        .await?;

    Ok(())
}
