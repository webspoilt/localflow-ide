use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum TaskComplexity {
    SimpleCodeFix,
    MediumComplexity,
    ComplexRefactor,
    ArchitecturePlanning,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HardwareProfile {
    pub has_cuda: bool,
    pub vram_gb: usize,
    pub system_ram_gb: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RouteDecision {
    pub complexity: TaskComplexity,
    pub selected_model: String,
    pub is_local: bool,
    pub cost_per_1k_tokens: f64,
    pub explanation: String,
}

pub struct AdaptiveOrchestrator;

impl Default for AdaptiveOrchestrator {
    fn default() -> Self {
        Self::new()
    }
}

impl AdaptiveOrchestrator {
    pub fn new() -> Self {
        Self
    }

    /// Evaluates task requirements and system hardware to select the best model execution target
    pub fn route_task(&self, complexity: TaskComplexity, hardware: &HardwareProfile, max_cost_usd_limit: f64) -> RouteDecision {
        match complexity {
            TaskComplexity::SimpleCodeFix => {
                // If local hardware is capable, route to local 3B model (Ollama)
                if hardware.has_cuda && hardware.vram_gb >= 6 {
                    RouteDecision {
                        complexity,
                        selected_model: "ollama:llama3:3b".to_string(),
                        is_local: true,
                        cost_per_1k_tokens: 0.0,
                        explanation: "Routed to local 3B model on Ollama. VRAM (>6GB) is sufficient for real-time latency with zero API token cost.".to_string(),
                    }
                } else {
                    // Fallback to small/cheap cloud model
                    RouteDecision {
                        complexity,
                        selected_model: "gpt-3.5-turbo".to_string(),
                        is_local: false,
                        cost_per_1k_tokens: 0.0015,
                        explanation: "Routed to cheap API model. Local hardware is VRAM-constrained (<6GB), so a low-tier remote endpoint is selected to minimize cost.".to_string(),
                    }
                }
            }
            TaskComplexity::ArchitecturePlanning => {
                // Large reasoning model needed
                if hardware.has_cuda && hardware.vram_gb >= 12 {
                    RouteDecision {
                        complexity,
                        selected_model: "ollama:qwen2.5-coder:14b".to_string(),
                        is_local: true,
                        cost_per_1k_tokens: 0.0,
                        explanation: "Routed to local 14B coder model on Ollama. VRAM (>12GB) permits running high-capacity local parameter structures without API charges.".to_string(),
                    }
                } else {
                    RouteDecision {
                        complexity,
                        selected_model: "gpt-4o-mini".to_string(),
                        is_local: false,
                        cost_per_1k_tokens: 0.0006,
                        explanation: "Routed to remote mini model. Planning requires a moderate reasoning size but local VRAM (<12GB) is insufficient to run a local 14B model efficiently.".to_string(),
                    }
                }
            }
            TaskComplexity::MediumComplexity => {
                RouteDecision {
                    complexity,
                    selected_model: "gpt-4o-mini".to_string(),
                    is_local: false,
                    cost_per_1k_tokens: 0.0006,
                    explanation: "Routed to remote medium-tier model to balance reasoning capability and speed.".to_string(),
                }
            }
            TaskComplexity::ComplexRefactor => {
                // Always use best cloud API model for high complexity unless strict cost limit is hit
                if max_cost_usd_limit < 0.01 {
                    // Cost limit too low, fallback to local 14B model if possible
                    if hardware.has_cuda && hardware.vram_gb >= 12 {
                        RouteDecision {
                            complexity,
                            selected_model: "ollama:deepseek-coder:15b".to_string(),
                            is_local: true,
                            cost_per_1k_tokens: 0.0,
                            explanation: "Routed to local 15B model due to strict cost limits, avoiding API billing metrics.".to_string(),
                        }
                    } else {
                        RouteDecision {
                            complexity,
                            selected_model: "gpt-4o-mini".to_string(),
                            is_local: false,
                            cost_per_1k_tokens: 0.0006,
                            explanation: "Routed to cheaper remote model. VRAM is insufficient for local 15B models, and strict cost limits block premium API paths.".to_string(),
                        }
                    }
                } else {
                    RouteDecision {
                        complexity,
                        selected_model: "claude-3-5-sonnet".to_string(),
                        is_local: false,
                        cost_per_1k_tokens: 0.018,
                        explanation: "Routed to Claude 3.5 Sonnet. Premium reasoning layer selected to ensure safety, architectural consistency, and optimal code quality for deep refactors.".to_string(),
                    }
                }
            }
        }
    }
}
