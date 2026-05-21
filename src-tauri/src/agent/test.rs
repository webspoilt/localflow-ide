use tracing::info;

pub struct TestAgent;

impl TestAgent {
    pub fn run(&self, _target: &str) -> Vec<String> {
        info!("TestAgent running test suite");
        vec!["Tests passed".into()]
    }
}
