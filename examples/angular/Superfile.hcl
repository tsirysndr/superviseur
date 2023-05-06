project = "angular"

service "angular" {
  "type" = "exec"
  "command" = "npm start"
  "working_dir" = "./angular"
  "description" = "Angular example"
  "depends_on" = []
  "env" = {}
  "autostart" = true
  "autorestart" = false
  "namespace" = "demo_namespace"
  "stdout" = "/tmp/angular-stdout.log"
  "stderr" = "/tmp/angular-stderr.log"
  "port" = 4200
  flox = {
    "environment" = ".#angular"
  }
  build = {
    "command" = "npm install"
  }
}
