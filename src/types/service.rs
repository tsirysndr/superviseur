use tabled::Tabled;

#[derive(Default, Tabled)]
pub struct Service {
    #[tabled(rename = "ID")]
    pub id: String,
    #[tabled(rename = "NAME")]
    pub name: String,
    #[tabled(rename = "STATUS")]
    pub status: String,
    #[tabled(rename = "COMMAND", display_with = "display_command")]
    pub command: String,
    #[tabled(rename = "TYPE")]
    pub r#type: String,
    #[tabled(skip)]
    pub description: String,
    #[tabled(skip)]
    pub depends_on: Vec<String>,
    #[tabled(rename = "PORT", display_with = "display_port")]
    pub port: Option<u32>,
}

fn display_command(command: &str) -> String {
    if command.len() > 20 {
        format!("\"{}...\"", &command[..20])
    } else {
        format!("\"{}\"", command.to_string())
    }
}

fn display_port(port: &Option<u32>) -> String {
    match port {
        Some(port) => port.clone().to_string(),
        None => "-".to_string(),
    }
}
