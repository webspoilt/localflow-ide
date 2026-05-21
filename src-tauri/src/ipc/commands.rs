use std::sync::Arc;
use tokio::sync::Mutex;
use uuid::Uuid;
use serde::{Deserialize, Serialize};
use tracing::info;
use std::path::Path;

use crate::scheduler::{TaskDefinition, TaskPriority};
use crate::scheduler::TaskQueue;
use crate::pty::TerminalManager;
use crate::engine::ProcessRunner;
use crate::supervisor::Supervisor;
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

#[derive(Debug, Serialize, Deserialize)]
pub struct CommandResult {
    pub exit_code: i32,
    pub stdout: String,
    pub stderr: String,
    pub duration_ms: u64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FileEntry {
    pub id: String,
    pub name: String,
    pub path: String,
    #[serde(rename = "type")]
    pub file_type: String,
    pub language: Option<String>,
    pub children: Option<Vec<FileEntry>>,
}

use crate::governor::ResourceGovernor;

pub struct AppState {
    pub task_queue: TaskQueue,
    pub supervisor: Arc<Supervisor>,
    pub process_runner: ProcessRunner,
    pub terminal_manager: TerminalManager,
    pub sandbox: Sandbox,
    pub resource_governor: Arc<ResourceGovernor>,
    pub start_time: chrono::DateTime<chrono::Utc>,
}

#[tauri::command]
pub async fn health(state: tauri::State<'_, Arc<Mutex<AppState>>>) -> Result<HealthResponse, String> {
    let app_state = state.lock().await;
    let uptime = (chrono::Utc::now() - app_state.start_time).num_seconds() as u64;
    let active_tasks = app_state.supervisor.active_count().await;
    let queue_length = app_state.task_queue.len() as usize;
    Ok(HealthResponse {
        status: "healthy".to_string(),
        version: env!("CARGO_PKG_VERSION").to_string(),
        uptime,
        active_tasks,
        queue_length,
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
    info!(task_id = %task_id, "Task queued");
    Ok(TaskResponse { task_id, status: "queued".to_string() })
}

#[tauri::command]
pub async fn execute_command(
    state: tauri::State<'_, Arc<Mutex<AppState>>>,
    command: String,
) -> Result<CommandResult, String> {
    let app_state = state.lock().await;

    app_state.sandbox.validate_command(&command).map_err(|e| e)?;

    let definition = TaskDefinition {
        id: Uuid::new_v4(),
        command: Some(command),
        args: vec![],
        cwd: None,
        env: std::collections::HashMap::new(),
        timeout_ms: 30000,
        max_retries: 1,
        priority: TaskPriority::Normal,
    };

    let result = app_state.process_runner.execute(&definition).await
        .map_err(|e| e.to_string())?;

    info!(exit_code = result.exit_code, duration_ms = result.duration_ms, "Command executed");
    Ok(CommandResult {
        exit_code: result.exit_code,
        stdout: result.stdout,
        stderr: result.stderr,
        duration_ms: result.duration_ms,
    })
}

#[tauri::command]
pub async fn cancel_task(
    state: tauri::State<'_, Arc<Mutex<AppState>>>,
    task_id: String,
) -> Result<bool, String> {
    let task_uuid = Uuid::parse_str(&task_id).map_err(|e| format!("Invalid task ID: {}", e))?;
    let app_state = state.lock().await;
    let cancelled = app_state.supervisor.cancel_task(task_uuid).await;
    Ok(cancelled)
}

#[tauri::command]
pub async fn create_terminal(
    state: tauri::State<'_, Arc<Mutex<AppState>>>,
) -> Result<String, String> {
    let app_state = state.lock().await;
    let session_id = app_state.terminal_manager.create_session(None, 120, 40).await?;
    Ok(session_id.to_string())
}

#[tauri::command]
pub async fn terminal_write(
    state: tauri::State<'_, Arc<Mutex<AppState>>>,
    session_id: String,
    data: String,
) -> Result<(), String> {
    let session_uuid = Uuid::parse_str(&session_id).map_err(|e| format!("Invalid session ID: {}", e))?;
    let app_state = state.lock().await;
    app_state.terminal_manager.write_input(session_uuid, &data).await?;
    Ok(())
}

#[tauri::command]
pub async fn terminal_resize(
    state: tauri::State<'_, Arc<Mutex<AppState>>>,
    session_id: String,
    columns: u16,
    rows: u16,
) -> Result<(), String> {
    let session_uuid = Uuid::parse_str(&session_id).map_err(|e| format!("Invalid session ID: {}", e))?;
    let app_state = state.lock().await;
    app_state.terminal_manager.resize_session(session_uuid, columns, rows).await?;
    Ok(())
}

#[tauri::command]
pub async fn close_terminal(
    state: tauri::State<'_, Arc<Mutex<AppState>>>,
    session_id: String,
) -> Result<bool, String> {
    let session_uuid = Uuid::parse_str(&session_id).map_err(|e| format!("Invalid session ID: {}", e))?;
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

#[tauri::command]
pub async fn read_directory(
    path: String,
) -> Result<String, String> {
    let dir_path = Path::new(&path);
    if !dir_path.exists() {
        return Err(format!("Path does not exist: {}", path));
    }
    if !dir_path.is_dir() {
        return Err(format!("Path is not a directory: {}", path));
    }

    let mut entries: Vec<FileEntry> = Vec::new();

    let read_dir = std::fs::read_dir(dir_path)
        .map_err(|e| format!("Failed to read directory: {}", e))?;

    for entry in read_dir.flatten() {
        let entry_path = entry.path();
        let file_name = entry.file_name().to_string_lossy().to_string();

        if file_name.starts_with('.') {
            continue;
        }

        let is_dir = entry_path.is_dir();
        let ext = entry_path.extension()
            .and_then(|e| e.to_str())
            .unwrap_or("");

        let language = match ext {
            "ts" | "tsx" => Some("typescript"),
            "js" | "jsx" | "mjs" => Some("javascript"),
            "json" => Some("json"),
            "rs" => Some("rust"),
            "css" | "scss" | "less" => Some("css"),
            "html" => Some("html"),
            "md" => Some("markdown"),
            "py" => Some("python"),
            "go" => Some("go"),
            "toml" => Some("toml"),
            "yaml" | "yml" => Some("yaml"),
            "sh" | "bash" => Some("shell"),
            _ => None,
        }.map(|s| s.to_string());

        entries.push(FileEntry {
            id: Uuid::new_v4().to_string(),
            name: file_name,
            path: entry_path.to_string_lossy().to_string(),
            file_type: if is_dir { "directory".to_string() } else { "file".to_string() },
            language,
            children: if is_dir { Some(vec![]) } else { None },
        });
    }

    entries.sort_by(|a, b| {
        match (a.file_type.as_str(), b.file_type.as_str()) {
            ("directory", "directory") => a.name.to_lowercase().cmp(&b.name.to_lowercase()),
            ("directory", _) => std::cmp::Ordering::Less,
            (_, "directory") => std::cmp::Ordering::Greater,
            _ => a.name.to_lowercase().cmp(&b.name.to_lowercase()),
        }
    });

    serde_json::to_string(&entries).map_err(|e| format!("Failed to serialize: {}", e))
}

#[tauri::command]
pub async fn read_file(
    state: tauri::State<'_, Arc<Mutex<AppState>>>,
    path: String,
) -> Result<String, String> {
    let app_state = state.lock().await;
    app_state.sandbox.validate_path(&path)?;
    std::fs::read_to_string(&path)
        .map_err(|e| format!("Failed to read file: {}", e))
}

#[tauri::command]
pub async fn write_file(
    state: tauri::State<'_, Arc<Mutex<AppState>>>,
    path: String,
    content: String,
) -> Result<(), String> {
    let app_state = state.lock().await;
    app_state.sandbox.validate_path(&path)?;
    
    // Ensure parent directories exist
    if let Some(parent) = std::path::Path::new(&path).parent() {
        std::fs::create_dir_all(parent)
            .map_err(|e| format!("Failed to create directories: {}", e))?;
    }
    
    std::fs::write(&path, content)
        .map_err(|e| format!("Failed to write file: {}", e))
}

pub fn build_handler(builder: tauri::Builder<tauri::Wry>) -> tauri::Builder<tauri::Wry> {
    builder.invoke_handler(tauri::generate_handler![
        health,
        execute_task,
        execute_command,
        cancel_task,
        create_terminal,
        terminal_write,
        terminal_resize,
        close_terminal,
        list_terminals,
        read_directory,
        read_file,
        write_file,
    ])
}