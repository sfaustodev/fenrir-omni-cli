use crate::brain::decision_layer::DecisionEngine;
use crate::task_management::chain_coordinator::{ChainOfCaralhoManager, ExternalTarefinhaPlan};
use crate::task_management::{Complexity, Priority};
use anyhow::{Context, Result};
use indicatif::ProgressBar;
use reqwest::Client;
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use std::env;
use std::time::Duration;

#[derive(Debug)]
struct GeminiSummary {
    english: String,
    keywords: Vec<String>,
    summary: String,
    actions: Vec<String>,
}

#[derive(Debug)]
struct ClaudeSanitized {
    sanitized_summary: String,
    removed_items: Vec<String>,
    safe_actions: Vec<String>,
}

#[derive(Debug, Deserialize)]
struct GeminiResponse {
    english: Option<String>,
    keywords: Option<Vec<String>>,
    summary: Option<String>,
    actions: Option<Vec<String>>,
}

#[derive(Debug, Deserialize)]
struct ClaudeResponse {
    sanitized_summary: Option<String>,
    removed_items: Option<Vec<String>>,
    safe_actions: Option<Vec<String>>,
}

#[derive(Debug, Deserialize)]
struct GrokResponse {
    tarefinhas: Vec<GrokTarefinha>,
}

#[derive(Debug, Deserialize)]
struct GrokTarefinha {
    titulo: String,
    descricao: String,
    priority: Option<String>,
    complexity: Option<String>,
    estimated_minutes: Option<u16>,
    dependencies: Option<Vec<String>>,
    async_ok: Option<bool>,
}

// API Request/Response structures
#[derive(Debug, Serialize)]
struct GeminiRequest {
    contents: Vec<GeminiContent>,
    generation_config: Option<GeminiConfig>,
}

#[derive(Debug, Serialize, Deserialize)]
struct GeminiContent {
    parts: Vec<GeminiPart>,
}

#[derive(Debug, Serialize, Deserialize)]
struct GeminiPart {
    text: String,
}

#[derive(Debug, Serialize)]
struct GeminiConfig {
    temperature: f32,
    max_output_tokens: i32,
}

#[derive(Debug, Deserialize)]
struct GeminiApiResponse {
    candidates: Vec<GeminiCandidate>,
}

#[derive(Debug, Deserialize)]
struct GeminiCandidate {
    content: GeminiContent,
}

#[derive(Debug, Serialize)]
struct ClaudeRequest {
    model: String,
    max_tokens: i32,
    messages: Vec<ClaudeMessage>,
}

#[derive(Debug, Serialize)]
struct ClaudeMessage {
    role: String,
    content: String,
}

#[derive(Debug, Deserialize)]
struct ClaudeApiResponse {
    content: Vec<ClaudeContentBlock>,
}

#[derive(Debug, Deserialize)]
struct ClaudeContentBlock {
    text: String,
}

#[derive(Debug, Serialize)]
struct GrokRequest {
    model: String,
    messages: Vec<GrokMessage>,
    temperature: f32,
}

#[derive(Debug, Serialize, Deserialize)]
struct GrokMessage {
    role: String,
    content: String,
}

#[derive(Debug, Deserialize)]
struct GrokApiResponse {
    choices: Vec<GrokChoice>,
}

#[derive(Debug, Deserialize)]
struct GrokChoice {
    message: GrokMessage,
}

pub async fn process_natural_request(input: &str, pb: Option<&ProgressBar>) -> Result<()> {
    let mut decisions = DecisionEngine::new();

    set_progress(pb, "🧠 Gemini: traduzindo e extraindo ações...");
    let gemini_summary = match gemini_translate_and_extract(input).await {
        Ok(summary) => {
            decisions.commit(
                "gemini".to_string(),
                "Traduzir e extrair ações".to_string(),
                true,
                "ok".to_string(),
            );
            summary
        }
        Err(err) => {
            decisions.commit(
                "gemini".to_string(),
                "Traduzir e extrair ações".to_string(),
                false,
                err.to_string(),
            );
            GeminiSummary {
                english: input.to_string(),
                keywords: vec![],
                summary: input.to_string(),
                actions: vec![],
            }
        }
    };

    set_progress(pb, "🔥 Claude: limpando comandos perigosos...");
    let claude_sanitized = match claude_sanitize(&gemini_summary).await {
        Ok(sanitized) => {
            decisions.commit(
                "claude".to_string(),
                "Sanitizar ações perigosas".to_string(),
                true,
                "ok".to_string(),
            );
            sanitized
        }
        Err(err) => {
            decisions.commit(
                "claude".to_string(),
                "Sanitizar ações perigosas".to_string(),
                false,
                err.to_string(),
            );
            ClaudeSanitized {
                sanitized_summary: gemini_summary.summary.clone(),
                removed_items: vec![],
                safe_actions: gemini_summary.actions.clone(),
            }
        }
    };

    set_progress(pb, "🚀 Grok: quebrando em tarefinhas...");
    let grok_plan = match grok_decompose(&claude_sanitized).await {
        Ok(plan) => {
            decisions.commit(
                "grok".to_string(),
                "Dividir em tarefinhas".to_string(),
                true,
                "ok".to_string(),
            );
            Some(plan)
        }
        Err(err) => {
            decisions.commit(
                "grok".to_string(),
                "Dividir em tarefinhas".to_string(),
                false,
                err.to_string(),
            );
            None
        }
    };

    let mut chain = ChainOfCaralhoManager::new();
    if let Some(plan) = grok_plan {
        let external = plan
            .tarefinhas
            .into_iter()
            .map(map_grok_tarefinha)
            .collect();
        chain.create_batch_from_external(claude_sanitized.sanitized_summary.clone(), external)?;
    } else {
        chain.create_batch_from_goal(claude_sanitized.sanitized_summary.clone())?;
    }

    chain.scream_chain_of_caralho(&claude_sanitized.sanitized_summary);

    if let Some(batch) = chain.caderninhos.last().map(|b| b.batch_id.clone()) {
        chain.process_batch(&batch).await?;
    }

    chain.process_pilha_async().await?;

    let (accepted, rejected) = decisions.stats();
    set_progress(
        pb,
        &format!(
            "✅ Pipeline finalizado. Decisões: {} aceitas, {} rejeitadas.",
            accepted, rejected
        ),
    );

    Ok(())
}

fn map_grok_tarefinha(task: GrokTarefinha) -> ExternalTarefinhaPlan {
    ExternalTarefinhaPlan {
        titulo: task.titulo,
        descricao: task.descricao,
        priority: task.priority.as_deref().and_then(map_priority),
        complexity: task.complexity.as_deref().and_then(map_complexity),
        estimated_minutes: task.estimated_minutes,
        dependencies: task.dependencies.unwrap_or_default(),
        async_ok: task.async_ok.unwrap_or(true),
    }
}

fn map_priority(value: &str) -> Option<Priority> {
    match value.to_lowercase().as_str() {
        "critical" => Some(Priority::Critical),
        "high" => Some(Priority::High),
        "medium" => Some(Priority::Medium),
        "low" => Some(Priority::Low),
        _ => None,
    }
}

fn map_complexity(value: &str) -> Option<Complexity> {
    match value.to_lowercase().as_str() {
        "junior" => Some(Complexity::Junior),
        "pleno" => Some(Complexity::Pleno),
        "senior" => Some(Complexity::Senior),
        "godmode" | "god_mode" => Some(Complexity::GodMode),
        _ => None,
    }
}

fn set_progress(pb: Option<&ProgressBar>, message: &str) {
    if let Some(pb) = pb {
        pb.set_message(message.to_string());
    } else {
        println!("{}", message);
    }
}

async fn gemini_translate_and_extract(input: &str) -> Result<GeminiSummary> {
    let api_key =
        env::var("GEMINI_API_KEY").context("GEMINI_API_KEY not found in environment")?;

    let prompt = format!(
        "Translate the user request to English. Extract action keywords and summarize intent.\nReturn ONLY JSON with keys: english, keywords, summary, actions.\nUser request: {input}"
    );

    let client = Client::builder()
        .timeout(Duration::from_secs(30))
        .build()?;

    let request_body = GeminiRequest {
        contents: vec![GeminiContent {
            parts: vec![GeminiPart { text: prompt }],
        }],
        generation_config: Some(GeminiConfig {
            temperature: 0.7,
            max_output_tokens: 1024,
        }),
    };

    let response = client
        .post(format!(
            "https://generativelanguage.googleapis.com/v1beta/models/gemini-3-flash-preview:generateContent?key={}",
            api_key
        ))
        .json(&request_body)
        .send()
        .await
        .context("Failed to call Gemini API")?;

    if !response.status().is_success() {
        let error_text = response.text().await?;
        return Err(anyhow::anyhow!("Gemini API error: {}", error_text));
    }

    let api_response: GeminiApiResponse = response.json().await?;
    let output = api_response
        .candidates
        .first()
        .and_then(|c| c.content.parts.first())
        .map(|p| p.text.clone())
        .unwrap_or_default();

    let parsed: GeminiResponse =
        parse_json_response(&output).context("Failed to parse Gemini JSON response")?;

    Ok(GeminiSummary {
        english: parsed.english.unwrap_or_else(|| input.to_string()),
        keywords: parsed.keywords.unwrap_or_default(),
        summary: parsed.summary.unwrap_or_else(|| input.to_string()),
        actions: parsed.actions.unwrap_or_default(),
    })
}

async fn claude_sanitize(summary: &GeminiSummary) -> Result<ClaudeSanitized> {
    // Try multiple possible API key variables for Claude
    let api_key = env::var("GLM_API_KEY")
        .or_else(|_| env::var("GLM_4_6_KEY"))
        .or_else(|_| env::var("GLM_KEY"))
        .or_else(|_| env::var("GLM_API_KEY"))
        .context("GLM_API_KEY or equivalent not found in environment")?;

    let prompt = format!(
        "You are a safety filter. Remove dangerous operations (e.g., rm -rf ./, destructive deletes).\nReturn ONLY JSON with keys: sanitized_summary, removed_items, safe_actions.\nInput summary: {}\nActions: {:?}\nKeywords: {:?}",
        summary.summary, summary.actions, summary.keywords
    );

    let client = Client::builder()
        .timeout(Duration::from_secs(30))
        .build()?;

    let request_body = ClaudeRequest {
        model: "claude-3-5-sonnet-20241022".to_string(),
        max_tokens: 1024,
        messages: vec![ClaudeMessage {
            role: "user".to_string(),
            content: prompt,
        }],
    };

    let response = client
        .post("https://api.anthropic.com/v1/messages")
        .header("x-api-key", api_key)
        .header("anthropic-version", "2023-06-01")
        .header("content-type", "application/json")
        .json(&request_body)
        .send()
        .await
        .context("Failed to call Claude API")?;

    if !response.status().is_success() {
        let error_text = response.text().await?;
        return Err(anyhow::anyhow!("Claude API error: {}", error_text));
    }

    let api_response: ClaudeApiResponse = response.json().await?;
    let output = api_response
        .content
        .first()
        .map(|c| c.text.clone())
        .unwrap_or_default();

    let parsed: ClaudeResponse =
        parse_json_response(&output).context("Failed to parse Claude JSON response")?;

    Ok(ClaudeSanitized {
        sanitized_summary: parsed
            .sanitized_summary
            .unwrap_or_else(|| summary.summary.clone()),
        removed_items: parsed.removed_items.unwrap_or_default(),
        safe_actions: parsed
            .safe_actions
            .unwrap_or_else(|| summary.actions.clone()),
    })
}

async fn grok_decompose(sanitized: &ClaudeSanitized) -> Result<GrokResponse> {
    // Try multiple possible API key variables for Grok
    let api_key = env::var("GROK_API_KEY")
        .or_else(|_| env::var("XAI_API_KEY"))
        .or_else(|_| env::var("GLI_KEY"))
        .or_else(|_| env::var("KAT_KEY"))
        .context("GROK_API_KEY or equivalent not found in environment")?;

    let prompt = format!(
        "Divide the request into tarefinhas for execution. Return ONLY JSON with:\n{{\"tarefinhas\":[{{\"titulo\":\"...\",\"descricao\":\"...\",\"priority\":\"Critical|High|Medium|Low\",\"complexity\":\"Junior|Pleno|Senior|GodMode\",\"estimated_minutes\":10,\"dependencies\":[],\"async_ok\":true}}]}}\nRequest: {}\nSafe actions: {:?}\nRemoved items: {:?}",
        sanitized.sanitized_summary, sanitized.safe_actions, sanitized.removed_items
    );

    let client = Client::builder()
        .timeout(Duration::from_secs(30))
        .build()?;

    let request_body = GrokRequest {
        model: "grok-beta".to_string(),
        messages: vec![GrokMessage {
            role: "user".to_string(),
            content: prompt,
        }],
        temperature: 0.7,
    };

    let response = client
        .post("https://api.x.ai/v1/chat/completions")
        .header("authorization", format!("Bearer {}", api_key))
        .header("content-type", "application/json")
        .json(&request_body)
        .send()
        .await
        .context("Failed to call Grok API")?;

    if !response.status().is_success() {
        let error_text = response.text().await?;
        return Err(anyhow::anyhow!("Grok API error: {}", error_text));
    }

    let api_response: GrokApiResponse = response.json().await?;
    let output = api_response
        .choices
        .first()
        .map(|c| c.message.content.clone())
        .unwrap_or_default();

    // Try to parse as GrokResponse first
    let parsed: Result<GrokResponse> = parse_json_response(&output);
    if let Ok(response) = parsed {
        return Ok(response);
    }

    // Fallback: try parsing as a list of tarefinhas
    let list: Vec<GrokTarefinha> =
        parse_json_response(&output).context("Failed to parse Grok JSON response (list)")?;
    Ok(GrokResponse { tarefinhas: list })
}

fn parse_json_response<T: DeserializeOwned>(raw: &str) -> Result<T> {
    if let Ok(parsed) = serde_json::from_str::<T>(raw) {
        return Ok(parsed);
    }

    let cleaned = strip_code_fences(raw);
    if let Ok(parsed) = serde_json::from_str::<T>(&cleaned) {
        return Ok(parsed);
    }

    let extracted = extract_json_block(&cleaned).context("No JSON block found")?;
    serde_json::from_str::<T>(extracted).context("JSON parse failed after extraction")
}

fn strip_code_fences(input: &str) -> String {
    let trimmed = input.trim();
    if trimmed.starts_with("```") {
        let without = trimmed
            .trim_start_matches("```json")
            .trim_start_matches("```")
            .trim();
        if let Some(end) = without.rfind("```") {
            return without[..end].trim().to_string();
        }
    }
    trimmed.to_string()
}

fn extract_json_block(input: &str) -> Option<&str> {
    let start = input.find('{')?;
    let end = input.rfind('}')?;
    if end > start {
        Some(&input[start..=end])
    } else {
        None
    }
}
