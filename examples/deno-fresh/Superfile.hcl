project = "deno-fresh"

service "deno" {
  type = "docker"
  command = "./dev.ts"
  working_dir = "."
  description = "Deno example app"
  depends_on = []
  env = {}
  port = 8000

  #use "flox" {
  #  environment = ".#deno-fresh"
  #}

  use "docker" {
  # volumes = ["./data:/data"]
  }
}
