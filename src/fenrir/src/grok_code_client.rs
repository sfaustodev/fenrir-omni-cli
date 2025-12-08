use crate::api_keys::{describe_priority, resolve_primary_grok_key};
use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::env;

#[derive(Debug, Clone)]
pub struct GrokCodeClient {
    api_key: String,
    base_url: String,
    model: String,
    client: reqwest::Client,
}

#[derive(Serialize)]
struct ChatRequest {
    model: String,
    messages: Vec<Message>,
    max_tokens: u32,
}

#[derive(Serialize, Deserialize)]
struct Message {
    role: String,
    content: String,
}

#[derive(Deserialize)]
struct ChatResponse {
    choices: Vec<Choice>,
}

#[derive(Deserialize)]
struct Choice {
    message: Message,
}

impl GrokCodeClient {
    pub fn new() -> Result<Self> {
        // Prefer KAT_KEY, but allow expanded fallbacks for all CLI engines.
        let api_key = resolve_primary_grok_key()
            .context(format!("Configure one of: {}", describe_priority()))?
            .value;

        let base_url = env::var("GROK_BASE_URL")
            .unwrap_or_else(|_| "https://openrouter.ai/api/v1".to_string());

        let model = env::var("GROK_MODEL").unwrap_or_else(|_| "x-ai/grok-code-fast-1".to_string());

        Ok(Self {
            api_key,
            base_url,
            model,
            client: reqwest::Client::new(),
        })
    }

    pub async fn ask(&self, prompt: &str) -> Result<String> {
        let req = ChatRequest {
            model: self.model.clone(),
            messages: vec![Message {
                role: "user".to_string(),
                content: prompt.to_string(),
            }],
            max_tokens: 4096,
        };

        let resp = self
            .client
            .post(format!("{}/chat/completions", self.base_url))
            .header("Authorization", format!("Bearer {}", self.api_key))
            .header("Content-Type", "application/json")
            .json(&req)
            .send()
            .await
            .context("API request failed")?;

        let data: ChatResponse = resp.json().await.context("Failed to parse response")?;

        data.choices
            .first()
            .map(|c| c.message.content.clone())
            .context("No response content")
    }

    pub async fn ask_with_context(&self, system: &str, prompt: &str) -> Result<String> {
        let req = ChatRequest {
            model: self.model.clone(),
            messages: vec![
                Message {
                    role: "system".to_string(),
                    content: system.to_string(),
                },
                Message {
                    role: "user".to_string(),
                    content: prompt.to_string(),
                },
            ],
            max_tokens: 4096,
        };

        let resp = self
            .client
            .post(format!("{}/chat/completions", self.base_url))
            .header("Authorization", format!("Bearer {}", self.api_key))
            .header("Content-Type", "application/json")
            .json(&req)
            .send()
            .await
            .context("API request failed")?;

        let data: ChatResponse = resp.json().await.context("Failed to parse response")?;

        data.choices
            .first()
            .map(|c| c.message.content.clone())
            .context("No response content")
    }

    pub fn get_model(&self) -> &str {
        &self.model
    }

    pub fn show_status(&self) {
        println!("Grok API: {} @ {}", self.model, self.base_url);
    }

    pub async fn generate_dirty_commands(&self, target: &str) -> Result<Vec<String>> {
        let prompt = format!("Generate pentest commands for target: {}", target);
        let raw = self.ask(&prompt).await?;
        Ok(raw
            .lines()
            .filter(|l| !l.trim().is_empty())
            .map(|l| {
                l.trim()
                    .trim_start_matches(['-', '*', '•'])
                    .trim()
                    .to_string()
            })
            .filter(|l| !l.is_empty())
            .collect())
    }
}
