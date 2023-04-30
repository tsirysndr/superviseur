import gleam/io
import project.{Project, new_project}
import base.{Client}

pub fn connect() -> Client {
  io.println("Connecting to server...")
  Client
}

pub fn with_project(client: Client, name: String) -> Project {
  io.println("Connecting to project...")
  new_project(name, client)
}
