use crate::{superviseur::provider::kv::kv::Provider, types::configuration::ConfigurationData};

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

pub const CONFIG_EXAMPLE_WITH_USE: &str = r#"
project = "deno-fresh"

service "deno" {
  type = "exec"
  command = "./dev.ts"
  working_dir = "."
  description = "Deno example app"
  depends_on = []
  env = {}
  port = 8000

  use "devbox" {
    packages = ["deno"]
  }

}
"#;

pub const CONFIG_EXAMPLE_WITH_ENV: &str = r#"
project = "deno-fresh"

service "deno" {
  type = "exec"
  command = "./dev.ts"
  working_dir = "."
  description = "Deno example app"
  depends_on = []
  env = {
    "PORT" = "8000"
  }
  port = 8000

  use "devbox" {
    packages = ["deno"]
  }

}
"#;

#[test]
pub fn can_save_configuration() {
    let provider = Provider::default();
    let mut config = hcl::from_str::<ConfigurationData>(CONFIG_EXAMPLE).unwrap();
    config.context = Some(".".to_string());
    let mut kv_pairs = provider.save_configuration("project_id", config).unwrap();
    kv_pairs.sort_by(|a, b| a.key.cmp(&b.key));

    assert_eq!(kv_pairs.len(), 12);
    assert_eq!(kv_pairs[0].key, "superviseur/project_id/context");
    assert_eq!(kv_pairs[0].value, ".");
    assert_eq!(kv_pairs[1].key, "superviseur/project_id/project");
    assert_eq!(kv_pairs[1].value, "demo");
    assert_eq!(
        kv_pairs[2].key,
        "superviseur/project_id/services/demo/command"
    );
    assert_eq!(kv_pairs[2].value, "ping $GITHUB_DOMAIN");
    assert_eq!(
        kv_pairs[3].key,
        "superviseur/project_id/services/demo/dependencies"
    );
    assert_eq!(kv_pairs[3].value, "");
    assert_eq!(
        kv_pairs[4].key,
        "superviseur/project_id/services/demo/depends_on"
    );
    assert_eq!(kv_pairs[4].value, "");
    assert_eq!(
        kv_pairs[5].key,
        "superviseur/project_id/services/demo/description"
    );
    assert_eq!(kv_pairs[5].value, "Ping Service Example");
    assert_eq!(kv_pairs[6].key, "superviseur/project_id/services/demo/env");
    assert_eq!(kv_pairs[6].value, "GITHUB_DOMAIN=github.com");
    assert_eq!(kv_pairs[7].key, "superviseur/project_id/services/demo/name");
    assert_eq!(kv_pairs[7].value, "demo");
    assert_eq!(
        kv_pairs[8].key,
        "superviseur/project_id/services/demo/stderr"
    );
    assert_eq!(kv_pairs[8].value, "/tmp/demo-stderr.log");
    assert_eq!(
        kv_pairs[9].key,
        "superviseur/project_id/services/demo/stdout"
    );
    assert_eq!(kv_pairs[9].value, "/tmp/demo-stdout.log");
    assert_eq!(
        kv_pairs[10].key,
        "superviseur/project_id/services/demo/type"
    );
    assert_eq!(kv_pairs[10].value, "exec");
    assert_eq!(
        kv_pairs[11].key,
        "superviseur/project_id/services/demo/working_dir"
    );
    assert_eq!(kv_pairs[11].value, "/tmp");
}

#[test]
pub fn can_build_configuration() {
    let provider = Provider::default();
    let mut config = hcl::from_str::<ConfigurationData>(CONFIG_EXAMPLE_WITH_USE).unwrap();
    config.context = Some(".".to_string());
    provider.save_configuration("project_id", config).unwrap();
    let configuration = provider.build_configuration("project_id").unwrap();
    let service = configuration.services.get("deno").unwrap();
    assert_eq!(configuration.project, "deno-fresh");
    assert_eq!(configuration.services.len(), 1);
    assert_eq!(service.name, "deno");
    assert_eq!(service.r#type, "exec");
    assert_eq!(service.command, "./dev.ts");
    assert_eq!(service.working_dir, ".");
    assert_eq!(service.description, Some("Deno example app".to_string()));
    assert_eq!(service.depends_on.len(), 0);
    assert_eq!(service.dependencies.len(), 0);
    assert_eq!(service.env.len(), 0);
    assert_eq!(service.port, Some(8000));
}

#[test]
pub fn can_build_configuration_with_env() {
    let provider = Provider::default();
    let mut config = hcl::from_str::<ConfigurationData>(CONFIG_EXAMPLE_WITH_ENV).unwrap();
    config.context = Some(".".to_string());
    provider.save_configuration("project_id", config).unwrap();
    let configuration = provider.build_configuration("project_id").unwrap();
    let service = configuration.services.get("deno").unwrap();
    assert_eq!(configuration.project, "deno-fresh");
    assert_eq!(configuration.services.len(), 1);
    assert_eq!(service.name, "deno");
    assert_eq!(service.r#type, "exec");
    assert_eq!(service.command, "./dev.ts");
    assert_eq!(service.working_dir, ".");
    assert_eq!(service.description, Some("Deno example app".to_string()));
    assert_eq!(service.depends_on.len(), 0);
    assert_eq!(service.dependencies.len(), 0);
    assert_eq!(service.env.len(), 1);
    assert_eq!(service.env.get("PORT").unwrap(), "8000");
    assert_eq!(service.port, Some(8000));
}
