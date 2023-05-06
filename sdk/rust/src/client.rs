use std::time::Duration;

use crate::graphql::query::project_query::ProjectQueryProject;
use graphql_client::{GraphQLQuery, Response};
use serde::{de::DeserializeOwned, Serialize};
use surf::{Config, Error, Url};

const BASE_URL: &str = "http://localhost:5478";

use crate::{
    graphql::query::{self, project_query, projects_query},
    project::Project,
};

#[derive(Debug, Serialize)]
pub struct RawQuery {
    pub query: String,
}

#[derive(Default, Clone)]
pub struct Client {
    pub http_client: surf::Client,
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
        let body = query::ProjectQuery::build_query(variables);
        let response_body = self
            .execute_query::<Response<project_query::ResponseData>>(&body)
            .await?;
        let p: ProjectQueryProject = response_body.data.expect("missing response data").project;
        Ok(Project {
            client: self.clone(),
            ..Project::from(p)
        })
    }

    pub async fn projects(&self) -> Result<Vec<Project>, Error> {
        let variables = projects_query::Variables {};
        let body = query::ProjectsQuery::build_query(variables);
        let response_body = self
            .execute_query::<Response<projects_query::ResponseData>>(&body)
            .await?;
        let projects = response_body.data.expect("missing response data").projects;

        Ok(projects
            .into_iter()
            .map(|p| Project {
                client: self.clone(),
                ..Project::from(p)
            })
            .collect())
    }

    pub async fn execute_query<T: DeserializeOwned>(
        &self,
        body: &impl Serialize,
    ) -> Result<T, Error> {
        let mut response = self.http_client.post("/graphql").body_json(&body)?.await?;
        let response_body = response.body_json::<T>().await?;
        Ok(response_body)
    }

    pub async fn send_query(&self, body: &str) -> Result<String, Error> {
        let mut response = self
            .http_client
            .post("/graphql")
            .body_json(&RawQuery {
                query: body.to_string(),
            })?
            .await?;
        Ok(response.body_string().await?)
    }
}

pub fn connect() -> Client {
    let http_client = Config::new()
        .set_base_url(Url::parse(BASE_URL).unwrap())
        .set_timeout(Some(Duration::from_secs(5)))
        .try_into()
        .unwrap();
    Client { http_client }
}
