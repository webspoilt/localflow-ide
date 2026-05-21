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
use crate::governor::ResourceGovernor;

// Import new cognitive systems
use crate::brain::execution_graph::ExecutionGraph;
use crate::brain::architecture_graph::ArchitectureGraph;
use crate::brain::failure_analysis::FailureAnalysisEngine;
use crate::brain::recovery::RecoveryEngine;
use crate::memory::memory_graph::MemoryGraph;
use crate::telemetry::cost_engine::CostEngine;
use crate::telemetry::timeline_engine::TimelineEngine;
use crate::verification::evidence_engine::EvidenceEngine;
use crate::model::adaptive_orchestrator::AdaptiveOrchestrator;
use crate::model::consensus::ConsensusEngine;
use crate::brain::explainability::ExplainabilityEngine;
use crate::sandbox::security_graph::SecurityGraph;
use crate::sandbox::sandbox_v2::SandboxV2;
use crate::inspector::health_engine::HealthEngine;

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

pub struct AppState {
    pub task_queue: TaskQueue,
    pub supervisor: Arc<Supervisor>,
    pub process_runner: ProcessRunner,
    pub terminal_manager: TerminalManager,
    pub sandbox: Sandbox,
    pub resource_governor: Arc<ResourceGovernor>,
    pub start_time: chrono::DateTime<chrono::Utc>,
    
    // Cognitive Engines
    pub execution_graph: Arc<Mutex<ExecutionGraph>>,
    pub architecture_graph: Arc<Mutex<ArchitectureGraph>>,
    pub failure_analysis: Arc<Mutex<FailureAnalysisEngine>>,
    pub recovery: Arc<Mutex<RecoveryEngine>>,
    pub memory_graph: Arc<Mutex<MemoryGraph>>,
    pub cost_engine: Arc<CostEngine>,
    pub timeline_engine: Arc<Mutex<TimelineEngine>>,
    pub evidence_engine: Arc<Mutex<EvidenceEngine>>,
    pub adaptive_orchestrator: Arc<AdaptiveOrchestrator>,
    pub consensus_engine: Arc<ConsensusEngine>,
    pub explainability: Arc<Mutex<ExplainabilityEngine>>,
    pub security_graph: Arc<Mutex<SecurityGraph>>,
    pub sandbox_v2: Arc<Mutex<SandboxV2>>,
    pub health_engine: Arc<HealthEngine>,
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

#[tauri::command]
pub async fn scan_architecture_graph(
    state: tauri::State<'_, Arc<Mutex<AppState>>>,
    base_path: String,
) -> Result<(), String> {
    let app_state = state.lock().await;
    let mut arch = app_state.architecture_graph.lock().await;
    arch.scan_repository(&base_path)
}

#[tauri::command]
pub async fn query_architecture_graph(
    state: tauri::State<'_, Arc<Mutex<AppState>>>,
    target_id: String,
) -> Result<serde_json::Value, String> {
    let app_state = state.lock().await;
    let arch = app_state.architecture_graph.lock().await;
    let impacted = arch.query_impacted_by(&target_id);
    let deps = arch.query_dependencies_of(&target_id);
    Ok(serde_json::json!({
        "target_id": target_id,
        "impacted_files": impacted,
        "dependencies": deps,
    }))
}

#[tauri::command]
pub async fn get_speculative_predictions(
    state: tauri::State<'_, Arc<Mutex<AppState>>>,
    complexity: u32,
    risk: u32,
    estimated_hours: u32,
) -> Result<serde_json::Value, String> {
    let app_state = state.lock().await;
    let opt = crate::brain::options::OptionStrategy {
        name: "Speculative Option".to_string(),
        description: "Evaluated plan path".to_string(),
        complexity,
        estimated_hours,
        risk,
    };
    let scored = crate::brain::matrix::ScoredOption {
        option: opt.clone(),
        total_score: 80,
        breakdown: crate::brain::matrix::ScoreBreakdown {
            performance: 80,
            security: 80,
            maintainability: 80,
            complexity_penalty: 0,
            testability: 80,
        },
    };
    let _sim_result = app_state.task_queue.clone(); // unused, just using strategy simulator directly
    let sim = crate::brain::simulator::StrategySimulator::new();
    let result = sim.simulate(&[opt], &[scored]);
    
    serde_json::to_value(&result).map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn get_repository_health(
    state: tauri::State<'_, Arc<Mutex<AppState>>>,
    base_path: String,
) -> Result<serde_json::Value, String> {
    let app_state = state.lock().await;
    let health = app_state.health_engine.calculate_health(&base_path);
    serde_json::to_value(&health).map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn get_health_trend(
    state: tauri::State<'_, Arc<Mutex<AppState>>>,
) -> Result<serde_json::Value, String> {
    let app_state = state.lock().await;
    let trend = app_state.health_engine.get_health_trend();
    serde_json::to_value(&trend).map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn get_timeline_predictions(
    state: tauri::State<'_, Arc<Mutex<AppState>>>,
) -> Result<serde_json::Value, String> {
    let app_state = state.lock().await;
    let timeline = app_state.timeline_engine.lock().await;
    let pred = timeline.predict_evolution();
    serde_json::to_value(&pred).map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn get_explainability_decisions(
    state: tauri::State<'_, Arc<Mutex<AppState>>>,
) -> Result<serde_json::Value, String> {
    let app_state = state.lock().await;
    let exp = app_state.explainability.lock().await;
    serde_json::to_value(&exp.get_decisions()).map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn add_explainability_decision(
    state: tauri::State<'_, Arc<Mutex<AppState>>>,
    decision_type: String,
    choice_made: String,
    alternatives_rejected: Vec<String>,
    rationale: String,
) -> Result<String, String> {
    let app_state = state.lock().await;
    let mut exp = app_state.explainability.lock().await;
    let id = exp.record_decision(&decision_type, &choice_made, alternatives_rejected, &rationale);
    Ok(id)
}

#[tauri::command]
pub async fn get_cost_summary(
    state: tauri::State<'_, Arc<Mutex<AppState>>>,
) -> Result<serde_json::Value, String> {
    let app_state = state.lock().await;
    let (cost, input, output, energy) = app_state.cost_engine.get_aggregates().await;
    let resource = app_state.cost_engine.resource_record.lock().await;
    Ok(serde_json::json!({
        "total_cost_usd": cost,
        "input_tokens": input,
        "output_tokens": output,
        "energy_wh": energy,
        "cpu_utilization": resource.cpu_utilization,
        "gpu_utilization": resource.gpu_utilization,
        "ram_mb": resource.ram_mb,
    }))
}

#[tauri::command]
pub async fn update_resource_costs(
    state: tauri::State<'_, Arc<Mutex<AppState>>>,
    cpu: f64,
    gpu: f64,
    ram: u64,
    active_duration_sec: f64,
) -> Result<serde_json::Value, String> {
    let app_state = state.lock().await;
    let updated = app_state.cost_engine.update_resource_metrics(cpu, gpu, ram, active_duration_sec).await;
    serde_json::to_value(&updated).map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn get_security_incidents(
    state: tauri::State<'_, Arc<Mutex<AppState>>>,
) -> Result<serde_json::Value, String> {
    let app_state = state.lock().await;
    let sec = app_state.security_graph.lock().await;
    serde_json::to_value(&sec.incidents).map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn validate_sandbox_write(
    state: tauri::State<'_, Arc<Mutex<AppState>>>,
    path: String,
) -> Result<String, String> {
    let app_state = state.lock().await;
    let mut sec = app_state.security_graph.lock().await;
    let permission = sec.validate_file_write(&path);
    let res = match permission {
        crate::sandbox::security_graph::Permission::Allow => "Allow",
        crate::sandbox::security_graph::Permission::Deny => "Deny",
        crate::sandbox::security_graph::Permission::AuditRequired => "AuditRequired",
    };
    Ok(res.to_string())
}

#[tauri::command]
pub async fn check_destructive_command(
    state: tauri::State<'_, Arc<Mutex<AppState>>>,
    command: String,
) -> Result<bool, String> {
    let app_state = state.lock().await;
    let mut sec = app_state.security_graph.lock().await;
    let is_destructive_str = sec.validate_command(&command);
    let sandbox = app_state.sandbox_v2.lock().await;
    let is_destructive = sandbox.is_destructive_action(&command) || is_destructive_str.is_err();
    Ok(is_destructive)
}

#[tauri::command]
pub async fn virtual_read_file(
    state: tauri::State<'_, Arc<Mutex<AppState>>>,
    path: String,
) -> Result<String, String> {
    let app_state = state.lock().await;
    let sandbox = app_state.sandbox_v2.lock().await;
    sandbox.virtual_read(&path)
}

#[tauri::command]
pub async fn virtual_write_file(
    state: tauri::State<'_, Arc<Mutex<AppState>>>,
    path: String,
    content: String,
) -> Result<(), String> {
    let app_state = state.lock().await;
    let mut sandbox = app_state.sandbox_v2.lock().await;
    sandbox.virtual_write(&path, &content);
    
    // Also scan for potential secret leaks when virtual write occurs
    let mut sec = app_state.security_graph.lock().await;
    sec.scan_content_for_secrets(&content, &path);
    
    Ok(())
}

#[tauri::command]
pub async fn commit_virtual_changes(
    state: tauri::State<'_, Arc<Mutex<AppState>>>,
) -> Result<(), String> {
    let app_state = state.lock().await;
    let mut sandbox = app_state.sandbox_v2.lock().await;
    sandbox.commit_virtual_changes()
}

#[tauri::command]
pub async fn get_execution_graph(
    state: tauri::State<'_, Arc<Mutex<AppState>>>,
) -> Result<serde_json::Value, String> {
    let app_state = state.lock().await;
    let graph = app_state.execution_graph.lock().await;
    Ok(serde_json::json!({
        "nodes": graph.nodes,
        "edges": graph.edges,
        "history": graph.history,
    }))
}

#[tauri::command]
pub async fn add_simulated_node(
    state: tauri::State<'_, Arc<Mutex<AppState>>>,
    name: String,
    node_type_str: String,
    triggered_by: String,
    reason: String,
    input_hash: String,
) -> Result<String, String> {
    let app_state = state.lock().await;
    let mut graph = app_state.execution_graph.lock().await;
    use crate::brain::execution_graph::NodeType;
    let node_type = match node_type_str.as_str() {
        "TaskNode" => NodeType::TaskNode,
        "CodeNode" => NodeType::CodeNode,
        "BuildNode" => NodeType::BuildNode,
        "AgentNode" => NodeType::AgentNode,
        "VerificationNode" => NodeType::VerificationNode,
        "DecisionNode" => NodeType::DecisionNode,
        _ => NodeType::DependencyNode,
    };
    let id = graph.add_node(&name, node_type, &triggered_by, &reason, &input_hash);
    
    // Add simulated edge if there are previous nodes
    if graph.nodes.len() > 1 {
        let prev_id = graph.nodes[graph.nodes.len() - 2].id;
        let _ = graph.add_edge(prev_id, id);
    }
    
    Ok(id.to_string())
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
        scan_architecture_graph,
        query_architecture_graph,
        get_speculative_predictions,
        get_repository_health,
        get_health_trend,
        get_timeline_predictions,
        get_explainability_decisions,
        add_explainability_decision,
        get_cost_summary,
        update_resource_costs,
        get_security_incidents,
        validate_sandbox_write,
        check_destructive_command,
        virtual_read_file,
        virtual_write_file,
        commit_virtual_changes,
        get_execution_graph,
        add_simulated_node,
    ])
}