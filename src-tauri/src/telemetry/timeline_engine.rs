use serde::{Serialize, Deserialize};
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimelineSnapshot {
    pub timestamp: DateTime<Utc>,
    pub total_files: usize,
    pub dependency_count: usize,
    pub average_build_time_ms: u64,
    pub total_lines_of_code: usize,
    pub todo_count: usize,
    pub code_issues_count: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimelinePredictions {
    pub estimated_build_time_30d_ms: u64,
    pub estimated_todo_count_30d: usize,
    pub next_predicted_bottleneck: String,
    pub maintenance_risk_score: f64, // 0.0 to 1.0
    pub upgrade_risks: Vec<String>,
}

pub struct TimelineEngine {
    pub history: Vec<TimelineSnapshot>,
}

impl Default for TimelineEngine {
    fn default() -> Self {
        Self::new()
    }
}

impl TimelineEngine {
    pub fn new() -> Self {
        // Initialize with some seed historical events to establish baseline trends
        let now = Utc::now();
        let history = vec![
            TimelineSnapshot {
                timestamp: now - chrono::Duration::days(14),
                total_files: 80,
                dependency_count: 45,
                average_build_time_ms: 12000,
                total_lines_of_code: 15000,
                todo_count: 8,
                code_issues_count: 12,
            },
            TimelineSnapshot {
                timestamp: now - chrono::Duration::days(7),
                total_files: 88,
                dependency_count: 48,
                average_build_time_ms: 14500,
                total_lines_of_code: 16500,
                todo_count: 11,
                code_issues_count: 15,
            },
            TimelineSnapshot {
                timestamp: now,
                total_files: 95,
                dependency_count: 52,
                average_build_time_ms: 16200,
                total_lines_of_code: 18000,
                todo_count: 14,
                code_issues_count: 19,
            },
        ];

        Self { history }
    }

    pub fn record_snapshot(&mut self, files: usize, deps: usize, build_ms: u64, loc: usize, todos: usize, issues: usize) {
        self.history.push(TimelineSnapshot {
            timestamp: Utc::now(),
            total_files: files,
            dependency_count: deps,
            average_build_time_ms: build_ms,
            total_lines_of_code: loc,
            todo_count: todos,
            code_issues_count: issues,
        });
    }

    /// Predicts future system evolution metrics using simple linear extrapolation
    pub fn predict_evolution(&self) -> TimelinePredictions {
        if self.history.len() < 2 {
            return TimelinePredictions {
                estimated_build_time_30d_ms: 18000,
                estimated_todo_count_30d: 15,
                next_predicted_bottleneck: "Insufficient history to predict evolution".to_string(),
                maintenance_risk_score: 0.20,
                upgrade_risks: vec![],
            };
        }

        let first = &self.history[0];
        let last = &self.history[self.history.len() - 1];

        let days = (last.timestamp - first.timestamp).num_days() as f64;
        let days_factor = if days > 0.0 { 30.0 / days } else { 1.0 };

        // Delta calculations
        let build_delta = (last.average_build_time_ms as f64 - first.average_build_time_ms as f64) * days_factor;
        let todo_delta = (last.todo_count as f64 - first.todo_count as f64) * days_factor;
        let dep_delta = (last.dependency_count as f64 - first.dependency_count as f64) * days_factor;

        let estimated_build_time_30d_ms = ((last.average_build_time_ms as f64 + build_delta).max(0.0)) as u64;
        let estimated_todo_count_30d = ((last.todo_count as f64 + todo_delta).max(0.0)) as usize;

        // Formulate bottleneck notices
        let next_predicted_bottleneck = if estimated_build_time_30d_ms > 30000 {
            "Build time predicted to exceed 30 seconds, saturation risk in rust build queue".to_string()
        } else if todo_delta > 10.0 {
            "Technical debt expansion (TODO count increasing rapidly)".to_string()
        } else {
            "No immediate resource/architectural bottlenecks forecasted".to_string()
        };

        // Calculate maintenance risk (0.0 to 1.0)
        let mut maintenance_risk_score = 0.15;
        if last.code_issues_count > 25 {
            maintenance_risk_score += 0.25;
        }
        if build_delta > 5000.0 {
            maintenance_risk_score += 0.20;
        }
        if last.todo_count > 20 {
            maintenance_risk_score += 0.15;
        }
        if maintenance_risk_score > 0.95 {
            maintenance_risk_score = 0.95;
        }

        // Detect dependency version drifts / security warnings
        let mut upgrade_risks = Vec::new();
        if dep_delta > 5.0 {
            upgrade_risks.push("Rapid growth in external dependencies introduces potential dependency hell or supply chain vulnerabilities.".to_string());
        }
        if last.dependency_count > 50 {
            upgrade_risks.push("Crate tree contains high node volume. Recommended to audit Cargo.toml for duplicate crates or outdated packages.".to_string());
        }

        TimelinePredictions {
            estimated_build_time_30d_ms,
            estimated_todo_count_30d,
            next_predicted_bottleneck,
            maintenance_risk_score,
            upgrade_risks,
        }
    }
}
