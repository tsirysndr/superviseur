use std::{collections::HashMap, fmt::Display, str::FromStr};

use anyhow::Error;
use chrono::{DateTime, Duration, Utc};
use tabled::Tabled;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum State {
    Starting,
    Running,
    Sleeping,
    Waiting,
    Zombie,
    Stopping,
    Stopped,
    TracingStop,
    Dead,
    Wakekill,
    Waking,
    Parked,
    Idle,
    Locked,
    WaitingForCpu,
    Unknown,
    Building,
}

impl Default for State {
    fn default() -> Self {
        State::Unknown
    }
}

impl FromStr for State {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Running" => Ok(State::Running),
            "Sleeping" => Ok(State::Sleeping),
            "Waiting" => Ok(State::Waiting),
            "Zombie" => Ok(State::Zombie),
            "Stopped" => Ok(State::Stopped),
            "TracingStop" => Ok(State::TracingStop),
            "Dead" => Ok(State::Dead),
            "Wakekill" => Ok(State::Wakekill),
            "Waking" => Ok(State::Waking),
            "Parked" => Ok(State::Parked),
            "Idle" => Ok(State::Idle),
            "Locked" => Ok(State::Locked),
            "WaitingForCpu" => Ok(State::WaitingForCpu),
            _ => Err(Error::msg("Unknown state")),
        }
    }
}

impl Display for State {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            State::Starting => write!(f, "Starting"),
            State::Running => write!(f, "Running"),
            State::Sleeping => write!(f, "Sleeping"),
            State::Waiting => write!(f, "Waiting"),
            State::Zombie => write!(f, "Zombie"),
            State::Stopping => write!(f, "Stopping"),
            State::Stopped => write!(f, "Stopped"),
            State::TracingStop => write!(f, "TracingStop"),
            State::Dead => write!(f, "Dead"),
            State::Wakekill => write!(f, "Wakekill"),
            State::Waking => write!(f, "Waking"),
            State::Parked => write!(f, "Parked"),
            State::Idle => write!(f, "Idle"),
            State::Locked => write!(f, "Locked"),
            State::WaitingForCpu => write!(f, "WaitingForCpu"),
            State::Unknown => write!(f, "Unknown"),
            State::Building => write!(f, "Building"),
        }
    }
}

#[derive(Default, Tabled, Clone, Debug)]
pub struct Process {
    #[tabled(rename = "NAME")]
    pub name: String,
    #[tabled(skip)]
    pub description: Option<String>,
    #[tabled(display_with = "display_option", rename = "PID")]
    pub pid: Option<u32>,
    #[tabled(skip)]
    pub uid: Option<u32>,
    #[tabled(skip)]
    pub gid: Option<u32>,
    #[tabled(skip)]
    pub state: State,
    #[tabled(skip)]
    pub cpu: Option<f32>,
    #[tabled(skip)]
    pub mem: Option<f32>,
    #[tabled(display_with = "display_up_time", rename = "STATUS")]
    pub up_time: Option<DateTime<Utc>>,
    #[tabled(rename = "COMMAND", display_with = "display_command")]
    pub command: String,
    #[tabled(skip)]
    pub working_dir: String,
    #[tabled(skip)]
    pub env: HashMap<String, String>,
    #[tabled(skip)]
    pub project: String,
    #[tabled(rename = "TYPE")]
    pub r#type: String,
    #[tabled(skip)]
    pub auto_restart: bool,
    #[tabled(skip)]
    pub stdout: String,
    #[tabled(skip)]
    pub stderr: String,
    #[tabled(rename = "SERVICE_ID")]
    pub service_id: String,
    #[tabled(rename = "PORT", display_with = "display_port")]
    pub port: Option<u32>,
}

fn display_option<T: ToString>(value: &Option<T>) -> String {
    match value {
        Some(v) => v.to_string(),
        None => "?".to_string(),
    }
}

fn display_up_time(value: &Option<DateTime<Utc>>) -> String {
    match value {
        Some(v) => format!("Up {}", format_duration(Utc::now() - *v)),
        None => "Stopped".to_string(),
    }
}

pub fn format_duration(duration: Duration) -> String {
    if duration < Duration::seconds(60) {
        return format!("{} seconds ago", duration.num_seconds());
    }
    if duration < Duration::minutes(60) {
        let minutes = duration.num_minutes();
        return format!(
            "{} {} ago",
            minutes,
            if minutes == 1 { "minute" } else { "minutes" }
        );
    }
    if duration < Duration::hours(24) {
        let hours = duration.num_hours();
        return format!(
            "{} {} ago",
            hours,
            if hours == 1 { "hour" } else { "hours" }
        );
    }
    let days = duration.num_days();
    format!("{} {} ago", days, if days == 1 { "day" } else { "days" })
}

fn display_port(port: &Option<u32>) -> String {
    match port {
        Some(0) => "-".to_string(),
        Some(port) => port.clone().to_string(),
        None => "-".to_string(),
    }
}

fn display_command(command: &str) -> String {
    if command.len() > 20 {
        format!("\"{}...\"", &command[..20])
    } else {
        format!("\"{}\"", command.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_display_command() {
        assert_eq!(
            display_command("echo \"Hello World\""),
            "\"echo \"Hello World\"\""
        );
        assert_eq!(
            display_command("echo \"Hello World\" && sleep 1"),
            "\"echo \"Hello World\" &...\""
        );
        assert_eq!(display_port(&Some(0)), "-".to_string());
        assert_eq!(display_port(&Some(8080)), "8080".to_string());
    }

    #[test]
    fn test_display_option() {
        assert_eq!(display_option(&Some(0)), "0".to_string());
        assert_eq!(display_option(&Some("test")), "test".to_string());
        assert_eq!(display_option(&None::<String>), "?".to_string());
    }

    #[test]
    fn test_display_up_time() {
        assert_eq!(
            display_up_time(&Some(Utc::now() - Duration::seconds(10))),
            "Up 10 seconds ago".to_string()
        );
        assert_eq!(
            display_up_time(&Some(Utc::now() - Duration::minutes(1))),
            "Up 1 minute ago".to_string()
        );
        assert_eq!(
            display_up_time(&Some(Utc::now() - Duration::minutes(2))),
            "Up 2 minutes ago".to_string()
        );
        assert_eq!(
            display_up_time(&Some(Utc::now() - Duration::hours(1))),
            "Up 1 hour ago".to_string()
        );
        assert_eq!(
            display_up_time(&Some(Utc::now() - Duration::hours(2))),
            "Up 2 hours ago".to_string()
        );
        assert_eq!(
            display_up_time(&Some(Utc::now() - Duration::days(1))),
            "Up 1 day ago".to_string()
        );
        assert_eq!(
            display_up_time(&Some(Utc::now() - Duration::days(2))),
            "Up 2 days ago".to_string()
        );
        assert_eq!(
            display_up_time(&None::<DateTime<Utc>>),
            "Stopped".to_string()
        );
    }
}
