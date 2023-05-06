import project.{stop}
import client.{connect, get_project}

pub fn main() {
  connect()
  |> get_project("test")
  |> stop("deno")
}
