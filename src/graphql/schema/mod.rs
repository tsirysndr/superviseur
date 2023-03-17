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
