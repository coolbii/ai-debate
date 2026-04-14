use anyhow::{anyhow, Context, Result};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone)]
pub struct OllamaClient {
    pub endpoint: String,
    pub default_model: String,
    client: reqwest::Client,
}

#[derive(Serialize)]
struct OllamaRequest {
    model: String,
    prompt: String,
    stream: bool,
    format: serde_json::Value,
    options: OllamaOptions,
}

#[derive(Serialize)]
struct OllamaOptions {
    temperature: f32,
    num_predict: u32,
}

#[derive(Deserialize)]
struct OllamaResponse {
    response: String,
}

/// Structured output expected from each debater turn
#[derive(Debug, Deserialize, Serialize)]
pub struct DebaterOutput {
    pub public_speech: String,
    pub key_claims: Vec<String>,
    pub round_summary: String,
    #[serde(default)]
    pub attack_targets: Vec<String>,
    #[serde(default)]
    pub defense_targets: Vec<String>,
}

impl OllamaClient {
    pub fn new(endpoint: String, default_model: String) -> Self {
        Self {
            endpoint,
            default_model,
            client: reqwest::Client::builder()
                .timeout(std::time::Duration::from_secs(120))
                .build()
                .expect("failed to build reqwest client"),
        }
    }

    /// Call Ollama /api/generate and parse the structured JSON output.
    /// Retries up to 2 times on failure (network or JSON parse).
    pub async fn generate(&self, model: &str, prompt: &str) -> Result<DebaterOutput> {
        let max_retries = 2u32;
        let mut last_err = anyhow!("no attempt made");

        for attempt in 0..=max_retries {
            if attempt > 0 {
                let delay = std::time::Duration::from_millis(1000 * attempt as u64);
                tracing::warn!("Ollama retry {}/{} after {:?}", attempt, max_retries, delay);
                tokio::time::sleep(delay).await;
            }

            match self.generate_once(model, prompt).await {
                Ok(output) => return Ok(output),
                Err(e) => {
                    tracing::warn!("Ollama attempt {} failed: {}", attempt + 1, e);
                    last_err = e;
                }
            }
        }

        Err(last_err.context(format!("failed after {} retries", max_retries)))
    }

    async fn generate_once(&self, model: &str, prompt: &str) -> Result<DebaterOutput> {
        let json_schema = serde_json::json!({
            "type": "object",
            "properties": {
                "public_speech":   { "type": "string" },
                "key_claims":      { "type": "array", "items": { "type": "string" } },
                "round_summary":   { "type": "string" },
                "attack_targets":  { "type": "array", "items": { "type": "string" } },
                "defense_targets": { "type": "array", "items": { "type": "string" } }
            },
            "required": ["public_speech", "key_claims", "round_summary"]
        });

        let req = OllamaRequest {
            model: model.to_string(),
            prompt: prompt.to_string(),
            stream: false,
            format: json_schema,
            options: OllamaOptions { temperature: 0.8, num_predict: 512 },
        };

        let url = format!("{}/api/generate", self.endpoint);
        let resp = self
            .client
            .post(&url)
            .json(&req)
            .send()
            .await
            .context("failed to reach Ollama")?;

        if !resp.status().is_success() {
            let status = resp.status();
            let body = resp.text().await.unwrap_or_default();
            return Err(anyhow!("Ollama returned {}: {}", status, body));
        }

        let ollama_resp: OllamaResponse = resp.json().await.context("failed to parse Ollama envelope")?;

        serde_json::from_str::<DebaterOutput>(&ollama_resp.response)
            .context("failed to parse debater JSON output")
    }

    pub async fn healthcheck(&self) -> Result<bool> {
        let url = format!("{}/api/tags", self.endpoint);
        let resp = self.client.get(&url).send().await?;
        Ok(resp.status().is_success())
    }
}
