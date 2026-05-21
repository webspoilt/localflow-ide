use std::collections::HashMap;
use std::path::Path;
use serde::{Serialize, Deserialize};
use chrono::{DateTime, Utc};
use std::fs;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileSnapshot {
    pub file_path: String,
    pub content: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Snapshot {
    pub id: String,
    pub timestamp: DateTime<Utc>,
    pub files: Vec<FileSnapshot>,
    pub description: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecoveryAttempt {
    pub task_id: String,
    pub retry_count: usize,
    pub max_retries: usize,
    pub last_error: String,
    pub requires_human_gate: bool,
    pub human_approved: Option<bool>,
}

pub struct RecoveryEngine {
    pub snapshots: Vec<Snapshot>,
    pub active_retries: HashMap<String, RecoveryAttempt>,
}

impl Default for RecoveryEngine {
    fn default() -> Self {
        Self::new()
    }
}

impl RecoveryEngine {
    pub fn new() -> Self {
        Self {
            snapshots: Vec::new(),
            active_retries: HashMap::new(),
        }
    }

    /// Captures a snapshot of specific files.
    pub fn create_snapshot(&mut self, files: Vec<String>, description: &str) -> Result<String, String> {
        let mut snapshot_files = Vec::new();

        for file in files {
            let path = Path::new(&file);
            if path.exists() && path.is_file() {
                let content = fs::read_to_string(path).map_err(|e| e.to_string())?;
                snapshot_files.push(FileSnapshot {
                    file_path: file.clone(),
                    content,
                });
            }
        }

        let id = uuid::Uuid::new_v4().to_string();
        let snapshot = Snapshot {
            id: id.clone(),
            timestamp: Utc::now(),
            files: snapshot_files,
            description: description.to_string(),
        };

        self.snapshots.push(snapshot);
        Ok(id)
    }

    /// Restores a captured snapshot.
    pub fn restore_snapshot(&self, snapshot_id: &str) -> Result<(), String> {
        let snapshot = self.snapshots.iter()
            .find(|s| s.id == snapshot_id)
            .ok_or_else(|| "Snapshot not found".to_string())?;

        for file_snap in &snapshot.files {
            let path = Path::new(&file_snap.file_path);
            if let Some(parent) = path.parent() {
                fs::create_dir_all(parent).map_err(|e| e.to_string())?;
            }
            fs::write(path, &file_snap.content).map_err(|e| e.to_string())?;
        }

        Ok(())
    }

    /// Registers a retry attempt. Enforces max retry ceiling.
    /// If retry count is exceeded, sets requires_human_gate = true.
    pub fn register_failure(&mut self, task_id: &str, error_msg: &str, max_retries: usize) -> RecoveryAttempt {
        let entry = self.active_retries.entry(task_id.to_string()).or_insert_with(|| RecoveryAttempt {
            task_id: task_id.to_string(),
            retry_count: 0,
            max_retries,
            last_error: error_msg.to_string(),
            requires_human_gate: false,
            human_approved: None,
        });

        entry.retry_count += 1;
        entry.last_error = error_msg.to_string();

        if entry.retry_count >= entry.max_retries {
            entry.requires_human_gate = true;
        }

        entry.clone()
    }

    /// Approves recovery by a human.
    pub fn human_approve_retry(&mut self, task_id: &str, approved: bool) -> Result<(), String> {
        let attempt = self.active_retries.get_mut(task_id)
            .ok_or_else(|| "No active recovery session found for task".to_string())?;

        attempt.human_approved = Some(approved);
        if approved {
            // Reset retry count to allow another run
            attempt.retry_count = 0;
            attempt.requires_human_gate = false;
        }

        Ok(())
    }

    /// Checks if a task can be retried.
    pub fn can_retry(&self, task_id: &str) -> bool {
        if let Some(attempt) = self.active_retries.get(task_id) {
            if attempt.requires_human_gate {
                // If it requires human gate, it is only allowed if explicitly approved
                matches!(attempt.human_approved, Some(true))
            } else {
                attempt.retry_count < attempt.max_retries
            }
        } else {
            true
        }
    }
}
