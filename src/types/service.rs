pub struct Service {
    pub name: String,
    pub status: String,
    pub pid: u32,
    pub r#type: String,
    pub description: String,
    pub depends_on: Vec<String>,
}
