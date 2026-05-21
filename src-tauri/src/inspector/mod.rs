use std::sync::Arc;
use tokio::sync::Mutex;
use tracing::info;

pub struct RuntimeInspector {
    snapshot: Arc<Mutex<InspectorSnapshot>>,
}

#[derive(Default, Clone)]
pub struct InspectorSnapshot {
    pub active_tasks: usize,
    pub memory_usage_mb: u64,
    pub uptime_seconds: u64,
}

impl RuntimeInspector {
    pub fn new() -> Self {
        info!("Runtime Inspector initialized");
        Self { snapshot: Arc::new(Mutex::new(InspectorSnapshot::default())) }
    }

    pub async fn snapshot(&self) -> InspectorSnapshot {
        self.snapshot.lock().await.clone()
    }
}