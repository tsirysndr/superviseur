use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Template {
    pub name: String,
    pub description: String,
    pub repository_url: String,
    pub version: String,
}
