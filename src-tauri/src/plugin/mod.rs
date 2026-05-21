use std::collections::HashMap;
use tracing::info;

pub struct PluginHost {
    plugins: HashMap<String, PluginMetadata>,
}

pub struct PluginMetadata {
    pub name: String,
    pub version: String,
}

impl PluginHost {
    pub fn new() -> Self {
        info!("Plugin Host initialized");
        Self { plugins: HashMap::new() }
    }

    pub fn register(&mut self, name: &str, version: &str) {
        self.plugins.insert(name.to_string(), PluginMetadata {
            name: name.to_string(),
            version: version.to_string(),
        });
    }

    pub fn list(&self) -> Vec<&PluginMetadata> {
        self.plugins.values().collect()
    }
}