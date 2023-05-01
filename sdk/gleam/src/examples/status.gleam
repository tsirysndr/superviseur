import project.{status}
import client.{connect, get_project}

pub fn main() {
  connect()
  |> get_project("test")
  |> status("deno")
}
