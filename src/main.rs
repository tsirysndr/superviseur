use anyhow::Error;
use clap::{arg, Command};
use superviseur::{
    cmd::{
        config::execute_config, init::execute_init, list::execute_list, log::execute_log,
        new::execute_new, restart::execute_restart, start::execute_start, status::execute_status,
        stop::execute_stop, tail::execute_tail,
    },
    server,
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
        .subcommand(Command::new("start").about("Start a process"))
        .subcommand(Command::new("stop").about("Stop a process"))
        .subcommand(Command::new("restart").about("Restart a process"))
        .subcommand(Command::new("status").about("Get the status of a process"))
        .subcommand(Command::new("list").about("List all processes"))
        .subcommand(Command::new("log").about("Get the log of a process"))
        .subcommand(Command::new("tail").about("Tail the log of a process"))
        .subcommand(Command::new("config").about("Get the config of a process"))
        .subcommand(Command::new("init").about("Initialize the superviseur config"))
        .subcommand(Command::new("new").about("Create a new service config"))
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
        Some(("start", _)) => execute_start(),
        Some(("stop", _)) => execute_stop(),
        Some(("restart", _)) => execute_restart(),
        Some(("status", _)) => execute_status(),
        Some(("list", _)) => execute_list(),
        Some(("log", _)) => execute_log(),
        Some(("tail", _)) => execute_tail(),
        Some(("config", _)) => execute_config(),
        Some(("init", _)) => execute_init(),
        Some(("new", _)) => execute_new(),
        Some(("serve", args)) => {
            let port = args.value_of("port").unwrap();
            let port = port.parse::<u16>().unwrap();
            server::exec(port).await?;
        }
        _ => println!("No subcommand was used"),
    }
    Ok(())
}
