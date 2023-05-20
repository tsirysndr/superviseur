project = "spiderlightning-http-demo"

service "http_server" {
  type = "wasm"
  command = "target/wasm32-wasi/debug/server-warp.wasm"
  working_dir = "."
  description = "HTTP server demo"
  depends_on = []
  env = {}
  port = 8080

  use "wasm" {
    runtime "wasmedge" { }
  }
}
