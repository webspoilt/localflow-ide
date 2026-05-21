#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::sync::Arc;
use tokio::sync::{mpsc, Mutex};
use tracing::info;
use tauri::Manager;

use localflow_ide::events::RuntimeEvent;
use localflow_ide::ipc::commands::AppState;
use localflow_ide::scheduler::TaskQueue;
use localflow_ide::supervisor::Supervisor;
use localflow_ide::engine::ProcessRunner;
use localflow_ide::pty::TerminalManager;
use localflow_ide::sandbox::Sandbox;
use localflow_ide::telemetry::logger::TelemetryLogger;
use localflow_ide::governor::ResourceGovernor;

#[tokio::main]
async fn main() {
    TelemetryLogger::init();

    let start_time = chrono::Utc::now();
    let (event_tx, mut event_rx) = mpsc::unbounded_channel::<RuntimeEvent>();

    let task_queue = Arc::new(TaskQueue::new(event_tx.clone()));
    let process_runner = Arc::new(ProcessRunner::new());
    let terminal_manager = TerminalManager::new(event_tx.clone());
    let sandbox = Sandbox::new();
    let resource_governor = Arc::new(ResourceGovernor::new(2048, 80, 4));

    let supervisor = Arc::new(Supervisor::new(
        task_queue.clone(),
        process_runner.clone(),
        resource_governor.clone(),
        event_tx.clone(),
        4,
        3,
    ));

    let supervisor_dispatch = supervisor.clone();
    tokio::spawn(async move {
        supervisor_dispatch.run().await;
    });

    let _ = event_tx.send(RuntimeEvent::RuntimeStarted);
    let _model_router = Arc::new(localflow_ide::model::ModelRouter::new());

    let execution_graph = Arc::new(Mutex::new(localflow_ide::brain::execution_graph::ExecutionGraph::new()));
    let architecture_graph = Arc::new(Mutex::new(localflow_ide::brain::architecture_graph::ArchitectureGraph::new()));
    let failure_analysis = Arc::new(Mutex::new(localflow_ide::brain::failure_analysis::FailureAnalysisEngine::new()));
    let recovery = Arc::new(Mutex::new(localflow_ide::brain::recovery::RecoveryEngine::new()));
    let memory_graph = Arc::new(Mutex::new(localflow_ide::memory::memory_graph::MemoryGraph::new()));
    let cost_engine = Arc::new(localflow_ide::telemetry::cost_engine::CostEngine::new());
    let timeline_engine = Arc::new(Mutex::new(localflow_ide::telemetry::timeline_engine::TimelineEngine::new()));
    let evidence_engine = Arc::new(Mutex::new(localflow_ide::verification::evidence_engine::EvidenceEngine::new()));
    let adaptive_orchestrator = Arc::new(localflow_ide::model::adaptive_orchestrator::AdaptiveOrchestrator::new());
    let consensus_engine = Arc::new(localflow_ide::model::consensus::ConsensusEngine::new());
    let explainability = Arc::new(Mutex::new(localflow_ide::brain::explainability::ExplainabilityEngine::new()));
    let security_graph = Arc::new(Mutex::new(localflow_ide::sandbox::security_graph::SecurityGraph::new()));
    let sandbox_v2 = Arc::new(Mutex::new(localflow_ide::sandbox::sandbox_v2::SandboxV2::new()));
    let health_engine = Arc::new(localflow_ide::inspector::health_engine::HealthEngine::new());

    let app_state_arc = Arc::new(Mutex::new(AppState {
        task_queue: task_queue.as_ref().clone(),
        supervisor: supervisor.clone(),
        process_runner: process_runner.as_ref().clone(),
        terminal_manager,
        sandbox,
        resource_governor,
        start_time,
        execution_graph,
        architecture_graph,
        failure_analysis,
        recovery,
        memory_graph,
        cost_engine,
        timeline_engine,
        evidence_engine,
        adaptive_orchestrator,
        consensus_engine,
        explainability,
        security_graph,
        sandbox_v2,
        health_engine,
    }));

    let builder = tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .setup(move |app| {
            app.manage(app_state_arc);
            
            let app_handle = app.handle().clone();
            tokio::spawn(async move {
                use tauri::Emitter;
                use localflow_ide::events::RuntimeEvent;
                while let Some(event) = event_rx.recv().await {
                    match &event {
                        RuntimeEvent::TerminalOutput { session_id, data, stream } => {
                            let log_payload = serde_json::json!({
                                "sessionId": session_id.to_string(),
                                "data": data,
                                "stream": stream,
                                "timestamp": chrono::Utc::now().timestamp_millis(),
                            });
                            let _ = app_handle.emit("runtime:log", log_payload);
                        }
                        _ => {
                            let _ = app_handle.emit("runtime:event", event);
                        }
                    }
                }
            });

            info!(pid = %std::process::id(), "LocalFlow IDE runtime initialized");
            Ok(())
        });

    let builder = localflow_ide::ipc::commands::build_handler(builder);

    builder
        .run(tauri::generate_context!())
        .expect("Failed to run LocalFlow IDE");
}
