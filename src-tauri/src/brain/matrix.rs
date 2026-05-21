use crate::brain::options::OptionStrategy;

#[derive(Clone, Debug)]
pub struct ScoredOption {
    pub option: OptionStrategy,
    pub total_score: u32,
    pub breakdown: ScoreBreakdown,
}

#[derive(Clone, Debug)]
pub struct ScoreBreakdown {
    pub performance: u32,
    pub security: u32,
    pub maintainability: u32,
    pub complexity_penalty: u32,
    pub testability: u32,
}

#[derive(Clone)]
pub struct DecisionMatrix;

impl DecisionMatrix {
    pub fn new() -> Self {
        Self
    }

    pub fn evaluate(&self, options: &[OptionStrategy]) -> Vec<ScoredOption> {
        options.iter().map(|opt| {
            let performance: u32 = match opt.complexity { 1 => 8, 2 => 6, _ => 4 };
            let security: u32 = match opt.risk { 1 => 9, 2 => 6, _ => 4 };
            let maintainability: u32 = match opt.complexity { 1 => 4, 2 => 7, _ => 9 };
            let complexity_penalty: u32 = opt.complexity * 3;
            let testability: u32 = if opt.estimated_hours < 4 { 4 } else { 8 };
            let total_score = (performance + security + maintainability + testability).saturating_sub(complexity_penalty);
            ScoredOption {
                option: opt.clone(),
                total_score,
                breakdown: ScoreBreakdown { performance, security, maintainability, complexity_penalty, testability },
            }
        }).collect()
    }
}
