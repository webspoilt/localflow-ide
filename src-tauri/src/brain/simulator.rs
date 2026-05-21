use crate::brain::matrix::ScoredOption;
use crate::brain::options::OptionStrategy;

#[derive(Clone, Debug)]
pub struct SimulationResult {
    pub predictions: Vec<StrategyPrediction>,
    pub best: OptionStrategy,
}

#[derive(Clone, Debug)]
pub struct StrategyPrediction {
    pub option_name: String,
    pub build_failures: u32,
    pub test_coverage: u32,
}

pub struct StrategySimulator;

impl StrategySimulator {
    pub fn new() -> Self {
        Self
    }

    pub fn simulate(&self, options: &[OptionStrategy], scores: &[ScoredOption]) -> SimulationResult {
        let predictions: Vec<StrategyPrediction> = options.iter().map(|opt| {
            StrategyPrediction {
                option_name: opt.name.clone(),
                build_failures: if opt.complexity > 2 { 2 } else { 0 },
                test_coverage: match opt.complexity { 1 => 30, 2 => 60, _ => 85 },
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
