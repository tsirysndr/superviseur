use anyhow::Error;
use clap::{arg, Command, SubCommand};
use superviseur::{
    cmd::{
        build::execute_build,
        config::execute_config,
        init::execute_init,
        list::execute_list,
        log::{execute_log, execute_search_log},
        new::execute_new,
        preview::execute_preview,
        project::{execute_get_project, execute_list_projects},
        ps::execute_ps,
        restart::execute_restart,
        start::execute_start,
        status::execute_status,
        stop::execute_stop,
        tail::execute_tail,
        ui::execute_ui,
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
                .arg(arg!([name] "The name of the service to start"))
                .about("Start all services or a specific one"),
        )
        .subcommand(
            Command::new("stop")
                .arg(arg!([name] "The name of the service to stop"))
                .about("Stop all services or a specific one"),
        )
        .subcommand(
            Command::new("restart")
                .arg(arg!([name] "The name of the service to restart"))
                .about("Restart all services or a specific one"),
        )
        .subcommand(
            Command::new("status")
                .arg(arg!(<name> "The name of the service to get the status of"))
                .about("Get the status of a service"),
        )
        .subcommand(
            Command::new("list")
                .visible_alias("ls")
                .about("List all services"),
        )
        .subcommand(Command::new("ps").about("List all running processes"))
        .subcommand(
            Command::new("log")
                .arg(arg!(<name> "The name of the process to get the log of"))
                .arg(arg!(--follow -f "Follow the log"))
                .about("Get the log of a process"),
        )
        .subcommand(
            Command::new("tail")
                .arg(arg!(<name> "The name of the process to tail the log of"))
                .arg(arg!(--follow -f "Follow the log"))
                .arg(arg!(--lines -n [lines] "The number of lines to tail"))
                .about("Tail the log of a process"),
        )
        .subcommand(
            Command::new("config")
                .arg(arg!(<name> "The name of the service to get the config of"))
                .about("Get the config of a service"),
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
        .subcommand(Command::new("daemon").about("Start the superviseur daemon"))
        .subcommand(Command::new("up")
        .arg(arg!(--build "Build all services before starting"))
        .about("Start all services"))
        .subcommand(Command::new("down").about("Stop all services"))
        .subcommand(Command::new("ui").about("Start the superviseur dashboard"))
        .subcommand(Command::new("build")
            .arg(arg!([name] "The name of the service to build, if not specified, all services will be built"))
        .about("Build all services or a specific one"))
        .subcommand(
            Command::new("project")
                .subcommand(
                    Command::new("details")
                        .arg(arg!(<id> "The id of the project to get the details of"))
                        .about("Get the details of a project"),
                )
                .subcommand(
                    Command::new("list")
                        .alias("ls")
                        .about("List all projects"),
                )
            .about("Manage projects")
        )
        .subcommand(
            Command::new("open")
                .arg(arg!(<name> "The name of the service to preview"))
                .about("Open URL of a service in the browser"),   
        )
        .subcommand(
            Command::new("search-log")
                .arg(arg!(<name> "The name of the service to search the log of"))
                .arg(arg!(<query> "The query to search"))
                .about("Search the log of a service"),
        )
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let matches = cli().get_matches();

    match matches.subcommand() {
        Some(("start", args)) => {
            let name = args.value_of("name");
            let build = args.is_present("build");
            execute_start(name, build).await?;
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
        Some(("ps", _)) => execute_ps().await?,
        Some(("log", args)) => {
            let name = args.value_of("name");
            let follow = args.is_present("follow");
            execute_log(name.unwrap(), follow).await?;
        }
        Some(("tail", args)) => {
            let name = args.value_of("name");
            let follow = args.is_present("follow");
            let lines = args.value_of("lines");
            let lines = lines.map(|l| l.parse::<usize>().unwrap()).unwrap_or(10);
            execute_tail(name.unwrap(), follow, lines).await?;
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
            server::exec(port, true).await?;
        }
        Some(("daemon", _)) => server::exec(5476, false).await?,
        Some(("up", args)) => {
            let build = args.is_present("build");
            execute_start(None, build).await?
        }
        Some(("down", _)) => execute_stop(None).await?,
        Some(("ui", _)) => execute_ui().await?,
        Some(("build", args)) => {
            let name = args.value_of("name");
            execute_build(name).await?;
        }
        Some(("project", args)) => match args.subcommand() {
            Some(("details", args)) => {
                let id = args.value_of("id").unwrap();
                execute_get_project(id).await?;
            }
            Some(("list", _)) => execute_list_projects().await?,
            _ => SubCommand::with_name("project").print_help()?,
        },
        Some(("preview", args)) => {
            let name = args.value_of("name");
            execute_preview(name.unwrap()).await?;
        }
        Some(("search-log", args)) => {
            let name = args.value_of("name");
            let query = args.value_of("query");
            execute_search_log(name.unwrap(), query.unwrap()).await?;
        }
        _ => cli().print_help()?,
    }
    Ok(())
}
