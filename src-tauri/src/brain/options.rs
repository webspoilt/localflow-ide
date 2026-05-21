use crate::brain::context::ContextSnapshot;
use crate::brain::goal::ParsedGoal;

#[derive(Clone, Debug)]
pub struct OptionStrategy {
    pub name: String,
    pub description: String,
    pub complexity: u32,
    pub estimated_hours: u32,
    pub risk: u32,
}

pub struct OptionGenerator;

impl OptionGenerator {
    pub fn new() -> Self {
        Self
    }

    pub fn generate(&self, _goal: &ParsedGoal, _ctx: &ContextSnapshot) -> Vec<OptionStrategy> {
        vec![
            OptionStrategy {
                name: "Minimal".into(),
                description: "Fastest implementation with minimal changes".into(),
                complexity: 1,
                estimated_hours: 2,
                risk: 2,
            },
            OptionStrategy {
                name: "Balanced".into(),
                description: "Well-structured implementation with tests".into(),
                complexity: 2,
                estimated_hours: 5,
                risk: 1,
            },
            OptionStrategy {
                name: "Complete".into(),
                description: "Full implementation with docs, tests, and optimization".into(),
                complexity: 3,
                estimated_hours: 8,
                risk: 1,
            },
        ]
    }
}
