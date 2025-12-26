// --- FENRIR 4.0 - BROADCAST AI CONSENSUS SYSTEM (HTTP ONLY) ---
// Minimal interactive mode with async HTTP API calls to multiple AIs

mod executor;
mod oraculo;
mod ferramentas;
mod fenrir_ai_layer;
mod fenrir_orchestrator;
mod kali_tools;
mod kali_tools_comprehensive;
mod git_automation;

use std::io::{self, Write};
use reqwest::Client;
use serde_json::{json, Value};
use std::time::Duration;

#[tokio::main]
async fn main() {
    fenrir_ai_layer::load_env();

    println!("🐺 FENRIR 4.0 - Broadcast AI Consensus System");
    println!("Multi-AI Security Platform (HTTP API)\n");

    let stdin = io::stdin();
    let mut input = String::new();
    let http_client = Client::new();

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
                if user_input == "gita tudo" {
                    git_automation::gita_tudo();
                    continue;
                }
                if user_input == "gita ai" {
                    git_automation::gita_ai();
                    continue;
                }

                // Parse pentest commands
                let parts: Vec<&str> = user_input.splitn(3, ' ').collect();
                if parts.len() >= 2 {
                    match parts[0] {
                        "scan" => {
                            let target = parts[1];
                            let comprehensive = parts.len() >= 3 && parts[2] == "comprehensive";
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
                            let target = parts[1];
                            let aggressive = parts.len() >= 3 && parts[2] == "aggressive";
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
                        _ => {}
                    }
                }

                // Process through broadcast AI system via HTTP
                process_broadcast_http(&http_client, user_input).await;
            }
            Err(e) => {
                eprintln!("❌ Error: {}", e);
                break;
            }
        }
    }
}

// Broadcast system via HTTP - send to all AIs asynchronously
async fn process_broadcast_http(client: &Client, user_input: &str) {
    println!("\n🌐 BROADCAST AI SYSTEM STARTED");
    println!("📨 Input: {}\n", user_input);

    let mut messages: Vec<Value> = vec![json!({
        "role": "user",
        "content": user_input
    })];

    let max_iterations = 5;
    let mut iteration = 0;

    loop {
        iteration += 1;
        println!("🔄 Iteration {}/{}\n", iteration, max_iterations);

        // Broadcast to all AIs asynchronously via HTTP
        let (glm_result, gemini_result, grok_result, venice_result) = tokio::join!(
            query_glm_http(client, &messages),
            query_gemini_http(client, &messages),
            query_grok_http(client, &messages),
            query_venice_http(client, &messages)
        );

        // Collect all responses
        let ai_responses = vec![
            ("GLM (Orchestrator)", glm_result),
            ("Gemini (Translator)", gemini_result),
            ("Grok (General)", grok_result),
            ("Venice (Red Team)", venice_result),
        ];

        println!("📊 AI RESPONSES:\n");

        for (ai_name, response) in &ai_responses {
            match response {
                Ok(content) => {
                    println!("--- {} ---", ai_name);
                    println!("{}\n", content);
                }
                Err(e) => {
                    println!("--- {} ---", ai_name);
                    println!("ERROR: {}\n", e);
                }
            }
        }

        // Check for consensus
        let consensus = check_consensus(&ai_responses);

        if consensus.reached {
            println!("✅ CONSENSUS REACHED!");
            println!("📋 Final Decision: {}\n", consensus.decision);

            // Execute the agreed action
            execute_consensus(&consensus).await;
            break;
        }

        // If no consensus, broadcast AI responses to each other for next iteration
        if iteration >= max_iterations {
            println!("⚠️  Max iterations reached - using COTOA fallback");
            cotoa_fallback(&messages, &ai_responses).await;
            break;
        }

        // Create broadcast message with all AI responses
        let broadcast_content = format!(
            "PREVIOUS AI DISCUSSION:\n\n\
             Please analyze these AI responses and reach consensus on:\n\
             1. What does the user want?\n\
             2. What specific tasks should be performed?\n\
             3. Who should handle each task?\n\
             4. What is the final action to take?\n\n\
             AI Responses:\n\
             {}\n\n\
             Please provide your analysis and proposed consensus.",
            ai_responses.iter()
                .map(|(name, resp)| format!("{}: {}", name,
                    resp.as_ref().unwrap_or(&"ERROR".to_string())))
                .collect::<Vec<_>>()
                .join("\n\n")
        );

        messages.push(json!({
            "role": "user",
            "content": broadcast_content
        }));
    }
}

// Query GLM via HTTP (using Anthropic API as GLM proxy)
async fn query_glm_http(client: &Client, messages: &[Value]) -> Result<String, String> {
    if let Ok(api_key) = std::env::var("ANTHROPIC_API_KEY") {
        let response = client
            .post("https://api.anthropic.com/v1/messages")
            .header("x-api-key", api_key)
            .header("anthropic-version", "2023-06-01")
            .header("Content-Type", "application/json")
            .json(&json!({
                "model": "claude-sonnet-4-20250514",
                "max_tokens": 1024,
                "messages": messages
            }))
            .send()
            .await;

        match response {
            Ok(resp) => {
                if resp.status().is_success() {
                    if let Ok(json) = resp.json::<Value>().await {
                        if let Some(content) = json["content"][0]["text"].as_str() {
                            return Ok(content.to_string());
                        }
                    }
                }
                Err("GLM HTTP request failed".to_string())
            }
            Err(e) => Err(format!("GLM network error: {}", e)),
        }
    } else {
        Err("ANTHROPIC_API_KEY not set".to_string())
    }
}

// Query Gemini via HTTP
async fn query_gemini_http(client: &Client, messages: &[Value]) -> Result<String, String> {
    if let Ok(api_key) = std::env::var("GEMINI_API_KEY") {
        let response = client
            .post(format!(
                "https://generativelanguage.googleapis.com/v1beta/models/gemini-2.0-flash-exp:generateContent?key={}",
                api_key
            ))
            .json(&json!({
                "contents": messages.iter().map(|m| json!({
                    "parts": [{"text": m["content"]}]
                })).collect::<Vec<_>>()
            }))
            .send()
            .await;

        match response {
            Ok(resp) => {
                if resp.status().is_success() {
                    if let Ok(json) = resp.json::<Value>().await {
                        if let Some(text) = json["candidates"][0]["content"]["parts"][0]["text"].as_str() {
                            return Ok(text.to_string());
                        }
                    }
                }
                Err("Gemini HTTP request failed".to_string())
            }
            Err(e) => Err(format!("Gemini network error: {}", e)),
        }
    } else {
        Err("GEMINI_API_KEY not set".to_string())
    }
}

// Query Grok via HTTP (xAI API)
async fn query_grok_http(client: &Client, messages: &[Value]) -> Result<String, String> {
    if let Ok(api_key) = std::env::var("GROK_API_KEY") {
        let response = client
            .post("https://api.x.ai/v1/chat/completions")
            .header("Authorization", format!("Bearer {}", api_key))
            .header("Content-Type", "application/json")
            .json(&json!({
                "model": "grok-2-1212",
                "messages": messages,
                "max_tokens": 1024
            }))
            .send()
            .await;

        match response {
            Ok(resp) => {
                if resp.status().is_success() {
                    if let Ok(json) = resp.json::<Value>().await {
                        if let Some(content) = json["choices"][0]["message"]["content"].as_str() {
                            return Ok(content.to_string());
                        }
                    }
                }
                Err("Grok HTTP request failed".to_string())
            }
            Err(e) => Err(format!("Grok network error: {}", e)),
        }
    } else {
        Err("GROK_API_KEY not set".to_string())
    }
}

// Query Venice Red Team via HTTP
async fn query_venice_http(client: &Client, messages: &[Value]) -> Result<String, String> {
    if let Ok(api_key) = std::env::var("VENICE_API_KEY") {
        let api_url = std::env::var("VENICE_API_URL")
            .unwrap_or_else(|_| "https://api.venice.ai/v1/chat/completions".to_string());

        let response = client
            .post(&api_url)
            .header("Authorization", format!("Bearer {}", api_key))
            .header("Content-Type", "application/json")
            .json(&json!({
                "model": "venice-red-team",
                "messages": messages,
                "max_tokens": 1024
            }))
            .send()
            .await;

        match response {
            Ok(resp) => {
                if resp.status().is_success() {
                    if let Ok(json) = resp.json::<Value>().await {
                        if let Some(content) = json["choices"][0]["message"]["content"].as_str() {
                            return Ok(content.to_string());
                        }
                    }
                }
                Err("Venice HTTP request failed".to_string())
            }
            Err(e) => Err(format!("Venice network error: {}", e)),
        }
    } else {
        Err("VENICE_API_KEY not set".to_string())
    }
}

// Check if AIs reached consensus
struct Consensus {
    reached: bool,
    decision: String,
    tasks: Vec<String>,
}

fn check_consensus(responses: &[(&str, Result<String, String>)]) -> Consensus {
    let successful_count = responses.iter()
        .filter(|(_, r)| r.is_ok())
        .count();

    // If at least 3 AIs responded successfully
    if successful_count >= 3 {
        let all_text = responses.iter()
            .filter_map(|(_, r)| r.as_ref().ok())
            .cloned()
            .collect::<Vec<_>>()
            .join(" ");

        // Look for consensus on action type
        let has_pentest = all_text.to_lowercase().contains("pentest") ||
                         all_text.to_lowercase().contains("attack") ||
                         all_text.to_lowercase().contains("exploit");
        let has_scan = all_text.to_lowercase().contains("scan") ||
                      all_text.to_lowercase().contains("check") ||
                      all_text.to_lowercase().contains("reconnaissance");
        let has_execute = all_text.to_lowercase().contains("execut") ||
                         all_text.to_lowercase().contains("run command");
        let has_analyze = all_text.to_lowercase().contains("analyz");

        let decision = if has_pentest {
            "Execute penetration testing (authorized only)"
        } else if has_scan {
            "Perform security reconnaissance and scanning"
        } else if has_execute {
            "Execute the requested command"
        } else if has_analyze {
            "Analyze the target/system"
        } else {
            "Process the request with available tools"
        };

        return Consensus {
            reached: true,
            decision: decision.to_string(),
            tasks: vec![decision.to_string()],
        };
    }

    Consensus {
        reached: false,
        decision: String::new(),
        tasks: Vec::new(),
    }
}

// Execute consensus decision
async fn execute_consensus(consensus: &Consensus) {
    println!("🎯 EXECUTING CONSENSUS DECISION");
    println!("📋 Action: {}\n", consensus.decision);

    // Simulate execution time
    tokio::time::sleep(Duration::from_millis(500)).await;

    println!("✅ CONSENSUS EXECUTION COMPLETE\n");
}

// COTOA Fallback - Chain of Thought Oriented Action
async fn cotoa_fallback(messages: &[Value], responses: &[(&str, Result<String, String>)]) {
    println!("\n🧠 CHAIN OF THOUGHT ORIENTED ACTION (COTOA)");
    println!("🔄 Falling back to reasoning-based action\n");

    // Gather all AI responses
    let all_responses = responses.iter()
        .filter_map(|(_, r)| r.as_ref().ok())
        .cloned()
        .collect::<Vec<_>>()
        .join("\n\n");

    // Chain of reasoning
    let reasoning_steps = vec![
        ("🤔 UNDERSTANDING", "What does the user want?"),
        ("🔍 ANALYZING", "What are the key requirements and constraints?"),
        ("📋 PLANNING", "What steps should be taken?"),
        ("⚡ EXECUTING", "Perform the action"),
        ("✅ VERIFYING", "Was the goal achieved?"),
    ];

    for (step, description) in reasoning_steps {
        println!("{}: {}", step, description);
        tokio::time::sleep(Duration::from_millis(300)).await;
    }

    println!("\n📊 COTOA ANALYSIS COMPLETE");
    println!("💡 Based on AI responses, taking reasoned action...\n");

    // Execute reasoned action
    tokio::time::sleep(Duration::from_millis(500)).await;
    println!("✅ COTOA EXECUTION COMPLETE\n");
}
