# Superviseur

A simple process supervisor for UNIX-like systems. Currently only supports running executables, but I plan to add support for managing wasm and docker containers in the future.

Project Status: 🐲 Unstable, alpha-ish quality.
## 🚚 Installation

```bash
git clone https://github.com/tsirysndr/superviseur
cd superviseur
cargo install --path .
```

## 🚀 Usage

```
USAGE:
    superviseur [SUBCOMMAND]

OPTIONS:
    -h, --help       Print help information
    -V, --version    Print version information

SUBCOMMANDS:
    config     Get the config of a process
    help       Print this message or the help of the given subcommand(s)
    init       Initialize the superviseur config
    list       List all processes
    log        Get the log of a process
    new        Create a new service config
    restart    Restart all processes or a specific one
    serve      Start the superviseur server
    start      Start all processes or a specific one
    status     Get the status of a process
    stop       Stop all processes or a specific one
    tail       Tail the log of a process
```

## 📚 Getting Started

### Start the server

Superviseur uses a server-client architecture. The server is responsible for managing the processes, while the client is responsible for communicating with the server.

```bash
superviseur serve
```

### Initialize a new project

```bash
superviseur new
```

Start by initializing a new project. This will create a `Superfile.hcl` file in the current directory with the following contents:

```hcl
project = "demo"
services = [
  {
    "name" = "demo"
    "type" = "exec"
    "command" = "echo hello $NAME"
    "working_dir" = "/tmp"
    "description" = "Demo service"
    "depends_on" = []
    "env" = {
      "NAME" = "world"
    }
    "autostart" = true
    "namespace" = "demo_namespace"
    "port" = 5060
    "stdout" = "/tmp/demo-stdout.log"
    "stderr" = "/tmp/demo-stderr.log"
  }
]
```


## 📝 License
[MPL](LICENSE)