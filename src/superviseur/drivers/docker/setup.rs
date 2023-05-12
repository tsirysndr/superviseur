use owo_colors::OwoColorize;
use shiplift::{Docker, NetworkCreateOptions, VolumeCreateOptions};

use crate::types::configuration::ConfigurationData;

pub async fn setup_docker(cfg: &ConfigurationData) -> anyhow::Result<()> {
    let docker = Docker::new();
    if let Some(network_settings) = &cfg.network_settings {
        for (name, settings) in network_settings {
            println!("=> Creating network {}", name.bright_green());
            match docker
                .networks()
                .create(
                    &NetworkCreateOptions::builder(name)
                        .driver(
                            settings
                                .driver
                                .as_ref()
                                .map(|x| x.as_str())
                                .unwrap_or("bridge"),
                        )
                        .build(),
                )
                .await
            {
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
            match docker
                .volumes()
                .create(&VolumeCreateOptions::builder().name(name).build())
                .await
            {
                Ok(_) => {}
                Err(e) => {
                    println!("Error creating volume: {}", e);
                }
            }
        }
    }

    Ok(())
}
