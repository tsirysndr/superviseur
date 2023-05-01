import project.{stop_all}
import client.{connect, get_project}

pub fn main() {
  connect()
  |> get_project("test")
  |> stop_all()
}
