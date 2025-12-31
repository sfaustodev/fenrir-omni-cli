// --- FENRIR 4.0 - DIRECT AI COMMANDS ---
// Simple interface with direct AI calls

mod executor;
mod oraculo;
mod ferramentas;
mod fenrir_ai_layer;
mod fenrir_orchestrator;
mod kali_tools;
mod kali_tools_comprehensive;
mod git_automation;
mod cli;
mod confirm;
mod http_client;
mod secrets;
mod metrics;
mod health;
mod circuit_breaker;
mod solana;
mod zcash;
mod liquidity;
mod plugins;
mod wrapper;
mod sandbox;
mod bugbounty;
mod osint;
mod net;
mod cli;
mod confirm;
mod http_client;
mod secrets;
mod metrics;
mod health;
mod circuit_breaker;
mod solana;
mod zcash;
mod liquidity;
mod plugins;
mod wrapper;
mod sandbox;
mod bugbounty;
mod osint;
mod net;

use std::io::{self, Write};
use reqwest::Client;
use serde_json::{json, Value};
use executor::FenrirTask;

// AI Command Translation System
struct ParsedCommand {
    command: String,
    explanation: String,
}

// Use AI to translate natural language into bash commands
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

Examples:
User: "cd .."
You: {"command": "cd ..", "explanation": "Change to parent directory"}

User: "listar arquivos"
You: {"command": "ls -la", "explanation": "List all files"}

User: "onde estou"
You: {"command": "pwd", "explanation": "Print working directory"}

User: "limpar tela"
You: {"command": "clear", "explanation": "Clear terminal screen"}

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

    let json_response: Value = response
        .json()
        .await
        .map_err(|e| format!("Failed to parse response: {}", e))?;

    let content = json_response["choices"][0]["message"]["content"]
        .as_str()
        .ok_or_else(|| "No content in response".to_string())?;

    // Parse the JSON response
    let parsed: Value = serde_json::from_str(content)
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
    })
}

#[tokio::main]
async fn main() {
    fenrir_ai_layer::load_env();

    if std::env::args().len() > 1 {
        if let Err(err) = cli::run_cli().await {
            eprintln!("❌ {}", err);
        }
        return;
    }

    if std::env::args().len() > 1 {
        if let Err(err) = cli::run_cli().await {
            eprintln!("❌ {}", err);
        }
        return;
    }

    println!("🐺 FENRIR 4.0 - AI-Powered Command Translation");
    println!("Security Testing Platform\n");

    let stdin = io::stdin();
    let mut input = String::new();
    let http_client = Client::new();

    println!("🎯 Special Commands:");
    println!("  scan <target> [comprehensive]  - Security scan");
    println!("  bite <target> [aggressive]     - Penetration test");
    println!("  grok \"prompt\"                  - Query Grok AI");
    println!("  gita tudo                       - Git: add, commit, push");
    println!("  gita ai                         - Git: add, commit");
    println!("  exit                            - Exit");
    println!("\n💬 OR just type natural language (English/Portuguese):");
    println!("   \"cd ..\"  \"listar arquivos\"  \"onde estou\"  \"limpar\"\n");

    loop {
        print!("🐺 fenrir> ");
        io::stdout().flush().unwrap();

        input.clear();
        match stdin.read_line(&mut input) {
            Ok(0) => {
                println!("\n🐺 Exiting...\n");
                break;
            }
            Ok(_) => {
                let user_input = input.trim();
                if user_input.is_empty() {
                    continue;
                }
                if user_input == "exit" || user_input == "quit" || user_input == "sair" {
                    println!("\n🐺 Bye!...\n");
                    break;
                }

                // Git commands
                if user_input == "gita tudo" {
                    git_automation::gita_tudo();
                    continue;
                }
                if user_input == "gita ai" {
                    git_automation::gita_ai();
                    continue;
                }

                // Parse commands with arguments
                let parts: Vec<&str> = user_input.splitn(2, ' ').collect();
                if parts.len() >= 2 {
                    match parts[0] {
                        "scan" => {
                            let target = parts[1].split_whitespace().next().unwrap_or("");
                            let comprehensive = parts[1].contains("comprehensive");
                            if target.is_empty() {
                                println!("❌ Usage: scan <target> [comprehensive]\n");
                                continue;
                            }
                            let config = kali_tools::ScanConfig {
                                target: target.to_string(),
                                scan_type: if comprehensive {
                                    kali_tools::ScanType::Comprehensive
                                } else {
                                    kali_tools::ScanType::Quick
                                },
                                depth: if comprehensive {
                                    kali_tools::ScanDepth::Deep
                                } else {
                                    kali_tools::ScanDepth::Surface
                                },
                                output_format: kali_tools::ScanOutput::Console,
                            };
                            match kali_tools::scan(target, config).await {
                                Ok(result) => {
                                    println!("\n✅ SCAN COMPLETE");
                                    println!("🎯 Target: {}", result.target);
                                    println!("🔍 Open Ports: {}", result.open_ports.len());
                                    println!("🛡️  Risk Score: {}/100", result.risk_score);
                                    println!("📋 Security Plan:\n{}\n", result.security_plan);
                                }
                                Err(e) => println!("❌ Scan failed: {}\n", e),
                            }
                            continue;
                        }
                        "bite" | "morder" => {
                            let target = parts[1].split_whitespace().next().unwrap_or("");
                            let aggressive = parts[1].contains("aggressive");
                            if target.is_empty() {
                                println!("❌ Usage: bite <target> [aggressive]\n");
                                continue;
                            }
                            let config = kali_tools::BiteConfig {
                                target: target.to_string(),
                                tools: vec![],
                                intensity: if aggressive {
                                    kali_tools::BiteIntensity::Aggressive
                                } else {
                                    kali_tools::BiteIntensity::Cautious
                                },
                                categories: vec![],
                                auto_exploit: false,
                                report_path: None,
                            };
                            match kali_tools::bite(target, config).await {
                                Ok(result) => {
                                    println!("\n✅ BITE COMPLETE - FENRIR HAS DEVOURED THE TARGET");
                                    println!("🎯 Success: {}", result.success);
                                    println!("🔍 Findings: {}", result.findings.len());
                                    println!("💥 Vulnerabilities: {}", result.vulnerabilities.len());
                                    if !result.vulnerabilities.is_empty() {
                                        println!("📊 Vulnerabilities:\n{}", result.vulnerabilities.join("\n"));
                                    }
                                    println!("📄 Report:\n{}\n", result.report);
                                }
                                Err(e) => println!("❌ Bite failed: {}\n", e),
                            }
                            continue;
                        }
                        "grok" => {
                            let prompt = parts[1].trim_matches('"').trim_matches('\'');
                            match query_grok(&http_client, prompt).await {
                                Ok(response) => {
                                    println!("\n🤖 GROK RESPONSE:\n{}\n", response);
                                }
                                Err(e) => {
                                    println!("❌ Grok error: {}\n", e);
                                }
                            }
                            continue;
                        }
                        _ => {
                            // Unknown command - use AI to translate and execute
                            match translate_with_ai(&http_client, user_input).await {
                                Ok(parsed) => {
                                    println!("🤖 AI understood: {}", parsed.explanation);
                                    println!("🔧 Executing: {}\n", parsed.command);
                                    executor::handle_execute_command(&parsed.command);
                                }
                                Err(e) => {
                                    println!("❌ AI translation failed: {}\n", e);
                                    println!("💡 Try commands like: scan, bite, grok, gita tudo, or natural language\n");
                                }
                            }
                        }
                    }
                } else {
                    // Single word or simple command - use AI translation
                    match translate_with_ai(&http_client, user_input).await {
                        Ok(parsed) => {
                            println!("🤖 AI understood: {}", parsed.explanation);
                            println!("🔧 Executing: {}\n", parsed.command);
                            executor::handle_execute_command(&parsed.command);
                        }
                        Err(e) => {
                            println!("❌ AI translation failed: {}\n", e);
                            println!("💡 Try commands like: scan, bite, grok, gita tudo, or natural language\n");
                        }
                    }
                }
            }
            Err(e) => {
                eprintln!("❌ Error: {}", e);
                break;
            }
        }
    }
}

async fn query_grok(client: &Client, prompt: &str) -> Result<String, String> {
    let api_key = std::env::var("GROK_API_KEY")
        .or_else(|_| std::env::var("XAI_API_KEY"))
        .map_err(|_| "GROK_API_KEY or XAI_API_KEY not set".to_string())?;

    let response = client
        .post("https://api.x.ai/v1/chat/completions")
        .header("Authorization", format!("Bearer {}", api_key))
        .header("Content-Type", "application/json")
        .json(&json!({
            "model": "grok-3",
            "messages": [{"role": "user", "content": prompt}],
            "max_tokens": 4096
        }))
        .send()
        .await
        .map_err(|e| format!("Request failed: {}", e))?;

    if !response.status().is_success() {
        let status = response.status();
        let error_text = response.text().await.unwrap_or_default();
        return Err(format!("API error ({}): {}", status, error_text));
    }

    let json: Value = response
        .json()
        .await
        .map_err(|e| format!("Failed to parse response: {}", e))?;

    json["choices"][0]["message"]["content"]
        .as_str()
        .map(|s| s.to_string())
        .ok_or_else(|| "No content in response".to_string())
}
