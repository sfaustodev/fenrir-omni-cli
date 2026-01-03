use chrono::Utc;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::fs::{self, OpenOptions};
use std::io::{BufRead, BufReader, Write};
use std::path::PathBuf;

#[derive(Debug, Clone)]
pub struct ParsedCommand {
    pub command: String,
    pub explanation: String,
    pub source: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct CommandLogEntry {
    timestamp: String,
    input: String,
    command: String,
    explanation: String,
    source: String,
    valid: bool,
}

#[derive(Debug)]
struct Candidate {
    command: String,
    explanation: String,
    confidence: u8,
    source: String,
}

/// Faz parsing com redundância e logging.
pub async fn parse_command(client: &Client, user_input: &str) -> Result<ParsedCommand, String> {
    let mut candidates = Vec::new();
    if let Some(history) = load_from_history(user_input) {
        candidates.push(Candidate {
            command: history.command,
            explanation: "Reuso de histórico confiável".to_string(),
            confidence: 80,
            source: "history".to_string(),
        });
    }

    match translate_with_ai(client, user_input).await {
        Ok(parsed) => {
            let valid = validate_command(&parsed.command);
            log_entry(user_input, &parsed.command, &parsed.explanation, "ai", valid);
            if valid {
                candidates.push(Candidate {
                    command: parsed.command,
                    explanation: parsed.explanation,
                    confidence: 60,
                    source: "ai".to_string(),
                });
            }
        }
        Err(err) => {
            log_entry(user_input, "echo 'AI failed'", &err, "ai_error", false);
        }
    }

    if let Some(heuristic) = heuristic_parse(user_input) {
        candidates.push(heuristic);
    }

    if let Some(selected) = candidates.into_iter().max_by_key(|c| c.confidence) {
        log_entry(
            user_input,
            &selected.command,
            &selected.explanation,
            &format!("selected:{}", selected.source),
            true,
        );
        return Ok(ParsedCommand {
            command: selected.command,
            explanation: selected.explanation,
            source: selected.source,
        });
    }

    Err("Falha ao interpretar entrada".to_string())
}

fn validate_command(command: &str) -> bool {
    if command.trim().is_empty() {
        return false;
    }
    if command.contains('\n') || command.contains('\r') {
        return false;
    }
    if command.len() > 4096 {
        return false;
    }
    true
}

fn heuristic_parse(input: &str) -> Option<Candidate> {
    let normalized = input.trim().to_lowercase();
    if normalized == "listar" || normalized.contains("listar arquivos") {
        return Some(Candidate {
            command: "ls -la".to_string(),
            explanation: "Listar arquivos".to_string(),
            confidence: 90,
            source: "heuristic".to_string(),
        });
    }
    if normalized == "onde estou" || normalized == "pwd" {
        return Some(Candidate {
            command: "pwd".to_string(),
            explanation: "Mostrar diretório atual".to_string(),
            confidence: 90,
            source: "heuristic".to_string(),
        });
    }
    if normalized == "limpar" || normalized == "limpar tela" || normalized == "clear" {
        return Some(Candidate {
            command: "clear".to_string(),
            explanation: "Limpar terminal".to_string(),
            confidence: 90,
            source: "heuristic".to_string(),
        });
    }
    if normalized.starts_with("cd ") {
        return Some(Candidate {
            command: input.trim().to_string(),
            explanation: "Trocar diretório".to_string(),
            confidence: 85,
            source: "heuristic".to_string(),
        });
    }
    None
}

fn history_path() -> PathBuf {
    let base = dirs::config_dir().unwrap_or_else(|| PathBuf::from("."));
    base.join("fenrir").join("command_log.jsonl")
}

fn log_entry(input: &str, command: &str, explanation: &str, source: &str, valid: bool) {
    let path = history_path();
    if let Some(parent) = path.parent() {
        let _ = fs::create_dir_all(parent);
    }
    let entry = CommandLogEntry {
        timestamp: Utc::now().to_rfc3339(),
        input: input.to_string(),
        command: command.to_string(),
        explanation: explanation.to_string(),
        source: source.to_string(),
        valid,
    };
    if let Ok(mut file) = OpenOptions::new().create(true).append(true).open(path) {
        let _ = writeln!(file, "{}", serde_json::to_string(&entry).unwrap_or_default());
    }
}

fn load_from_history(input: &str) -> Option<CommandLogEntry> {
    let path = history_path();
    let file = fs::File::open(path).ok()?;
    let reader = BufReader::new(file);
    reader
        .lines()
        .flatten()
        .filter_map(|line| serde_json::from_str::<CommandLogEntry>(&line).ok())
        .rev()
        .find(|entry| entry.input == input && entry.valid)
}

async fn translate_with_ai(client: &Client, user_input: &str) -> Result<ParsedCommand, String> {
    let api_key = std::env::var("GROK_API_KEY")
        .or_else(|_| std::env::var("XAI_API_KEY"))
        .map_err(|_| "GROK_API_KEY or XAI_API_KEY not set".to_string())?;

    let system_prompt = r#"You are a command-line translator. Convert the user's natural language request into a bash command.

Rules:
1. Return ONLY a valid JSON object with TWO fields: "command" and "explanation"
2. "command": the exact bash command to execute
3. "explanation": brief explanation of what the command does
4. For simple requests like "cd .." or "listar", return the direct bash equivalent
5. Support both English and Portuguese
6. If the request is unclear, return {"command": "echo 'Could not understand'", "explanation": "Unable to parse"}

Return ONLY the JSON, no other text."#;

    let response = client
        .post("https://api.x.ai/v1/chat/completions")
        .header("Authorization", format!("Bearer {}", api_key))
        .header("Content-Type", "application/json")
        .json(&json!({
            "model": "grok-3",
            "messages": [
                {"role": "system", "content": system_prompt},
                {"role": "user", "content": user_input}
            ],
            "max_tokens": 500,
            "temperature": 0.3
        }))
        .send()
        .await
        .map_err(|e| format!("Request failed: {}", e))?;

    if !response.status().is_success() {
        let status = response.status();
        let error_text = response.text().await.unwrap_or_default();
        return Err(format!("API error ({}): {}", status, error_text));
    }

    let json_response: serde_json::Value = response
        .json()
        .await
        .map_err(|e| format!("Failed to parse response: {}", e))?;

    let content = json_response["choices"][0]["message"]["content"]
        .as_str()
        .ok_or_else(|| "No content in response".to_string())?;

    let parsed: serde_json::Value = serde_json::from_str(content)
        .map_err(|_| format!("Invalid JSON response: {}", content))?;

    let command = parsed["command"]
        .as_str()
        .unwrap_or("echo 'Invalid command'")
        .to_string();

    let explanation = parsed["explanation"]
        .as_str()
        .unwrap_or("No explanation")
        .to_string();

    Ok(ParsedCommand {
        command,
        explanation,
        source: "ai".to_string(),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn heuristic_matches() {
        let parsed = heuristic_parse("listar arquivos").unwrap();
        assert_eq!(parsed.command, "ls -la");
    }

    #[test]
    fn validation_blocks_newlines() {
        assert!(!validate_command("ls\nrm -rf /"));
        assert!(validate_command("ls -la"));
    }

    #[test]
    fn log_roundtrip() {
        let dir = tempfile::tempdir().unwrap();
        std::env::set_var("XDG_CONFIG_HOME", dir.path());
        log_entry("pwd", "pwd", "ok", "test", true);
        let entry = load_from_history("pwd");
        assert!(entry.is_some());
    }
}
