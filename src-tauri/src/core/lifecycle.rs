use std::sync::Arc;
use tokio::sync::{mpsc, watch};
use tracing::{info, warn};

use super::supervisor::Supervisor;
use crate::events::RuntimeEvent;

pub enum LifecycleCommand {
    Shutdown(String),
    Restart,
    Pause,
    Resume,
}

pub struct LifecycleManager {
    supervisor: Arc<Supervisor>,
    event_sender: mpsc::UnboundedSender<RuntimeEvent>,
    shutdown_tx: watch::Sender<bool>,
    shutdown_rx: watch::Receiver<bool>,
}

impl LifecycleManager {
    pub fn new(
        supervisor: Arc<Supervisor>,
        event_sender: mpsc::UnboundedSender<RuntimeEvent>,
    ) -> Self {
        let (shutdown_tx, shutdown_rx) = watch::channel(false);
        Self {
            supervisor,
            event_sender,
            shutdown_tx,
            shutdown_rx,
        }
    }

    pub fn shutdown_receiver(&self) -> watch::Receiver<bool> {
        self.shutdown_rx.clone()
    }

    pub async fn handle_command(&self, command: LifecycleCommand) {
        match command {
            LifecycleCommand::Shutdown(reason) => {
                info!(reason = %reason, "Runtime shutting down");
                let _ = self
                    .event_sender
                    .send(RuntimeEvent::RuntimeShutdown { reason: reason.clone() });
                let _ = self.shutdown_tx.send(true);

                let active = self.supervisor.active_count().await;
                if active > 0 {
                    warn!(active_tasks = active, "Waiting for active tasks");
                    tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
                }
            }
            LifecycleCommand::Restart => {
                info!("Runtime restart requested");
            }
            LifecycleCommand::Pause => {
                info!("Runtime paused");
            }
            LifecycleCommand::Resume => {
                info!("Runtime resumed");
            }
        }
    }
}
