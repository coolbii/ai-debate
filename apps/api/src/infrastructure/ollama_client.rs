#[derive(Debug, Clone)]
pub struct OllamaClient {
    pub endpoint: String,
    pub model: String,
}

impl OllamaClient {
    pub fn new(endpoint: String, model: String) -> Self {
        Self { endpoint, model }
    }

    pub fn healthcheck(&self) -> String {
        format!("ollama:{} model:{}", self.endpoint, self.model)
    }
}
