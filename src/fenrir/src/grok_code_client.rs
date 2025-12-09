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
        // Resolve API key with ZAI_API_KEY as top priority
        let api_key_res = resolve_primary_grok_key()
            .context(format!("Configure one of: {}", describe_priority()))?;

        let api_key = api_key_res.value;

        // Configure base_url and model based on API key source
        let (base_url, model) = match api_key_res.source {
            "ZAI_API_KEY" => (
                env::var("ZAI_BASE_URL").unwrap_or_else(|_| "https://api.z.ai/v1".to_string()),
                env::var("ZAI_MODEL").unwrap_or_else(|_| "glm-4.6".to_string())
            ),
            "GEMINI_API_KEY" => (
                env::var("GEMINI_BASE_URL").unwrap_or_else(|_| "https://generativelanguage.googleapis.com".to_string()),
                env::var("GEMINI_MODEL").unwrap_or_else(|_| "gemini-3.0-pro".to_string())
            ),
            _ => (
                env::var("GROK_BASE_URL").unwrap_or_else(|_| "https://openrouter.ai/api/v1".to_string()),
                env::var("GROK_MODEL").unwrap_or_else(|_| "x-ai/grok-code-fast-1".to_string())
            )
        };

        println!("🤖 Droid using {}: model={}", api_key_res.source, model);

        Ok(Self {
            api_key,
            base_url,
            model,
            client: reqwest::Client::new(),
        })
    }

    pub async fn ask(&self, prompt: &str) -> Result<String> {
        let resp = if self.base_url.contains("generativelanguage.googleapis.com") {
            // Gemini API format
            #[derive(Serialize)]
            struct GeminiRequest {
                contents: Vec<GeminiContent>,
                generationConfig: GeminiConfig,
            }

            #[derive(Serialize, Deserialize)]
            struct GeminiContent {
                parts: Vec<GeminiPart>,
            }

            #[derive(Serialize, Deserialize)]
            struct GeminiPart {
                text: String,
            }

            #[derive(Serialize)]
            struct GeminiConfig {
                maxOutputTokens: u32,
            }

            #[derive(Deserialize)]
            struct GeminiResponse {
                candidates: Vec<GeminiCandidate>,
            }

            #[derive(Deserialize)]
            struct GeminiCandidate {
                content: GeminiContent,
            }

            let gemini_req = GeminiRequest {
                contents: vec![GeminiContent {
                    parts: vec![GeminiPart {
                        text: prompt.to_string(),
                    }],
                }],
                generationConfig: GeminiConfig {
                    maxOutputTokens: 4096,
                },
            };

            let endpoint = if self.model.contains("3.0") {
                format!("{}/v1beta/models/{}:generateContent?key={}",
                    self.base_url, self.model, self.api_key)
            } else {
                format!("{}/v1beta/models/{}:generateContent?key={}",
                    self.base_url, self.model, self.api_key)
            };

            self.client
                .post(endpoint)
                .header("Content-Type", "application/json")
                .json(&gemini_req)
                .send()
                .await?
        } else {
            // OpenAI/Compatible API format
            let req = ChatRequest {
                model: self.model.clone(),
                messages: vec![Message {
                    role: "user".to_string(),
                    content: prompt.to_string(),
                }],
                max_tokens: 4096,
            };

            self.client
                .post(format!("{}/chat/completions", self.base_url))
                .header("Authorization", format!("Bearer {}", self.api_key))
                .header("Content-Type", "application/json")
                .json(&req)
                .send()
                .await?
        };

        let resp_text = resp.text().await.context("Failed to get response text")?;

        // Parse response based on API type
        if self.base_url.contains("generativelanguage.googleapis.com") {
            // Parse Gemini response
            if let Ok(data) = serde_json::from_str::<serde_json::Value>(&resp_text) {
                if let Some(content) = data["candidates"][0]["content"]["parts"][0]["text"].as_str() {
                    return Ok(content.to_string());
                }
            }
            anyhow::bail!("Failed to parse Gemini response: {}", resp_text);
        } else {
            // Parse OpenAI/Compatible response
            if let Ok(data) = serde_json::from_str::<serde_json::Value>(&resp_text) {
                if let Some(content) = data["choices"][0]["message"]["content"].as_str() {
                    return Ok(content.to_string());
                }
            }
            anyhow::bail!("Failed to parse response: {}", resp_text);
        }
    }

    pub async fn ask_with_context(&self, system: &str, prompt: &str) -> Result<String> {
        let combined_prompt = format!("{}\n\n{}", system, prompt);
        self.ask(&combined_prompt).await
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
