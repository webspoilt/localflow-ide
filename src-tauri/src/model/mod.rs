use tracing::info;

#[derive(Debug, Clone)]
pub enum ProviderKind {
    OpenAI,
    Anthropic,
    Ollama,
    Custom(String),
}

pub struct ProviderConfig {
    pub kind: ProviderKind,
    pub api_key: Option<String>,
    pub base_url: Option<String>,
    pub model: String,
    pub max_tokens: u32,
    pub timeout_secs: u32,
}

pub struct ModelRouter {
    providers: Vec<ProviderConfig>,
    active_idx: usize,
}

impl ModelRouter {
    pub fn new() -> Self {
        info!("Model Router initialized");
        Self { providers: vec![], active_idx: 0 }
    }

    pub fn register(&mut self, config: ProviderConfig) {
        info!(model = %config.model, kind = ?config.kind, "Provider registered");
        self.providers.push(config);
    }

    pub fn active(&self) -> Option<&ProviderConfig> {
        self.providers.get(self.active_idx)
    }

    pub fn fallback(&mut self) -> Option<&ProviderConfig> {
        self.active_idx = (self.active_idx + 1) % self.providers.len().max(1);
        self.active()
    }

    pub fn providers(&self) -> &[ProviderConfig] {
        &self.providers
    }
}
