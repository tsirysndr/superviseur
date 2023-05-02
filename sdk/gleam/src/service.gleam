import gleam/option.{None, Option, Some}
import flox.{Flox}

pub type Service {
  Service(
    name: String,
    service_type: String,
    command: String,
    description: String,
    depends_on: List(String),
    env: List(String),
    auto_restart: Bool,
    auto_start: Bool,
    namespace: String,
    port: Int,
    stdout: String,
    stderr: String,
    flox: Option(Flox),
    build_command: String,
    enable_docker: Option(Bool),
    enable_nix: Option(Bool),
  )
}

pub fn new_service() -> Service {
  Service(
    "",
    "",
    "",
    "",
    [],
    [],
    False,
    False,
    "default",
    0,
    "",
    "",
    None,
    "",
    None,
    None,
  )
}

pub fn with_name(service: Service, name: String) -> Service {
  Service(..service, name: name)
}

pub fn with_type(service: Service, service_type: String) -> Service {
  Service(..service, service_type: service_type)
}

pub fn with_command(service: Service, command: String) -> Service {
  Service(..service, command: command)
}

pub fn with_depends_on(service: Service, depends_on: List(String)) -> Service {
  Service(..service, depends_on: depends_on)
}

pub fn with_description(service: Service, description: String) -> Service {
  Service(..service, description: description)
}

pub fn with_env(service: Service, env: List(String)) -> Service {
  Service(..service, env: env)
}

pub fn with_auto_restart(service: Service, auto_restart: Bool) -> Service {
  Service(..service, auto_restart: auto_restart)
}

pub fn with_auto_start(service: Service, auto_start: Bool) -> Service {
  Service(..service, auto_start: auto_start)
}

pub fn with_namespace(service: Service, namespace: String) -> Service {
  Service(..service, namespace: namespace)
}

pub fn with_port(service: Service, port: Int) -> Service {
  Service(..service, port: port)
}

pub fn with_stdout(service: Service, stdout: String) -> Service {
  Service(..service, stdout: stdout)
}

pub fn with_stderr(service: Service, stderr: String) -> Service {
  Service(..service, stderr: stderr)
}

pub fn with_flox(service: Service, flox: Flox) -> Service {
  Service(..service, flox: Some(flox))
}

pub fn with_build_command(service: Service, build_command: String) -> Service {
  Service(..service, build_command: build_command)
}

pub fn with_enable_docker(service: Service, enable_docker: Bool) -> Service {
  Service(..service, enable_docker: Some(enable_docker))
}

pub fn with_enable_nix(service: Service, enable_nix: Bool) -> Service {
  Service(..service, enable_nix: Some(enable_nix))
}
