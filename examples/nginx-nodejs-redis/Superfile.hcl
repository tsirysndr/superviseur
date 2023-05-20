project = "nginx-nodejs-redis"

service "nodejs" {
  type = "docker"
  command = "npm start"
  working_dir = "./web"
  description = "Ping Service Example"
  depends_on = ["redis"]
  wait_for = ["redis"]
  env = {
    REDIS_HOST = "redis"
  }
  port = 5005
  build = {
    command = "npm install"
  }

  #use "flox" {
  #  environment = ".#nginx-nodejs-redis"
  #}

  use "docker" { }
}

service "redis" {
  type = "docker"
  command = "redis-server"
  stop_command = "redis-cli shutdown"
  working_dir = "."
  description = "Redis Service Example"
  depends_on = []
  env = {}
  port = 6379
  
  #use "flox" {
  #  environment = ".#nginx-nodejs-redis"
  #}

  use "docker" {
    image = "redislabs/redismod:edge"
    ports = ["6379:6379"]
  }
}
