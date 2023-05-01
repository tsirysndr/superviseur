import client.{connect, projects}

pub fn main() {
  connect()
  |> projects()
}
