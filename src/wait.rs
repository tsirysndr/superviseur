use std::{io::Read, net::TcpStream, time::Duration};

use anyhow::Error;

use crate::types::configuration::Service;

pub fn wait_for_service(svc: &Service) -> Result<(), Error> {
    let host = "127.0.0.1";
    match svc.port {
        Some(port) => {
            const TIMEOUT: u64 = 5;
            // check tcp connection
            let mut stream = TcpStream::connect((host, port))?;
            stream.set_read_timeout(Some(Duration::from_secs(TIMEOUT)))?;
            let mut buffer = [0; 1024];
            let _ = stream.read(&mut buffer)?;
        }
        None => {}
    };
    Ok(())
}
