use anyhow::Error;
use owo_colors::OwoColorize;

use crate::{types::BANNER, webui::start_webui};

pub async fn execute_ui() -> Result<(), Error> {
    println!("{}", BANNER.bright_purple());
    println!(
        "Starting dashboard ui on {} 🚀",
        "http://localhost:5478".cyan()
    );
    start_webui().await?;
    Ok(())
}
