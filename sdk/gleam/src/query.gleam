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
    "name: \"" <> service.name <> "\", command: \"" <> service.command <> "\"" <> build_depends_on(
      service.depends_on,
    )
    |> replace(", ]", "]") <> build_env(service.env)
    |> replace(", ]", "]")
  params
}

fn build_depends_on(depends_on: List(String)) -> String {
  case depends_on {
    [] -> ""
    _ ->
      ", dependsOn: [" <> fold(
        depends_on,
        "",
        fn(acc, d) { acc <> "\"" <> d <> "\", " },
      ) <> "]"
  }
}

fn build_env(env: List(String)) -> String {
  case env {
    [] -> ""
    _ ->
      ", env: [" <> fold(env, "", fn(acc, e) { acc <> "\"" <> e <> "\", " }) <> "]"
  }
}
