import gleam/json.{object, string}
import gleam/list.{fold}
import gleam/string.{replace}
import service.{Service}

pub fn encode_body(body: String) -> String {
  object([#("query", string(body))])
  |> json.to_string()
}

pub fn build_nested_with_service_query(services: List(Service)) -> String {
  let query = ""
  let sub_query =
    fold(
      services,
      query,
      fn(acc, service) {
        "withService(service: { " <> build_params(service) <> " }) { " <> acc <> " }"
      },
    )
  sub_query
  |> replace("{  }", "{ id stdout }")
}

fn build_params(service: Service) -> String {
  let params =
    "name: \"" <> service.name <> "\", command: \"" <> service.command <> "\""
  // TODO: add env vars
  // TODO: add dependencies
  params
}
