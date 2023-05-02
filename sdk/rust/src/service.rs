use std::collections::HashMap;

#[derive(Debug, Default)]
pub struct Service {
    pub name: String,
    pub command: String,
    pub r#type: String,
    pub working_dir: String,
    pub description: String,
    pub depends_on: Vec<String>,
    pub env: HashMap<String, String>,
    pub auto_start: bool,
    pub auto_restart: bool,
    pub namespace: String,
    pub stdout: String,
    pub stderr: String,
    pub build_command: String,
    pub flox_enviroment: Option<String>,
    pub enable_docker: Option<bool>,
    pub enable_nix: Option<bool>,
}

impl Service {
    pub fn with_name(mut self, name: &str) -> Service {
        self.name = name.to_string();
        self
    }

    pub fn with_command(mut self, command: &str) -> Service {
        self.command = command.to_string();
        self
    }

    pub fn with_type(mut self, r#type: &str) -> Service {
        self.r#type = r#type.to_string();
        self
    }

    pub fn with_working_dir(mut self, working_dir: &str) -> Service {
        self.working_dir = working_dir.to_string();
        self
    }

    pub fn with_description(mut self, description: &str) -> Service {
        self.description = description.to_string();
        self
    }

    pub fn with_depends_on(mut self, depends_on: Vec<String>) -> Service {
        self.depends_on = depends_on;
        self
    }

    pub fn with_env(mut self, env: HashMap<String, String>) -> Service {
        self.env = env;
        self
    }

    pub fn with_auto_start(mut self, auto_start: bool) -> Service {
        self.auto_start = auto_start;
        self
    }

    pub fn with_auto_restart(mut self, auto_restart: bool) -> Service {
        self.auto_restart = auto_restart;
        self
    }

    pub fn with_namespace(mut self, namespace: &str) -> Service {
        self.namespace = namespace.to_string();
        self
    }

    pub fn with_stdout(mut self, stdout: &str) -> Service {
        self.stdout = stdout.to_string();
        self
    }

    pub fn with_stderr(mut self, stderr: &str) -> Service {
        self.stderr = stderr.to_string();
        self
    }

    pub fn with_build_command(mut self, build_command: &str) -> Service {
        self.build_command = build_command.to_string();
        self
    }

    pub fn with_flox_enviroment(mut self, flox_enviroment: &str) -> Service {
        self.flox_enviroment = Some(flox_enviroment.to_string());
        self
    }

    pub fn with_enable_docker(mut self, enable_docker: bool) -> Service {
        self.enable_docker = Some(enable_docker);
        self
    }

    pub fn with_enable_nix(mut self, enable_nix: bool) -> Service {
        self.enable_nix = Some(enable_nix);
        self
    }
}

pub fn new_service() -> Service {
    Service {
        ..Default::default()
    }
}
