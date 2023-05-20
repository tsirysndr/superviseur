project = "spin-demo"

service "http" {
  type = "wasm"
  command = "target/wasm32-wasi/release/http_server.wasm"
  working_dir = "."
  description = "HTTP server demo"
  depends_on = []
  env = { }
  port = 3000

  use "wasm" {
    runtime "spin" { }
  }
}
