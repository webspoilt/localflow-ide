use serde::{Serialize, Deserialize};
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DecisionExplanation {
    pub id: String,
    pub timestamp: DateTime<Utc>,
    pub decision_type: String, // "ModelSelection" | "ArchitectureSelection" | "AgentAssignment" | "AlternativeRejection" | "FailurePrediction"
    pub choice_made: String,
    pub alternatives_rejected: Vec<String>,
    pub rationale: String,
}

pub struct ExplainabilityEngine {
    pub log: Vec<DecisionExplanation>,
}

impl Default for ExplainabilityEngine {
    fn default() -> Self {
        Self::new()
    }
}

impl ExplainabilityEngine {
    pub fn new() -> Self {
        // Initialize with some seed explanations to demonstrate functionality
        let now = Utc::now();
        let log = vec![
            DecisionExplanation {
                id: uuid::Uuid::new_v4().to_string(),
                timestamp: now - chrono::Duration::minutes(15),
                decision_type: "ModelSelection".to_string(),
                choice_made: "ollama:llama3:3b".to_string(),
                alternatives_rejected: vec!["gpt-4o".to_string(), "claude-3-5-sonnet".to_string()],
                rationale: "Selected local 3B model because the request was a simple lint format fix. Local compute has zero token cost and sub-second execution latency, whereas cloud APIs would be slow and costly.".to_string(),
            },
            DecisionExplanation {
                id: uuid::Uuid::new_v4().to_string(),
                timestamp: now - chrono::Duration::minutes(5),
                decision_type: "ArchitectureSelection".to_string(),
                choice_made: "Modular Subsystem Registration in lib.rs".to_string(),
                alternatives_rejected: vec!["Monolithic single file main.rs integration".to_string()],
                rationale: "Selected modular submodules to enforce encapsulation. This reduces risk of build/compile blocks when parallel developers make changes to distinct cognitive engine layers.".to_string(),
            },
        ];

        Self { log }
    }

    /// Add a new decision explanation
    pub fn record_decision(&mut self, dtype: &str, choice: &str, rejected: Vec<String>, rationale: &str) -> String {
        let id = uuid::Uuid::new_v4().to_string();
        let explanation = DecisionExplanation {
            id: id.clone(),
            timestamp: Utc::now(),
            decision_type: dtype.to_string(),
            choice_made: choice.to_string(),
            alternatives_rejected: rejected,
            rationale: rationale.to_string(),
        };
        self.log.push(explanation);
        id
    }

    /// Get all recorded decisions
    pub fn get_decisions(&self) -> Vec<DecisionExplanation> {
        self.log.clone()
    }
}
