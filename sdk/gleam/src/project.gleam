import gleam/io
import base.{Client, send}
import service.{Service}
import query.{build_nested_with_service_query}
import gleam/json.{object, string}
import gleam/option.{None, Option, Some}

pub type Project {
  Project(
    id: String,
    name: String,
    services: List(Service),
    client: Client,
    context: Option(String),
  )
}

pub fn new_project(name: String, client: Client) -> Project {
  Project("", name, [], client, None)
}

pub fn with_context(project: Project, ctx: String) -> Project {
  Project(..project, context: Some(ctx))
}

pub fn with_service(project: Project, service: Service) -> Project {
  io.print("Adding service ")
  io.println(service.name)
  Project(..project, services: [service, ..project.services])
}

pub fn start_all(project: Project) {
  io.println("Starting all services...")
  let body =
    object([
      #(
        "query",
        string(
          "mutation StartAll($projectId: ID!) { start(projectId: $projectId) { name pid serviceId command } }",
        ),
      ),
      #("variables", object([#("projectId", string(project.id))])),
    ])
    |> json.to_string()
  project.client
  |> send(body)
}

pub fn stop_all(project: Project) {
  io.println("Stopping all services...")
  let body =
    object([
      #(
        "query",
        string(
          "mutation StopAll($projectId: ID!) { stop(projectId: $projectId) { name pid serviceId command } }",
        ),
      ),
      #("variables", object([#("projectId", string(project.id))])),
    ])
    |> json.to_string()
  project.client
  |> send(body)
}

pub fn restart_all(project: Project) {
  io.println("Restarting all services...")
  let body =
    object([
      #(
        "query",
        string(
          "mutation RestartAll($projectId: ID!) { restart(projectId: $projectId) { name pid serviceId command } }",
        ),
      ),
      #("variables", object([#("projectId", string(project.id))])),
    ])
    |> json.to_string()
  project.client
  |> send(body)
}

pub fn start(project: Project, service: String) {
  io.println("Starting service...")
  let body =
    object([
      #(
        "query",
        string(
          "mutation Start($id: ID, $projectId: ID!) { start(id: $id, projectId: $projectId) { name pid serviceId command } }",
        ),
      ),
      #(
        "variables",
        object([#("id", string(service)), #("projectId", string(project.id))]),
      ),
    ])
    |> json.to_string()
  project.client
  |> send(body)
}

pub fn stop(project: Project, service: String) {
  io.println("Stopping service...")
  let body =
    object([
      #(
        "query",
        string(
          "mutation Stop($id: ID, $projectId: ID!) { stop(id: $id, projectId: $projectId) { name pid serviceId command } }",
        ),
      ),
      #(
        "variables",
        object([#("id", string(service)), #("projectId", string(project.id))]),
      ),
    ])
    |> json.to_string()
  project.client
  |> send(body)
}

pub fn restart(project: Project, service: String) {
  io.println("Restarting service...")
  let body =
    object([
      #(
        "query",
        string(
          "mutation Restart($id: ID, $projectId: ID!) { restart(id: $id, projectId: $projectId) { name pid serviceId command } }",
        ),
      ),
      #(
        "variables",
        object([#("id", string(service)), #("projectId", string(project.id))]),
      ),
    ])
    |> json.to_string()
  project.client
  |> send(body)
}

pub fn status(project: Project, service: String) {
  io.println("Checking service status...")
  let body =
    object([
      #(
        "query",
        string(
          "query Status($id: ID) { status(id: $id) { name pid serviceId command } }",
        ),
      ),
      #("variables", object([#("id", string(service))])),
    ])
    |> json.to_string()
  project.client
  |> send(body)
}

pub fn list_services(project: Project) {
  io.println("Listing services...")
  let body =
    object([
      #(
        "query",
        string(
          "query Services($projectId: ID!) { services(projectId: $projectId) { id command name status } }",
        ),
      ),
      #("variables", object([#("projectId", string(project.id))])),
    ])
    |> json.to_string()
  project.client
  |> send(body)
}

pub fn logs(project: Project, service: String) {
  io.println("Showing service logs...")
  let body =
    object([
      #(
        "query",
        string(
          "query Logs($id: ID!, $projectId: ID!) { logs(id: $id, projectId: $projectId) { lines } }",
        ),
      ),
      #(
        "variables",
        object([#("id", string(service)), #("projectId", string(project.id))]),
      ),
    ])
    |> json.to_string()
  project.client
  |> send(body)
}

pub fn stdout(project: Project) {
  let nested_query = build_nested_with_service_query(project.services)

  let context = case project.context {
    Some(context) -> context
    None -> "default"
  }

  let body =
    object([
      #(
        "query",
        string(
          "mutation NewProject($name: String!, $context: String!) { newProject(name: $name, context: $context) {" <> nested_query <> " } }",
        ),
      ),
      #(
        "variables",
        object([#("name", string(project.name)), #("context", string(context))]),
      ),
    ])
    |> json.to_string()
  project.client
  |> send(body)
}

pub fn ps(project: Project) {
  io.println("Fetch running services...")
  let body =
    object([
      #(
        "query",
        string("query Processes { processes { command pid type upTime } }"),
      ),
    ])
    |> json.to_string()
  project.client
  |> send(body)
}

pub fn processes(project: Project) {
  io.println("Fetch running services...")
  project
  |> ps
}

pub fn add_env_var(
  project: Project,
  service_id: String,
  name: String,
  value: String,
) {
  io.println("Adding environment variable...")
  let body =
    object([
      #(
        "query",
        string(
          "mutation CreateEnvVar($id: ID!, $projectId: ID!, $name: String!, $value: String!) { addEnvVar(id: $id, projectId: $projectId, name: $name, value: $value) { id env } }",
        ),
      ),
      #(
        "variables",
        object([
          #("id", string(service_id)),
          #("projectId", string(project.id)),
          #("name", string(name)),
          #("value", string(value)),
        ]),
      ),
    ])
    |> json.to_string()

  project.client
  |> send(body)
}

pub fn remove_env_var(project: Project, service_id: String, name: String) {
  io.println("Removing environment variable...")
  let body =
    object([
      #(
        "query",
        string(
          "mutation DeleteEnvVar($id: ID!, $projectId: ID!, $name: String!) { deleteEnvVar(id: $id, projectId: $projectId, name: $name) { id env } }",
        ),
      ),
      #(
        "variables",
        object([
          #("id", string(service_id)),
          #("projectId", string(project.id)),
          #("name", string(name)),
        ]),
      ),
    ])
    |> json.to_string()

  project.client
  |> send(body)
}

pub fn update_env_var(
  project: Project,
  service_id: String,
  name: String,
  value: String,
) {
  io.println("Updating environment variable...")
  let body =
    object([
      #(
        "query",
        string(
          "mutation UpdateEnvVar($id: ID!, $projectId: ID!, $name: String!, $value: String!) { updateEnvVar(id: $id, projectId: $projectId, name: $name, value: $value) { id env } }",
        ),
      ),
      #(
        "variables",
        object([
          #("id", string(service_id)),
          #("projectId", string(project.id)),
          #("name", string(name)),
          #("value", string(value)),
        ]),
      ),
    ])
    |> json.to_string()

  project.client
  |> send(body)
}
