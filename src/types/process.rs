use std::collections::HashMap;

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

impl ToString for State {
    fn to_string(&self) -> String {
        match self {
            State::Running => "Running".to_string(),
            State::Sleeping => "Sleeping".to_string(),
            State::Waiting => "Waiting".to_string(),
            State::Zombie => "Zombie".to_string(),
            State::Stopped => "Stopped".to_string(),
            State::TracingStop => "TracingStop".to_string(),
            State::Dead => "Dead".to_string(),
            State::Wakekill => "Wakekill".to_string(),
            State::Waking => "Waking".to_string(),
            State::Parked => "Parked".to_string(),
            State::Idle => "Idle".to_string(),
            State::Locked => "Locked".to_string(),
            State::WaitingForCpu => "WaitingForCpu".to_string(),
            State::Unknown => "Unknown".to_string(),
        }
    }
}

pub struct Process {
    pub name: String,
    pub pid: Option<u32>,
    pub uid: Option<u32>,
    pub gid: Option<u32>,
    pub state: State,
    pub cpu: Option<f32>,
    pub mem: Option<f32>,
    pub time: Option<String>,
    pub command: String,
    pub working_dir: String,
    pub port: Option<u16>,
    pub env: HashMap<String, String>,
}
