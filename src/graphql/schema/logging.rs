use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use async_graphql::*;
use futures_util::Stream;
use tokio::sync::mpsc;

use crate::{
    graphql::simple_broker::SimpleBroker,
    superviseur::SuperviseurCommand,
    types::{configuration::ConfigurationData, process::Process},
};

use super::objects::log::Log;

#[derive(Default, Clone)]
pub struct LoggingQuery;

#[Object]
impl LoggingQuery {
    async fn tail(&self, ctx: &Context<'_>, id: ID) -> Log {
        let config_file_path = ctx.data::<String>().unwrap();
        let cmd_tx = ctx.data::<mpsc::UnboundedSender<SuperviseurCommand>>();
        let processes = ctx.data::<Arc<Mutex<Vec<(Process, String)>>>>().unwrap();
        let config_map = ctx
            .data::<Arc<Mutex<HashMap<String, ConfigurationData>>>>()
            .unwrap();
        Log {
            lines: vec!["tail".to_string()],
        }
    }

    async fn logs(&self, ctx: &Context<'_>, id: ID) -> Log {
        let config_file_path = ctx.data::<String>().unwrap();
        let cmd_tx = ctx.data::<mpsc::UnboundedSender<SuperviseurCommand>>();
        let processes = ctx.data::<Arc<Mutex<Vec<(Process, String)>>>>().unwrap();
        let config_map = ctx
            .data::<Arc<Mutex<HashMap<String, ConfigurationData>>>>()
            .unwrap();
        Log {
            lines: vec!["logs".to_string()],
        }
    }
}

#[derive(Default, Clone)]
pub struct LoggingSubscription;

#[Subscription]
impl LoggingSubscription {
    async fn tail(&self, ctx: &Context<'_>, id: ID) -> impl Stream<Item = String> {
        SimpleBroker::<String>::subscribe()
    }

    async fn logs(&self, ctx: &Context<'_>, id: ID) -> impl Stream<Item = String> {
        SimpleBroker::<String>::subscribe()
    }
}
