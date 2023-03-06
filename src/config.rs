use std::path::Path;

use anyhow::Error;
use owo_colors::OwoColorize;

use crate::types::SUPERFILE;

pub fn verify_if_config_file_is_present() -> Result<(), Error> {
    if !Path::new("Superfile.hcl").exists() {
        return Err(Error::msg(format!(
            "{} not found in current directory, please create one by running {}",
            SUPERFILE.bright_green(),
            "`superviseur new`".cyan()
        )));
    }
    Ok(())
}
