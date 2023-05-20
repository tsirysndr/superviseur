project = "react-nginx"

service "react" {
  type = "exec"
  command = "npm start"
  working_dir = "."
  description = "React app"
  depends_on = []
  env = {}
  flox = { 
    environment = ".#react-nginx"
  }

  use "flox" {
    environment = ".#react-nginx"
  }
}
