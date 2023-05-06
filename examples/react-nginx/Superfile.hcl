project = "react-nginx"

service "react" {
  type = "exec"
  command = "npm start"
  working_dir = "."
  description = "React app"
  depends_on = []
  env = { }
  autostart = true
  autorestart = false
  namespace = "demo_namespace"
  stdout = "/tmp/react-stdout.log"
  stderr = "/tmp/react-stderr.log"
  flox = { 
    environment = ".#react-nginx"
  }
}
