use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
    sync::{Arc, Mutex},
};

use async_graphql::*;
use futures_util::Stream;

use crate::{
    graphql::{schema::objects::subscriptions::TailLogStream, simple_broker::SimpleBroker},
    types::configuration::ConfigurationData,
};

use super::objects::{log::Log, subscriptions};

#[derive(Default, Clone)]
pub struct LoggingQuery;

#[Object]
impl LoggingQuery {
    async fn tail(
        &self,
        ctx: &Context<'_>,
        id: ID,
        num_lines: Option<usize>,
    ) -> Result<Log, Error> {
        let config_file_path = ctx.data::<String>().unwrap();
        let config_map = ctx
            .data::<Arc<Mutex<HashMap<String, ConfigurationData>>>>()
            .unwrap();

        let config_map = config_map.lock().unwrap();

        if !config_map.contains_key(config_file_path.as_str()) {
            return Err(Error::new("Config file not found"));
        }

        let config = config_map.get(config_file_path.as_str()).unwrap();

        let service = config
            .services
            .iter()
            .find(|s| s.id == Some(id.to_string()))
            .ok_or_else(|| Error::new("Service not found"))?;

        let log_file = File::open(&service.stdout).map_err(|e| Error::new(e.to_string()))?;

        let reader = BufReader::new(log_file);

        // Read the last `num_lines` lines of the file
        let mut lines: Vec<String> = reader
            .lines()
            .map(|line| line.unwrap())
            .collect::<Vec<String>>();

        let num_lines = num_lines.unwrap_or(10);
        let tail_lines = lines.split_off(lines.len().saturating_sub(num_lines));

        Ok(Log { lines: tail_lines })
    }

    async fn logs(&self, ctx: &Context<'_>, id: ID) -> Result<Log, Error> {
        let config_file_path = ctx.data::<String>().unwrap();
        let config_map = ctx
            .data::<Arc<Mutex<HashMap<String, ConfigurationData>>>>()
            .unwrap();

        let config_map = config_map.lock().unwrap();

        if !config_map.contains_key(config_file_path.as_str()) {
            return Err(Error::new("Config file not found"));
        }

        let config = config_map.get(config_file_path.as_str()).unwrap();

        let service = config
            .services
            .iter()
            .find(|s| s.id == Some(id.to_string()))
            .ok_or_else(|| Error::new("Service not found"))?;

        let log_file = File::open(&service.stdout).map_err(|e| Error::new(e.to_string()))?;

        let reader = BufReader::new(log_file);

        let mut lines = vec![];
        for line in reader.lines() {
            let line = line.map_err(|e| Error::new(e.to_string()))?;
            lines.push(line);
        }

        Ok(Log { lines })
    }
}

#[derive(Default, Clone)]
pub struct LoggingSubscription;

#[Subscription]
impl LoggingSubscription {
    async fn tail(&self, _ctx: &Context<'_>, _id: ID) -> impl Stream<Item = TailLogStream> {
        SimpleBroker::<TailLogStream>::subscribe()
    }

    async fn logs(
        &self,
        _ctx: &Context<'_>,
        _id: ID,
    ) -> impl Stream<Item = subscriptions::LogStream> {
        SimpleBroker::<subscriptions::LogStream>::subscribe()
    }
}
