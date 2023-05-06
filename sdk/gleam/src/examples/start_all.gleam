import project.{start_all}
import client.{connect, get_project}

pub fn main() {
  connect()
  |> get_project("test")
  |> start_all()
}
