project = "spiderlightning-http-demo"

service "tcp_echo_server" {
  type = "wasm"
  command = "target/wasm32-wasi/debug/tcp-echo-server.wasm"
  working_dir = "."
  description = "TCP echo server demo"
  depends_on = []
  env = {}
  port = 4000

  use "wasm" {
    runtime "lunatic" { }
  }
}
