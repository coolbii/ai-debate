use anyhow::{anyhow, Context, Result};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone)]
pub struct OllamaClient {
    pub endpoint: String,
    pub default_model: String,
    client: reqwest::Client,
}

#[derive(Serialize)]
struct ChatMessage {
    role: String,
    content: String,
}

#[derive(Serialize)]
struct OllamaChatRequest {
    model: String,
    messages: Vec<ChatMessage>,
    stream: bool,
    options: OllamaOptions,
}

#[derive(Serialize)]
struct OllamaOptions {
    temperature: f32,
    num_predict: u32,
}

#[derive(Deserialize)]
struct OllamaChatResponse {
    message: ChatMessageContent,
}

#[derive(Deserialize)]
struct ChatMessageContent {
    content: String,
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
        // qwen3's format:"json" silently returns empty content for certain topics.
        // Instead, we request JSON via the prompt itself and parse manually.
        let full_prompt = format!(
            "{}\n\n\
            Respond in this exact JSON format and nothing else:\n\
            {{\"public_speech\": \"your speech here\", \"key_claims\": [\"claim1\", \"claim2\"], \"round_summary\": \"one sentence summary\"}}\n\
            /no_think",
            prompt,
        );

        let req = OllamaChatRequest {
            model: model.to_string(),
            messages: vec![
                ChatMessage { role: "user".to_string(), content: full_prompt },
            ],
            stream: false,
            // qwen3 thinking mode can consume tokens from num_predict budget.
            // /no_think is unreliable, so allocate enough for thinking + content.
            options: OllamaOptions { temperature: 0.8, num_predict: 4096 },
        };

        let url = format!("{}/api/chat", self.endpoint);
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

        let ollama_resp: OllamaChatResponse = resp.json().await.context("failed to parse Ollama chat response")?;

        let raw = &ollama_resp.message.content;
        tracing::debug!("Ollama raw content ({} chars), done_reason may be length if thinking consumed budget", raw.len());

        // If /no_think failed and content is empty, try to extract from thinking field
        if raw.trim().is_empty() {
            tracing::warn!("Empty content — /no_think likely failed, thinking consumed token budget");
            return Err(anyhow!("Ollama returned empty response (thinking consumed token budget)"));
        }

        // Try direct parse first
        if let Ok(output) = serde_json::from_str::<DebaterOutput>(raw) {
            return Ok(output);
        }

        // Try extracting JSON from markdown code blocks or surrounding text
        let extracted = extract_json(raw);
        if let Ok(output) = serde_json::from_str::<DebaterOutput>(&extracted) {
            tracing::info!("Parsed output after JSON extraction");
            return Ok(output);
        }

        // Last resort: build a fallback from whatever text we got
        tracing::warn!("Could not parse structured output, using raw text as speech. Raw: {}", &raw[..raw.len().min(300)]);
        Ok(DebaterOutput {
            public_speech: extract_text_content(raw),
            key_claims: vec![],
            round_summary: "Model returned unstructured output".to_string(),
            attack_targets: vec![],
            defense_targets: vec![],
        })
    }

    #[allow(dead_code)]
    pub async fn healthcheck(&self) -> Result<bool> {
        let url = format!("{}/api/tags", self.endpoint);
        let resp = self.client.get(&url).send().await?;
        Ok(resp.status().is_success())
    }
}

/// Extract JSON object from text that may contain markdown fences or extra text.
fn extract_json(raw: &str) -> String {
    let trimmed = raw.trim();

    // Strip ```json ... ``` or ``` ... ```
    if let Some(start) = trimmed.find("```") {
        let after_fence = &trimmed[start + 3..];
        // Skip optional language tag (e.g. "json")
        let content_start = after_fence.find('\n').map(|i| i + 1).unwrap_or(0);
        let content = &after_fence[content_start..];
        if let Some(end) = content.find("```") {
            return content[..end].trim().to_string();
        }
    }

    // Find first '{' and last '}' — extract that substring
    if let (Some(start), Some(end)) = (trimmed.find('{'), trimmed.rfind('}')) {
        if start < end {
            return trimmed[start..=end].to_string();
        }
    }

    trimmed.to_string()
}

/// Strip thinking tags and extract readable text from raw model output.
fn extract_text_content(raw: &str) -> String {
    let mut text = raw.to_string();
    // Remove <think>...</think> blocks
    while let Some(start) = text.find("<think>") {
        if let Some(end) = text.find("</think>") {
            text = format!("{}{}", &text[..start], &text[end + 8..]);
        } else {
            text = text[..start].to_string();
            break;
        }
    }
    let trimmed = text.trim();
    if trimmed.is_empty() {
        "[No response generated]".to_string()
    } else {
        trimmed.to_string()
    }
}
