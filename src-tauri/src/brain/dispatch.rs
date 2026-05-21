use tokio::sync::mpsc;
use tracing::info;

use crate::brain::dag::TaskDag;
use crate::events::RuntimeEvent;
use crate::scheduler::{TaskDefinition, TaskPriority};

pub struct AgentDispatcher {
    event_sender: mpsc::UnboundedSender<RuntimeEvent>,
}

impl AgentDispatcher {
    pub fn new(event_sender: mpsc::UnboundedSender<RuntimeEvent>) -> Self {
        Self { event_sender }
    }

    pub async fn dispatch(&self, dag: &TaskDag) -> Vec<TaskDefinition> {
        let sorted = dag.topo_sort();
        info!(count = sorted.len(), "Brain dispatching tasks");

        sorted.iter().map(|node| {
            let definition = TaskDefinition {
                id: node.id,
                command: Some(format!("agent:{}", node.label)),
                args: vec![],
                cwd: None,
                env: std::collections::HashMap::new(),
                timeout_ms: 60000,
                max_retries: 2,
                priority: TaskPriority::Normal,
            };
            let _ = self.event_sender.send(RuntimeEvent::TaskCreated {
                task_id: node.id,
                definition: Some(serde_json::to_value(&definition).unwrap_or_default()),
            });
            definition
        }).collect()
    }
}
