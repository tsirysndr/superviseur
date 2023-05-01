import gleam/io
import project.{Project, new_project}
import base.{Client}
import gleam/http.{Http, Post}
import gleam/http/request

pub fn connect() -> Client {
  io.println("Connecting to server...")
  let request =
    request.new()
    |> request.set_method(Post)
    |> request.set_scheme(Http)
    |> request.set_host("localhost")
    |> request.set_port(5478)
    |> request.set_path("/graphql")
    |> request.set_header("content-type", "application/json")
  Client(request)
}

pub fn with_project(client: Client, name: String) -> Project {
  io.println("Connecting to project...")
  new_project(name, client)
}

pub fn get_project(client: Client, id: String) -> Project {
  io.println("Fetching project...")
  new_project(id, client)
}

pub fn projects(client: Client) -> List(String) {
  io.println("Fetching projects...")
  []
}
