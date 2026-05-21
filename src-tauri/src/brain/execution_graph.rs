use std::collections::{HashMap, HashSet};
use uuid::Uuid;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum NodeType {
    TaskNode,
    CodeNode,
    BuildNode,
    AgentNode,
    VerificationNode,
    DecisionNode,
    DependencyNode,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum NodeStatus {
    Pending,
    Running,
    Completed,
    Failed(String),
    RolledBack,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExecutionNode {
    pub id: Uuid,
    pub name: String,
    pub node_type: NodeType,
    pub status: NodeStatus,
    pub trigger_reason: String,
    pub triggered_by: String, // e.g., "user", "agent_1"
    pub input_hash: String,
    pub output_hash: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExecutionEdge {
    pub from: Uuid,
    pub to: Uuid,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExecutionGraph {
    pub nodes: Vec<ExecutionNode>,
    pub edges: Vec<ExecutionEdge>,
    pub history: Vec<String>,
}

impl Default for ExecutionGraph {
    fn default() -> Self {
        Self::new()
    }
}

impl ExecutionGraph {
    pub fn new() -> Self {
        Self {
            nodes: Vec::new(),
            edges: Vec::new(),
            history: Vec::new(),
        }
    }

    pub fn add_node(&mut self, name: &str, node_type: NodeType, triggered_by: &str, reason: &str, input_hash: &str) -> Uuid {
        let id = Uuid::new_v4();
        let node = ExecutionNode {
            id,
            name: name.to_string(),
            node_type,
            status: NodeStatus::Pending,
            trigger_reason: reason.to_string(),
            triggered_by: triggered_by.to_string(),
            input_hash: input_hash.to_string(),
            output_hash: None,
        };
        self.nodes.push(node);
        self.history.push(format!("Node {} ({}) added by {}", id, name, triggered_by));
        id
    }

    pub fn add_edge(&mut self, from: Uuid, to: Uuid) -> Result<(), String> {
        if !self.nodes.iter().any(|n| n.id == from) || !self.nodes.iter().any(|n| n.id == to) {
            return Err("Both nodes must exist in the graph".to_string());
        }
        self.edges.push(ExecutionEdge { from, to });
        self.history.push(format!("Edge added from {} to {}", from, to));
        Ok(())
    }

    pub fn update_status(&mut self, id: Uuid, status: NodeStatus) {
        if let Some(node) = self.nodes.iter_mut().find(|n| n.id == id) {
            node.status = status.clone();
            self.history.push(format!("Node {} status updated to {:?}", id, status));
        }
    }

    pub fn update_output_hash(&mut self, id: Uuid, output_hash: &str) {
        if let Some(node) = self.nodes.iter_mut().find(|n| n.id == id) {
            node.output_hash = Some(output_hash.to_string());
        }
    }

    /// Performs topological sort of the graph. Returns list of node IDs.
    pub fn resolve_dependencies(&self) -> Result<Vec<Uuid>, String> {
        let mut in_degree: HashMap<Uuid, usize> = HashMap::new();
        let mut adj: HashMap<Uuid, Vec<Uuid>> = HashMap::new();

        for node in &self.nodes {
            in_degree.insert(node.id, 0);
            adj.insert(node.id, Vec::new());
        }

        for edge in &self.edges {
            *in_degree.entry(edge.to).or_insert(0) += 1;
            adj.entry(edge.from).or_insert_with(Vec::new).push(edge.to);
        }

        let mut queue: Vec<Uuid> = in_degree
            .iter()
            .filter(|&(_, &deg)| deg == 0)
            .map(|(&id, _)| id)
            .collect();

        let mut result = Vec::new();

        while let Some(u) = queue.pop() {
            result.push(u);
            if let Some(neighbors) = adj.get(&u) {
                for &v in neighbors {
                    let deg = in_degree.get_mut(&v).unwrap();
                    *deg -= 1;
                    if *deg == 0 {
                        queue.push(v);
                    }
                }
            }
        }

        if result.len() != self.nodes.len() {
            return Err("Cycle detected in execution DAG".to_string());
        }

        Ok(result)
    }

    /// Resolves parallel execution batches.
    /// Each element of the outer vector represents a batch of nodes that can run concurrently.
    pub fn get_parallel_batches(&self) -> Result<Vec<Vec<Uuid>>, String> {
        let mut in_degree: HashMap<Uuid, usize> = HashMap::new();
        let mut adj: HashMap<Uuid, Vec<Uuid>> = HashMap::new();

        for node in &self.nodes {
            in_degree.insert(node.id, 0);
            adj.insert(node.id, Vec::new());
        }

        for edge in &self.edges {
            *in_degree.entry(edge.to).or_insert(0) += 1;
            adj.entry(edge.from).or_insert_with(Vec::new).push(edge.to);
        }

        let mut current_batch: Vec<Uuid> = in_degree
            .iter()
            .filter(|&(_, &deg)| deg == 0)
            .map(|(&id, _)| id)
            .collect();

        let mut batches = Vec::new();
        let mut visited = HashSet::new();

        while !current_batch.is_empty() {
            batches.push(current_batch.clone());
            let mut next_batch = Vec::new();

            for &u in &current_batch {
                visited.insert(u);
                if let Some(neighbors) = adj.get(&u) {
                    for &v in neighbors {
                        let deg = in_degree.get_mut(&v).unwrap();
                        *deg -= 1;
                        if *deg == 0 {
                            next_batch.push(v);
                        }
                    }
                }
            }
            current_batch = next_batch;
        }

        if visited.len() != self.nodes.len() {
            return Err("Cycle or orphan node detected in DAG".to_string());
        }

        Ok(batches)
    }

    /// Rollback lineage: given a node ID, find all nodes that depend transitively on it
    pub fn get_rollback_lineage(&self, failed_node: Uuid) -> Vec<Uuid> {
        let mut adj: HashMap<Uuid, Vec<Uuid>> = HashMap::new();
        for edge in &self.edges {
            adj.entry(edge.from).or_insert_with(Vec::new).push(edge.to);
        }

        let mut lineage = Vec::new();
        let mut queue = vec![failed_node];
        let mut visited = HashSet::new();
        visited.insert(failed_node);

        while let Some(u) = queue.pop() {
            if u != failed_node {
                lineage.push(u);
            }
            if let Some(neighbors) = adj.get(&u) {
                for &v in neighbors {
                    if visited.insert(v) {
                        queue.push(v);
                    }
                }
            }
        }
        lineage
    }

    /// Failure tracing details: trace a failure back to its root trigger,
    /// what depended on it, and what changed.
    pub fn get_failure_trace(&self, failed_node: Uuid) -> Option<serde_json::Value> {
        let node = self.nodes.iter().find(|n| n.id == failed_node)?;
        let error_msg = match &node.status {
            NodeStatus::Failed(err) => err.clone(),
            _ => "Not failed".to_string(),
        };

        // Find incoming dependencies (what triggered this node)
        let incoming: Vec<String> = self.edges.iter()
            .filter(|e| e.to == failed_node)
            .filter_map(|e| self.nodes.iter().find(|n| n.id == e.from).map(|n| format!("{} ({:?})", n.name, n.node_type)))
            .collect();

        // Find outgoing dependencies (what will fail because of this node)
        let dependents = self.get_rollback_lineage(failed_node);
        let dependent_names: Vec<String> = dependents.iter()
            .filter_map(|&id| self.nodes.iter().find(|n| n.id == id).map(|n| format!("{} ({:?})", n.name, n.node_type)))
            .collect();

        Some(serde_json::json!({
            "failed_node": node.name,
            "failed_node_id": node.id,
            "node_type": node.node_type,
            "error": error_msg,
            "triggered_by": node.triggered_by,
            "trigger_reason": node.trigger_reason,
            "input_hash": node.input_hash,
            "dependencies": incoming,
            "dependents_affected": dependent_names,
        }))
    }

    /// Checks if a node can be incrementally recomputed or skipped.
    /// Returns true if it can be skipped.
    pub fn should_skip(&self, id: Uuid, current_input_hash: &str) -> bool {
        if let Some(node) = self.nodes.iter().find(|n| n.id == id) {
            // Check status of dependencies first.
            let mut dep_ok = true;
            for edge in &self.edges {
                if edge.to == id {
                    if let Some(dep_node) = self.nodes.iter().find(|n| n.id == edge.from) {
                        if dep_node.status != NodeStatus::Completed {
                            dep_ok = false;
                            break;
                        }
                    }
                }
            }

            if !dep_ok {
                return false;
            }

            // Skip if input hash matches and previous run was successful
            node.status == NodeStatus::Completed && node.input_hash == current_input_hash
        } else {
            false
        }
    }
}
