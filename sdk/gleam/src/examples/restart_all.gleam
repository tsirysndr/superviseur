import project.{restart_all}
import client.{connect, get_project}

pub fn main() {
  connect()
  |> get_project("test")
  |> restart_all()
}
