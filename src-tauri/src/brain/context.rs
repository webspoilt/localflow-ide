use tracing::info;

pub struct ContextSnapshot {
    pub languages: Vec<String>,
    pub frameworks: Vec<String>,
    pub total_files: usize,
    pub has_tests: bool,
    pub has_ci: bool,
    pub dependency_count: usize,
}

pub struct ContextCollector;

impl ContextCollector {
    pub fn new() -> Self {
        Self
    }

    pub async fn collect(&self, _goal: &str) -> ContextSnapshot {
        info!("ContextCollector analyzing repository");
        ContextSnapshot {
            languages: vec!["Rust".into(), "TypeScript".into()],
            frameworks: vec!["Tauri".into(), "React".into()],
            total_files: 0,
            has_tests: false,
            has_ci: true,
            dependency_count: 0,
        }
    }
}
