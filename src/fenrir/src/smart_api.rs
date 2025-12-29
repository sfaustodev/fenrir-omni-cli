//! # Smart API Integration Module
//!
//! Uses HTTP requests via bash functions for AI APIs
//! Automatically detects if running in CLI mode and cooperates intelligently

use std::process::Command;
use anyhow::{Result, Context};
use crate::http_client::get_shared_client;

/// Smart API client that uses bash HTTP functions
pub struct SmartAPIClient {
    pub prefer_bash: bool,
    pub cli_mode: bool,
}

impl SmartAPIClient {
    pub fn new() -> Self {
        // Detect if we're in CLI mode
        let cli_mode = std::env::var("FENRIR_CLI_MODE").is_ok()
            || std::env::var("CLAI_DESKTOP_PARENT_PROCESS_ID").is_ok();

        // Check if bash API functions are available
        let prefer_bash = Self::check_bash_apis();

        Self {
            prefer_bash,
            cli_mode,
        }
    }

    /// Check if bash API functions are available
    fn check_bash_apis() -> bool {
        Command::new("zsh")
            .arg("-c")
            .arg("type gemini &>/dev/null && type grok &>/dev/null && type zai &>/dev/null")
            .output()
            .map(|out| out.status.success())
            .unwrap_or(false)
    }

    /// Call Gemini API via bash function
    pub fn call_gemini(&self, prompt: &str) -> Result<String> {
        if self.prefer_bash {
            self.call_bash_api("gemini", prompt)
        } else {
            self.call_direct_gemini(prompt)
        }
    }

    /// Call Grok/XAI API via bash function
    pub fn call_grok(&self, prompt: &str) -> Result<String> {
        if self.prefer_bash {
            self.call_bash_api("grok", prompt)
        } else {
            self.call_direct_grok(prompt)
        }
    }

    /// Call ZAI API via bash function
    pub fn call_zai(&self, prompt: &str) -> Result<String> {
        if self.prefer_bash {
            self.call_bash_api("zai", prompt)
        } else {
            self.call_direct_zai(prompt)
        }
    }

    /// Call Qwen API via bash function
    pub fn call_qwen(&self, prompt: &str) -> Result<String> {
        if self.prefer_bash {
            self.call_bash_api("qwen", prompt)
        } else {
            self.call_direct_qwen(prompt)
        }
    }

    /// Call API via bash function (intelligent cooperation)
    fn call_bash_api(&self, api_name: &str, prompt: &str) -> Result<String> {
        // If in CLI mode, notify the CLI we're making an API call
        if self.cli_mode {
            eprintln!("🔥 Fenrir: Calling {} API via bash...", api_name);
        }

        let sanitized_prompt = prompt.replace('"', "'\"'\"'");
        let command = format!("{} '{}'", api_name, sanitized_prompt);

        let output = Command::new("zsh")
            .arg("-c")
            .arg(&command)
            .output()
            .context("Failed to execute bash API function")?;

        if !output.status.success() {
            let error = String::from_utf8_lossy(&output.stderr);
            anyhow::bail!("API call failed: {}", error);
        }

        let response = String::from_utf8_lossy(&output.stdout).to_string();
        Ok(response)
    }

    /// Direct HTTP call to Gemini (fallback)
    fn call_direct_gemini(&self, prompt: &str) -> Result<String> {
        let api_key = std::env::var("GEMINI_API_KEY")
            .context("GEMINI_API_KEY not set")?;

        let client = get_shared_client();
        let url = format!(
            "https://generativelanguage.googleapis.com/v1beta/models/gemini-2.5-pro:generateContent?key={}",
            api_key
        );

        let payload = serde_json::json!({
            "contents": [{
                "parts": [{"text": prompt}]
            }]
        });

        let response = client
            .post(&url)
            .json(&payload)
            .send()
            .await?
            .text()
            .await?;

        Ok(response)
    }

    /// Direct HTTP call to Grok (fallback)
    fn call_direct_grok(&self, prompt: &str) -> Result<String> {
        let api_key = std::env::var("GROK_API_KEY")
            .context("GROK_API_KEY not set")?;
        let base_url = std::env::var("GROK_BASE_URL")
            .unwrap_or_else(|_| "https://api.x.ai/v1".to_string());

        let client = get_shared_client();
        let url = format!("{}/chat/completions", base_url);

        let payload = serde_json::json!({
            "model": "grok-2-1212312",
            "messages": [{"role": "user", "content": prompt}],
            "temperature": 0.7
        });

        let response = client
            .post(&url)
            .header("Authorization", format!("Bearer {}", api_key))
            .json(&payload)
            .send()
            .await?
            .text()
            .await?;

        Ok(response)
    }

    /// Direct HTTP call to ZAI (fallback)
    fn call_direct_zai(&self, prompt: &str) -> Result<String> {
        let api_key = std::env::var("ZAI_API_KEY")
            .context("ZAI_API_KEY not set")?;
        let base_url = std::env::var("ANTHROPIC_BASE_URL")
            .unwrap_or_else(|_| "https://api.z.ai/api/anthropic".to_string());

        let client = get_shared_client();
        let url = format!("{}/v1/messages", base_url);

        let payload = serde_json::json!({
            "model": "glm-4.7",
            "max_tokens": 4096,
            "messages": [{"role": "user", "content": prompt}]
        });

        let response = client
            .post(&url)
            .header("x-api-key", api_key)
            .header("anthropic-version", "2023-06-01")
            .json(&payload)
            .send()
            .await?
            .text()
            .await?;

        Ok(response)
    }

    /// Direct HTTP call to Qwen (fallback)
    fn call_direct_qwen(&self, prompt: &str) -> Result<String> {
        let api_key = std::env::var("QWEN3_API_KEY")
            .context("QWEN3_API_KEY not set")?;

        let client = get_shared_client();
        let url = "https://dashscope.aliyuncs.com/compatible-mode/v1/chat/completions";

        let payload = serde_json::json!({
            "model": "qwen-max",
            "messages": [{"role": "user", "content": prompt}],
            "temperature": 0.7
        });

        let response = client
            .post(url)
            .header("Authorization", format!("Bearer {}", api_key))
            .json(&payload)
            .send()
            .await?
            .text()
            .await?;

        Ok(response)
    }

    /// Smart routing - choose best API based on prompt and context
    pub fn smart_call(&self, prompt: &str) -> Result<String> {
        // Detect prompt type and route to appropriate API
        if prompt.contains("code") || prompt.contains("debug") || prompt.contains("fix") {
            // Code tasks -> ZAI (GLM 4.7)
            eprintln!("🔥 Routing to ZAI (GLM 4.7) for code task");
            self.call_zai(prompt)
        } else if prompt.contains("fast") || prompt.contains("quick") {
            // Fast tasks -> Gemini
            eprintln!("🔥 Routing to Gemini for fast response");
            self.call_gemini(prompt)
        } else if prompt.contains("creative") || prompt.contains("write") {
            // Creative tasks -> Grok
            eprintln!("🔥 Routing to Grok for creative task");
            self.call_grok(prompt)
        } else {
            // Default -> ZAI
            eprintln!("🔥 Routing to default API (ZAI)");
            self.call_zai(prompt)
        }
    }
}

impl Default for SmartAPIClient {
    fn default() -> Self {
        Self::new()
    }
}

/// Create a global smart API client instance
pub fn get_smart_api_client() -> SmartAPIClient {
    SmartAPIClient::new()
}
