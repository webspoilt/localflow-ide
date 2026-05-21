use crate::brain::matrix::ScoredOption;
use crate::brain::options::OptionStrategy;
use serde::{Serialize, Deserialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SpeculativeAssessment {
    pub build_impact_risk: f64,    // 0.0 (low) to 1.0 (high)
    pub dependency_bloat_risk: f64,
    pub memory_impact_mb: u64,
    pub security_risk_score: f64,
    pub estimated_time_ms: u64,
    pub confidence_score: f64,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct StrategyPrediction {
    pub option_name: String,
    pub build_failures: u32,
    pub test_coverage: u32,
    pub assessment: SpeculativeAssessment,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SimulationResult {
    pub predictions: Vec<StrategyPrediction>,
    pub best: OptionStrategy,
}

pub struct StrategySimulator;

impl Default for StrategySimulator {
    fn default() -> Self {
        Self::new()
    }
}

impl StrategySimulator {
    pub fn new() -> Self {
        Self
    }

    pub fn simulate(&self, options: &[OptionStrategy], scores: &[ScoredOption]) -> SimulationResult {
        let predictions: Vec<StrategyPrediction> = options.iter().map(|opt| {
            // Speculative impact calculation based on complexity and risk parameters
            let complexity_factor = opt.complexity as f64; // e.g. 1 to 5
            let risk_factor = opt.risk as f64;

            let build_impact_risk = (complexity_factor * 0.15 + risk_factor * 0.10).min(1.0);
            let dependency_bloat_risk = if complexity_factor > 3.0 { 0.50 } else { 0.10 };
            let memory_impact_mb = 256 + (complexity_factor * 128.0) as u64;
            
            // Security risk calculation
            let security_risk_score = (risk_factor * 0.18).min(1.0);
            let estimated_time_ms = (opt.estimated_hours as u64 * 3600 * 1000) / 10; // scaled time estimate

            // Confidence score calculation: 1.0 - weighted risk average
            let weighted_risk = (build_impact_risk * 0.35) 
                + (dependency_bloat_risk * 0.15) 
                + (security_risk_score * 0.50);
            let confidence_score = (1.0 - weighted_risk).max(0.0).min(1.0);

            StrategyPrediction {
                option_name: opt.name.clone(),
                build_failures: if opt.complexity > 2 { 2 } else { 0 },
                test_coverage: match opt.complexity {
                    1 => 30,
                    2 => 60,
                    _ => 85,
                },
                assessment: SpeculativeAssessment {
                    build_impact_risk,
                    dependency_bloat_risk,
                    memory_impact_mb,
                    security_risk_score,
                    estimated_time_ms,
                    confidence_score,
                },
            }
        }).collect();

        let best = scores.iter()
            .max_by_key(|s| s.total_score)
            .map(|s| s.option.clone())
            .unwrap_or_else(|| options[0].clone());

        SimulationResult { predictions, best }
    }

    pub fn best_option(&self) -> OptionStrategy {
        OptionStrategy {
            name: "Balanced".into(),
            description: "default".into(),
            complexity: 2,
            estimated_hours: 5,
            risk: 1,
        }
    }
}
