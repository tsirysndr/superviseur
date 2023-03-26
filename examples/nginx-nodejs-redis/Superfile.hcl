project = "nginx-nodejs-redis"
services = [
  {
    "name" = "nodejs"
    "type" = "exec"
    "command" = "npm start"
    "working_dir" = "/tmp"
    "description" = "Ping Service Example"
    "depends_on" = []
    "env" = {}
    "autostart" = true
    "autorestart" = false
    "namespace" = "demo_namespace"
    "stdout" = "/tmp/nodejs-stdout.log"
    "stderr" = "/tmp/nodejs-stderr.log"
    flox = {
      "environment" = ".#nginx-nodejs-redis"
    }
  },
  {
    "name" = "redis"
    "type" = "exec"
    "command" = "redis-server"
    "description" = "Redis Service Example"
    "depends_on" = []
    "autostart" = true
    "autorestart" = false
    "namespace" = "demo_namespace"
    "stdout" = "/tmp/redis-stdout.log"
    "stderr" = "/tmp/redis-stderr.log"
    flox = {
      "environment" = ".#nginx-nodejs-redis"
    }
  },
  {
    "name" = "nginx"
    "type" = "exec"
    "command" = "nginx -g 'daemon off;'"
    "description" = "Nginx Service Example"
    "depends_on" = ["nodejs", "redis"]
    "autostart" = true
    "autorestart" = false
    "namespace" = "demo_namespace"
    "stdout" = "/tmp/nginx-stdout.log"
    "stderr" = "/tmp/nginx-stderr.log"
    flox = {
      "environment" = ".#nginx-nodejs-redis"
    }
  }
]
