use std::collections::{HashMap, HashSet, VecDeque};
use std::path::Path;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum MemoryNodeType {
    Convention,
    Decision,
    Fix,
    Preference,
    Target,
    Pattern,
    FailureRecord,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryNode {
    pub id: String,
    pub node_type: MemoryNodeType,
    pub title: String,
    pub description: String,
    pub tags: Vec<String>,
    pub metadata: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryEdge {
    pub from: String,
    pub to: String,
    pub relationship: String, // "relates_to" | "addresses" | "replaces" | "depends_on"
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryGraph {
    pub nodes: HashMap<String, MemoryNode>,
    pub edges: Vec<MemoryEdge>,
}

impl Default for MemoryGraph {
    fn default() -> Self {
        Self::new()
    }
}

impl MemoryGraph {
    pub fn new() -> Self {
        Self {
            nodes: HashMap::new(),
            edges: Vec::new(),
        }
    }

    pub fn add_node(&mut self, id: &str, node_type: MemoryNodeType, title: &str, description: &str, tags: Vec<String>, metadata: HashMap<String, String>) {
        let node = MemoryNode {
            id: id.to_string(),
            node_type,
            title: title.to_string(),
            description: description.to_string(),
            tags,
            metadata,
        };
        self.nodes.insert(id.to_string(), node);
    }

    pub fn add_edge(&mut self, from: &str, to: &str, relationship: &str) {
        if self.nodes.contains_key(from) && self.nodes.contains_key(to) {
            self.edges.push(MemoryEdge {
                from: from.to_string(),
                to: to.to_string(),
                relationship: relationship.to_string(),
            });
        }
    }

    pub fn search_by_tag(&self, tag: &str) -> Vec<&MemoryNode> {
        self.nodes.values()
            .filter(|n| n.tags.iter().any(|t| t.to_lowercase() == tag.to_lowercase()))
            .collect()
    }

    pub fn search_by_type(&self, node_type: MemoryNodeType) -> Vec<&MemoryNode> {
        self.nodes.values()
            .filter(|n| n.node_type == node_type)
            .collect()
    }

    pub fn get_related_nodes(&self, node_id: &str) -> Vec<&MemoryNode> {
        let mut related_ids = HashSet::new();
        for edge in &self.edges {
            if edge.from == node_id {
                related_ids.insert(&edge.to);
            } else if edge.to == node_id {
                related_ids.insert(&edge.from);
            }
        }

        self.nodes.values()
            .filter(|n| related_ids.contains(&n.id))
            .collect()
    }

    /// Load memory graph from a JSON file.
    pub fn load_from_file<P: AsRef<Path>>(&mut self, path: P) -> Result<(), String> {
        let path_ref = path.as_ref();
        if !path_ref.exists() {
            return Ok(()); // Nothing to load, start clean
        }
        let data = std::fs::read_to_string(path_ref).map_err(|e| e.to_string())?;
        let loaded: MemoryGraph = serde_json::from_str(&data).map_err(|e| e.to_string())?;
        self.nodes = loaded.nodes;
        self.edges = loaded.edges;
        Ok(())
    }

    /// Save memory graph to a JSON file.
    pub fn save_to_file<P: AsRef<Path>>(&self, path: P) -> Result<(), String> {
        let path_ref = path.as_ref();
        if let Some(parent) = path_ref.parent() {
            std::fs::create_dir_all(parent).map_err(|e| e.to_string())?;
        }
        let serialized = serde_json::to_string_pretty(self).map_err(|e| e.to_string())?;
        std::fs::write(path_ref, serialized).map_err(|e| e.to_string())?;
        Ok(())
    }
}
