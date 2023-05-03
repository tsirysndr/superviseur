import project.{list_services}
import client.{connect, get_project}

pub fn main() {
  connect()
  |> get_project("obese-ants")
  |> list_services()
}
