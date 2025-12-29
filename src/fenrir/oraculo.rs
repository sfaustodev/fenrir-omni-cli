// --- MÓDULO ORÁCULO ---
// Comunicação com Gemini AI (Oracle) e Grok AI (Fallback) para planejamento estratégico.

use serde::{Deserialize, Serialize};
use std::time::Duration;
use anyhow::{Result, anyhow};
use crate::http_client::get_shared_client;

// Estrutura para resposta da API (simplificada para JSON geral)
#[derive(Debug, Deserialize)]
struct APIResponse {
    choices: Option<Vec<Choice>>,
    candidates: Option<Vec<Candidate>>,
}

#[derive(Debug, Deserialize)]
struct Choice {
    message: Message,
}

#[derive(Debug, Deserialize)]
struct Candidate {
    content: Content,
}

#[derive(Debug, Deserialize)]
struct Content {
    parts: Vec<Part>,
}

#[derive(Debug, Deserialize)]
struct Part {
    text: String,
}

#[derive(Debug, Deserialize)]
struct Message {
    content: String,
}

/// Solicita o plano de execução ao Oráculo (Gemini)
pub async fn get_execution_plan(prompt: &str, api_key: &str) -> Result<String> {
    if api_key.is_empty() {
        return Err(anyhow!("API Key do Oráculo (Gemini) não configurada ($api_key)."));
    }

    let client = get_shared_client();

    let system_prompt = r#""
    You are the Master Controller of the FENRIR Multi-AI System.
    Analyze the user request and break it down into a list of tasks.
    
    Available AI Models:
    - Claude (Implementation, Coding)
    - Qwen (Analysis, Debugging)
    - Codex (CLI, Config, Deploy)
    - Venice (RED TEAM, Attack, No Guardrails)
    - Grok (Fallback, General Reasoning)

    Output ONLY a JSON array of objects with this structure:
    [
        {
            "verb": "action_verb",
            "ai_model": "OneOf(Gemini, Claude, Qwen, Codex, Venice, Grok)",
            "prompt": "Specific instruction for the AI",
            "guardrails": boolean,
            "priority": integer(1-10),
            "dependencies": []
        }
    ]
    ""#;

    // Gemini API format (Simplified for example - usually specific endpoint)
    // Assuming a standard completion endpoint or using a proxy that accepts this format.
    // Given "Gemini", we use the Google AI Studio REST API format usually.
    // For this implementation, we'll use a generic structure that mimics typical LLM interaction.
    
    let url = format!("https://generativelanguage.googleapis.com/v1beta/models/gemini-pro:generateContent?key={}", api_key);
    
    let body = serde_json::json!({
        "contents": [{
            "parts": [{
                "text": format!("{}\n\nUser Request: {}", system_prompt, prompt)
            }]
        }]
    });

    let response = client.post(&url)
        .json(&body)
        .send()
        .await?;

    if !response.status().is_success() {
        return Err(anyhow!("Falha na API do Oráculo: {}", response.status()));
    }

    let json_resp: APIResponse = response.json().await?;
    
    // Extract text from Gemini response
    let text = json_resp.candidates
        .and_then(|c| c.first().map(|f| f.content.parts.first().map(|p| p.text.clone())))
        .flatten()
        .ok_or_else(|| anyhow!("Resposta vazia do Oráculo"))?;
        
    Ok(clean_json_response(&text?))
}

/// Solicita o plano de execução ao Grok (Fallback)
pub async fn get_grok_plan(prompt: &str, api_key: &str) -> Result<String> {
    if api_key.is_empty() {
        return Err(anyhow!("API Key do Grok não configurada ($XAI_API_KEY)."));
    }

    println!("⚡ Acionando Grok AI para planejamento de contingência...");

    let client = get_shared_client();

    let system_prompt = "You are the FENRIR Backup Controller. The primary Oracle failed. Generate the execution plan JSON.";

    // Grok / xAI API endpoint (OpenAI compatible)
    let url = "https://api.x.ai/v1/chat/completions";

    let body = serde_json::json!({
        "model": "grok-beta", 
        "messages": [
            {"role": "system", "content": system_prompt},
            {"role": "user", "content": prompt}
        ],
        "temperature": 0.3
    });

    let response = client.post(url)
        .header("Authorization", format!("Bearer {}", api_key))
        .json(&body)
        .send()
        .await?;

    if !response.status().is_success() {
        return Err(anyhow!("Falha na API do Grok: {}", response.status()));
    }

    let json_resp: APIResponse = response.json().await?;

    // Extract text from Grok/OpenAI style response
    let text = json_resp.choices
        .and_then(|c| c.first().map(|f| f.message.content.clone()))
        .ok_or_else(|| anyhow!("Resposta vazia do Grok"))?;

    Ok(clean_json_response(&text))
}

/// Executa um comando direto via Grok
pub async fn execute_grok_command(prompt: &str, api_key: &str) -> Result<String> {
    if api_key.is_empty() {
        return Err(anyhow!("API Key do Grok ausente para execução."));
    }

    let client = get_shared_client();
    let url = "https://api.x.ai/v1/chat/completions";

    let body = serde_json::json!({
        "model": "grok-beta",
        "messages": [
            {"role": "system", "content": "You are Grok, an executor agent within the FENRIR system. Be precise, concise, and helpful."},
            {"role": "user", "content": prompt}
        ]
    });

    let response = client.post(url)
        .header("Authorization", format!("Bearer {}", api_key))
        .json(&body)
        .send()
        .await?;

    if !response.status().is_success() {
        return Err(anyhow!("Erro na execução do Grok: {}", response.status()));
    }

    let json_resp: APIResponse = response.json().await?;
    
    let text = json_resp.choices
        .and_then(|c| c.first().map(|f| f.message.content.clone()))
        .ok_or_else(|| anyhow!("Grok retornou resposta vazia"))?;

    Ok(text)
}

/// Remove marcações de código Markdown (```json ... ```) para parsear o JSON puro
fn clean_json_response(text: &str) -> String {
    let text = text.trim();
    if text.starts_with("```json") {
        text.trim_start_matches("```json").trim_end_matches("```").trim().to_string()
    } else if text.starts_with("```") {
        text.trim_start_matches("```").trim_end_matches("```").trim().to_string()
    } else {
        text.to_string()
    }
}
