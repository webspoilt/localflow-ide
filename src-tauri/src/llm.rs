use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::time::Duration;
use tracing::{info, error};
use anyhow::{Result, Context};

#[derive(Serialize)]
struct OllamaRequest {
    model: String,
    prompt: String,
    stream: bool,
}

#[derive(Deserialize)]
pub struct OllamaResponse {
    pub response: String,
    pub done: bool,
}

#[derive(Clone)]
pub struct LocalLlmEngine {
    client: Client,
    endpoint: String,
    default_model: String,
}

impl LocalLlmEngine {
    pub fn new(host: &str, port: u16, model: &str) -> Self {
        let client = Client::builder()
            .timeout(Duration::from_secs(120))
            .build()
            .expect("Failed to build HTTP client for Ollama");

        Self {
            client,
            endpoint: format!("http://{}:{}/api/generate", host, port),
            default_model: model.to_string(),
        }
    }

    pub fn default() -> Self {
        Self::new("127.0.0.1", 11434, "llama3")
    }

    pub async fn generate(&self, prompt: &str) -> Result<String> {
        info!("Sending prompt to Ollama (model: {})", self.default_model);

        let req = OllamaRequest {
            model: self.default_model.clone(),
            prompt: prompt.to_string(),
            stream: false,
        };

        let response = self.client
            .post(&self.endpoint)
            .json(&req)
            .send()
            .await
            .context("Failed to connect to Ollama. Is it running?")?;

        if !response.status().is_success() {
            let error_text = response.text().await.unwrap_or_default();
            error!("Ollama error: {}", error_text);
            return Err(anyhow::anyhow!("Ollama API error: {}", error_text));
        }

        let result: OllamaResponse = response.json()
            .await
            .context("Failed to parse Ollama response")?;

        Ok(result.response)
    }
}
