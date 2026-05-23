pub mod memory_graph;

use std::collections::HashMap;
use std::path::PathBuf;
use tracing::info;

pub struct MemoryStore {
    store: HashMap<String, String>,
    storage_path: Option<PathBuf>,
    dirty: bool,
}

impl MemoryStore {
    pub fn new(storage_dir: Option<PathBuf>) -> Self {
        let mut store = Self {
            store: HashMap::new(),
            storage_path: storage_dir.map(|d| d.join("memory_store.json")),
            dirty: false,
        };
        if let Some(ref path) = store.storage_path {
            if path.exists() {
                if let Ok(content) = std::fs::read_to_string(path) {
                    if let Ok(data) = serde_json::from_str::<HashMap<String, String>>(&content) {
                        store.store = data;
                        info!("MemoryStore loaded {} entries from disk", store.store.len());
                    }
                }
            }
        }
        info!("MemoryStore initialized");
        store
    }

    pub fn set(&mut self, key: &str, value: &str) {
        self.store.insert(key.to_string(), value.to_string());
        self.dirty = true;
        self.maybe_flush();
    }

    pub fn get(&self, key: &str) -> Option<&String> {
        self.store.get(key)
    }

    pub fn all(&self) -> &HashMap<String, String> {
        &self.store
    }

    fn maybe_flush(&mut self) {
        if let Some(ref path) = self.storage_path {
            if self.dirty {
                if let Some(parent) = path.parent() {
                    let _ = std::fs::create_dir_all(parent);
                }
                if let Ok(json) = serde_json::to_string(&self.store) {
                    let _ = std::fs::write(path, json);
                    self.dirty = false;
                }
            }
        }
    }
}

impl Drop for MemoryStore {
    fn drop(&mut self) {
        self.maybe_flush();
    }
}
