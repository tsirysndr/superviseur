project = "angular"
services = [
  {
    "name" = "angular"
    "type" = "exec"
    "command" = "npm run start"
    "working_dir" = "./angular"
    "description" = "Angular example"
    "depends_on" = []
    "env" = {}
    "autostart" = true
    "autorestart" = false
    "namespace" = "demo_namespace"
    "stdout" = "/tmp/angular-stdout.log"
    "stderr" = "/tmp/angular-stderr.log"
    flox = {
      "environment" = ".#angular"
    }
  }
]
