use std::{collections::HashMap, io::Write};

use owo_colors::OwoColorize;

use crate::types::configuration::{ConfigFormat, ConfigurationData, Service};

pub fn execute_new(cfg_format: ConfigFormat) {
    let mut env = HashMap::new();
    env.insert("NAME".to_string(), "world".to_string());

    let config = ConfigurationData {
        project: "demo".to_string(),
        services: vec![Service {
            name: "demo".to_string(),
            r#type: "exec".to_string(),
            command: "echo hello $NAME".to_string(),
            working_dir: "/tmp".to_string(),
            description: Some("Demo service".to_string()),
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
