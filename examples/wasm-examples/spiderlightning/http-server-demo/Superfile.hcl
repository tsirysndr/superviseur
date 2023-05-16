project = "spiderlightning-http-demo"

service "http_server" {
  type = "wasm"
  command = "target/wasm32-wasi/release/http_server_lib.wasm"
  working_dir = "."
  description = "HTTP server demo"
  depends_on = []
  env = {}
  autostart = true
  autorestart = false
  namespace = "http_namespace"
  stdout = "/tmp/http_server-stdout.log"
  stderr = "/tmp/demo_server-stderr.log"
  port = 3000

  use "wasm" {
    runtime "spiderlightning" { }
  }
}
