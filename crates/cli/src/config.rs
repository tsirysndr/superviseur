use std::path::Path;

use anyhow::Error;
use owo_colors::OwoColorize;

use superviseur_types::{SUPERFILE, SUPERFILE_TOML};

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

#[cfg(test)]
pub mod tests {
    use super::*;

    pub const CONFIG_EXAMPLE: &str = r#"
    project = "demo"

    service "demo" {
      type = "exec"
      command = "ping $GITHUB_DOMAIN"
      working_dir = "/tmp"
      description = "Ping Service Example"
      depends_on = []
      env = {
        "GITHUB_DOMAIN" = "github.com"
      }
      stdout = "/tmp/demo-stdout.log"
      stderr = "/tmp/demo-stderr.log"
    }    
    "#;

    #[test]
    fn test_verify_if_config_file_is_present() {
        // create a default config file
        let current_dir = std::env::current_dir().unwrap();
        let config_file = current_dir.join(SUPERFILE);
        std::fs::write(config_file, CONFIG_EXAMPLE).unwrap();
        let result = verify_if_config_file_is_present();
        assert!(result.is_ok());
    }
}
