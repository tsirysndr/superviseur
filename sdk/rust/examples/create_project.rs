use std::fs::canonicalize;
use superviseur_client::{client::connect, service::new_service};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let deno_fresh = new_service()
        .with_name("deno-fresh")
        .with_command("./dev.ts")
        .with_env("PORT", "8000");

    let project_dir = canonicalize("../../examples/deno-fresh")?;

    connect()
        .new_project("deno-example")
        .with_context(project_dir.to_str().unwrap())
        .with_service(deno_fresh)
        .stdout()
        .await?;

    Ok(())
}
