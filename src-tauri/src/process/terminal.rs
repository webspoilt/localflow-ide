use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::{mpsc, Mutex};
use tracing::{info, warn, debug};
use uuid::Uuid;

use crate::events::RuntimeEvent;

#[derive(Debug, Clone)]
pub struct TerminalSession {
    pub id: Uuid,
    pub cwd: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub columns: u16,
    pub rows: u16,
    pub active: bool,
}

pub struct TerminalManager {
    sessions: Arc<Mutex<HashMap<Uuid, TerminalSession>>>,
    event_sender: mpsc::UnboundedSender<RuntimeEvent>,
}

impl TerminalManager {
    pub fn new(event_sender: mpsc::UnboundedSender<RuntimeEvent>) -> Self {
        Self {
            sessions: Arc::new(Mutex::new(HashMap::new())),
            event_sender,
        }
    }

    pub async fn create_session(
        &self,
        cwd: Option<String>,
        columns: u16,
        rows: u16,
    ) -> Uuid {
        let session_id = Uuid::new_v4();
        let session = TerminalSession {
            id: session_id,
            cwd: cwd.unwrap_or_else(|| std::env::current_dir()
                .map(|p| p.to_string_lossy().to_string())
                .unwrap_or_else(|_| "/".to_string())),
            created_at: chrono::Utc::now(),
            columns,
            rows,
            active: true,
        };

        self.sessions.lock().await.insert(session_id, session);
        let _ = self.event_sender.send(RuntimeEvent::TerminalCreated {
            session_id,
        });

        info!(session_id = %session_id, "Terminal session created");
        session_id
    }

    pub async fn close_session(&self, session_id: Uuid) -> bool {
        let mut sessions = self.sessions.lock().await;
        if sessions.remove(&session_id).is_some() {
            let _ = self.event_sender.send(RuntimeEvent::TerminalClosed {
                session_id,
            });
            info!(session_id = %session_id, "Terminal session closed");
            true
        } else {
            warn!(session_id = %session_id, "Terminal session not found");
            false
        }
    }

    pub async fn resize_session(
        &self,
        session_id: Uuid,
        columns: u16,
        rows: u16,
    ) -> bool {
        let mut sessions = self.sessions.lock().await;
        if let Some(session) = sessions.get_mut(&session_id) {
            session.columns = columns;
            session.rows = rows;
            debug!(session_id = %session_id, columns, rows, "Terminal resized");
            true
        } else {
            false
        }
    }

    pub async fn list_sessions(&self) -> Vec<TerminalSession> {
        self.sessions
            .lock()
            .await
            .values()
            .cloned()
            .collect()
    }
}
