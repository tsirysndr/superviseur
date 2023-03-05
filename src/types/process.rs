use std::{collections::HashMap, fmt::Display};

use tabled::Tabled;

pub enum State {
    Running,
    Sleeping,
    Waiting,
    Zombie,
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
}

impl Default for State {
    fn default() -> Self {
        State::Unknown
    }
}

impl Display for State {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            State::Running => write!(f, "Running"),
            State::Sleeping => write!(f, "Sleeping"),
            State::Waiting => write!(f, "Waiting"),
            State::Zombie => write!(f, "Zombie"),
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
        }
    }
}

#[derive(Default, Tabled)]
pub struct Process {
    #[tabled(rename = "NAME")]
    pub name: String,
    #[tabled(display_with = "display_option", rename = "PID")]
    pub pid: Option<u32>,
    #[tabled(display_with = "display_option", rename = "UID")]
    pub uid: Option<u32>,
    #[tabled(display_with = "display_option", rename = "GID")]
    pub gid: Option<u32>,
    #[tabled(rename = "STATE")]
    pub state: State,
    #[tabled(display_with = "display_option", rename = "CPU")]
    pub cpu: Option<f32>,
    #[tabled(display_with = "display_option", rename = "MEM")]
    pub mem: Option<f32>,
    #[tabled(display_with = "display_option", rename = "TIME")]
    pub time: Option<String>,
    #[tabled(rename = "COMMAND")]
    pub command: String,
    #[tabled(skip)]
    pub working_dir: String,
    #[tabled(display_with = "display_option", rename = "PORT")]
    pub port: Option<u16>,
    #[tabled(skip)]
    pub env: HashMap<String, String>,
}

fn display_option<T: ToString>(value: &Option<T>) -> String {
    match value {
        Some(v) => v.to_string(),
        None => "-".to_string(),
    }
}
