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
    #[tabled(rename = "COMMAND")]
    pub command: String,
    #[tabled(skip)]
    pub working_dir: String,
    #[tabled(skip)]
    pub port: Option<u16>,
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
