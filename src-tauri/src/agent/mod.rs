pub mod code;

use std::sync::Arc;
use tokio::sync::mpsc;
use tracing::info;
use uuid::Uuid;

use crate::events::RuntimeEvent;
use crate::scheduler::{TaskDefinition, TaskQueue, TaskPriority};
use crate::supervisor::Supervisor;

pub struct AgentSystem {
    task_queue: Arc<TaskQueue>,
    supervisor: Arc<Supervisor>,
    event_sender: mpsc::UnboundedSender<RuntimeEvent>,
}

impl AgentSystem {
    pub fn new(
        task_queue: Arc<TaskQueue>,
        supervisor: Arc<Supervisor>,
        event_sender: mpsc::UnboundedSender<RuntimeEvent>,
    ) -> Self {
        info!("AgentSystem initialized: dispatches tasks to Supervisor");
        Self { task_queue, supervisor, event_sender }
    }

    pub fn dispatch_task(&self, command: &str, cwd: Option<String>) -> Uuid {
        let definition = TaskDefinition {
            id: Uuid::new_v4(),
            command: Some(command.to_string()),
            args: vec![],
            cwd,
            env: std::collections::HashMap::new(),
            timeout_ms: 120000,
            max_retries: 2,
            priority: TaskPriority::Normal,
        };
        let id = self.task_queue.enqueue(definition);
        info!(task_id = %id, command = %command, "Agent dispatched task");
        id
    }

    pub async fn cancel_task(&self, task_id: Uuid) -> bool {
        self.supervisor.cancel_task(task_id).await
    }
}
