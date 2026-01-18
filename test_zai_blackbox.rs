// Simple test for Zai/Blackbox integration
// Run with: rustc test_zai_blackbox.rs && ./test_zai_blackbox

use std::env;
use reqwest::Client;
use serde_json::json;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🐺 FENRIR - Zai/Blackbox Test");
    println!("============================\n");

    // Test ZAI API Key
    match env::var("ZAI_API_KEY") {
        Ok(key) => {
            println!("✅ ZAI_API_KEY loaded");
            println!("🔑 Key length: {} characters\n", key.len());

            // Test Zai API call
            println!("🧠 Testing Zai Orchestrator...");
            match test_zai_orchestrator(&key).await {
                Ok(response) => println!("✅ Zai Response: {}\n", response),
                Err(e) => println!("❌ Zai Error: {}\n", e),
            }
        }
        Err(_) => println!("❌ ZAI_API_KEY not found\n"),
    }

    // Test BLACKBOX API Key
    match env::var("BLACKBOX_API_KEY") {
        Ok(key) => {
            println!("✅ BLACKBOX_API_KEY loaded");
            println!("🔑 Key length: {} characters\n", key.len());

            // Test Blackbox API call
            println!("🤖 Testing Blackbox AI...");
            match test_blackbox(&key).await {
                Ok(response) => println!("✅ Blackbox Response: {}\n", response),
                Err(e) => println!("❌ Blackbox Error: {}\n", e),
            }
        }
        Err(_) => println!("❌ BLACKBOX_API_KEY not found\n"),
    }

    println!("🎯 Test Complete!");
    Ok(())
}

async fn test_zai_orchestrator(api_key: &str) -> Result<String, String> {
    let client = Client::new();

    let response = client
        .post("https://api.blackbox.ai/v1/chat/completions")
        .header("Authorization", format!("Bearer {}", api_key))
        .header("Content-Type", "application/json")
        .json(&json!({
            "model": "blackboxai-pro",
            "messages": [
                {
                    "role": "system",
                    "content": "You are ZAI, the Fenrir Orchestrator - the main brain of the security platform. Provide strategic guidance."
                },
                {
                    "role": "user",
                    "content": "Hello Zai, what is your purpose?"
                }
            ],
            "max_tokens": 100
        }))
        .send()
        .await
        .map_err(|e| format!("Request failed: {}", e))?;

    if !response.status().is_success() {
        return Err(format!("API error: {}", response.status()));
    }

    let json: serde_json::Value = response
        .json()
        .await
        .map_err(|e| format!("Parse error: {}", e))?;

    json["choices"][0]["message"]["content"]
        .as_str()
        .map(|s| s.to_string())
        .ok_or_else(|| "No content in response".to_string())
}

async fn test_blackbox(api_key: &str) -> Result<String, String> {
    let client = Client::new();

    let response = client
        .post("https://api.blackbox.ai/v1/chat/completions")
        .header("Authorization", format!("Bearer {}", api_key))
        .header("Content-Type", "application/json")
        .json(&json!({
            "model": "blackboxai-pro",
            "messages": [
                {
                    "role": "system",
                    "content": "You are a helpful AI assistant."
                },
                {
                    "role": "user",
                    "content": "Hello Blackbox, what can you help with?"
                }
            ],
            "max_tokens": 100
        }))
        .send()
        .await
        .map_err(|e| format!("Request failed: {}", e))?;

    if !response.status().is_success() {
        return Err(format!("API error: {}", response.status()));
    }

    let json: serde_json::Value = response
        .json()
        .await
        .map_err(|e| format!("Parse error: {}", e))?;

    json["choices"][0]["message"]["content"]
        .as_str()
        .map(|s| s.to_string())
        .ok_or_else(|| "No content in response".to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_env_vars() {
        // Just check if env vars are accessible
        let _zai = env::var("ZAI_API_KEY");
        let _blackbox = env::var("BLACKBOX_API_KEY");
        // Test passes if no panic
    }
}
