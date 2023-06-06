pub mod build;
pub mod code;
pub mod config;
pub mod init;
pub mod list;
pub mod log;
pub mod new;
pub mod preview;
pub mod project;
pub mod ps;
pub mod restart;
pub mod start;
pub mod status;
pub mod stop;
pub mod tail;
pub mod ui;

macro_rules! event_handler {
    ($client:expr, $service:expr, $config_file_path:expr) => {
        tokio::spawn(async move {
            let request = tonic::Request::new(EventsRequest {
                service: $service,
                config_file_path: $config_file_path,
            });

            let response = $client.events(request).await?;
            let mut stream = response.into_inner();

            while let Some(message) = stream.message().await? {
                match message.event.as_str() {
                    SERVICE_STARTING => {
                        println!("-> Starting {} ...", message.service.bright_green());
                    }
                    SERVICE_BUILDING => {
                        println!("-> Building {} ...", message.service.bright_green());
                    }
                    SERVICE_CRASHED => {
                        println!("{} has crashed", message.service.bright_green());
                    }
                    SERVICE_ERROR => {
                        println!(
                            "{} has encountered an error",
                            message.service.bright_green()
                        );
                        println!("{}", message.output);
                    }
                    SERVICE_RESTARTING => {
                        println!("-> Restarting {} ...", message.service.bright_green());
                    }
                    SERVICE_STOPPING => {
                        println!("-> Stopping {} ...", message.service.bright_green());
                    }
                    SERVICE_LOGS => {
                        let prefix = format!("{} | ", message.service.cyan());
                        print!("{}{}", prefix, message.output);
                    }
                    SERVICE_SETUP_ENV => {
                        println!(
                            "-> Setting up environment for {} ...",
                            message.service.bright_green()
                        );
                        let prefix = format!("{} | ", message.service.cyan());
                        println!("{} {}", prefix, message.output);
                    }
                    ALL_SERVICES_BUILT => {
                        println!("-> All services have been built");
                        break;
                    }
                    ALL_SERVICES_RESTARTED => {
                        println!("-> All services have been restarted");
                        break;
                    }
                    ALL_SERVICES_STOPPED => {
                        println!("-> All services have been stopped");
                        break;
                    }
                    _ => {}
                }
            }
            Ok::<(), Error>(())
        })
    };
}

pub(crate) use event_handler;
