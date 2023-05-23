use crate::kv::{inmemory::store::InMemory, Store, StoreConfig};

#[test]
fn can_put() {
    let store = InMemory::new(StoreConfig::default());
    store.put("key", "value").unwrap();
    assert_eq!(store.get("key").unwrap().value, "value");
}

#[test]
fn can_delete() {
    let store = InMemory::new(StoreConfig::default());
    store.put("key", "value").unwrap();
    store.delete("key").unwrap();
    assert!(store.get("key").is_err());
}

#[test]
fn can_get() {
    let store = InMemory::new(StoreConfig::default());
    store.put("key", "value").unwrap();
    assert_eq!(store.get("key").unwrap().value, "value");
}

#[test]
fn can_call_exists() {
    let store = InMemory::new(StoreConfig::default());
    store.put("key", "value").unwrap();
    assert!(store.exists("key").unwrap());
}

#[test]
fn can_watch() {
    let store = InMemory::new(StoreConfig::default());
    let receiver = store.watch("key").unwrap();
    store.put("key", "value").unwrap();
    assert_eq!(receiver.recv().unwrap().value, "value");
}

#[test]
fn can_list() {
    let store = InMemory::new(StoreConfig::default());
    store.put("superviseur/project/name", "demo").unwrap();
    store.put("superviseur/project/owner", "tsiry").unwrap();
    store
        .put("superviseur/project/services/app", "express")
        .unwrap();
    let list = store.list("superviseur/project").unwrap();
    assert_eq!(list.len(), 3);
}

#[test]
fn can_watch_tree() {
    let store = InMemory::new(StoreConfig::default());
    let receiver = store.watch_tree("superviseur/project").unwrap();
    store.put("superviseur/project/name", "demo").unwrap();
    store.put("superviseur/project/owner", "tsiry").unwrap();
    store
        .put("superviseur/project/services/app", "express")
        .unwrap();
    let list = receiver.recv().unwrap();
    assert_eq!(list.len(), 1);
    let list = receiver.recv().unwrap();
    assert_eq!(list.len(), 2);
    let list = receiver.recv().unwrap();
    assert_eq!(list.len(), 3);
}

#[test]
fn can_delete_tree() {
    let store = InMemory::new(StoreConfig::default());
    store.put("superviseur/project/name", "demo").unwrap();
    store.put("superviseur/project/owner", "tsiry").unwrap();
    store
        .put("superviseur/project/services/app", "express")
        .unwrap();
    store.delete_tree("superviseur/project").unwrap();
    assert!(store.get("superviseur/project/name").is_err());
    assert!(store.get("superviseur/project/owner").is_err());
    assert!(store.get("superviseur/project/services/app").is_err());
}
