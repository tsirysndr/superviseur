project = "nginx-golang-mysql"
services = [
  {
    "name" = "nginx"
    "type" = "exec"
    "command" = "nginx --version"
    "working_dir" = "."
    "description" = "Nginx Service Example"
    "depends_on" = ["go"]
    "env" = { }
    "autostart" = true
    "autorestart" = false
    "namespace" = "demo_namespace"
    "stdout" = "/tmp/nginx-stdout.log"
    "stderr" = "/tmp/nginx-stderr.log"
    flox = {
      "environment" = ".#nginx-golang-mysql"
    }
  },
   {
    "name" = "go"
    "type" = "exec"
    "command" = "go version"
    "working_dir" = "."
    "description" = "Go Service Example"
    "depends_on" = ["mysql"]
    "env" = { }
    "autostart" = true
    "autorestart" = false
    "namespace" = "demo_namespace"
    "stdout" = "/tmp/go-stdout.log"
    "stderr" = "/tmp/go-stderr.log"
    flox = {
      "environment" = ".#nginx-golang-mysql"
    }
  },
   {
    "name" = "mysql"
    "type" = "exec"
    "command" = "mysqld --datadir=$MYSQL_DATADIR --pid-file=$MYSQL_PID_FILE --socket=$PWD/$MYSQL_UNIX_PORT"
    "stop_command" = "mysqladmin -u root --socket=$PWD/$MYSQL_UNIX_PORT shutdown"
    "working_dir" = "."
    "description" = "MySQL Service Example"
    "depends_on" = []
    "env" = {
      MYSQL_HOME = "./.mysql"
      MYSQL_DATADIR = "./.mysql/data"
      MYSQL_PID_FILE = "./.mysql/./mysql.pid"
      MYSQL_UNIX_PORT = "./.mysql/mysql.sock"
    }
    "autostart" = true
    "autorestart" = false
    "namespace" = "demo_namespace"
    "stdout" = "/tmp/mysql-stdout.log"
    "stderr" = "/tmp/mysql-stderr.log"
    flox = {
      "environment" = ".#nginx-golang-mysql"
    }
  }
]
