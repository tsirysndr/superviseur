project = "angular"

service "angular" {
  type = "exec"
  command = "npm start"
  working_dir = "./angular"
  description = "Angular example"
  depends_on = []
  env = {}

  port = 4200
  
  build = {
    command = "npm install"
  }

  use "flox" {
    environment = ".#angular"
  }
}
