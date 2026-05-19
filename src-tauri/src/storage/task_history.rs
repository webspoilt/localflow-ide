use std::path::PathBuf;
use tokio::sync::Mutex;
use tracing::{info, error};
use uuid::Uuid;
use serde::{Deserialize, Serialize};

use crate::core::Task;
use crate::core::TaskStatus;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskRecord {
    pub id: Uuid,
    pub command: String,
    pub status: String,
    pub created_at: i64,
    pub started_at: Option<i64>,
    pub completed_at: Option<i64>,
    pub attempts: u32,
    pub exit_code: Option<i32>,
    pub stdout: String,
    pub stderr: String,
}

pub struct TaskHistoryStore {
    records: Mutex<Vec<TaskRecord>>,
    storage_path: PathBuf,
}

impl TaskHistoryStore {
    pub fn new(storage_dir: PathBuf) -> Self {
        let path = storage_dir.join("task_history.json");
        let records = if path.exists() {
            std::fs::read_to_string(&path)
                .ok()
                .and_then(|content| serde_json::from_str(&content).ok())
                .unwrap_or_default()
        } else {
            vec![]
        };

        info!(path = %path.display(), record_count = records.len(), "Task history loaded");

        Self {
            records: Mutex::new(records),
            storage_path: path,
        }
    }

    pub async fn record_task(&self, task: &Task) {
        let record = TaskRecord {
            id: task.id,
            command: task.definition.command.clone().unwrap_or_default(),
            status: match &task.status {
                TaskStatus::Pending => "pending".to_string(),
                TaskStatus::Queued => "queued".to_string(),
                TaskStatus::Running => "running".to_string(),
                TaskStatus::Completed(code) => format!("completed:{}", code),
                TaskStatus::Failed(_) => "failed".to_string(),
                TaskStatus::Cancelled => "cancelled".to_string(),
                TaskStatus::TimedOut => "timed_out".to_string(),
            },
            created_at: task.created_at.timestamp(),
            started_at: task.started_at.map(|t| t.timestamp()),
            completed_at: task.completed_at.map(|t| t.timestamp()),
            attempts: task.attempts,
            exit_code: match &task.status {
                TaskStatus::Completed(code) => Some(*code),
                _ => None,
            },
            stdout: task.stdout.clone(),
            stderr: task.stderr.clone(),
        };

        let mut records = self.records.lock().await;
        if let Some(existing) = records.iter_mut().find(|r| r.id == task.id) {
            *existing = record;
        } else {
            records.push(record);
        }

        if let Err(e) = self.flush(&records).await {
            error!("Failed to persist task history: {}", e);
        }
    }

    pub async fn get_history(&self) -> Vec<TaskRecord> {
        let records = self.records.lock().await;
        let mut history: Vec<TaskRecord> = records.clone();
        history.sort_by(|a, b| b.created_at.cmp(&a.created_at));
        history.truncate(100);
        history
    }

    async fn flush(&self, records: &[TaskRecord]) -> Result<(), std::io::Error> {
        if let Some(parent) = self.storage_path.parent() {
            tokio::fs::create_dir_all(parent).await?;
        }
        let json = serde_json::to_string_pretty(records)?;
        tokio::fs::write(&self.storage_path, json).await
    }
}
