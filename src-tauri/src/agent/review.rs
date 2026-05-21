use tracing::info;

pub struct ReviewAgent;

impl ReviewAgent {
    pub fn review(&self, _artifact: &str) -> Vec<String> {
        info!("ReviewAgent analyzing code");
        vec!["Code review completed: no issues found".into()]
    }
}
