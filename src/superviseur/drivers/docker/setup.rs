use bollard::{network::CreateNetworkOptions, volume::CreateVolumeOptions, Docker};
use owo_colors::OwoColorize;

use crate::types::configuration::ConfigurationData;

pub async fn setup_docker(cfg: &ConfigurationData) -> anyhow::Result<()> {
    let docker = Docker::connect_with_local_defaults()?;
    if let Some(network_settings) = &cfg.network_settings {
        for (name, settings) in network_settings {
            println!("=> Creating network {}", name.bright_green());
            let options = CreateNetworkOptions {
                name: name.clone(),
                driver: settings.driver.clone().unwrap_or("bridge".to_string()),
                ..Default::default()
            };
            match docker.create_network(options).await {
                Ok(_) => {}
                Err(e) => {
                    println!("Error creating network: {}", e);
                }
            }
        }
    }

    if let Some(volume_settings) = &cfg.volume_settings {
        for (name, _) in volume_settings {
            println!("=> Creating volume {}", name.bright_green());
            let option = CreateVolumeOptions {
                name: name.clone(),
                ..Default::default()
            };
            match docker.create_volume(option).await {
                Ok(_) => {}
                Err(e) => {
                    println!("Error creating volume: {}", e);
                }
            }
        }
    }
    Ok(())
}
