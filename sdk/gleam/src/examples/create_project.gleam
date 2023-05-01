import project.{stdout, with_context, with_service}
import service.{new_service, with_command, with_name}
import client.{connect, with_project}

pub fn main() {
  let deno_fresh =
    new_service()
    |> with_name("deno-fresh")
    |> with_command("./dev.ts")

  let project =
    connect()
    |> with_project("deno-example")
    |> with_context(
      "/Users/tsirysandratraina/Documents/GitHub/superviseur/examples/deno-fresh",
    )
    |> with_service(deno_fresh)

  project
  |> stdout()
}
