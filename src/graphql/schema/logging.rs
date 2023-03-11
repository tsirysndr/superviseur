use async_graphql::*;
use futures_util::Stream;

use crate::graphql::simple_broker::SimpleBroker;

use super::objects::log::Log;

#[derive(Default, Clone)]
pub struct LoggingQuery;

#[Object]
impl LoggingQuery {
    async fn tail(&self, id: ID) -> Log {
        Log {
            lines: vec!["tail".to_string()],
        }
    }

    async fn logs(&self, id: ID) -> Log {
        Log {
            lines: vec!["logs".to_string()],
        }
    }
}

#[derive(Default, Clone)]
pub struct LoggingSubscription;

#[Subscription]
impl LoggingSubscription {
    async fn tail(&self, id: ID) -> impl Stream<Item = String> {
        SimpleBroker::<String>::subscribe()
    }

    async fn logs(&self, id: ID) -> impl Stream<Item = String> {
        SimpleBroker::<String>::subscribe()
    }
}
