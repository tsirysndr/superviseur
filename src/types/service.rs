use tabled::Tabled;

#[derive(Default, Tabled)]
pub struct Service {
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
}

fn display_command(command: &str) -> String {
    if command.len() > 20 {
        format!("\"{}...\"", &command[..20])
    } else {
        format!("\"{}\"", command.to_string())
    }
}
