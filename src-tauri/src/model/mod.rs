use tracing::{info, warn};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ProviderKind {
    Ollama,
    OpenAI,
    Anthropic,
    OpenRouter,
    LmStudio,
    OpenClaw,
    OmniClaw,
    Custom(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProviderConfig {
    pub kind: ProviderKind,
    pub model: String,
    pub base_url: String,
    pub api_key: Option<String>,
    pub max_tokens: u32,
    pub timeout_secs: u32,
    pub context_window: u32,
    pub ram_mb: u32,
    pub vram_mb: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InferenceRequest {
    pub provider: String,
    pub model: String,
    pub prompt: String,
    pub system_prompt: Option<String>,
    pub temperature: Option<f32>,
    pub max_tokens: Option<u32>,
    pub stream: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InferenceResponse {
    pub text: String,
    pub provider: String,
    pub model: String,
    pub latency_ms: u64,
    pub tokens_in: u32,
    pub tokens_out: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProviderHealth {
    pub provider: String,
    pub model: String,
    pub available: bool,
    pub latency_ms: u64,
    pub error: Option<String>,
}

pub struct ModelRouter {
    providers: Vec<ProviderConfig>,
    active_idx: usize,
    http_client: reqwest::Client,
}

impl ModelRouter {
    pub fn new() -> Self {
        info!("ModelRouter initialized");
        Self {
            providers: vec![],
            active_idx: 0,
            http_client: reqwest::Client::builder()
                .timeout(std::time::Duration::from_secs(30))
                .build()
                .unwrap_or_default(),
        }
    }

    pub fn register(&mut self, config: ProviderConfig) {
        info!(model = %config.model, kind = ?config.kind, "Provider registered");
        self.providers.push(config);
    }

    pub fn register_ollama(&mut self, model: &str) {
        self.register(ProviderConfig {
            kind: ProviderKind::Ollama,
            model: model.to_string(),
            base_url: "http://localhost:11434".into(),
            api_key: None,
            max_tokens: 4096,
            timeout_secs: 120,
            context_window: 8192,
            ram_mb: 2048,
            vram_mb: 1024,
        });
    }

    pub fn register_openai(&mut self, model: &str, api_key: &str) {
        self.register(ProviderConfig {
            kind: ProviderKind::OpenAI,
            model: model.to_string(),
            base_url: "https://api.openai.com/v1".into(),
            api_key: Some(api_key.to_string()),
            max_tokens: 16384,
            timeout_secs: 60,
            context_window: 128000,
            ram_mb: 0,
            vram_mb: 0,
        });
    }

    pub async fn infer(&self, request: &InferenceRequest) -> Result<InferenceResponse, String> {
        let config = self.providers.get(self.active_idx)
            .ok_or_else(|| "No provider configured".to_string())?;

        info!(provider = ?config.kind, model = %config.model, "Starting inference");
        let _start = std::time::Instant::now();

        let response = match config.kind {
            ProviderKind::Ollama => self.infer_ollama(config, request).await,
            ProviderKind::OpenAI => self.infer_openai(config, request).await,
            ProviderKind::Anthropic => self.infer_anthropic(config, request).await,
            _ => Err(format!("Provider {:?} not yet implemented", config.kind)),
        };

        match response {
            Ok(resp) => {
                info!(latency = resp.latency_ms, provider = ?config.kind, "Inference complete");
                Ok(resp)
            }
            Err(e) => {
                warn!(error = %e, provider = ?config.kind, "Inference failed");
                Err(e)
            }
        }
    }

    async fn infer_ollama(&self, config: &ProviderConfig, request: &InferenceRequest) -> Result<InferenceResponse, String> {
        let url = format!("{}/api/generate", config.base_url.trim_end_matches('/'));
        let body = serde_json::json!({
            "model": config.model,
            "prompt": request.prompt,
            "system": request.system_prompt,
            "stream": false,
            "options": {
                "temperature": request.temperature.unwrap_or(0.7),
                "num_predict": request.max_tokens.unwrap_or(config.max_tokens),
            }
        });

        let start = std::time::Instant::now();
        let resp = self.http_client.post(&url)
            .json(&body)
            .send()
            .await
            .map_err(|e| format!("Ollama request failed: {}", e))?;

        let latency = start.elapsed().as_millis() as u64;
        let data: serde_json::Value = resp.json().await
            .map_err(|e| format!("Ollama parse failed: {}", e))?;

        let text = data["response"].as_str()
            .unwrap_or("")
            .to_string();

        Ok(InferenceResponse {
            text,
            provider: "ollama".into(),
            model: config.model.clone(),
            latency_ms: latency,
            tokens_in: data["prompt_eval_count"].as_u64().unwrap_or(0) as u32,
            tokens_out: data["eval_count"].as_u64().unwrap_or(0) as u32,
        })
    }

    async fn infer_openai(&self, config: &ProviderConfig, request: &InferenceRequest) -> Result<InferenceResponse, String> {
        let url = format!("{}/chat/completions", config.base_url.trim_end_matches('/'));
        let api_key = config.api_key.as_deref().ok_or("No API key for OpenAI")?;

        let mut messages = vec![];
        if let Some(system) = &request.system_prompt {
            messages.push(serde_json::json!({"role": "system", "content": system}));
        }
        messages.push(serde_json::json!({"role": "user", "content": request.prompt}));

        let body = serde_json::json!({
            "model": config.model,
            "messages": messages,
            "temperature": request.temperature.unwrap_or(0.7),
            "max_tokens": request.max_tokens.unwrap_or(config.max_tokens),
            "stream": false,
        });

        let start = std::time::Instant::now();
        let resp = self.http_client.post(&url)
            .header("Authorization", format!("Bearer {}", api_key))
            .json(&body)
            .send()
            .await
            .map_err(|e| format!("OpenAI request failed: {}", e))?;

        let latency = start.elapsed().as_millis() as u64;
        let data: serde_json::Value = resp.json().await
            .map_err(|e| format!("OpenAI parse failed: {}", e))?;

        let text = data["choices"][0]["message"]["content"].as_str()
            .unwrap_or("")
            .to_string();

        let usage = &data["usage"];
        Ok(InferenceResponse {
            text,
            provider: "openai".into(),
            model: config.model.clone(),
            latency_ms: latency,
            tokens_in: usage["prompt_tokens"].as_u64().unwrap_or(0) as u32,
            tokens_out: usage["completion_tokens"].as_u64().unwrap_or(0) as u32,
        })
    }

    async fn infer_anthropic(&self, config: &ProviderConfig, request: &InferenceRequest) -> Result<InferenceResponse, String> {
        let url = "https://api.anthropic.com/v1/messages";
        let api_key = config.api_key.as_deref().ok_or("No API key for Anthropic")?;

        let mut messages = vec![];
        messages.push(serde_json::json!({"role": "user", "content": request.prompt}));

        let body = serde_json::json!({
            "model": config.model,
            "messages": messages,
            "system": request.system_prompt,
            "max_tokens": request.max_tokens.unwrap_or(config.max_tokens),
            "stream": false,
        });

        let start = std::time::Instant::now();
        let resp = self.http_client.post(url)
            .header("x-api-key", api_key)
            .header("anthropic-version", "2023-06-01")
            .json(&body)
            .send()
            .await
            .map_err(|e| format!("Anthropic request failed: {}", e))?;

        let latency = start.elapsed().as_millis() as u64;
        let data: serde_json::Value = resp.json().await
            .map_err(|e| format!("Anthropic parse failed: {}", e))?;

        let text = data["content"][0]["text"].as_str()
            .unwrap_or("")
            .to_string();

        let usage = &data["usage"];
        Ok(InferenceResponse {
            text,
            provider: "anthropic".into(),
            model: config.model.clone(),
            latency_ms: latency,
            tokens_in: usage["input_tokens"].as_u64().unwrap_or(0) as u32,
            tokens_out: usage["output_tokens"].as_u64().unwrap_or(0) as u32,
        })
    }

    pub async fn health(&self) -> Vec<ProviderHealth> {
        let mut results = vec![];
        for config in &self.providers {
            let start = std::time::Instant::now();
            let available = match &config.kind {
                ProviderKind::Ollama => {
                    let url = format!("{}/api/tags", config.base_url.trim_end_matches('/'));
                    self.http_client.get(&url).send().await.is_ok()
                }
                ProviderKind::OpenAI => {
                    let url = format!("{}/models", config.base_url.trim_end_matches('/'));
                    let key = config.api_key.as_deref().unwrap_or("");
                    self.http_client.get(&url)
                        .header("Authorization", format!("Bearer {}", key))
                        .send().await.is_ok()
                }
                _ => false,
            };
            results.push(ProviderHealth {
                provider: format!("{:?}", config.kind).to_lowercase(),
                model: config.model.clone(),
                available,
                latency_ms: start.elapsed().as_millis() as u64,
                error: if available { None } else { Some("unreachable".into()) },
            });
        }
        results
    }

    pub fn switch_provider(&mut self, name: &str) -> Result<(), String> {
        let idx = self.providers.iter().position(|p| {
            let pname = format!("{:?}", p.kind).to_lowercase();
            pname == name.to_lowercase() || p.model == name
        });
        match idx {
            Some(i) => { self.active_idx = i; Ok(()) }
            None => Err(format!("Provider '{}' not found", name)),
        }
    }

    pub fn active(&self) -> Option<&ProviderConfig> {
        self.providers.get(self.active_idx)
    }

    pub fn providers(&self) -> &[ProviderConfig] {
        &self.providers
    }
}
