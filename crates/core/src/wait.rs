use std::{net::TcpStream, thread::sleep, time::Duration};

use anyhow::Error;

use superviseur_types::configuration::Service;

pub fn wait_for_service(svc: &Service, max: u32) -> Result<(), Error> {
    let host = "127.0.0.1";
    match svc.port {
        Some(port) => {
            println!("Waiting for service {} on port {}", svc.name, port);
            let stream = TcpStream::connect(format!("{}:{}", host, port));
            if stream.is_err() {
                if max > 0 {
                    sleep(Duration::from_secs(1));
                    return wait_for_service(svc, max - 1);
                } else {
                    return Err(Error::msg(format!(
                        "Cannot connect to the service {} on port {}",
                        svc.name, port
                    )));
                }
            } else {
                println!("Service {} is up and running", svc.name);
            }
        }
        None => {}
    };
    Ok(())
}
