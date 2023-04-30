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
  )
}

pub fn new_service() -> Service {
  Service("", "", "", "", [], [], False, False, "default", 0, "", "", None, "")
}

pub fn with_name(service: Service, name: String) -> Service {
  Service(
    name,
    service.service_type,
    service.command,
    service.description,
    service.depends_on,
    service.env,
    service.auto_restart,
    service.auto_start,
    service.namespace,
    service.port,
    service.stdout,
    service.stderr,
    service.flox,
    service.build_command,
  )
}

pub fn with_type(service: Service, service_type: String) -> Service {
  Service(
    service.name,
    service_type,
    service.command,
    service.description,
    service.depends_on,
    service.env,
    service.auto_restart,
    service.auto_start,
    service.namespace,
    service.port,
    service.stdout,
    service.stderr,
    service.flox,
    service.build_command,
  )
}

pub fn with_command(service: Service, command: String) -> Service {
  Service(
    service.name,
    service.service_type,
    command,
    service.description,
    service.depends_on,
    service.env,
    service.auto_restart,
    service.auto_start,
    service.namespace,
    service.port,
    service.stdout,
    service.stderr,
    service.flox,
    service.build_command,
  )
}

pub fn with_depends_on(service: Service, depends_on: List(String)) -> Service {
  Service(
    service.name,
    service.service_type,
    service.command,
    service.description,
    depends_on,
    service.env,
    service.auto_restart,
    service.auto_start,
    service.namespace,
    service.port,
    service.stdout,
    service.stderr,
    service.flox,
    service.build_command,
  )
}

pub fn with_description(service: Service, description: String) -> Service {
  Service(
    service.name,
    service.service_type,
    service.command,
    description,
    service.depends_on,
    service.env,
    service.auto_restart,
    service.auto_start,
    service.namespace,
    service.port,
    service.stdout,
    service.stderr,
    service.flox,
    service.build_command,
  )
}

pub fn with_env(service: Service, env: List(String)) -> Service {
  Service(
    service.name,
    service.service_type,
    service.command,
    service.description,
    service.depends_on,
    env,
    service.auto_restart,
    service.auto_start,
    service.namespace,
    service.port,
    service.stdout,
    service.stderr,
    service.flox,
    service.build_command,
  )
}

pub fn with_auto_restart(service: Service, auto_restart: Bool) -> Service {
  Service(
    service.name,
    service.service_type,
    service.command,
    service.description,
    service.depends_on,
    service.env,
    auto_restart,
    service.auto_start,
    service.namespace,
    service.port,
    service.stdout,
    service.stderr,
    service.flox,
    service.build_command,
  )
}

pub fn with_auto_start(service: Service, auto_start: Bool) -> Service {
  Service(
    service.name,
    service.service_type,
    service.command,
    service.description,
    service.depends_on,
    service.env,
    service.auto_restart,
    auto_start,
    service.namespace,
    service.port,
    service.stdout,
    service.stderr,
    service.flox,
    service.build_command,
  )
}

pub fn with_namespace(service: Service, namespace: String) -> Service {
  Service(
    service.name,
    service.service_type,
    service.command,
    service.description,
    service.depends_on,
    service.env,
    service.auto_restart,
    service.auto_start,
    namespace,
    service.port,
    service.stdout,
    service.stderr,
    service.flox,
    service.build_command,
  )
}

pub fn with_port(service: Service, port: Int) -> Service {
  Service(
    service.name,
    service.service_type,
    service.command,
    service.description,
    service.depends_on,
    service.env,
    service.auto_restart,
    service.auto_start,
    service.namespace,
    port,
    service.stdout,
    service.stderr,
    service.flox,
    service.build_command,
  )
}

pub fn with_stdout(service: Service, stdout: String) -> Service {
  Service(
    service.name,
    service.service_type,
    service.command,
    service.description,
    service.depends_on,
    service.env,
    service.auto_restart,
    service.auto_start,
    service.namespace,
    service.port,
    stdout,
    service.stderr,
    service.flox,
    service.build_command,
  )
}

pub fn with_stderr(service: Service, stderr: String) -> Service {
  Service(
    service.name,
    service.service_type,
    service.command,
    service.description,
    service.depends_on,
    service.env,
    service.auto_restart,
    service.auto_start,
    service.namespace,
    service.port,
    service.stdout,
    stderr,
    service.flox,
    service.build_command,
  )
}

pub fn with_flox(service: Service, flox: Flox) -> Service {
  Service(
    service.name,
    service.service_type,
    service.command,
    service.description,
    service.depends_on,
    service.env,
    service.auto_restart,
    service.auto_start,
    service.namespace,
    service.port,
    service.stdout,
    service.stderr,
    Some(flox),
    service.build_command,
  )
}

pub fn with_build_command(service: Service, build_command: String) -> Service {
  Service(
    service.name,
    service.service_type,
    service.command,
    service.description,
    service.depends_on,
    service.env,
    service.auto_restart,
    service.auto_start,
    service.namespace,
    service.port,
    service.stdout,
    service.stderr,
    service.flox,
    build_command,
  )
}
