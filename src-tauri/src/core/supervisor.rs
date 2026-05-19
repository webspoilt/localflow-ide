use std::sync::Arc;
use tokio::sync::{mpsc, Mutex, Semaphore};
use tokio::time::Duration;
use tracing::{info, warn, error};
use uuid::Uuid;

use crate::events::RuntimeEvent;
use crate::process::runner::ProcessRunner;

pub struct Supervisor {
    process_runner: Arc<ProcessRunner>,
    concurrency_limit: Arc<Semaphore>,
    active_tasks: Arc<Mutex<std::collections::HashMap<Uuid, tokio::task::JoinHandle<()>>>>,
    event_sender: mpsc::UnboundedSender<RuntimeEvent>,
    max_retries: u32,
}

impl Supervisor {
    pub fn new(
        _task_queue: Arc<super::TaskQueue>,
        process_runner: Arc<ProcessRunner>,
        event_sender: mpsc::UnboundedSender<RuntimeEvent>,
        max_concurrent: usize,
        max_retries: u32,
    ) -> Self {
        Self {
            process_runner,
            concurrency_limit: Arc::new(Semaphore::new(max_concurrent)),
            active_tasks: Arc::new(Mutex::new(std::collections::HashMap::new())),
            event_sender,
            max_retries,
        }
    }

    pub async fn run(&self) {
        loop {
            let _permit = self.concurrency_limit.clone().acquire_owned().await;
            if _permit.is_err() {
                error!("Failed to acquire concurrency permit");
                break;
            }
            tokio::time::sleep(Duration::from_millis(100)).await;
        }
    }

    pub async fn cancel_task(&self, task_id: Uuid) -> bool {
        let mut tasks = self.active_tasks.lock().await;
        if let Some(handle) = tasks.remove(&task_id) {
            handle.abort();
            let _ = self.event_sender.send(RuntimeEvent::TaskCancelled { task_id });
            info!(task_id = %task_id, "Task cancelled");
            true
        } else {
            warn!(task_id = %task_id, "Task not found for cancellation");
            false
        }
    }

    pub async fn active_count(&self) -> usize {
        self.active_tasks.lock().await.len()
    }
}
