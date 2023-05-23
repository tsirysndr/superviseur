use std::sync::mpsc::Receiver;

use crate::kv::{KVPair, Store, StoreConfig};
use anyhow::Error;
pub struct Consul {}

impl Consul {
    pub fn new(addrs: Vec<String>, config: StoreConfig) -> Self {
        Consul {}
    }
}

impl Store for Consul {
    fn put(&self, key: &str, value: &str) -> Result<(), Error> {
        Ok(())
    }

    fn get(&self, key: &str) -> Result<KVPair, Error> {
        todo!()
    }

    fn delete(&self, key: &str) -> Result<(), Error> {
        Ok(())
    }

    fn exists(&self, key: &str) -> Result<bool, Error> {
        todo!()
    }

    fn watch(&self, key: &str) -> Result<Receiver<KVPair>, Error> {
        todo!()
    }

    fn watch_tree(&self, key: &str) -> Result<Receiver<Vec<KVPair>>, Error> {
        todo!()
    }

    fn list(&self, key: &str) -> Result<Vec<KVPair>, Error> {
        todo!()
    }

    fn delete_tree(&self, key: &str) -> Result<(), Error> {
        Ok(())
    }
}
