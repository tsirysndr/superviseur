use anyhow::Error;
use std::{io::BufRead, process::Command, thread};
use tokio::sync::mpsc::UnboundedSender;

pub fn start_code_tunnel(sender: UnboundedSender<String>, dir: &str) -> Result<u32, Error> {
    verify_if_code_is_installed()?;

    let mut child = Command::new("code")
        .arg("tunnel")
        .arg("--accept-server-license-terms")
        .current_dir(dir)
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::piped())
        .spawn()
        .expect("failed to execute process");

    let pid = child.id();

    thread::spawn(move || {
        let stdout = child.stdout.take().expect("Failed to get stdout handle");
        let stderr = child.stderr.take().expect("Failed to get stderr handle");
        let stdout_reader = std::io::BufReader::new(stdout);
        let stderr_reader = std::io::BufReader::new(stderr);

        for line in stdout_reader.lines() {
            if let Ok(line) = line {
                sender.send(line).unwrap();
            }
        }

        for line in stderr_reader.lines() {
            if let Ok(line) = line {
                sender.send(line).unwrap();
            }
        }

        drop(sender);

        child.wait().expect("Failed to wait on child");
    });

    Ok(pid)
}

fn verify_if_code_is_installed() -> Result<(), Error> {
    let output = Command::new("code")
        .arg("--version")
        .output()
        .expect("failed to execute process");

    if !output.status.success() {
        return Err(Error::msg("Code is not installed"));
    }

    Ok(())
}
