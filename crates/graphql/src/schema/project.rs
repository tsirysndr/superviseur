use std::{
    collections::HashMap,
    io::Write,
    sync::{Arc, Mutex},
};

use async_graphql::{Context, Error, Object, ID};
use indexmap::IndexMap;
use names::Generator;
use superviseur_code::start_code_tunnel;
use superviseur_provider::kv::kv::Provider;
use superviseur_types::{
    command::SuperviseurCommand, configuration::ConfigurationData, PROJECTS_DIR,
};
use tokio::sync::mpsc;

use super::objects::{
    project::Project, project_configuration::ProjectConfiguration, service::Service,
};
use crate::{
    macros::{project_exists, send_event},
    schema::objects::subscriptions::{AllServicesStopped, ProjectOpened},
    simple_broker::SimpleBroker,
};

#[derive(Clone, Default)]
pub struct ProjectQuery;

#[Object]
impl ProjectQuery {
    async fn projects(&self, ctx: &Context<'_>) -> Result<Vec<Project>, Error> {
        let project_map = ctx.data::<Arc<Mutex<HashMap<String, String>>>>().unwrap();
        let provider = ctx.data::<Arc<Provider>>().unwrap();

        let project_map = project_map.lock().unwrap();

        let projects = provider
            .all_projects()
            .map_err(|e| Error::new(e.to_string()))?;
        let projects = projects
            .iter()
            .map(|(id, name, _)| {
                let config_path = match project_map.clone().into_iter().find(|(_, v)| v == id) {
                    Some((k, _)) => Some(k),
                    None => None,
                };

                Project {
                    id: ID(id.clone()),
                    name: name.clone(),
                    config_path,
                }
            })
            .collect();
        Ok(projects)
    }

    async fn project(&self, ctx: &Context<'_>, id: ID) -> Result<Project, Error> {
        let project_map = ctx.data::<Arc<Mutex<HashMap<String, String>>>>().unwrap();
        let provider = ctx.data::<Arc<Provider>>().unwrap();

        let project_map = project_map.lock().unwrap();

        let project_id = id.to_string();

        project_exists!(provider, project_id);

        let config = provider.build_configuration(&project_id)?;

        let config_path = match project_map
            .clone()
            .into_iter()
            .find(|(_, v)| v == &project_id)
        {
            Some((k, _)) => Some(k),
            None => None,
        };

        Ok(Project {
            id: ID(project_id),
            name: config.project.clone(),
            config_path,
        })
    }
}

#[derive(Clone, Default)]
pub struct ProjectMutation;

#[Object]
impl ProjectMutation {
    async fn new_project(
        &self,
        ctx: &Context<'_>,
        name: String,
        context: Option<String>,
        from: Option<String>,
    ) -> Result<ProjectConfiguration, Error> {
        let provider = ctx.data::<Arc<Provider>>().unwrap();

        let mut generator = Generator::default();
        let id = generator.next().unwrap();

        let project_dir = match context.clone() {
            Some(c) => Some(c),
            None => {
                let dir = format!(
                    "{}/{}/{}",
                    dirs::home_dir().unwrap().to_str().unwrap(),
                    PROJECTS_DIR,
                    id
                );
                std::fs::create_dir_all(&dir).unwrap();
                Some(dir)
            }
        };

        let config = ConfigurationData {
            project: name.clone(),
            services: IndexMap::new(),
            context: project_dir.clone(),
            network_settings: None,
            volume_settings: None,
        };

        if context.is_none() {
            let mut config_file =
                std::fs::File::create(format!("{}/Superfile.hcl", project_dir.clone().unwrap()))?;
            let config = hcl::to_string(&config)?;
            config_file.write_all(config.as_bytes())?;
        }

        let projects = provider
            .all_projects()
            .map_err(|e| Error::new(e.to_string()))?;
        if projects.into_iter().any(|(_, _, ctx)| Some(ctx) == context) {
            return Err(Error::new("Project already exists with this context"));
        }

        provider.save_configuration(&id, config)?;

        return Ok(ProjectConfiguration {
            id: ID(id),
            name,
            context: project_dir.unwrap(),
            ..Default::default()
        });
    }

    async fn delete_project(&self, ctx: &Context<'_>, id: ID) -> Result<Project, Error> {
        let project_id = id.to_string();
        let cmd_tx = ctx
            .data::<mpsc::UnboundedSender<SuperviseurCommand>>()
            .unwrap();
        let provider = ctx.data::<Arc<Provider>>().unwrap();
        let project_map = ctx.data::<Arc<Mutex<HashMap<String, String>>>>().unwrap();

        project_exists!(provider, project_id);

        let config = provider
            .build_configuration(&project_id)
            .map_err(|e| Error::new(e.to_string()))?;

        send_event!(
            config.project,
            config.services,
            cmd_tx,
            Stop,
            AllServicesStopped
        );

        let project_map = project_map.lock().unwrap();
        let config_path = match project_map
            .clone()
            .into_iter()
            .find(|(_, v)| v == &project_id)
        {
            Some((k, _)) => Some(k),
            None => None,
        };

        provider.delete_configuration(&project_id)?;

        Ok(Project {
            id: ID(project_id),
            config_path,
            ..Default::default()
        })
    }

    async fn open_project(&self, ctx: &Context<'_>, id: ID) -> Result<Project, Error> {
        let project_id = id.to_string();
        let provider = ctx.data::<Arc<Provider>>().unwrap();
        let project_map = ctx.data::<Arc<Mutex<HashMap<String, String>>>>().unwrap();

        project_exists!(provider, project_id);

        let (_, context) = provider.project(&project_id)?;

        let config = provider
            .build_configuration(&project_id)
            .map_err(|e| Error::new(e.to_string()))?;

        let project_map = project_map.lock().unwrap();
        let config_path = match project_map
            .clone()
            .into_iter()
            .find(|(_, v)| v == &project_id)
        {
            Some((k, _)) => Some(k),
            None => None,
        };

        let (sender, mut receiver) = tokio::sync::mpsc::unbounded_channel();
        let id = project_id.clone();

        tokio::spawn(async move {
            loop {
                match receiver.recv().await {
                    Some(line) => {
                        println!("{}", line);
                        SimpleBroker::publish(ProjectOpened {
                            id: id.clone(),
                            line,
                        });
                    }
                    None => {
                        break;
                    }
                }
            }
        });

        start_code_tunnel(sender, &context)?;

        Ok(Project {
            id: ID(project_id),
            config_path,
            name: config.project,
            ..Default::default()
        })
    }
}
