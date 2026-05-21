use tracing::info;

pub struct ParsedGoal {
    pub raw: String,
    pub intent: String,
    pub scope: GoalScope,
}

pub enum GoalScope {
    Feature,
    Refactor,
    Bugfix,
    Performance,
    Security,
    Unknown,
}

pub struct GoalParser;

impl GoalParser {
    pub fn new() -> Self {
        Self
    }

    pub fn parse(&self, raw: &str) -> ParsedGoal {
        info!(raw, "GoalParser analyzing");
        let lower = raw.to_lowercase();
        let scope = if lower.contains("fix") || lower.contains("bug") {
            GoalScope::Bugfix
        } else if lower.contains("refactor") || lower.contains("clean") {
            GoalScope::Refactor
        } else if lower.contains("perf") || lower.contains("fast") {
            GoalScope::Performance
        } else if lower.contains("secur") || lower.contains("auth") {
            GoalScope::Security
        } else if lower.contains("add") || lower.contains("feature") || lower.contains("implement") {
            GoalScope::Feature
        } else {
            GoalScope::Unknown
        };
        ParsedGoal { raw: raw.to_string(), intent: raw.to_string(), scope }
    }
}
