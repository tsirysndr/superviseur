use anyhow::Error;
use clap::{arg, Command};
use superviseur::{
    cmd::{
        config::execute_config, init::execute_init, list::execute_list, log::execute_log,
        new::execute_new, restart::execute_restart, start::execute_start, status::execute_status,
        stop::execute_stop, tail::execute_tail,
    },
    server,
    types::configuration::ConfigFormat,
};

fn cli() -> Command<'static> {
    const VESRION: &str = env!("CARGO_PKG_VERSION");
    Command::new("superviseur")
        .version(VESRION)
        .author("Tsiry Sandratraina <tsiry.sndr@aol.com>")
        .about(
            r#"
         _____                             _                     
        / ___/__  ______  ___  ______   __(_)_______  __  _______
        \__ \/ / / / __ \/ _ \/ ___/ | / / / ___/ _ \/ / / / ___/
       ___/ / /_/ / /_/ /  __/ /   | |/ / (__  )  __/ /_/ / /    
      /____/\__,_/ .___/\___/_/    |___/_/____/\___/\__,_/_/     
                /_/                                              
      
A simple process supervisor"#,
        )
        .subcommand(
            Command::new("start")
                .arg(arg!([name] "The name of the process to start"))
                .about("Start all processes or a specific one"),
        )
        .subcommand(
            Command::new("stop")
                .arg(arg!([name] "The name of the process to stop"))
                .about("Stop all processes or a specific one"),
        )
        .subcommand(
            Command::new("restart")
                .arg(arg!([name] "The name of the process to restart"))
                .about("Restart all processes or a specific one"),
        )
        .subcommand(
            Command::new("status")
                .arg(arg!(<name> "The name of the process to get the status of"))
                .about("Get the status of a process"),
        )
        .subcommand(Command::new("list").about("List all processes"))
        .subcommand(
            Command::new("log")
                .arg(arg!(<name> "The name of the process to get the log of"))
                .about("Get the log of a process"),
        )
        .subcommand(
            Command::new("tail")
                .arg(arg!(<name> "The name of the process to tail the log of"))
                .about("Tail the log of a process"),
        )
        .subcommand(
            Command::new("config")
                .arg(arg!(<name> "The name of the process to get the config of"))
                .about("Get the config of a process"),
        )
        .subcommand(
            Command::new("init")
                .arg(arg!(--toml "Initialize the superviseur config in toml format"))
                .about("Initialize the superviseur config"),
        )
        .subcommand(
            Command::new("new")
                .arg(arg!(--toml "Create a new service config in toml format"))
                .about("Create a new service config"),
        )
        .subcommand(
            Command::new("serve")
                .arg(arg!([port] "The port to listen on").default_value("5476"))
                .about("Start the superviseur server"),
        )
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let matches = cli().get_matches();

    match matches.subcommand() {
        Some(("start", args)) => {
            let name = args.value_of("name");
            execute_start(name).await?;
        }
        Some(("stop", args)) => {
            let name = args.value_of("name");
            execute_stop(name).await?;
        }
        Some(("restart", args)) => {
            let name = args.value_of("name");
            execute_restart(name).await?;
        }
        Some(("status", args)) => {
            let name = args.value_of("name");
            execute_status(name.unwrap()).await?;
        }
        Some(("list", _)) => execute_list().await?,
        Some(("log", args)) => {
            let name = args.value_of("name");
            execute_log(name.unwrap()).await?;
        }
        Some(("tail", args)) => {
            let name = args.value_of("name");
            execute_tail(name.unwrap()).await?;
        }
        Some(("config", args)) => {
            let name = args.value_of("name");
            execute_config(name.unwrap());
        }
        Some(("init", args)) => match args.is_present("toml") {
            true => execute_init(ConfigFormat::TOML),
            false => execute_init(ConfigFormat::HCL),
        },
        Some(("new", args)) => match args.is_present("toml") {
            true => execute_new(ConfigFormat::TOML),
            false => execute_new(ConfigFormat::HCL),
        },
        Some(("serve", args)) => {
            let port = args.value_of("port").unwrap();
            let port = port.parse::<u16>().unwrap();
            server::exec(port).await?;
        }
        _ => println!("No subcommand was used"),
    }
    Ok(())
}
