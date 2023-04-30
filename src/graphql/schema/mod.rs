use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use anyhow::Error;
use async_graphql::{MergedObject, MergedSubscription};

use self::{
    control::{ControlMutation, ControlQuery, ControlSubscription},
    logging::{LoggingQuery, LoggingSubscription},
};

pub mod control;
pub mod logging;
pub mod objects;

#[derive(Default, MergedObject)]
pub struct Query(ControlQuery, LoggingQuery);

#[derive(Default, MergedObject)]
pub struct Mutation(ControlMutation);

#[derive(Default, MergedSubscription)]
pub struct Subscription(LoggingSubscription, ControlSubscription);

pub fn get_project_id(
    path: String,
    project_map: &Arc<Mutex<HashMap<String, String>>>,
) -> Result<String, Error> {
    let project_map = project_map.lock().unwrap();
    project_map
        .get(&path)
        .map(|x| x.clone())
        .ok_or_else(|| anyhow::anyhow!("The project with path {} is not loaded", path))
}
