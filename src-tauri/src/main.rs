#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod core;
mod events;
mod git;
mod ipc;
mod llm;
mod process;
mod sandbox;
mod storage;
mod telemetry;

use std::sync::Arc;
use tokio::sync::{mpsc, Mutex};
use tracing::info;
use tauri::Manager;

use crate::core::{TaskQueue, Supervisor};
use crate::events::RuntimeEvent;
use crate::ipc::commands::AppState;
use crate::process::{ProcessRunner, TerminalManager};
use crate::sandbox::Sandbox;
use crate::telemetry::logger::TelemetryLogger;

#[tokio::main]
async fn main() {
    TelemetryLogger::init();

    let start_time = chrono::Utc::now();
    let (event_tx, mut _event_rx) = mpsc::unbounded_channel::<RuntimeEvent>();

    let task_queue = Arc::new(TaskQueue::new(event_tx.clone()));
    let process_runner = Arc::new(ProcessRunner::new());
    let terminal_manager = TerminalManager::new(event_tx.clone());
    let sandbox = Sandbox::new();

    let _supervisor = Arc::new(Supervisor::new(
        task_queue.clone(),
        process_runner.clone(),
        event_tx.clone(),
        4,
        3,
    ));

    let _ = event_tx.send(RuntimeEvent::RuntimeStarted);

    let app_state_arc = Arc::new(Mutex::new(AppState {
        task_queue: task_queue.as_ref().clone(),
        process_runner: process_runner.as_ref().clone(),
        terminal_manager,
        sandbox,
        start_time,
    }));

    let builder = tauri::Builder::default()
        .setup(move |app| {
            app.manage(app_state_arc);
            app.manage(llm::LocalLlmEngine::default());
            info!(pid = %std::process::id(), "Zynta Studio runtime initialized");
            Ok(())
        });

    let builder = crate::ipc::commands::build_handler(builder);

    builder
        .run(tauri::generate_context!())
        .expect("Failed to run Zynta Studio");
}
