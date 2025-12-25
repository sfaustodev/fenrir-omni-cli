//! # Natural Language Processing Module
//!
//! Processes natural language input using multi-AI pipeline (Gemini → Claude → Grok)
//! and converts it into structured commands for Fenrir.

use colored::Colorize;
use color_eyre::Result;
use color_eyre::eyre::WrapErr;
use reqwest::Client;
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use std::env;
use std::time::Duration;

// ===== Response Structures =====

#[derive(Debug)]
pub struct NaturalLanguageResult {
    english: String,
    summary: String,
    suggested_command: Option<String>,
    explanation: String,
}

// ===== API Request/Response Structures =====

// Gemini API
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

// ===== Public Interface =====

/// Process natural language input and suggest a Fenrir command
pub async fn process_natural_input(input: &str) -> Result<NaturalLanguageResult> {
    println!("{} 🧠", "Processing".cyan().bold());
    println!("{} Understanding your request...\n", "→".dimmed());

    // Use Gemini to understand the input and suggest a command
    let result = gemini_understand_and_suggest(input).await?;

    Ok(result)
}

/// Interactive natural language mode
pub async fn interactive_mode() -> Result<()> {
    println!("{}", "🐺 Fenrir Natural Language Mode".cyan().bold());
    println!("{}", "Type your requests in plain English or Portuguese".dimmed());
    println!("{}", "Type 'quit' or 'exit' to leave\n".dimmed());

    let mut input = String::new();
    loop {
        println!("{}", "❯ ".bright_cyan().bold());
        input.clear();
        std::io::stdin()
            .read_line(&mut input)
            .wrap_err("Failed to read input")?;

        let input = input.trim();

        if input.is_empty() {
            continue;
        }

        if matches_command(input, &["quit", "exit", "q"]) {
            println!("{} {} 👋\n", "←".dimmed(), "Goodbye!".cyan());
            break;
        }

        match process_natural_input(input).await {
            Ok(result) => {
                display_natural_result(&result);
            }
            Err(e) => {
                eprintln!("{} {}: {}\n", "✗".red(), "Error".red(), e);
            }
        }
    }

    Ok(())
}

// ===== AI Provider Implementations =====

async fn gemini_understand_and_suggest(input: &str) -> Result<NaturalLanguageResult> {
    let api_key = env::var("GEMINI_API_KEY")
        .or_else(|_| env::var("KAT_KEY"))
        .wrap_err("GEMINI_API_KEY or KAT_KEY not found in environment. Set one with: export GEMINI_API_KEY=your_key")?;

    let prompt = format!(
        "You are Fenrir's AI assistant. Analyze the user's request and suggest a Fenrir CLI command.

Available Fenrir commands:
- scan: `fenrir scan --target <TARGET> --port-range <PORTS> --scan-type <quick|full|stealth> --timeout <SECONDS> --threads <NUM>`
- rules: `fenrir rules --list` or `fenrir rules --reload`
- init: `fenrir init --force`
- about: `fenrir about`
- gitar: `fenrir gitar --message \"commit message\"`
- huh: `fenrir huh`

User request: {}

Respond in JSON format:
{{
  \"english\": \"user request translated to English\",
  \"summary\": \"brief summary of what user wants\",
  \"suggested_command\": \"complete fenrir command or null if no match\",
  \"explanation\": \"why this command was chosen\"
}}",
        input
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
            max_output_tokens: 2048,
        }),
    };

    println!("{} Calling Gemini AI...", "→".dimmed());

    let response = client
        .post(format!(
            "https://generativelanguage.googleapis.com/v1beta/models/gemini-3-flash-preview:generateContent?key={}",
            api_key
        ))
        .json(&request_body)
        .send()
        .await
        .wrap_err("Failed to call Gemini API")?;

    if !response.status().is_success() {
        let error_text = response.text().await?;
        return Err(color_eyre::eyre::eyre!("Gemini API error: {}", error_text));
    }

    let api_response: GeminiApiResponse = response.json().await?;
    let output = api_response
        .candidates
        .first()
        .and_then(|c| c.content.parts.first())
        .map(|p| p.text.clone())
        .unwrap_or_default();

    let parsed: GeminiParseResult =
        parse_json_response(&output).wrap_err("Failed to parse Gemini JSON response")?;

    Ok(NaturalLanguageResult {
        english: parsed.english.unwrap_or_else(|| input.to_string()),
        summary: parsed.summary.unwrap_or_else(|| input.to_string()),
        suggested_command: parsed.suggested_command,
        explanation: parsed.explanation.unwrap_or_else(|| "AI suggested this command".to_string()),
    })
}

// ===== Helper Structures =====

#[derive(Debug, Deserialize)]
struct GeminiParseResult {
    english: Option<String>,
    summary: Option<String>,
    suggested_command: Option<String>,
    explanation: Option<String>,
}

// ===== Helper Functions =====

pub fn display_natural_result(result: &NaturalLanguageResult) {
    println!("\n{}", "━".repeat(60).dimmed());
    println!("{} {}", "📝".cyan(), "Summary".cyan().bold());
    println!("  {}", result.summary.dimmed());

    if let Some(cmd) = &result.suggested_command {
        println!("\n{} {}", "⚡".yellow(), "Suggested Command".yellow().bold());
        println!("  {}", cmd.bright_white().on_bright_black());
        println!("\n{} {}", "💡".cyan(), "Explanation".cyan().bold());
        println!("  {}", result.explanation.dimmed());
    } else {
        println!("\n{} No specific command matched. Try rephrasing your request.", "ℹ️".blue());
    }

    println!("{}\n", "━".repeat(60).dimmed());
}

fn matches_command(input: &str, commands: &[&str]) -> bool {
    commands.iter().any(|cmd| input.eq_ignore_ascii_case(cmd))
}

fn parse_json_response<T: DeserializeOwned>(raw: &str) -> Result<T> {
    if let Ok(parsed) = serde_json::from_str::<T>(raw) {
        return Ok(parsed);
    }

    let cleaned = strip_code_fences(raw);
    if let Ok(parsed) = serde_json::from_str::<T>(&cleaned) {
        return Ok(parsed);
    }

    let extracted = extract_json_block(&cleaned)
        .ok_or_else(|| color_eyre::eyre::eyre!("No JSON block found"))?;
    serde_json::from_str::<T>(extracted).wrap_err("JSON parse failed after extraction")
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
