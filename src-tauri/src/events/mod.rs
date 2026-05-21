use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", content = "payload")]
#[serde(rename_all = "camelCase")]
pub enum RuntimeEvent {
    // Task lifecycle
    #[serde(rename = "TASK_CREATED")]
    TaskCreated {
        task_id: Uuid,
        #[serde(skip_serializing_if = "Option::is_none")]
        definition: Option<serde_json::Value>,
    },
    #[serde(rename = "TASK_QUEUED")]
    TaskQueued { task_id: Uuid },
    #[serde(rename = "TASK_STARTED")]
    TaskStarted { task_id: Uuid },
    #[serde(rename = "TASK_PROGRESS")]
    TaskProgress {
        task_id: Uuid,
        progress: f64,
        message: String,
    },
    #[serde(rename = "TASK_COMPLETED")]
    TaskCompleted {
        task_id: Uuid,
        exit_code: i32,
        stdout: String,
        stderr: String,
    },
    #[serde(rename = "TASK_FAILED")]
    TaskFailed {
        task_id: Uuid,
        error: String,
        stdout: String,
        stderr: String,
    },
    #[serde(rename = "TASK_CANCELLED")]
    TaskCancelled { task_id: Uuid },

    // Terminal lifecycle
    #[serde(rename = "TERMINAL_CREATED")]
    TerminalCreated { session_id: Uuid },
    #[serde(rename = "TERMINAL_OUTPUT")]
    TerminalOutput {
        session_id: Uuid,
        data: String,
        stream: String,
    },
    #[serde(rename = "TERMINAL_CLOSED")]
    TerminalClosed { session_id: Uuid },

    // Runtime lifecycle
    #[serde(rename = "RUNTIME_STARTED")]
    RuntimeStarted,
    #[serde(rename = "RUNTIME_SHUTDOWN")]
    RuntimeShutdown { reason: String },
    #[serde(rename = "RUNTIME_HEALTH")]
    RuntimeHealth {
        status: String,
        uptime: u64,
        active_tasks: usize,
        queue_length: usize,
    },

    // Error events
    #[serde(rename = "ERROR")]
    Error {
        source: String,
        message: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        details: Option<serde_json::Value>,
    },
}
