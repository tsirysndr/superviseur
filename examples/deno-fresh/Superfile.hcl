project = "deno-fresh"
services = [
  {
    "name" = "deno"
    "type" = "exec"
    "command" = "deno task start"
    "working_dir" = "."
    "description" = "Ping Service Example"
    "depends_on" = []
    "env" = {}
    "autostart" = true
    "autorestart" = false
    "namespace" = "deno_namespace"
    "stdout" = "/tmp/deno-stdout.log"
    "stderr" = "/tmp/deno-stderr.log"
    "port" = 8000
  }
]
