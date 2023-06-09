# Superviseur

<p>
  <a href="LICENSE" target="./LICENSE">
    <img alt="License: MPL-2.0" src="https://img.shields.io/badge/License-MPL-blue.svg" />
  </a>
  <a href="https://crates.io/crates/superviseur" target="_blank">
    <img src="https://img.shields.io/crates/v/superviseur.svg" />
  </a>
  
  <a href="https://crates.io/crates/superviseur" target="_blank">
    <img src="https://img.shields.io/crates/dr/superviseur" />
  </a>

  <a href="https://feat-webui--640724a8e12e5a011d6d59fb.chromatic.com" target="_blank">
  <img src="https://img.shields.io/badge/Storybook-FF4785?logo=storybook&logoColor=fff" />
  </a>
  <a href="https://github.com/tsirysndr/superviseur/actions/workflows/release.yml" target="_blank">
    <img alt="release" src="https://github.com/tsirysndr/superviseur/actions/workflows/release.yml/badge.svg" />
  </a>
  <a href="https://discord.gg/FeGJerUC" target="_blank">
    <img alt="discord-server" src="https://img.shields.io/discord/1103720908104929321?label=discord&logo=discord&color=5865F2">
  </a>
</p>

<img src="./astronauts.png" width="100%" style="margin-top: 20px; margin-bottom: 20px;" />

Superviseur is a tool for running multi-service applications on isolated environments (Nix or Docker) using HCL/TOML or any language with an [SDK](/sdk). It is designed to be used both from a CLI and a web interface.
Once you have a [Superfile.hcl](#initialize-a-new-project), you can create and start your application with a single command: `superviseur up`.


Project Status: 🐲 Unstable, alpha-ish quality.

<img src="./preview.png" width="100%" style="margin-top: 20px;margin-bottom: 20px;" />

## 🚚 Installation

```bash
# Install dependencies
brew install protobuf # macOS
sudo apt-get install -y protobuf-compiler # Ubuntu/Debian
curl -fsSL https://bun.sh/install | bash
# Compile
git clone https://github.com/tsirysndr/superviseur
cd superviseur/crates/webui/webui && bun install && bun run build && cd ../../../
cargo install --path crates/cli
```

### macOS/Linux

```bash
brew install tsirysndr/tap/superviseur
```
Or download the latest release for your platform [here](https://github.com/tsirysndr/superviseur/releases).

## 📦 Downloads
- `Mac`: arm64: [superviseur_v0.1.0-alpha.13_aarch64-apple-darwin.tar.gz](https://github.com/tsirysndr/superviseur/releases/download/v0.1.0-alpha.13/superviseur_v0.1.0-alpha.13_aarch64-apple-darwin.tar.gz) intel: [superviseur_v0.1.0-alpha.13_x86_64-apple-darwin.tar.gz](https://github.com/tsirysndr/superviseur/releases/download/v0.1.0-alpha.13/superviseur_v0.1.0-alpha.13_x86_64-apple-darwin.tar.gz)
- `Linux`: [superviseur_v0.1.0-alpha.13_x86_64-unknown-linux-gnu.tar.gz](https://github.com/tsirysndr/superviseur/releases/download/v0.1.0-alpha.13/superviseur_v0.1.0-alpha.13_x86_64-unknown-linux-gnu.tar.gz)

## 🚀 Usage

```
USAGE:
    superviseur [SUBCOMMAND]

OPTIONS:
    -h, --help       Print help information
    -V, --version    Print version information

SUBCOMMANDS:
    build         Build all services or a specific one
    config        Get the config of a service
    daemon        Start the superviseur daemon
    down          Stop all services
    help          Print this message or the help of the given subcommand(s)
    init          Create a new superviseur config (Superviseur.hcl)
    list          List all services [aliases: ls]
    log           Get the log of a process
    new           Create a new superviseur config (Superviseur.hcl)
    open          Open URL of a service in the browser
    project       Manage projects
    ps            List all running processes
    restart       Restart all services or a specific one
    search-log    Search the log of a service
    serve         Start the superviseur server
    start         Start all services or a specific one
    status        Get the status of a service
    stop          Stop all services or a specific one
    tail          Tail the log of a process
    ui            Start the superviseur dashboard
    up            Start all services
```

## 📚 Getting Started

### Start the server

Superviseur uses a server-client architecture. The server is responsible for managing the processes, while the client is responsible for communicating with the server.

```bash
superviseur daemon
```

### Initialize a new project

```bash
superviseur new
```

Start by initializing a new project. This will create a `Superfile.hcl` file in the current directory with the following contents:

```hcl
project = "demo"

service "demo" {
  type = "exec"
  command = "ping $GITHUB_DOMAIN"
  working_dir = "/tmp"
  description = "Ping Service Example"
  depends_on = []
  env = {
    "GITHUB_DOMAIN" = "github.com"
  }
  stdout = "/tmp/demo-stdout.log"
  stderr = "/tmp/demo-stderr.log"
}
```

### Start the service

```bash
superviseur start demo
```

Start the service by running the `start` command.

### Check the status

```bash
superviseur status demo
```
Output:
```
● demo - Ping Service Example
        Active: Running since 2023-03-05 19:17:56.512455 UTC; 17 seconds ago
           PID: 30887
       Command: ping $GITHUB_DOMAIN
     Directory: /tmp
           Log: /tmp/demo-stdout.log
        Stderr: /tmp/demo-stderr.log
   AutoRestart: false
          Type: exec
          Envs: GITHUB_DOMAIN=github.com
```

### Show running processes

```bash
superviseur ps
```

Output:
```
 NAME   PID     STATUS             COMMAND               TYPE 
 demo   31200   Up 4 seconds ago   ping $GITHUB_DOMAIN   exec 
```

### Stop the service

```bash
superviseur stop demo
```

### Show all services

```bash
superviseur ls
```

Output:
```
 NAME   STATUS    COMMAND                 TYPE 
 demo   STOPPED   "ping $GITHUB_DOMAIN"   exec 
```
## As a Github Action

You can use the [setup-superviseur](https://github.com/marketplace/actions/setup-superviseur) Github Action like so:

```yaml
- uses: tsirysndr/setup-superviseur@v1
  with:
    version: 'v0.1.0-alpha.13'
- run: superviseur --help
```

## 📖 Examples
See the [examples](examples) directory for more examples.
## 📝 License
[MPL](LICENSE)
