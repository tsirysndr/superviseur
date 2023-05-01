import gleam/json.{object, string}

pub fn encode_body(body: String) -> String {
  object([#("query", string(body))])
  |> json.to_string()
}
