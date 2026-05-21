use crate::brain::context::ContextSnapshot;
use crate::brain::goal::ParsedGoal;

pub struct QuestionGenerator;

impl QuestionGenerator {
    pub fn new() -> Self {
        Self
    }

    pub fn generate(&self, _goal: &ParsedGoal, _ctx: &ContextSnapshot) -> Vec<String> {
        vec![
            "What is the target scope? (feature/refactor/bugfix)".into(),
            "What are the acceptance criteria?".into(),
            "Are there existing test patterns to follow?".into(),
        ]
    }
}
