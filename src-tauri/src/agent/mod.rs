pub mod planner;
pub mod code;
pub mod review;
pub mod test;
pub mod security;

use std::sync::Arc;
use tokio::sync::mpsc;
use tracing::info;
use uuid::Uuid;

use crate::events::RuntimeEvent;
use crate::scheduler::TaskDefinition;
use crate::model::ModelRouter;

#[derive(Debug)]
pub enum AgentType {
    Planner,
    Code,
    Review,
    Test,
    Security,
}

#[allow(dead_code)]
pub struct AgentSystem {
    planner: planner::PlannerAgent,
    code: code::CodeAgent,
    review: review::ReviewAgent,
    test: test::TestAgent,
    security: security::SecurityAgent,
    model: Arc<ModelRouter>,
    event_sender: mpsc::UnboundedSender<RuntimeEvent>,
}

impl AgentSystem {
    pub fn new(model: Arc<ModelRouter>, event_sender: mpsc::UnboundedSender<RuntimeEvent>) -> Self {
        info!("Agent System initialized");
        Self {
            planner: planner::PlannerAgent,
            code: code::CodeAgent,
            review: review::ReviewAgent,
            test: test::TestAgent,
            security: security::SecurityAgent,
            model,
            event_sender,
        }
    }

    pub fn dispatch(&self, task: &TaskDefinition) -> Option<Uuid> {
        let agent_type = match task.command.as_deref() {
            Some(cmd) if cmd.starts_with("agent:planner") => AgentType::Planner,
            Some(cmd) if cmd.starts_with("agent:code") || cmd.starts_with("agent:execute") => AgentType::Code,
            Some(cmd) if cmd.starts_with("agent:review") => AgentType::Review,
            Some(cmd) if cmd.starts_with("agent:test") => AgentType::Test,
            Some(cmd) if cmd.starts_with("agent:security") => AgentType::Security,
            _ => return None,
        };

        info!(task_id = %task.id, agent = ?agent_type, "Agent dispatched");
        let _ = self.event_sender.send(RuntimeEvent::TaskStarted { task_id: task.id });
        Some(task.id)
    }
}
