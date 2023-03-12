use async_graphql::Schema;

use self::schema::{Mutation, Query, Subscription};

pub mod schema;
pub mod simple_broker;

pub type SuperviseurSchema = Schema<Query, Mutation, Subscription>;
