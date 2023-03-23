project = "react-nginx"
services = [
  {
    "name" = "demo"
    "type" = "exec"
    "command" = "ping $GITHUB_DOMAIN"
    "working_dir" = "/tmp"
    "description" = "Ping Service Example"
    "depends_on" = []
    "env" = {
      "GITHUB_DOMAIN" = "github.com"
    }
    "autostart" = true
    "autorestart" = false
    "namespace" = "demo_namespace"
    "stdout" = "/tmp/demo-stdout.log"
    "stderr" = "/tmp/demo-stderr.log"
    flox = { 
      "environment" = ".#react-nginx"
    }
  }
]
