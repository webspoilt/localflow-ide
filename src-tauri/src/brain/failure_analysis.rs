use std::collections::HashMap;
use chrono::{DateTime, Utc};
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FailureEvent {
    pub id: String,
    pub timestamp: DateTime<Utc>,
    pub category: String, // "build" | "lint" | "runtime" | "dependency" | "test" | "memory" | "gpu"
    pub message: String,
    pub affected_files: Vec<String>,
    pub metadata: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FailurePrediction {
    pub likely_next_failure: String,
    pub failure_probability: f64,
    pub category_risk_scores: HashMap<String, f64>,
    pub recommended_action: String,
}

pub struct FailureAnalysisEngine {
    pub events: Vec<FailureEvent>,
}

impl Default for FailureAnalysisEngine {
    fn default() -> Self {
        Self::new()
    }
}

impl FailureAnalysisEngine {
    pub fn new() -> Self {
        Self {
            events: Vec::new(),
        }
    }

    pub fn record_failure(&mut self, category: &str, message: &str, affected_files: Vec<String>, metadata: HashMap<String, String>) -> String {
        let id = uuid::Uuid::new_v4().to_string();
        let event = FailureEvent {
            id: id.clone(),
            timestamp: Utc::now(),
            category: category.to_string(),
            message: message.to_string(),
            affected_files,
            metadata,
        };
        self.events.push(event);
        id
    }

    /// Identifies if a failure is repeating based on message/category similarity.
    pub fn classify_repeat_failures(&self) -> Vec<(String, usize)> {
        let mut counts = HashMap::new();
        for event in &self.events {
            // Count by category + key message snippets
            let key = format!("{}: {}", event.category, event.message.chars().take(40).collect::<String>());
            *counts.entry(key).or_insert(0) += 1;
        }
        let mut sorted: Vec<(String, usize)> = counts.into_iter().collect();
        sorted.sort_by(|a, b| b.1.cmp(&a.1));
        sorted
    }

    /// Predicts failure probability and likely next failure.
    pub fn predict_likely_failure(&self, current_modified_files: &[String], memory_usage_mb: u64, gpu_utilization: f64) -> FailurePrediction {
        let mut category_risk = HashMap::new();
        category_risk.insert("build".to_string(), 0.05);
        category_risk.insert("lint".to_string(), 0.05);
        category_risk.insert("runtime".to_string(), 0.05);
        category_risk.insert("dependency".to_string(), 0.01);
        category_risk.insert("test".to_string(), 0.05);
        category_risk.insert("memory".to_string(), 0.02);
        category_risk.insert("gpu".to_string(), 0.02);

        // Increase risk based on historical events
        for event in &self.events {
            if let Some(risk) = category_risk.get_mut(&event.category) {
                *risk += 0.10; // add 10% risk per historical failure
            }

            // If event affected files that are currently modified
            for affected in &event.affected_files {
                if current_modified_files.contains(affected) {
                    if let Some(risk) = category_risk.get_mut(&event.category) {
                        *risk += 0.15; // add extra 15% risk for modified files
                    }
                }
            }
        }

        // Apply resource utilization scaling
        if memory_usage_mb > 16000 { // >16GB RAM pressure
            if let Some(risk) = category_risk.get_mut("memory") {
                *risk += 0.40;
            }
        }
        if gpu_utilization > 90.0 { // GPU pressure
            if let Some(risk) = category_risk.get_mut("gpu") {
                *risk += 0.35;
            }
        }

        // Clamp values to [0.0, 0.99]
        for val in category_risk.values_mut() {
            if *val > 0.99 {
                *val = 0.99;
            }
        }

        // Determine likely next failure
        let default_cat = "none".to_string();
        let (likely_cat, &max_risk) = category_risk.iter()
            .max_by(|a, b| a.1.partial_cmp(b.1).unwrap_or(std::cmp::Ordering::Equal))
            .unwrap_or((&default_cat, &0.0));

        let likely_next_failure = match likely_cat.as_str() {
            "build" => "Compilation failure in modified modules".to_string(),
            "lint" => "Format or type check failure in desktop frontend".to_string(),
            "runtime" => "Tauri thread pool saturation or IPC panic".to_string(),
            "dependency" => "Version collision in cargo crates or pnpm node_modules".to_string(),
            "test" => "Regression test failure in file workspace path".to_string(),
            "memory" => "Out of Memory (OOM) crash in node runner".to_string(),
            "gpu" => "Model context overflow or CUDA allocation exhaustion".to_string(),
            _ => "No high risk failures expected".to_string(),
        };

        let recommended_action = match likely_cat.as_str() {
            "build" => "Run `cargo check` and fix any compiler syntax errors.".to_string(),
            "lint" => "Run code formatting `pnpm lint` before pushing changes.".to_string(),
            "runtime" => "Verify IPC message payloads match Rust function signatures.".to_string(),
            "dependency" => "Audit node_modules or cargo.lock files.".to_string(),
            "test" => "Run automated unit test suites for modified file components.".to_string(),
            "memory" => "Collect garbage or clean up terminated workspace processes.".to_string(),
            "gpu" => "Reduce concurrent model context sizes or shift tasks to local CPUs.".to_string(),
            _ => "Maintain code reviews and monitor runtime telemetry log streams.".to_string(),
        };

        FailurePrediction {
            likely_next_failure,
            failure_probability: max_risk,
            category_risk_scores: category_risk,
            recommended_action,
        }
    }
}
