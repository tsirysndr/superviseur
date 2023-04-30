import gleam/io
import project.{
  list, logs, restart_all, start_all, status, stop_all, with_service,
}
import service.{new_service, with_name}
import client.{connect, with_project}

pub fn main() {
  io.println("Hello from superviseur_client!")
  let service1 =
    new_service()
    |> with_name("my-service-1")

  let service2 =
    new_service()
    |> with_name("my-service-2")

  let service3 =
    new_service()
    |> with_name("my-service-3")

  let project =
    connect()
    |> with_project("my-project")
    |> with_service(service1)
    |> with_service(service2)
    |> with_service(service3)

  project
  |> start_all()

  project
  |> logs("my-service-1")

  project
  |> list()

  project
  |> status("my-service-1")

  project
  |> restart_all()

  project
  |> stop_all()
}
