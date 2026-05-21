use std::sync::Arc;
use std::sync::atomic::{AtomicU64, Ordering};
use tokio::sync::{mpsc, Mutex};
use uuid::Uuid;
use tracing::{info, error};

use crate::events::RuntimeEvent;

#[derive(Debug, Clone, serde::Serialize)]
pub struct TaskDefinition {
    pub id: Uuid,
    pub command: Option<String>,
    pub args: Vec<String>,
    pub cwd: Option<String>,
    pub env: std::collections::HashMap<String, String>,
    pub timeout_ms: u64,
    pub max_retries: u32,
    pub priority: TaskPriority,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, serde::Serialize)]
pub enum TaskPriority {
    Low = 0,
    Normal = 1,
    High = 2,
    Critical = 3,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TaskStatus {
    Pending,
    Queued,
    Running,
    Completed(i32),
    Failed(String),
    Cancelled,
    TimedOut,
}

#[derive(Debug, Clone)]
pub struct Task {
    pub id: Uuid,
    pub definition: TaskDefinition,
    pub status: TaskStatus,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub started_at: Option<chrono::DateTime<chrono::Utc>>,
    pub completed_at: Option<chrono::DateTime<chrono::Utc>>,
    pub attempts: u32,
    pub stdout: String,
    pub stderr: String,
}

#[derive(Clone)]
pub struct TaskQueue {
    sender: mpsc::UnboundedSender<Task>,
    receiver: Arc<Mutex<mpsc::UnboundedReceiver<Task>>>,
    event_sender: mpsc::UnboundedSender<RuntimeEvent>,
    queued_count: Arc<AtomicU64>,
}

impl TaskQueue {
    pub fn new(event_sender: mpsc::UnboundedSender<RuntimeEvent>) -> Self {
        let (tx, rx) = mpsc::unbounded_channel();
        Self {
            sender: tx,
            receiver: Arc::new(Mutex::new(rx)),
            event_sender,
            queued_count: Arc::new(AtomicU64::new(0)),
        }
    }

    pub fn enqueue(&self, definition: TaskDefinition) -> Uuid {
        let id = Uuid::new_v4();
        let task = Task {
            id,
            definition,
            status: TaskStatus::Queued,
            created_at: chrono::Utc::now(),
            started_at: None,
            completed_at: None,
            attempts: 0,
            stdout: String::new(),
            stderr: String::new(),
        };

        if let Err(e) = self.sender.send(task) {
            error!("Failed to enqueue task: {}", e);
        } else {
            self.queued_count.fetch_add(1, Ordering::Release);
        }

        let _ = self.event_sender.send(RuntimeEvent::TaskQueued { task_id: id });
        info!(task_id = %id, "Task queued");

        id
    }

    pub async fn dequeue(&self) -> Option<Task> {
        let mut rx = self.receiver.lock().await;
        let task = rx.recv().await;
        if task.is_some() {
            self.queued_count.fetch_sub(1, Ordering::Release);
        }
        task
    }

    pub fn len(&self) -> u64 {
        self.queued_count.load(Ordering::Acquire)
    }

    pub fn sender(&self) -> mpsc::UnboundedSender<Task> {
        self.sender.clone()
    }
}
