use crate::scheduler::{TaskDefinition, TaskStatus};
use tokio::sync::mpsc;
use tracing::info;

use crate::engine::ProcessRunner;
use crate::events::RuntimeEvent;

pub struct CodeAgent;

impl CodeAgent {
    pub async fn execute(
        &self,
        task: &TaskDefinition,
        runner: &ProcessRunner,
        event_sender: &mpsc::UnboundedSender<RuntimeEvent>,
    ) -> Result<i32, String> {
        info!(task_id = %task.id, "CodeAgent executing");
        let result = runner.execute(task).await.map_err(|e| e.to_string())?;
        let _ = event_sender.send(RuntimeEvent::TaskCompleted {
            task_id: task.id,
            exit_code: result.exit_code,
            stdout: result.stdout,
            stderr: result.stderr,
        });
        Ok(result.exit_code)
    }
}
