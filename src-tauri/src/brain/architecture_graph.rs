use std::collections::{HashMap, HashSet, VecDeque};
use std::path::Path;
use serde::{Serialize, Deserialize};
use std::fs;
use regex::Regex;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ArchNodeType {
    File,
    Function,
    Module,
    Service,
    Dependency,
    Api,
    Test,
    Configuration,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArchNode {
    pub id: String, // Unique identifier (e.g., path, fully qualified symbol)
    pub name: String,
    pub node_type: ArchNodeType,
    pub details: HashMap<String, String>,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum ArchEdgeType {
    Imports,
    Calls,
    DependsOn,
    Tests,
    Inherits,
    Modifies,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArchEdge {
    pub from: String,
    pub to: String,
    pub edge_type: ArchEdgeType,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArchitectureGraph {
    pub nodes: HashMap<String, ArchNode>,
    pub edges: Vec<ArchEdge>,
}

impl Default for ArchitectureGraph {
    fn default() -> Self {
        Self::new()
    }
}

impl ArchitectureGraph {
    pub fn new() -> Self {
        Self {
            nodes: HashMap::new(),
            edges: Vec::new(),
        }
    }

    pub fn add_node(&mut self, id: &str, name: &str, node_type: ArchNodeType, details: HashMap<String, String>) {
        let node = ArchNode {
            id: id.to_string(),
            name: name.to_string(),
            node_type,
            details,
        };
        self.nodes.insert(id.to_string(), node);
    }

    pub fn add_edge(&mut self, from: &str, to: &str, edge_type: ArchEdgeType) {
        // Ensure both nodes exist (or insert placeholder nodes)
        if !self.nodes.contains_key(from) {
            self.add_node(from, from, ArchNodeType::Module, HashMap::new());
        }
        if !self.nodes.contains_key(to) {
            self.add_node(to, to, ArchNodeType::Module, HashMap::new());
        }
        self.edges.push(ArchEdge {
            from: from.to_string(),
            to: to.to_string(),
            edge_type,
        });
    }

    /// Performs directory scanning of a given project path.
    /// Uses regex heuristics to identify module structures and imports.
    pub fn scan_repository(&mut self, base_path: &str) -> Result<(), String> {
        let path = Path::new(base_path);
        if !path.exists() {
            return Err("Base path does not exist".to_string());
        }

        self.scan_dir(path, base_path)?;
        self.resolve_import_edges();
        Ok(())
    }

    fn scan_dir(&mut self, dir: &Path, base_path: &str) -> Result<(), String> {
        let read_dir = fs::read_dir(dir).map_err(|e| e.to_string())?;

        // Simple Regexes for finding imports/calls
        let ts_import_regex = Regex::new(r#"import\s+.*\s+from\s+['"]([^'"]+)['"]"#).unwrap();
        let rust_use_regex = Regex::new(r#"use\s+([^;]+);"#).unwrap();
        let fn_regex = Regex::new(r#"fn\s+([a-zA-Z0-9_]+)\s*\("#).unwrap();
        let ts_fn_regex = Regex::new(r#"(?:function|const)\s+([a-zA-Z0-9_]+)\s*=\s*(?:\([^)]*\)|[a-zA-Z0-9_]+)\s*=>"#).unwrap();

        for entry in read_dir.flatten() {
            let entry_path = entry.path();
            let relative_path = entry_path.strip_prefix(base_path)
                .unwrap_or(&entry_path)
                .to_string_lossy()
                .to_string()
                .replace('\\', "/");

            if relative_path.contains("node_modules") || relative_path.contains("target") || relative_path.contains(".git") {
                continue;
            }

            if entry_path.is_dir() {
                // Add directory node
                let mut details = HashMap::new();
                details.insert("type".to_string(), "directory".to_string());
                self.add_node(&relative_path, &relative_path, ArchNodeType::Module, details);
                self.scan_dir(&entry_path, base_path)?;
            } else if entry_path.is_file() {
                let ext = entry_path.extension().and_then(|e| e.to_str()).unwrap_or("");
                if !["rs", "ts", "tsx", "js", "jsx", "json", "toml"].contains(&ext) {
                    continue;
                }

                let mut details = HashMap::new();
                details.insert("extension".to_string(), ext.to_string());
                let node_type = if relative_path.contains("test") || relative_path.contains("spec") {
                    ArchNodeType::Test
                } else if ext == "json" || ext == "toml" {
                    ArchNodeType::Configuration
                } else {
                    ArchNodeType::File
                };

                self.add_node(&relative_path, &relative_path, node_type, details);

                // Scan file contents
                if let Ok(content) = fs::read_to_string(&entry_path) {
                    // Extract functions
                    if ext == "rs" {
                        for cap in fn_regex.captures_iter(&content) {
                            let fn_name = &cap[1];
                            let symbol_id = format!("{}:{}", relative_path, fn_name);
                            let mut details = HashMap::new();
                            details.insert("file".to_string(), relative_path.clone());
                            self.add_node(&symbol_id, fn_name, ArchNodeType::Function, details);
                            self.add_edge(&relative_path, &symbol_id, ArchEdgeType::DependsOn);
                        }
                        // Use declarations
                        for cap in rust_use_regex.captures_iter(&content) {
                            let import_path = cap[1].trim();
                            self.add_edge(&relative_path, import_path, ArchEdgeType::Imports);
                        }
                    } else if ["ts", "tsx", "js", "jsx"].contains(&ext) {
                        for cap in ts_fn_regex.captures_iter(&content) {
                            let fn_name = &cap[1];
                            let symbol_id = format!("{}:{}", relative_path, fn_name);
                            let mut details = HashMap::new();
                            details.insert("file".to_string(), relative_path.clone());
                            self.add_node(&symbol_id, fn_name, ArchNodeType::Function, details);
                            self.add_edge(&relative_path, &symbol_id, ArchEdgeType::DependsOn);
                        }
                        // TS/JS Imports
                        for cap in ts_import_regex.captures_iter(&content) {
                            let import_path = &cap[1];
                            self.add_edge(&relative_path, import_path, ArchEdgeType::Imports);
                        }
                    }
                }
            }
        }
        Ok(())
    }

    fn resolve_import_edges(&mut self) {
        // Simple heuristic to connect relative/symbol imports to actual files
        let node_ids: Vec<String> = self.nodes.keys().cloned().collect();
        let mut edges_to_add = Vec::new();

        for edge in &self.edges {
            if edge.edge_type == ArchEdgeType::Imports {
                // If it imports "crate::brain::goal" or "./goal", check match
                let to_target = &edge.to;
                let from_file = &edge.from;
                
                // Relative import helper
                if to_target.starts_with('.') {
                    if let Some(parent) = Path::new(from_file).parent() {
                        let combined = parent.join(to_target);
                        // Try mapping to .ts, .tsx, .rs files
                        for ext in &["ts", "tsx", "rs"] {
                            let target_path = combined.with_extension(ext).to_string_lossy().to_string().replace('\\', "/");
                            if node_ids.contains(&target_path) {
                                edges_to_add.push((from_file.clone(), target_path, ArchEdgeType::Imports));
                                break;
                            }
                        }
                    }
                } else {
                    // Symbol import matching: check if any file path contains/ends-with the symbol target
                    let target_lower = to_target.to_lowercase().replace("::", "/");
                    for nid in &node_ids {
                        if nid.to_lowercase().contains(&target_lower) {
                            edges_to_add.push((from_file.clone(), nid.clone(), ArchEdgeType::DependsOn));
                        }
                    }
                }
            }
        }

        for (from, to, etype) in edges_to_add {
            self.edges.push(ArchEdge { from, to, edge_type: etype });
        }

        // De-duplicate edges
        let mut seen = HashSet::new();
        self.edges.retain(|e| seen.insert((e.from.clone(), e.to.clone(), e.edge_type)));
    }

    /// Query: "Which modules/files break if X changes?"
    /// This is an upstream dependency search (nodes that depend on node X).
    /// That is, nodes from which there is a path to X.
    pub fn query_impacted_by(&self, target_id: &str) -> Vec<String> {
        let mut reverse_adj: HashMap<String, Vec<String>> = HashMap::new();
        for edge in &self.edges {
            reverse_adj.entry(edge.to.clone()).or_insert_with(Vec::new).push(edge.from.clone());
        }

        let mut impacted = Vec::new();
        let mut queue = VecDeque::new();
        let mut visited = HashSet::new();

        queue.push_back(target_id.to_string());
        visited.insert(target_id.to_string());

        while let Some(curr) = queue.pop_front() {
            if curr != target_id {
                impacted.push(curr.clone());
            }

            if let Some(parents) = reverse_adj.get(&curr) {
                for parent in parents {
                    if visited.insert(parent.clone()) {
                        queue.push_back(parent.clone());
                    }
                }
            }
        }

        impacted
    }

    /// Query: "What does X depend on?"
    /// Downstream dependency search.
    pub fn query_dependencies_of(&self, target_id: &str) -> Vec<String> {
        let mut adj: HashMap<String, Vec<String>> = HashMap::new();
        for edge in &self.edges {
            adj.entry(edge.from.clone()).or_insert_with(Vec::new).push(edge.to.clone());
        }

        let mut deps = Vec::new();
        let mut queue = VecDeque::new();
        let mut visited = HashSet::new();

        queue.push_back(target_id.to_string());
        visited.insert(target_id.to_string());

        while let Some(curr) = queue.pop_front() {
            if curr != target_id {
                deps.push(curr.clone());
            }

            if let Some(children) = adj.get(&curr) {
                for child in children {
                    if visited.insert(child.clone()) {
                        queue.push_back(child.clone());
                    }
                }
            }
        }

        deps
    }
}
