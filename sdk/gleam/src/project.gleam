import gleam/io
import base.{Client}
import service.{Service}

pub type Project {
  Project(name: String, services: List(Service), client: Client)
}

pub fn new_project(name: String, client: Client) -> Project {
  Project(name, [], client)
}

pub fn with_service(project: Project, service: Service) -> Project {
  io.print("Adding service ")
  io.println(service.name)
  Project(project.name, [service, ..project.services], project.client)
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

pub fn list(_project: Project) {
  io.println("Listing services...")
}

pub fn logs(_project: Project, _service: String) {
  io.println("Showing service logs...")
}
