import project.{ps}
import client.{connect, get_project}

pub fn main() {
  connect()
  |> get_project("obese-ants")
  |> ps()
}
