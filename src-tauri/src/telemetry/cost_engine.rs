use serde::{Serialize, Deserialize};
use std::sync::Arc;
use tokio::sync::Mutex;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelCostRecord {
    pub model_name: String,
    pub input_tokens: u64,
    pub output_tokens: u64,
    pub latency_ms: u64,
    pub estimated_cost_usd: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceCostRecord {
    pub cpu_utilization: f64, // 0.0 to 100.0
    pub gpu_utilization: f64, // 0.0 to 100.0
    pub ram_mb: u64,
    pub energy_wh: f64, // Watt-hours
}

pub struct CostEngine {
    pub model_records: Arc<Mutex<Vec<ModelCostRecord>>>,
    pub resource_record: Arc<Mutex<ResourceCostRecord>>,
}

impl Default for CostEngine {
    fn default() -> Self {
        Self::new()
    }
}

impl CostEngine {
    pub fn new() -> Self {
        Self {
            model_records: Arc::new(Mutex::new(Vec::new())),
            resource_record: Arc::new(Mutex::new(ResourceCostRecord {
                cpu_utilization: 12.5,
                gpu_utilization: 0.0,
                ram_mb: 1024,
                energy_wh: 0.15,
            })),
        }
    }

    /// Record a model call's token and latency metrics
    pub async fn record_model_call(&self, model_name: &str, input_tokens: u64, output_tokens: u64, latency_ms: u64) -> ModelCostRecord {
        // Simple heuristic rates per 1M tokens
        let (input_rate_1m, output_rate_1m) = match model_name.to_lowercase() {
            ref name if name.contains("gpt-4o") => (5.00, 15.00),
            ref name if name.contains("claude-3-5-sonnet") => (3.00, 15.00),
            ref name if name.contains("gpt-3.5") => (0.50, 1.50),
            ref name if name.contains("local") || name.contains("llama") || name.contains("mistral") => (0.0, 0.0), // Local model = $0.00 cost
            _ => (1.00, 3.00), // Default fallback rates
        };

        let estimated_cost_usd = ((input_tokens as f64 * input_rate_1m) + (output_tokens as f64 * output_rate_1m)) / 1_000_000.0;

        let record = ModelCostRecord {
            model_name: model_name.to_string(),
            input_tokens,
            output_tokens,
            latency_ms,
            estimated_cost_usd,
        };

        let mut records = self.model_records.lock().await;
        records.push(record.clone());
        record
    }

    /// Update resource utilization metrics and estimate energy footprint in Watt-hours
    pub async fn update_resource_metrics(&self, cpu: f64, gpu: f64, ram: u64, active_duration_sec: f64) -> ResourceCostRecord {
        // Average TDP: CPU = 65W, GPU = 200W
        // Power (W) = CPU_util * 65 + GPU_util * 200
        let cpu_watts = (cpu / 100.0) * 65.0;
        let gpu_watts = (gpu / 100.0) * 200.0;
        let idle_watts = 15.0; // Base laptop/system idle draw
        let total_watts = cpu_watts + gpu_watts + idle_watts;
        let delta_energy_wh = (total_watts * active_duration_sec) / 3600.0;

        let mut record = self.resource_record.lock().await;
        record.cpu_utilization = cpu;
        record.gpu_utilization = gpu;
        record.ram_mb = ram;
        record.energy_wh += delta_energy_wh;

        record.clone()
    }

    /// Retrieve aggregate statistics
    pub async fn get_aggregates(&self) -> (f64, u64, u64, f64) {
        let records = self.model_records.lock().await;
        let total_cost = records.iter().map(|r| r.estimated_cost_usd).sum();
        let total_input = records.iter().map(|r| r.input_tokens).sum();
        let total_output = records.iter().map(|r| r.output_tokens).sum();
        let energy = self.resource_record.lock().await.energy_wh;
        (total_cost, total_input, total_output, energy)
    }
}
