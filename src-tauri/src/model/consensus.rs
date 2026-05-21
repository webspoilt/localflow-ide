use serde::{Serialize, Deserialize};
use std::collections::HashSet;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelProposal {
    pub model_id: String,
    pub role: String, // "planner" | "reviewer" | "verifier"
    pub proposal: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConsensusResult {
    pub agreement_score: f64,
    pub conflict_detected: bool,
    pub conflicts: Vec<String>,
    pub final_consensus_output: String,
    pub requires_override: bool,
}

pub struct ConsensusEngine;

impl Default for ConsensusEngine {
    fn default() -> Self {
        Self::new()
    }
}

impl ConsensusEngine {
    pub fn new() -> Self {
        Self
    }

    /// Evaluates proposal alignment from multiple models, computing conflict lists and agreement scores
    pub fn evaluate_consensus(&self, proposals: &[ModelProposal]) -> ConsensusResult {
        if proposals.is_empty() {
            return ConsensusResult {
                agreement_score: 0.0,
                conflict_detected: false,
                conflicts: vec!["No proposals submitted".to_string()],
                final_consensus_output: String::new(),
                requires_override: true,
            };
        }

        if proposals.len() == 1 {
            return ConsensusResult {
                agreement_score: 1.0,
                conflict_detected: false,
                conflicts: vec![],
                final_consensus_output: proposals[0].proposal.clone(),
                requires_override: false,
            };
        }

        // Calculate line-level similarity between the planner and others
        let planner_proposal = proposals.iter()
            .find(|p| p.role == "planner")
            .unwrap_or(&proposals[0]);

        let planner_lines: HashSet<&str> = planner_proposal.proposal.lines().map(|l| l.trim()).collect();
        let mut similarity_sum = 0.0;
        let mut conflicts = Vec::new();

        for other in proposals {
            if other.model_id == planner_proposal.model_id {
                continue;
            }

            let other_lines: HashSet<&str> = other.proposal.lines().map(|l| l.trim()).collect();
            let intersection = planner_lines.intersection(&other_lines).count();
            let union = planner_lines.union(&other_lines).count();

            let jaccard = if union > 0 {
                intersection as f64 / union as f64
            } else {
                1.0
            };

            similarity_sum += jaccard;

            // If jaccard is low, note it as a conflict
            if jaccard < 0.60 {
                conflicts.push(format!(
                    "Model {} ({}) disagreed significantly with planner (similarity: {:.2}).",
                    other.model_id, other.role, jaccard
                ));
            }
        }

        let num_comparisons = (proposals.len() - 1) as f64;
        let agreement_score = similarity_sum / num_comparisons;

        let conflict_detected = !conflicts.is_empty();
        let requires_override = agreement_score < 0.70;

        ConsensusResult {
            agreement_score,
            conflict_detected,
            conflicts,
            final_consensus_output: planner_proposal.proposal.clone(),
            requires_override,
        }
    }
}
