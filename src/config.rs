use std::path::Path;

use anyhow::Error;
use owo_colors::OwoColorize;

use crate::types::{SUPERFILE, SUPERFILE_TOML};

pub fn verify_if_config_file_is_present() -> Result<(String, String), Error> {
    if !Path::new(SUPERFILE).exists() && !Path::new(SUPERFILE_TOML).exists() {
        return Err(Error::msg(format!(
            "{} not found in current directory, please create one by running {}",
            SUPERFILE.bright_green(),
            "`superviseur new`".cyan()
        )));
    }

    let current_dir = std::env::current_dir()?;

    if Path::new(SUPERFILE).exists() {
        let config = std::fs::read_to_string(current_dir.join(SUPERFILE))?;
        return Ok((config, String::from("hcl")));
    }

    let config = std::fs::read_to_string(current_dir.join(SUPERFILE_TOML))?;
    return Ok((config, String::from("toml")));
}
