import gleam/io
import base.{Client, send}
import service.{Service}
import gleam/json.{object, string}
import gleam/option.{None, Option, Some}

pub type Project {
  Project(
    name: String,
    services: List(Service),
    client: Client,
    context: Option(String),
  )
}

pub fn new_project(name: String, client: Client) -> Project {
  Project(name, [], client, None)
}

pub fn with_context(project: Project, ctx: String) -> Project {
  Project(..project, context: Some(ctx))
}

pub fn with_service(project: Project, service: Service) -> Project {
  io.print("Adding service ")
  io.println(service.name)
  Project(..project, services: [service, ..project.services])
}

pub fn start_all(_project: Project) {
  io.println("Starting all services...")
}

pub fn stop_all(_project: Project) {
  io.println("Stopping all services...")
}

pub fn restart_all(_project: Project) {
  io.println("Restarting all services...")
}

pub fn start(_project: Project, _service: String) {
  io.println("Starting service...")
}

pub fn stop(_project: Project, _service: String) {
  io.println("Stopping service...")
}

pub fn restart(_project: Project, _service: String) {
  io.println("Restarting service...")
}

pub fn status(_project: Project, _service: String) {
  io.println("Checking service status...")
}

pub fn list_services(_project: Project) {
  io.println("Listing services...")
}

pub fn logs(_project: Project, _service: String) {
  io.println("Showing service logs...")
}

pub fn stdout(project: Project) {
  let body =
    object([#("query", string("query Projects { projects { id name } }"))])
    |> json.to_string()
  project.client
  |> send(body)
}

pub fn ps(project: Project) {
  io.println("Fetch running services...")
}
