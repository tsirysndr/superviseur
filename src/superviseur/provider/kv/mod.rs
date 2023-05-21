use std::sync::mpsc::Receiver;

use self::consul::store::Consul;
use self::inmemory::store::InMemory;
use anyhow::{anyhow, Error};

#[cfg(test)]
pub mod tests;

pub mod consul;
pub mod inmemory;
pub mod kv;
pub mod macros;
pub mod store_wrapper;

#[derive(Default)]
pub struct StoreConfig {
    pub username: Option<String>,
    pub password: Option<String>,
    pub sync_period: Option<u64>,
    pub connection_timeout: Option<u64>,
}

#[derive(Clone)]
pub struct KVPair {
    pub key: String,
    pub value: String,
}

pub fn new_store(
    kv_type: &str,
    endpoints: Vec<String>,
    config: StoreConfig,
) -> Result<Box<dyn Store>, Error> {
    match kv_type {
        consul::STORE_NAME => Ok(Box::new(Consul::new(endpoints, config))),
        inmemory::STORE_NAME => Ok(Box::new(InMemory::new(config))),
        _ => Err(anyhow!("Unknown KV Store type: {}", kv_type)),
    }
}

pub trait Store {
    fn put(&self, key: &str, value: &str) -> Result<(), Error>;
    fn get(&self, key: &str) -> Result<KVPair, Error>;
    fn delete(&self, key: &str) -> Result<(), Error>;
    fn exists(&self, key: &str) -> Result<bool, Error>;
    fn watch(&self, key: &str) -> Result<Receiver<KVPair>, Error>;
    fn watch_tree(&self, key: &str) -> Result<Receiver<Vec<KVPair>>, Error>;
    fn list(&self, key: &str) -> Result<Vec<KVPair>, Error>;
    fn delete_tree(&self, key: &str) -> Result<(), Error>;
}
