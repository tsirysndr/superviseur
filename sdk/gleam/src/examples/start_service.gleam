import project.{start}
import client.{connect, get_project}

pub fn main() {
  connect()
  |> get_project("test")
  |> start("test")
}
