project = "spin-demo"

service "http" {
  type = "wasm"
  command = "target/wasm32-wasi/release/http_server.wasm"
  working_dir = "."
  description = "HTTP server demo"
  depends_on = []
  env = {
    "GITHUB_DOMAIN" = "github.com"
  }
  autostart = true
  autorestart = false
  namespace = "http_namespace"
  stdout = "/tmp/http-stdout.log"
  stderr = "/tmp/http-stderr.log"
  port = 3000

  use "wasm" {
    runtime "spin" { }
  }
}
