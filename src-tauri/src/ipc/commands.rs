use std::sync::Arc;
use tokio::sync::Mutex;
use uuid::Uuid;
use serde::{Deserialize, Serialize};
use tracing::{info, warn};

use crate::core::TaskDefinition;
use crate::core::TaskPriority;
use crate::core::TaskQueue;
use crate::process::{ProcessRunner, TerminalManager};
use crate::sandbox::Sandbox;

#[derive(Debug, Serialize, Deserialize)]
pub struct HealthResponse {
    pub status: String,
    pub version: String,
    pub uptime: u64,
    pub active_tasks: usize,
    pub queue_length: usize,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TaskResponse {
    pub task_id: Uuid,
    pub status: String,
}

pub struct AppState {
    pub task_queue: TaskQueue,
    pub process_runner: ProcessRunner,
    pub terminal_manager: TerminalManager,
    pub sandbox: Sandbox,
    pub start_time: chrono::DateTime<chrono::Utc>,
}

#[tauri::command]
pub async fn health(state: tauri::State<'_, Arc<Mutex<AppState>>>) -> Result<HealthResponse, String> {
    let app_state = state.lock().await;
    let uptime = (chrono::Utc::now() - app_state.start_time)
        .num_seconds() as u64;

    Ok(HealthResponse {
        status: "healthy".to_string(),
        version: env!("CARGO_PKG_VERSION").to_string(),
        uptime,
        active_tasks: 0,
        queue_length: 0,
    })
}

#[tauri::command]
pub async fn execute_task(
    state: tauri::State<'_, Arc<Mutex<AppState>>>,
    command: String,
    cwd: Option<String>,
) -> Result<TaskResponse, String> {
    let app_state = state.lock().await;

    app_state.sandbox.validate_command(&command).map_err(|e| e)?;

    let definition = TaskDefinition {
        id: Uuid::new_v4(),
        command: Some(command),
        args: vec![],
        cwd,
        env: std::collections::HashMap::new(),
        timeout_ms: 30000,
        max_retries: 3,
        priority: TaskPriority::Normal,
    };

    let task_id = app_state.task_queue.enqueue(definition);
    info!(task_id = %task_id, "Task submitted via IPC");

    Ok(TaskResponse {
        task_id,
        status: "queued".to_string(),
    })
}

#[tauri::command]
pub async fn cancel_task(
    _state: tauri::State<'_, Arc<Mutex<AppState>>>,
    task_id: String,
) -> Result<bool, String> {
    let task_uuid = Uuid::parse_str(&task_id).map_err(|e| format!("Invalid task ID: {}", e))?;
    warn!(task_id = %task_uuid, "Cancel requested");
    Ok(false)
}

#[tauri::command]
pub async fn create_terminal(
    state: tauri::State<'_, Arc<Mutex<AppState>>>,
    cwd: Option<String>,
) -> Result<String, String> {
    let app_state = state.lock().await;
    let session_id = app_state.terminal_manager.create_session(cwd, 120, 40).await;
    Ok(session_id.to_string())
}

#[tauri::command]
pub async fn close_terminal(
    state: tauri::State<'_, Arc<Mutex<AppState>>>,
    session_id: String,
) -> Result<bool, String> {
    let session_uuid = Uuid::parse_str(&session_id)
        .map_err(|e| format!("Invalid session ID: {}", e))?;
    let app_state = state.lock().await;
    app_state.terminal_manager.close_session(session_uuid).await;
    Ok(true)
}

#[tauri::command]
pub async fn list_terminals(
    state: tauri::State<'_, Arc<Mutex<AppState>>>,
) -> Result<Vec<String>, String> {
    let app_state = state.lock().await;
    let sessions = app_state.terminal_manager.list_sessions().await;
    Ok(sessions.iter().map(|s| s.id.to_string()).collect())
}

pub fn build_handler(builder: tauri::Builder<tauri::Wry>) -> tauri::Builder<tauri::Wry> {
    builder.invoke_handler(tauri::generate_handler![
        health,
        execute_task,
        cancel_task,
        create_terminal,
        close_terminal,
        list_terminals,
    ])
}
