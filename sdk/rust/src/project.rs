use crate::graphql::query::project_query::ProjectQueryProject;
use crate::graphql::query::projects_query::ProjectsQueryProjects;
use crate::graphql::query::{
    self, create_env_var, delete_env_var, logs_query, processes_query, restart_all_services,
    restart_service, services_query, start_all_services, start_service, status_query,
    stop_all_services, stop_service, update_env_var,
};
use crate::types::{self, Logs, Process};
use crate::{client::Client, query::build_nested_with_service_query, service::Service};
use graphql_client::{GraphQLQuery, Response};
use surf::Error;

#[derive(Default)]
pub struct Project {
    pub client: Client,
    pub id: String,
    pub name: String,
    pub context: String,
    pub services: Vec<Service>,
}

impl Project {
    pub fn new(client: Client) -> Project {
        Project {
            client,
            ..Default::default()
        }
    }

    pub fn with_context(mut self, context: &str) -> Self {
        self.context = context.to_string();
        self
    }

    pub fn with_name(mut self, name: &str) -> Self {
        self.name = name.to_string();
        self
    }

    pub fn with_service(mut self, service: Service) -> Self {
        self.services.push(service);
        self
    }

    pub async fn start(self, service_id: &str) -> Result<Process, Error> {
        let variables = start_service::Variables {
            id: Some(service_id.to_string()),
            project_id: self.id.clone(),
        };
        let body = query::StartService::build_query(variables);
        let response = self
            .client
            .execute_query::<Response<start_service::ResponseData>>(&body)
            .await?;
        let process = Process::from(response.data.expect("missing response data").start);
        Ok(process)
    }

    pub async fn stop(self, service_id: &str) -> Result<Process, Error> {
        let variables = stop_service::Variables {
            id: Some(service_id.to_string()),
            project_id: self.id.clone(),
        };
        let body = query::StopService::build_query(variables);
        let response = self
            .client
            .execute_query::<Response<stop_service::ResponseData>>(&body)
            .await?;
        let process = Process::from(response.data.expect("missing response data").stop);
        Ok(process)
    }

    pub async fn restart(self, service_id: &str) -> Result<Process, Error> {
        let variables = restart_service::Variables {
            id: Some(service_id.to_string()),
            project_id: self.id.clone(),
        };
        let body = query::RestartService::build_query(variables);
        let response = self
            .client
            .execute_query::<Response<restart_service::ResponseData>>(&body)
            .await?;
        let process = Process::from(response.data.expect("missing response data").restart);
        Ok(process)
    }

    pub async fn status(self, service_id: &str) -> Result<Process, Error> {
        let variables = status_query::Variables {
            id: service_id.to_string(),
        };
        let body = query::StatusQuery::build_query(variables);
        let response = self
            .client
            .execute_query::<Response<status_query::ResponseData>>(&body)
            .await?;
        let process = Process::from(response.data.expect("missing response data").status);
        Ok(process)
    }

    pub async fn start_all(self) -> Result<Process, Error> {
        let variables = start_all_services::Variables {
            project_id: self.id.clone(),
        };
        let body = query::StartAllServices::build_query(variables);
        let response = self
            .client
            .execute_query::<Response<start_all_services::ResponseData>>(&body)
            .await?;
        let process = Process::from(response.data.expect("missing response data").start);
        Ok(process)
    }

    pub async fn stop_all(self) -> Result<Process, Error> {
        let variables = stop_all_services::Variables {
            project_id: self.id.clone(),
        };
        let body = query::StopAllServices::build_query(variables);
        let response = self
            .client
            .execute_query::<Response<stop_all_services::ResponseData>>(&body)
            .await?;
        let process = Process::from(response.data.expect("missing response data").stop);
        Ok(process)
    }

    pub async fn restart_all(self) -> Result<Process, Error> {
        let variables = restart_all_services::Variables {
            project_id: self.id.clone(),
        };
        let body = query::RestartAllServices::build_query(variables);
        let response = self
            .client
            .execute_query::<Response<restart_all_services::ResponseData>>(&body)
            .await?;
        let process = Process::from(response.data.expect("missing response data").restart);
        Ok(process)
    }

    pub async fn services(self) -> Result<Vec<types::Service>, Error> {
        let variables = services_query::Variables {
            project_id: self.id.clone(),
        };
        let body = query::ServicesQuery::build_query(variables);
        let response = self
            .client
            .execute_query::<Response<services_query::ResponseData>>(&body)
            .await?;
        let services = response
            .data
            .expect("missing response data")
            .services
            .into_iter()
            .map(types::Service::from)
            .collect();
        Ok(services)
    }

    pub async fn processes(self) -> Result<Vec<Process>, Error> {
        let variables = processes_query::Variables {};
        let body = query::ProcessesQuery::build_query(variables);
        let response = self
            .client
            .execute_query::<Response<processes_query::ResponseData>>(&body)
            .await?;
        let processes = response
            .data
            .expect("missing response data")
            .processes
            .into_iter()
            .map(Process::from)
            .collect();
        Ok(processes)
    }

    pub async fn logs(self, service_id: &str) -> Result<Logs, Error> {
        let variables = logs_query::Variables {
            id: service_id.to_string(),
            project_id: self.id.clone(),
        };
        let body = query::LogsQuery::build_query(variables);
        let response = self
            .client
            .execute_query::<Response<logs_query::ResponseData>>(&body)
            .await?;
        let logs = Logs::from(response.data.expect("missing response data").logs);
        Ok(logs)
    }

    pub async fn stdout(self) -> Result<(), Error> {
        build_nested_with_service_query(self.services);
        Ok(())
    }

    pub async fn add_env_var(
        self,
        service_id: &str,
        name: &str,
        value: &str,
    ) -> Result<types::Service, Error> {
        let variables = create_env_var::Variables {
            id: service_id.to_string(),
            project_id: self.id.clone(),
            name: name.to_string(),
            value: value.to_string(),
        };
        let body = query::CreateEnvVar::build_query(variables);
        let response = self
            .client
            .execute_query::<Response<create_env_var::ResponseData>>(&body)
            .await?;
        let service =
            types::Service::from(response.data.expect("missing response data").create_env_var);
        Ok(service)
    }

    pub async fn remove_env_var(
        self,
        service_id: &str,
        name: &str,
    ) -> Result<types::Service, Error> {
        let variables = delete_env_var::Variables {
            id: service_id.to_string(),
            project_id: self.id.clone(),
            name: name.to_string(),
        };
        let body = query::DeleteEnvVar::build_query(variables);
        let response = self
            .client
            .execute_query::<Response<delete_env_var::ResponseData>>(&body)
            .await?;
        let service =
            types::Service::from(response.data.expect("missing response data").delete_env_var);
        Ok(service)
    }

    pub async fn update_env_var(
        self,
        service_id: &str,
        name: &str,
        value: &str,
    ) -> Result<types::Service, Error> {
        let variables = update_env_var::Variables {
            id: service_id.to_string(),
            project_id: self.id.clone(),
            name: name.to_string(),
            value: value.to_string(),
        };
        let body = query::UpdateEnvVar::build_query(variables);
        let response = self
            .client
            .execute_query::<Response<update_env_var::ResponseData>>(&body)
            .await?;
        let service =
            types::Service::from(response.data.expect("missing response data").update_env_var);
        Ok(service)
    }
}

impl From<ProjectQueryProject> for Project {
    fn from(project: ProjectQueryProject) -> Self {
        Project {
            id: project.id,
            name: project.name,
            ..Default::default()
        }
    }
}

impl From<ProjectsQueryProjects> for Project {
    fn from(project: ProjectsQueryProjects) -> Self {
        Project {
            id: project.id,
            name: project.name,
            ..Default::default()
        }
    }
}
