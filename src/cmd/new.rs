use std::{collections::HashMap, io::Write};

use owo_colors::OwoColorize;

use crate::types::configuration::{ConfigFormat, ConfigurationData, Service};

pub fn execute_new(cfg_format: ConfigFormat) {
    let mut env = HashMap::new();
    env.insert("GITHUB_DOMAIN".to_string(), "github.com".to_string());

    let config = ConfigurationData {
        project: "demo".to_string(),
        services: vec![Service {
            id: None,
            name: "demo".to_string(),
            r#type: "exec".to_string(),
            command: "ping $GITHUB_DOMAIN".to_string(),
            working_dir: "/tmp".to_string(),
            description: Some("Ping Service Example".to_string()),
            depends_on: vec![],
            env,
            autostart: true,
            autorestart: false,
            namespace: Some("demo_namespace".to_string()),
            port: 5060,
            stdout: "/tmp/demo-stdout.log".to_string(),
            stderr: "/tmp/demo-stderr.log".to_string(),
        }],
    };
    let serialized = match cfg_format {
        ConfigFormat::HCL => hcl::to_string(&config).unwrap(),
        ConfigFormat::TOML => toml::to_string_pretty(&config).unwrap(),
    };

    let ext = match cfg_format {
        ConfigFormat::HCL => "hcl",
        ConfigFormat::TOML => "toml",
    };

    let filename = format!("Superfile.{}", ext);
    let mut file = std::fs::File::create(&filename).unwrap();
    file.write_all(serialized.as_bytes()).unwrap();
    println!("Created {} ✨", filename.bright_green());
}
