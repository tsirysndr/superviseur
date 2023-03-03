pub struct Process {
    pub name: String,
    pub pid: u32,
    pub uid: u32,
    pub gid: u32,
    pub state: String,
    pub cpu: f32,
    pub mem: f32,
    pub time: String,
    pub command: String,
    pub working_dir: String,
}
