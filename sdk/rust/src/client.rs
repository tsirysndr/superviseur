use crate::project::Project;

#[derive(Default, Clone)]
pub struct Client {}

impl Client {
    pub fn new_project(&self, name: &str) -> Project {
        Project {
            client: self.clone(),
            name: name.to_string(),
            ..Default::default()
        }
    }

    pub fn project(&self, name: &str) -> Project {
        Project {
            client: self.clone(),
            name: name.to_string(),
            ..Default::default()
        }
    }

    pub fn projects(&self) -> Vec<Project> {
        vec![]
    }
}

pub fn connect() -> Client {
    Client {}
}
