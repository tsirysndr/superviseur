use anyhow::Error;
use std::{
    collections::HashMap,
    sync::{
        mpsc::{Receiver, Sender},
        Arc, Mutex,
    },
};

use crate::superviseur::provider::kv::{KVPair, Store, StoreConfig};

pub struct InMemory {
    store: Arc<Mutex<HashMap<String, String>>>,
    watchers: Arc<Mutex<HashMap<String, Sender<KVPair>>>>,
    tree_watchers: Arc<Mutex<HashMap<String, Sender<Vec<KVPair>>>>>,
}

impl InMemory {
    pub fn new(_config: StoreConfig) -> Self {
        InMemory {
            store: Arc::new(Mutex::new(HashMap::new())),
            watchers: Arc::new(Mutex::new(HashMap::new())),
            tree_watchers: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    fn notify_watchers(&self, key: &str, value: &str) {
        let watchers = self.watchers.lock().unwrap();
        if let Some(sender) = watchers.get(key) {
            let _ = sender.send(KVPair {
                key: key.to_string(),
                value: value.to_string(),
            });
        }
    }
}

impl Store for InMemory {
    fn put(&self, key: &str, value: &str) -> Result<(), Error> {
        self.store
            .lock()
            .unwrap()
            .insert(key.to_string(), value.to_string());
        self.notify_watchers(key, value);
        let tree_watchers = self.tree_watchers.lock().unwrap();

        for (k, sender) in tree_watchers.iter() {
            if key.starts_with(k) {
                sender.send(self.list(k)?)?;
            }
        }
        Ok(())
    }

    fn get(&self, key: &str) -> Result<KVPair, Error> {
        self.store
            .lock()
            .unwrap()
            .get(key)
            .map(|value| KVPair {
                key: key.to_string(),
                value: value.to_string(),
            })
            .ok_or_else(|| Error::msg("Key not found"))
    }

    fn delete(&self, key: &str) -> Result<(), Error> {
        if let Some(_) = self.store.lock().unwrap().remove(key) {
            self.notify_watchers(key, "");
            self.watchers.lock().unwrap().remove(key);
        }
        Ok(())
    }

    fn exists(&self, key: &str) -> Result<bool, Error> {
        Ok(self.store.lock().unwrap().contains_key(key))
    }

    fn watch(&self, key: &str) -> Result<Receiver<KVPair>, Error> {
        let (sender, receiver) = std::sync::mpsc::channel();
        let mut watchers = self.watchers.lock().unwrap();

        if watchers.contains_key(key) {
            return Err(Error::msg("Key already being watched"));
        }

        watchers.insert(key.to_string(), sender.clone());

        Ok(receiver)
    }

    fn watch_tree(&self, key: &str) -> Result<Receiver<Vec<KVPair>>, Error> {
        let (sender, receiver) = std::sync::mpsc::channel();
        let mut watchers = self.tree_watchers.lock().unwrap();

        if watchers.contains_key(key) {
            return Err(Error::msg("Key already being watched"));
        }

        watchers.insert(key.to_string(), sender.clone());

        Ok(receiver)
    }

    fn list(&self, key: &str) -> Result<Vec<KVPair>, Error> {
        Ok(self
            .store
            .lock()
            .unwrap()
            .iter()
            .filter(|(k, _)| k.starts_with(key))
            .map(|(key, value)| KVPair {
                key: key.to_string(),
                value: value.to_string(),
            })
            .collect::<Vec<KVPair>>())
    }

    fn delete_tree(&self, key: &str) -> Result<(), Error> {
        self.store
            .lock()
            .unwrap()
            .retain(|k, _| !k.starts_with(key));
        self.tree_watchers.lock().unwrap().remove(key);
        Ok(())
    }
}
