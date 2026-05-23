pub mod architecture_graph;
pub mod execution_graph;
pub mod recovery;

use tokio::sync::mpsc;
use tracing::info;

use crate::events::RuntimeEvent;
use crate::scheduler::TaskDefinition;

pub struct Brain {
    pub arch_graph: architecture_graph::ArchitectureGraph,
    pub exec_graph: execution_graph::ExecutionGraph,
    pub recovery: recovery::RecoveryEngine,
    event_sender: mpsc::UnboundedSender<RuntimeEvent>,
}

impl Brain {
    pub fn new(event_sender: mpsc::UnboundedSender<RuntimeEvent>) -> Self {
        info!("Brain initialized: architecture graph + execution DAG + recovery");
        Self {
            arch_graph: architecture_graph::ArchitectureGraph::new(),
            exec_graph: execution_graph::ExecutionGraph::new(),
            recovery: recovery::RecoveryEngine::new(),
            event_sender,
        }
    }

    pub fn scan_repository(&mut self, root: &str) -> Result<(), String> {
        info!("Brain scanning repository");
        self.arch_graph.scan_repository(root)
    }

    pub fn plan_from_dag(&mut self, goal: &str, tasks: Vec<TaskDefinition>) -> Vec<TaskDefinition> {
        let node_id = self.exec_graph.add_node(
            &format!("goal:{}", goal),
            execution_graph::NodeType::TaskNode,
            "brain",
            &format!("plan for {}", goal),
            "",
        );
        let mut planned = tasks;
        for task in &mut planned {
            let _ = self.exec_graph.add_edge(node_id, task.id);
        }
        planned
    }
}
