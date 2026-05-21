use std::collections::HashMap;
use std::io::{Read, Write};
use std::sync::Arc;
use tokio::sync::{mpsc, Mutex};
use tokio::task::JoinHandle;
use tracing::{info, warn, debug, error};
use uuid::Uuid;

use portable_pty::{ChildKiller, CommandBuilder, NativePtySystem, PtyPair, PtySize, PtySystem};

use crate::events::RuntimeEvent;

struct PtyProcess {
    pair: PtyPair,
    writer: Box<dyn Write + Send>,
    _child: Box<dyn ChildKiller + Send + Sync>,
    _reader_task: JoinHandle<()>,
}

#[derive(Clone)]
pub struct TerminalSession {
    pub id: Uuid,
    pub cwd: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub columns: u16,
    pub rows: u16,
    pub active: bool,
}

pub struct TerminalManager {
    sessions: Arc<Mutex<HashMap<Uuid, PtyProcess>>>,
    session_meta: Arc<Mutex<HashMap<Uuid, TerminalSession>>>,
    event_sender: mpsc::UnboundedSender<RuntimeEvent>,
}

impl TerminalManager {
    pub fn new(event_sender: mpsc::UnboundedSender<RuntimeEvent>) -> Self {
        Self {
            sessions: Arc::new(Mutex::new(HashMap::new())),
            session_meta: Arc::new(Mutex::new(HashMap::new())),
            event_sender,
        }
    }

    pub async fn create_session(
        &self,
        cwd: Option<String>,
        columns: u16,
        rows: u16,
    ) -> Result<Uuid, String> {
        let pty_system = NativePtySystem::default();

        let size = PtySize {
            rows,
            cols: columns,
            pixel_width: 0,
            pixel_height: 0,
        };

        let pair = pty_system.openpty(size).map_err(|e| format!("Failed to open PTY: {}", e))?;

        let dir = cwd.unwrap_or_else(|| {
            std::env::current_dir()
                .map(|p| p.to_string_lossy().to_string())
                .unwrap_or_else(|_| "/".to_string())
        });

        let mut cmd = if cfg!(target_os = "windows") {
            CommandBuilder::new("cmd.exe")
        } else {
            let mut c = CommandBuilder::new("bash");
            c.arg("--login");
            c
        };
        cmd.cwd(&dir);
        cmd.env("TERM", "xterm-256color");

        let child = pair.slave.spawn_command(cmd)
            .map_err(|e| format!("Failed to spawn shell: {}", e))?;

        let reader = pair.master.try_clone_reader()
            .map_err(|e| format!("Failed to clone PTY reader: {}", e))?;

        let writer = pair.master.take_writer()
            .map_err(|e| format!("Failed to get PTY writer: {}", e))?;

        let session_id = Uuid::new_v4();
        let event_sender = self.event_sender.clone();

        let reader_task = tokio::task::spawn_blocking(move || {
            let mut buf = [0u8; 4096];
            let mut reader: Box<dyn Read + Send> = reader;
            loop {
                match reader.read(&mut buf) {
                    Ok(0) => {
                        debug!(session_id = %session_id, "PTY reader EOF");
                        break;
                    }
                    Ok(n) => {
                        let data = String::from_utf8_lossy(&buf[..n]).to_string();
                        let _ = event_sender.send(RuntimeEvent::TerminalOutput {
                            session_id,
                            data,
                            stream: "stdout".to_string(),
                        });
                    }
                    Err(e) => {
                        error!(session_id = %session_id, error = %e, "PTY reader error");
                        break;
                    }
                }
            }
        });

        {
            let mut sessions = self.sessions.lock().await;
            sessions.insert(session_id, PtyProcess {
                pair,
                writer,
                _child: child,
                _reader_task: reader_task,
            });
        }

        {
            let mut meta = self.session_meta.lock().await;
            meta.insert(session_id, TerminalSession {
                id: session_id,
                cwd: dir,
                created_at: chrono::Utc::now(),
                columns,
                rows,
                active: true,
            });
        }

        let _ = self.event_sender.send(RuntimeEvent::TerminalCreated { session_id });
        info!(session_id = %session_id, "PTY terminal session created");
        Ok(session_id)
    }

    pub async fn write_input(&self, session_id: Uuid, data: &str) -> Result<(), String> {
        let mut sessions = self.sessions.lock().await;
        let process = sessions.get_mut(&session_id)
            .ok_or_else(|| format!("Session {} not found", session_id))?;

        process.writer.write_all(data.as_bytes())
            .and_then(|_| process.writer.flush())
            .map_err(|e| format!("Failed to write to PTY: {}", e))
    }

    pub async fn resize_session(
        &self,
        session_id: Uuid,
        columns: u16,
        rows: u16,
    ) -> Result<(), String> {
        let sessions = self.sessions.lock().await;
        let process = sessions.get(&session_id)
            .ok_or_else(|| format!("Session {} not found", session_id))?;

        let size = PtySize { rows, cols: columns, pixel_width: 0, pixel_height: 0 };
        process.pair.master.resize(size)
            .map_err(|e| format!("Failed to resize PTY: {}", e))?;

        if let Some(meta) = self.session_meta.lock().await.get_mut(&session_id) {
            meta.columns = columns;
            meta.rows = rows;
        }

        debug!(session_id = %session_id, columns, rows, "Terminal resized");
        Ok(())
    }

    pub async fn close_session(&self, session_id: Uuid) -> bool {
        let mut sessions = self.sessions.lock().await;
        if sessions.remove(&session_id).is_some() {
            self.session_meta.lock().await.remove(&session_id);
            let _ = self.event_sender.send(RuntimeEvent::TerminalClosed { session_id });
            info!(session_id = %session_id, "Terminal session closed");
            true
        } else {
            warn!(session_id = %session_id, "Terminal session not found");
            false
        }
    }

    pub async fn list_sessions(&self) -> Vec<TerminalSession> {
        let meta = self.session_meta.lock().await;
        meta.values().cloned().collect()
    }
}