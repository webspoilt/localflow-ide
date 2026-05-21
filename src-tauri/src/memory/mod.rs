use std::collections::HashMap;
use tracing::info;

pub struct MemoryStore {
    store: HashMap<String, String>,
}

impl MemoryStore {
    pub fn new() -> Self {
        info!("Memory Layer initialized");
        Self { store: HashMap::new() }
    }

    pub fn set(&mut self, key: &str, value: &str) {
        self.store.insert(key.to_string(), value.to_string());
    }

    pub fn get(&self, key: &str) -> Option<&String> {
        self.store.get(key)
    }
}