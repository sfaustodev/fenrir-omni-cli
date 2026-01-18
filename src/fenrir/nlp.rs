use chrono::Utc;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::fs::{self, OpenOptions};
use std::io::{BufRead, BufReader, Write};
use std::path::PathBuf;
use std::process::{Command, Stdio};
use std::sync::Arc;
use tokio::sync::Semaphore;
use tokio::task;
use regex::Regex;

// ============================================================================
// NEW FENRIR NLP PARADIGM - ZAI ORCHESTRATOR
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParsedCommand {
    pub command: String,
    pub explanation: String,
    pub source: String,
    pub subject: Option<String>,      // Target (what to attack/scan)
    pub context: Option<String>,      // Strategy/instructions
    pub keywords: Vec<String>,        // Attack types/sequences to trigger
    pub confidence: f32,
    pub needs_clarification: bool,
    pub clarification_questions: Vec<String>,
}

#[derive(Debug, Clone)]
struct SmartAttackSequence {
    pub keyword: String,
    pub description: String,
    pub tools: Vec<String>,
    pub stealth_first: bool,
    pub async_execution: bool,
    pub memory_limit_mb: u64,
}

// ============================================================================
// KEYWORD-TRIGGERED AUTOMATION SEQUENCES
// ============================================================================

fn get_smart_sequences() -> Vec<SmartAttackSequence> {
    vec![
        SmartAttackSequence {
            keyword: "password".to_string(),
            description: "Complete password cracking sequence".to_string(),
            tools: vec![
                "cewl".to_string(),      // Generate wordlist from target
                "crunch".to_string(),    // Generate custom wordlists
                "hydra".to_string(),     // Online brute force
                "hashcat".to_string(),   // GPU cracking
                "john".to_string(),      // CPU cracking
                "patator".to_string(),   // Multi-purpose brute forcer
            ],
            stealth_first: true,
            async_execution: true,
            memory_limit_mb: 666,
        },
        SmartAttackSequence {
            keyword: "scan".to_string(),
            description: "Network scanning sequence".to_string(),
            tools: vec![
                "nmap".to_string(),      // Stealth scan first
                "masscan".to_string(),   // Fast port scan
                "rustscan".to_string(),  // Modern scanner
                "nikto".to_string(),     // Web server scan
                "nuclei".to_string(),    // Vulnerability scan
            ],
            stealth_first: true,
            async_execution: true,
            memory_limit_mb: 666,
        },
        SmartAttackSequence {
            keyword: "social".to_string(),
            description: "Social engineering reconnaissance".to_string(),
            tools: vec![
                "theHarvester".to_string(),
                "sherlock".to_string(),
                "maltego".to_string(),
                "recon-ng".to_string(),
                "spiderfoot".to_string(),
            ],
            stealth_first: true,
            async_execution: false,
            memory_limit_mb: 2048,
        },
        SmartAttackSequence {
            keyword: "web".to_string(),
            description: "Web application testing".to_string(),
            tools: vec![
                "nikto".to_string(),
                "gobuster".to_string(),
                "ffuf".to_string(),
                "sqlmap".to_string(),
                "nuclei".to_string(),
            ],
            stealth_first: true,
            async_execution: true,
            memory_limit_mb: 666,
        },
        SmartAttackSequence {
            keyword: "wireless".to_string(),
            description: "Wireless network attacks".to_string(),
            tools: vec![
                "aircrack-ng".to_string(),
                "wifite".to_string(),
                "reaver".to_string(),
                "bully".to_string(),
                "kismet".to_string(),
            ],
            stealth_first: true,
            async_execution: false,
            memory_limit_mb: 2048,
        },
        SmartAttackSequence {
            keyword: "oauth".to_string(),
            description: "OAuth2 security testing".to_string(),
            tools: vec![
                "burpsuite".to_string(),
                "oauth2-tool".to_string(),
                "evilginx2".to_string(),
                "modlishka".to_string(),
            ],
            stealth_first: true,
            async_execution: false,
            memory_limit_mb: 2048,
        },
        SmartAttackSequence {
            keyword: "database".to_string(),
            description: "Database exploitation".to_string(),
            tools: vec![
                "sqlmap".to_string(),
                "odat".to_string(),
                "mssqlclient".to_string(),
                "tnscmd10g".to_string(),
            ],
            stealth_first: true,
            async_execution: true,
            memory_limit_mb: 666,
        },
        SmartAttackSequence {
            keyword: "forensic".to_string(),
            description: "Digital forensics analysis".to_string(),
            tools: vec![
                "autopsy".to_string(),
                "volatility".to_string(),
                "binwalk".to_string(),
                "foremost".to_string(),
                "bulk_extractor".to_string(),
            ],
            stealth_first: false, // Forensics doesn't need stealth
            async_execution: false,
            memory_limit_mb: 2048,
        },
    ]
}

// ============================================================================
// TRANSLATION LAYER - GEMINI FOR NON-ENGLISH
// ============================================================================

async fn translate_to_english(client: &Client, text: &str) -> Result<String, String> {
    // Check if already English (simple heuristic)
    let english_chars = text.chars()
        .filter(|c| c.is_ascii_alphabetic())
        .count();
    let total_chars = text.chars().count();

    if total_chars > 0 && (english_chars as f32 / total_chars as f32) > 0.8 {
        return Ok(text.to_string());
    }

    // Use Gemini for translation
    let api_key = std::env::var("GEMINI_API_KEY")
        .map_err(|_| "GEMINI_API_KEY not set for translation".to_string())?;

    let url = format!(
        "https://generativelanguage.googleapis.com/v1beta/models/gemini-pro:generateContent?key={}",
        api_key
    );

    let payload = json!({
        "contents": [{
            "parts": [{
                "text": format!("Translate this security testing request to English. Keep technical terms accurate. Only return the English translation, nothing else:\n\n{}", text)
            }]
        }]
    });

    let response = client
        .post(&url)
        .json(&payload)
        .timeout(std::time::Duration::from_secs(30))
        .send()
        .await
        .map_err(|e| format!("Translation request failed: {}", e))?;

    let json: serde_json::Value = response
        .json()
        .await
        .map_err(|e| format!("Failed to parse translation response: {}", e))?;

    if let Some(translated) = json["candidates"][0]["content"]["parts"][0]["text"].as_str() {
        Ok(translated.trim().to_string())
    } else {
        Err("Invalid translation response format".to_string())
    }
}

// ============================================================================
// ZAI ORCHESTRATOR - MAIN NLP BRAIN
// ============================================================================

pub async fn parse_command(client: &Client, user_input: &str) -> Result<ParsedCommand, String> {
    // Step 1: Translate to English if needed
    let english_input = translate_to_english(client, user_input).await
        .unwrap_or_else(|_| user_input.to_string());

    // Step 2: Extract components using ZAI (Grok API)
    let parsed = interpret_with_zai(client, &english_input).await?;

    // Step 3: Validate completeness
    let (needs_clarification, questions) = validate_input_completeness(&parsed);

    if needs_clarification {
        return Ok(ParsedCommand {
            command: "clarify".to_string(),
            explanation: "Input needs clarification".to_string(),
            source: "validation".to_string(),
            subject: parsed.subject,
            context: parsed.context,
            keywords: parsed.keywords,
            confidence: parsed.confidence,
            needs_clarification: true,
            clarification_questions: questions,
        });
    }

    // Step 4: Generate smart command sequence
    let final_command = generate_smart_sequence(&parsed);

    // Step 5: Log for audit
    log_interaction(user_input, &english_input, &parsed, &final_command);

    Ok(ParsedCommand {
        command: final_command,
        explanation: format!("Smart attack sequence triggered by keywords: {}", parsed.keywords.join(", ")),
        source: "zai_orchestrator".to_string(),
        subject: parsed.subject,
        context: parsed.context,
        keywords: parsed.keywords,
        confidence: parsed.confidence,
        needs_clarification: false,
        clarification_questions: vec![],
    })
}

// ============================================================================
// ZAI INTERPRETATION
// ============================================================================

#[derive(Debug, Deserialize)]
struct ZaiInterpretation {
    subject: Option<String>,
    context: Option<String>,
    keywords: Vec<String>,
    confidence: f32,
    reasoning: String,
}

/// Main ZAI interpretation function - tries ZAI_API_KEY first, then falls back to Grok
async fn interpret_with_zai(client: &Client, input: &str) -> Result<ZaiInterpretation, String> {
    // Try ZAI_API_KEY first
    if let Ok(api_key) = std::env::var("ZAI_API_KEY") {
        match interpret_with_zai_api(client, input, &api_key).await {
            Ok(result) => return Ok(result),
            Err(e) => {
                println!("⚠️  ZAI API failed: {}, falling back to Grok", e);
            }
        }
    }

    // Fallback to Grok
    interpret_with_grok(client, input).await
}

/// ZAI API interpretation
async fn interpret_with_zai_api(client: &Client, input: &str, api_key: &str) -> Result<ZaiInterpretation, String> {
    let system_prompt = r#"You are ZAI, the Fenrir Security Orchestrator. Analyze security testing requests and extract:

1. subject: The target (IP, domain, email, username, etc.)
2. context: Strategy or instructions (stealth, aggressive, specific techniques)
3. keywords: Attack types or tool categories to trigger

Available keyword sequences:
- password: hydra, hashcat, john, cewl, crunch, patator, medusa, ncrack
- scan: nmap, masscan, rustscan, nikto, nuclei
- recon: theHarvester, amass, subfinder, dnsrecon, whois
- social: sherlock, maltego, recon-ng, spiderfoot, holehe
- web: nikto, gobuster, ffuf, sqlmap, wpscan, xsstrike
- wireless: aircrack-ng, wifite, reaver, bully, kismet
- oauth: burpsuite, evilginx2, modlishka, mitmproxy
- database: sqlmap, odat, mssqlclient, mongoaudit
- forensic: autopsy, volatility, binwalk, foremost, bulk_extractor
- exploit: metasploit, searchsploit, msfvenom
- privesc: linpeas, winpeas, pspy, linux-exploit-suggester
- shell: netcat, socat, pwncat
- sniff: wireshark, tcpdump, ettercap, bettercap, responder

Return ONLY valid JSON with: subject, context, keywords (array), confidence (0-1), reasoning"#;

    // Try Venice AI endpoint (ZAI)
    let url = "https://api.venice.ai/api/v1/chat/completions";

    let payload = json!({
        "model": "llama-3.3-70b",
        "messages": [
            {"role": "system", "content": system_prompt},
            {"role": "user", "content": format!("Analyze this security request and extract subject, keywords, and context: {}", input)}
        ],
        "max_tokens": 1000,
        "temperature": 0.3
    });

    let response = client
        .post(url)
        .header("Authorization", format!("Bearer {}", api_key))
        .header("Content-Type", "application/json")
        .json(&payload)
        .timeout(std::time::Duration::from_secs(30))
        .send()
        .await
        .map_err(|e| format!("ZAI request failed: {}", e))?;

    if !response.status().is_success() {
        return Err(format!("ZAI API error: {}", response.status()));
    }

    let json: serde_json::Value = response
        .json()
        .await
        .map_err(|e| format!("Failed to parse ZAI response: {}", e))?;

    let content = json["choices"][0]["message"]["content"]
        .as_str()
        .ok_or("No content in ZAI response")?;

    // Extract JSON from response
    let json_start = content.find('{').unwrap_or(0);
    let json_end = content.rfind('}').unwrap_or(content.len());
    let json_str = &content[json_start..=json_end];

    let interpretation: ZaiInterpretation = serde_json::from_str(json_str)
        .map_err(|e| format!("Failed to parse ZAI interpretation: {}", e))?;

    Ok(interpretation)
}

async fn interpret_with_grok(client: &Client, input: &str) -> Result<ZaiInterpretation, String> {
    let api_key = std::env::var("GROK_API_KEY")
        .or_else(|_| std::env::var("XAI_API_KEY"))
        .map_err(|_| "GROK_API_KEY or XAI_API_KEY not set".to_string())?;

    let system_prompt = r#"You are GROK, the Fenrir Orchestrator. Analyze security testing requests and extract:

1. subject: The target (IP, domain, email, username, etc.)
2. context: Strategy or instructions (stealth, aggressive, specific techniques)
3. keywords: Attack types or tool categories to trigger

Available keyword sequences:
- password: hydra, hashcat, john, cewl, crunch, patator
- scan: nmap, masscan, rustscan, nikto, nuclei
- social: theHarvester, sherlock, maltego, recon-ng
- web: nikto, gobuster, ffuf, sqlmap, nuclei
- wireless: aircrack-ng, wifite, reaver, bully
- oauth: burpsuite, evilginx2, modlishka
- database: sqlmap, odat, mssqlclient
- forensic: autopsy, volatility, binwalk
- exploit: metasploit, armitage, cobaltstrike
- reverse-shell: netcat, socat, powershell-empire
- privilege-escalation: linpeas, winpeas, pspy, linux-exploit-suggester

Return ONLY valid JSON with: subject, context, keywords (array), confidence (0-1), reasoning"#;

    let url = "https://api.x.ai/v1/chat/completions";

    let payload = json!({
        "model": "grok-code-fast-1:free",
        "messages": [
            {"role": "system", "content": system_prompt},
            {"role": "user", "content": format!("Analyze this security request: {}", input)}
        ],
        "max_tokens": 1000,
        "temperature": 0.3
    });

    let response = client
        .post(url)
        .header("Authorization", format!("Bearer {}", api_key))
        .json(&payload)
        .timeout(std::time::Duration::from_secs(30))
        .send()
        .await
        .map_err(|e| format!("Grok request failed: {}", e))?;

    let json: serde_json::Value = response
        .json()
        .await
        .map_err(|e| format!("Failed to parse Grok response: {}", e))?;

    let content = json["choices"][0]["message"]["content"]
        .as_str()
        .ok_or("No content in Grok response")?;

    // Extract JSON from response (Grok might add extra text)
    let json_start = content.find('{').unwrap_or(0);
    let json_end = content.rfind('}').unwrap_or(content.len());
    let json_str = &content[json_start..=json_end];

    let interpretation: ZaiInterpretation = serde_json::from_str(json_str)
        .map_err(|e| format!("Failed to parse Grok interpretation: {}", e))?;

    Ok(interpretation)
}

// ============================================================================
// INPUT VALIDATION
// ============================================================================

fn validate_input_completeness(parsed: &ZaiInterpretation) -> (bool, Vec<String>) {
    let mut needs_clarification = false;
    let mut questions = Vec::new();

    if parsed.subject.is_none() {
        needs_clarification = true;
        questions.push("What is the target? (IP, domain, email, username, etc.)".to_string());
    }

    if parsed.keywords.is_empty() {
        needs_clarification = true;
        questions.push("What type of security testing? (password, scan, social, web, wireless, oauth, database, forensic)".to_string());
    }

    if parsed.confidence < 0.7 {
        needs_clarification = true;
        questions.push("Please clarify your request - I'm not confident I understood correctly.".to_string());
    }

    (needs_clarification, questions)
}

// ============================================================================
// SMART ATTACK EXECUTOR
// ============================================================================

#[derive(Debug)]
struct SmartAttackExecutor {
    semaphore: Arc<Semaphore>,
    memory_monitor: Arc<tokio::sync::Mutex<std::collections::HashMap<u32, u64>>>,
}

impl SmartAttackExecutor {
    fn new() -> Self {
        Self {
            semaphore: Arc::new(Semaphore::new(10)), // Max 10 concurrent tasks
            memory_monitor: Arc::new(tokio::sync::Mutex::new(std::collections::HashMap::new())),
        }
    }

    async fn execute_smart_sequence(&self, parsed: &ZaiInterpretation) -> String {
        let sequences = get_smart_sequences();
        let mut results = Vec::new();

        // Phase 1: Stealth Scan (always first)
        if parsed.keywords.iter().any(|k| k != "forensic" && k != "zero-day") {
            results.push("🐺 FENRIR STEALTH SCAN PHASE".to_string());
            let stealth_result = self.execute_stealth_scan(parsed.subject.as_deref()).await;
            let stealth_failed = stealth_result.contains("failed") || stealth_result.contains("error");
            results.push(stealth_result);

            // If stealth scan fails, fallback to aggressive
            if stealth_failed {
                results.push("⚠️  Stealth scan failed, switching to aggressive mode".to_string());
                let aggressive_result = self.execute_aggressive_scan(parsed.subject.as_deref()).await;
                results.push(aggressive_result);
            }
        }

        // Phase 2: Execute triggered sequences
        let mut async_tasks = Vec::new();
        let mut sequential_tasks = Vec::new();

        for keyword in &parsed.keywords {
            if let Some(sequence) = sequences.iter().find(|s| s.keyword == *keyword) {
                results.push(format!("🐺 FENRIR {} SEQUENCE", sequence.keyword.to_uppercase()));

                if sequence.async_execution {
                    let task = self.execute_async_sequence(sequence.clone(), parsed.subject.clone());
                    async_tasks.push(task);
                } else {
                    let task = self.execute_sequential_sequence(sequence.clone(), parsed.subject.clone());
                    sequential_tasks.push(task);
                }
            }
        }

        // Execute async tasks concurrently
        if !async_tasks.is_empty() {
            results.push("🚀 Launching async attack sequences...".to_string());
            let async_results = futures::future::join_all(async_tasks).await;
            for result in async_results {
                results.push(result);
            }
        }

        // Execute sequential tasks one by one
        if !sequential_tasks.is_empty() {
            results.push("🔄 Executing sequential attack sequences...".to_string());
            for task in sequential_tasks {
                let result = task.await;
                results.push(result);
            }
        }

        // If no keywords matched, fallback
        if results.len() <= 2 {
            results.push("❌ No specific sequence triggered - use natural language or keywords".to_string());
        }

        results.join("\n")
    }

    async fn execute_stealth_scan(&self, target: Option<&str>) -> String {
        let target_owned = target.unwrap_or("localhost").to_string();
        let permit = self.semaphore.acquire().await.unwrap();

        let result = task::spawn_blocking(move || {
            let output = Command::new("nmap")
                .args(&["-sS", "-T2", "-Pn", "--script", "vuln", "--max-retries", "1", &target_owned])
                .stdout(Stdio::piped())
                .stderr(Stdio::piped())
                .output();

            match output {
                Ok(out) => {
                    if out.status.success() {
                        format!("✅ Stealth scan completed for {}", target_owned)
                    } else {
                        format!("❌ Stealth scan failed for {}: {}", target_owned, String::from_utf8_lossy(&out.stderr))
                    }
                }
                Err(e) => format!("❌ Stealth scan error for {}: {}", target_owned, e),
            }
        }).await.unwrap();

        drop(permit);
        result
    }

    async fn execute_aggressive_scan(&self, target: Option<&str>) -> String {
        let target_owned = target.unwrap_or("localhost").to_string();
        let permit = self.semaphore.acquire().await.unwrap();

        let result = task::spawn_blocking(move || {
            let output = Command::new("nmap")
                .args(&["-sV", "-T4", "-A", "-p-", &target_owned])
                .stdout(Stdio::piped())
                .stderr(Stdio::piped())
                .output();

            match output {
                Ok(out) => {
                    if out.status.success() {
                        format!("✅ Aggressive scan completed for {}", target_owned)
                    } else {
                        format!("❌ Aggressive scan failed for {}: {}", target_owned, String::from_utf8_lossy(&out.stderr))
                    }
                }
                Err(e) => format!("❌ Aggressive scan error for {}: {}", target_owned, e),
            }
        }).await.unwrap();

        drop(permit);
        result
    }

    async fn execute_async_sequence(&self, sequence: SmartAttackSequence, subject: Option<String>) -> String {
        let permit = self.semaphore.acquire().await.unwrap();
        let memory_limit = sequence.memory_limit_mb;

        let result = task::spawn(async move {
            let mut results = Vec::new();

            if sequence.stealth_first {
                results.push(format!("   Phase 1: Stealth Mode ({}MB limit)", memory_limit));
            }

            for tool in sequence.tools {
                // Memory monitoring would go here in production
                let tool_check = Command::new(&tool)
                    .arg("--help")
                    .stdout(Stdio::null())
                    .stderr(Stdio::null())
                    .status();

                match tool_check {
                    Ok(status) if status.success() => {
                        results.push(format!("   ✅ {} available", tool));
                    }
                    _ => {
                        results.push(format!("   ❌ {} not available", tool));
                    }
                }
            }

            results.push(format!("   ✅ {} sequence completed", sequence.keyword));
            results.join("\n")
        }).await.unwrap();

        drop(permit);
        result
    }

    async fn execute_sequential_sequence(&self, sequence: SmartAttackSequence, subject: Option<String>) -> String {
        let mut results = Vec::new();

        results.push(format!("   Sequential {} execution ({}MB limit)", sequence.keyword, sequence.memory_limit_mb));

        for tool in sequence.tools {
            // Sequential execution - one at a time
            let tool_check = Command::new(&tool)
                .arg("--help")
                .stdout(Stdio::null())
                .stderr(Stdio::null())
                .status();

            match tool_check {
                Ok(status) if status.success() => {
                    results.push(format!("   ✅ {} available", tool));
                }
                _ => {
                    results.push(format!("   ❌ {} not available", tool));
                }
            }
        }

        results.push(format!("   ✅ {} sequence completed", sequence.keyword));
        results.join("\n")
    }
}

// ============================================================================
// SMART SEQUENCE GENERATION (LEGACY COMPATIBILITY)
// ============================================================================

fn generate_smart_sequence(parsed: &ZaiInterpretation) -> String {
    let executor = SmartAttackExecutor::new();
    // Note: This is synchronous wrapper for async executor
    // In production, this should be called from async context
    tokio::runtime::Runtime::new()
        .unwrap()
        .block_on(executor.execute_smart_sequence(parsed))
}

// ============================================================================
// USER DOUBLE-CHECK
// ============================================================================

pub async fn confirm_interpretation(client: &Client, parsed: &ParsedCommand) -> Result<bool, String> {
    println!("\n🤖 ZAI Interpretation:");
    println!("   Subject: {}", parsed.subject.as_ref().unwrap_or(&"None".to_string()));
    println!("   Context: {}", parsed.context.as_ref().unwrap_or(&"None".to_string()));
    println!("   Keywords: {}", parsed.keywords.join(", "));
    println!("   Confidence: {:.1}%", parsed.confidence * 100.0);
    println!("   Plan: {}", parsed.explanation);
    println!("\n❓ Is this interpretation correct? (yes/no): ");

    // In interactive mode, we'd read user input here
    // For now, assume confirmation if confidence > 0.8
    Ok(parsed.confidence > 0.8)
}

// ============================================================================
// LOGGING
// ============================================================================

fn log_interaction(original: &str, translated: &str, parsed: &ZaiInterpretation, command: &str) {
    let path = history_path();
    if let Some(parent) = path.parent() {
        let _ = fs::create_dir_all(parent);
    }

    let entry = json!({
        "timestamp": Utc::now().to_rfc3339(),
        "original_input": original,
        "translated_input": translated,
        "subject": parsed.subject,
        "context": parsed.context,
        "keywords": parsed.keywords,
        "confidence": parsed.confidence,
        "generated_command": command,
        "reasoning": parsed.reasoning
    });

    if let Ok(mut file) = OpenOptions::new().create(true).append(true).open(path) {
        let _ = writeln!(file, "{}", entry);
    }
}

fn history_path() -> PathBuf {
    let base = dirs::config_dir().unwrap_or_else(|| PathBuf::from("."));
    base.join("fenrir").join("zai_interactions.jsonl")
}

// ============================================================================
// LEGACY COMPATIBILITY
// ============================================================================

// Keep old functions for backward compatibility during transition
pub async fn parse_command_legacy(client: &Client, user_input: &str) -> Result<ParsedCommand, String> {
    // Fallback to old NLP if ZAI fails
    translate_with_ai(client, user_input).await.map(|cmd| ParsedCommand {
        command: cmd.command,
        explanation: cmd.explanation,
        source: cmd.source,
        subject: None,
        context: None,
        keywords: vec![],
        confidence: 0.5,
        needs_clarification: false,
        clarification_questions: vec![],
    })
}

async fn translate_with_ai(client: &Client, user_input: &str) -> Result<ParsedCommand, String> {
    // Keep old implementation as fallback
    let api_key = std::env::var("GROK_API_KEY")
        .or_else(|_| std::env::var("XAI_API_KEY"))
        .map_err(|_| "GROK_API_KEY or XAI_API_KEY not set".to_string())?;

    let system_prompt = r#"Convert natural language to bash command. Return JSON: {"command": "cmd", "explanation": "desc"}"#;

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
        return Err(format!("API error: {}", response.status()));
    }

    let json_response: serde_json::Value = response.json().await
        .map_err(|e| format!("Parse error: {}", e))?;

    let content = json_response["choices"][0]["message"]["content"]
        .as_str()
        .ok_or("No content")?;

    let parsed: serde_json::Value = serde_json::from_str(content)
        .map_err(|_| format!("Invalid JSON: {}", content))?;

    Ok(ParsedCommand {
        command: parsed["command"].as_str().unwrap_or("echo 'error'").to_string(),
        explanation: parsed["explanation"].as_str().unwrap_or("Error").to_string(),
        source: "legacy".to_string(),
        subject: None,
        context: None,
        keywords: vec![],
        confidence: 0.5,
        needs_clarification: false,
        clarification_questions: vec![],
    })
}
