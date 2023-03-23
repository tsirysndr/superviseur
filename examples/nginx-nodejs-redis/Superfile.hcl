project = "nginx-nodejs-redis"
services = [
  {
    "name" = "nodejs"
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
    "stdout" = "/tmp/nodejs-stdout.log"
    "stderr" = "/tmp/nodejs-stderr.log"
    flox = {
      "environment" = ".#nginx-nodejs-redis"
    }
  }
]
