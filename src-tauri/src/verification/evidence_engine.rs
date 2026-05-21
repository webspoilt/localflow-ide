use std::collections::HashMap;
use serde::{Serialize, Deserialize};
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum EvidenceType {
    BuildCheck,
    UnitTest,
    LintCheck,
    Benchmark,
    SecurityAudit,
    RuntimeValidation,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Evidence {
    pub id: String,
    pub timestamp: DateTime<Utc>,
    pub source: String, // e.g., "cargo test", "pnpm lint"
    pub evidence_type: EvidenceType,
    pub passed: bool,
    pub output_snippet: String,
    pub metrics: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VerificationReport {
    pub task_id: String,
    pub fully_resolved: bool,
    pub evidence_collected: Vec<Evidence>,
    pub missing_checks: Vec<String>,
}

pub struct EvidenceEngine {
    pub records: HashMap<String, Vec<Evidence>>, // task_id -> list of evidence
}

impl Default for EvidenceEngine {
    fn default() -> Self {
        Self::new()
    }
}

impl EvidenceEngine {
    pub fn new() -> Self {
        Self {
            records: HashMap::new(),
        }
    }

    /// Add a piece of verification evidence for a specific task
    pub fn record_evidence(&mut self, task_id: &str, source: &str, etype: EvidenceType, passed: bool, output: &str, metrics: HashMap<String, String>) -> String {
        let id = uuid::Uuid::new_v4().to_string();
        let evidence = Evidence {
            id: id.clone(),
            timestamp: Utc::now(),
            source: source.to_string(),
            evidence_type: etype,
            passed,
            output_snippet: output.chars().take(200).collect(),
            metrics,
        };

        self.records.entry(task_id.to_string()).or_insert_with(Vec::new).push(evidence);
        id
    }

    /// Evaluates if all required evidence gates are completed and passed
    pub fn verify_task_resolution(&self, task_id: &str, required_types: &[EvidenceType]) -> VerificationReport {
        let empty_vec = Vec::new();
        let evidence_list = self.records.get(task_id).unwrap_or(&empty_vec);

        let mut missing_checks = Vec::new();
        let mut fully_resolved = true;

        for req in required_types {
            // Find if there is a passed check of this type
            let has_passed = evidence_list.iter()
                .any(|e| e.evidence_type == *req && e.passed);

            if !has_passed {
                fully_resolved = false;
                let req_name = match req {
                    EvidenceType::BuildCheck => "Compilation Check",
                    EvidenceType::UnitTest => "Unit Test Suite",
                    EvidenceType::LintCheck => "Linter Verification",
                    EvidenceType::Benchmark => "Benchmark Performance Checks",
                    EvidenceType::SecurityAudit => "Security Dependency Scanner",
                    EvidenceType::RuntimeValidation => "Runtime Integration Tests",
                };
                missing_checks.push(req_name.to_string());
            }
        }

        VerificationReport {
            task_id: task_id.to_string(),
            fully_resolved: fully_resolved && !evidence_list.is_empty(),
            evidence_collected: evidence_list.clone(),
            missing_checks,
        }
    }
}
