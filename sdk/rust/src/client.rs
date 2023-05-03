use graphql_client::GraphQLQuery;
use surf::Error;

use crate::{
    graphql::query::{self, project as project_query},
    project::Project,
};

#[derive(Default, Clone)]
pub struct Client {
    http_client: surf::Client,
}

impl Client {
    pub fn new_project(&self, name: &str) -> Project {
        Project {
            client: self.clone(),
            name: name.to_string(),
            ..Default::default()
        }
    }

    pub async fn project(&self, name: &str) -> Result<Project, Error> {
        let variables = project_query::Variables {
            id: name.to_string(),
        };
        let body = query::Project::build_query(variables);
        self.http_client.post("/graphql").body_json(&body)?.await?;

        Ok(Project {
            client: self.clone(),
            name: name.to_string(),
            ..Default::default()
        })
    }

    pub fn projects(&self) -> Vec<Project> {
        vec![]
    }
}

pub fn connect() -> Client {
    let http_client = surf::Client::new();
    Client { http_client }
}
