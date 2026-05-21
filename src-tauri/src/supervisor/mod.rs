use std::sync::Arc;
use tokio::sync::{mpsc, Mutex, Semaphore};
use tokio::time::Duration;
use tracing::{info, warn, error};
use uuid::Uuid;

use crate::events::RuntimeEvent;
use crate::scheduler::TaskQueue;
use crate::engine::ProcessRunner;

use crate::governor::ResourceGovernor;

type TaskMap = Arc<Mutex<std::collections::HashMap<Uuid, tokio::task::JoinHandle<()>>>>;

pub struct Supervisor {
    task_queue: Arc<TaskQueue>,
    process_runner: Arc<ProcessRunner>,
    resource_governor: Arc<ResourceGovernor>,
    concurrency_limit: Arc<Semaphore>,
    active_tasks: TaskMap,
    event_sender: mpsc::UnboundedSender<RuntimeEvent>,
    max_retries: u32,
}

impl Supervisor {
    pub fn new(
        task_queue: Arc<TaskQueue>,
        process_runner: Arc<ProcessRunner>,
        resource_governor: Arc<ResourceGovernor>,
        event_sender: mpsc::UnboundedSender<RuntimeEvent>,
        max_concurrent: usize,
        max_retries: u32,
    ) -> Self {
        Self {
            task_queue,
            process_runner,
            resource_governor,
            concurrency_limit: Arc::new(Semaphore::new(max_concurrent)),
            active_tasks: Arc::new(Mutex::new(std::collections::HashMap::new())),
            event_sender,
            max_retries,
        }
    }

    pub async fn run(self: Arc<Self>) {
        info!("Supervisor starting task dispatch loop");
        loop {
            let permit = match self.concurrency_limit.clone().acquire_owned().await {
                Ok(p) => p,
                Err(_) => {
                    error!("Supervisor semaphore closed, stopping dispatch");
                    break;
                }
            };

            let task = match self.task_queue.dequeue().await {
                Some(t) => t,
                None => continue,
            };

            let supervisor = self.clone();
            let task_id = task.id;
            let handle = tokio::spawn(async move {
                supervisor.execute_task(task, permit).await;
            });

            self.active_tasks.lock().await.insert(task_id, handle);
        }
    }

    async fn execute_task(
        self: Arc<Self>,
        mut task: crate::scheduler::Task,
        _permit: tokio::sync::OwnedSemaphorePermit,
    ) {
        let task_id = task.id;
        
        let estimated_memory_mb = 256;
        let _lease = match self.resource_governor.try_acquire(estimated_memory_mb).await {
            Ok(l) => l,
            Err(err) => {
                let _ = self.event_sender.send(RuntimeEvent::TaskFailed {
                    task_id,
                    error: format!("Resource lease failed: {}", err),
                    stdout: String::new(),
                    stderr: String::new(),
                });
                error!(task_id = %task_id, error = %err, "Task failed to acquire resource lease");
                self.active_tasks.lock().await.remove(&task_id);
                return;
            }
        };

        let _ = self.event_sender.send(RuntimeEvent::TaskStarted { task_id });
        info!(task_id = %task_id, "Task started");

        #[allow(unused_assignments)]
        let mut last_error = String::new();
        let max_attempts = task.definition.max_retries.max(self.max_retries) + 1;

        for attempt in 1..=max_attempts {
            task.attempts = attempt;
            task.started_at = Some(chrono::Utc::now());

            let timeout = Duration::from_millis(task.definition.timeout_ms);
            let result = tokio::time::timeout(timeout, self.process_runner.execute(&task.definition)).await;

            match result {
                Ok(Ok(output)) => {
                    if output.exit_code == 0 {
                        let _ = self.event_sender.send(RuntimeEvent::TaskCompleted {
                            task_id,
                            exit_code: 0,
                            stdout: output.stdout,
                            stderr: output.stderr,
                        });
                        info!(task_id = %task_id, "Task completed successfully");
                    } else {
                        let _ = self.event_sender.send(RuntimeEvent::TaskFailed {
                            task_id,
                            error: format!("Exit code: {}", output.exit_code),
                            stdout: output.stdout,
                            stderr: output.stderr,
                        });
                        warn!(task_id = %task_id, exit_code = output.exit_code, "Task failed");
                    }
                    break;
                }
                Ok(Err(e)) => {
                    last_error = e.to_string();
                    error!(task_id = %task_id, attempt, error = %last_error, "Task execution error");
                }
                Err(_) => {
                    last_error = format!("Timed out after {}ms", task.definition.timeout_ms);
                    warn!(task_id = %task_id, attempt, timeout = task.definition.timeout_ms, "Task timed out");
                }
            }

            if attempt < max_attempts {
                info!(task_id = %task_id, attempt, "Retrying task");
            } else {
                let _ = self.event_sender.send(RuntimeEvent::TaskFailed {
                    task_id,
                    error: last_error.clone(),
                    stdout: task.stdout.clone(),
                    stderr: task.stderr.clone(),
                });
                error!(task_id = %task_id, error = %last_error, "Task failed after all retries");
            }
        }

        self.active_tasks.lock().await.remove(&task_id);
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
