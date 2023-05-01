import gleam/http/request.{Request}
import gleam/hackney
import gleam/io

pub type Client {
  Client(request: Request(String))
}

pub fn send(client: Client, query: String) {
  let request =
    client.request
    |> request.set_body(query)
  case hackney.send(request) {
    Ok(response) -> {
      io.println("Response:")
      io.println(response.body)
    }
    _ -> {
      io.println("Error:")
    }
  }
}
