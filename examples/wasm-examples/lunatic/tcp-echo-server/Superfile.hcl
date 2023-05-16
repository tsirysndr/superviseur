project = "spiderlightning-http-demo"

service "tcp_echo_server" {
  type = "wasm"
  command = "target/wasm32-wasi/debug/tcp-echo-server.wasm"
  working_dir = "."
  description = "TCP echo server demo"
  depends_on = []
  env = {}
  autostart = true
  autorestart = false
  namespace = "tcp_echo_namespace"
  stdout = "/tmp/tcp_echo_server-stdout.log"
  stderr = "/tmp/tcp_echo_server-stderr.log"
  port = 4000

  use "wasm" {
    runtime "lunatic" { }
  }
}
