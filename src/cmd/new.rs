use std::{collections::HashMap, io::Write};

use indexmap::IndexMap;
use inquire::Confirm;
use owo_colors::OwoColorize;

use crate::types::configuration::{ConfigFormat, ConfigurationData, Service};

pub fn execute_new(cfg_format: ConfigFormat) {
    let mut env = HashMap::new();
    env.insert("GITHUB_DOMAIN".to_string(), "github.com".to_string());

    let ext = match cfg_format {
        ConfigFormat::HCL => "hcl",
        ConfigFormat::TOML => "toml",
    };

    let filename = format!("Superfile.{}", ext);

    // verify if config file is present
    if std::path::Path::new(&filename).exists() {
        let answer = Confirm::new(
            format!(
                "A {} file already exists in this directory, do you want to overwrite it?",
                filename.bright_green()
            )
            .as_str(),
        )
        .with_default(false)
        .with_help_message("Press y to overwrite the file or n to exit")
        .prompt();
        if answer.is_err() || !answer.unwrap() {
            println!("Exiting...");
            return;
        }
    }

    let mut services = IndexMap::new();
    services.insert(
        String::from("demo"),
        Service {
            id: None,
            name: "demo".to_string(),
            r#type: "exec".to_string(),
            command: "ping $GITHUB_DOMAIN".to_string(),
            stop_command: None,
            working_dir: "/tmp".to_string(),
            watch_dir: None,
            description: Some("Ping Service Example".to_string()),
            depends_on: vec![],
            dependencies: vec![],
            env,
            autostart: None,
            autorestart: None,
            namespace: None,
            port: None,
            stdout: Some("/tmp/demo-stdout.log".to_string()),
            stderr: Some("/tmp/demo-stderr.log".to_string()),
            wait_for: None,
            build: None,
            r#use: None,
            deploy: None,
            test: None,
        },
    );

    let config = ConfigurationData {
        project: "demo".to_string(),
        context: None,
        services,
        network_settings: None,
        volume_settings: None,
    };
    let serialized = match cfg_format {
        ConfigFormat::HCL => hcl::to_string(&config).unwrap(),
        ConfigFormat::TOML => toml::to_string_pretty(&config).unwrap(),
    };

    let mut file = std::fs::File::create(&filename).unwrap();
    file.write_all(serialized.as_bytes()).unwrap();
    println!("Created {} ✨", filename.bright_green());
}
