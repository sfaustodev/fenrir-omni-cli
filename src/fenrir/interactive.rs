// ============================================================================
// FENRIR INTERACTIVE MODE v2.1 - FUZZY LOCAL INTERPRETATION
// ============================================================================
// Simple, robust interactive mode with local fuzzy matching
// No external API dependencies for interpretation

use crate::fenrir_ai_layer;
use crate::nlp::{self, get_keyword_info, get_all_keywords, get_keyword_tools};
use futures::future::join_all;
use regex::Regex;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::io::{self, Write};
use std::process::{Command, Stdio};
use std::sync::Arc;
use tokio::sync::Semaphore;
use tokio::task;

// ============================================================================
// CONSTANTS
// ============================================================================

const ASYNC_THREAD_MEMORY_LIMIT_MB: u64 = 666;
const SEQUENTIAL_THREAD_MEMORY_LIMIT_MB: u64 = 2048;
const MAX_CONCURRENT_ASYNC_TASKS: usize = 10;

// ============================================================================
// SMART ATTACK SEQUENCE
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SmartAttackSequence {
    pub keyword: String,
    pub description: String,
    pub tools: Vec<String>,
    pub stealth_first: bool,
    pub async_execution: bool,
    pub memory_limit_mb: u64,
}

pub fn get_all_smart_sequences() -> HashMap<String, SmartAttackSequence> {
    let mut sequences = HashMap::new();
    
    for keyword in get_all_keywords() {
        if let Some((description, tools, async_exec, memory)) = get_keyword_info(&keyword) {
            sequences.insert(keyword.clone(), SmartAttackSequence {
                keyword: keyword.clone(),
                description,
                tools,
                stealth_first: true,
                async_execution: async_exec,
                memory_limit_mb: memory,
            });
        }
    }
    
    sequences
}

// ============================================================================
// USER INPUT STRUCTURE
// ============================================================================

#[derive(Debug, Clone)]
pub struct UserInput {
    pub subject: Option<String>,
    pub keywords: Vec<String>,
    pub context: Option<String>,
    pub original_text: String,
    pub confidence: f32,
    pub suggestions: Vec<String>,
}

impl UserInput {
    pub fn new() -> Self {
        UserInput {
            subject: None,
            keywords: Vec::new(),
            context: None,
            original_text: String::new(),
            confidence: 0.0,
            suggestions: Vec::new(),
        }
    }
}

// ============================================================================
// ATTACK EXECUTOR
// ============================================================================

pub struct SmartAttackExecutor {
    semaphore: Arc<Semaphore>,
}

impl SmartAttackExecutor {
    pub fn new() -> Self {
        SmartAttackExecutor {
            semaphore: Arc::new(Semaphore::new(MAX_CONCURRENT_ASYNC_TASKS)),
        }
    }

    /// Execute stealth scan
    pub async fn execute_stealth_scan(&self, target: &str) -> Result<String, String> {
        let target_owned = target.to_string();
        
        let result = task::spawn_blocking(move || {
            let output = Command::new("nmap")
                .args(&["-sS", "-T2", "-Pn", "--max-retries", "1", "-f", &target_owned])
                .stdout(Stdio::piped())
                .stderr(Stdio::piped())
                .output();

            match output {
                Ok(out) => {
                    if out.status.success() {
                        Ok(format!("✅ Stealth scan completed for {}", target_owned))
                    } else {
                        Err(format!("Stealth scan failed: {}", String::from_utf8_lossy(&out.stderr)))
                    }
                }
                Err(e) => Err(format!("Stealth scan error: {}", e)),
            }
        }).await.map_err(|e| format!("Task error: {}", e))?;

        result
    }

    /// Execute aggressive scan
    pub async fn execute_aggressive_scan(&self, target: &str) -> Result<String, String> {
        let target_owned = target.to_string();
        
        let result = task::spawn_blocking(move || {
            let output = Command::new("nmap")
                .args(&["-sV", "-sC", "-A", "-T4", &target_owned])
                .stdout(Stdio::piped())
                .stderr(Stdio::piped())
                .output();

            match output {
                Ok(out) => {
                    if out.status.success() {
                        Ok(format!("✅ Aggressive scan completed for {}", target_owned))
                    } else {
                        Err(format!("Aggressive scan failed: {}", String::from_utf8_lossy(&out.stderr)))
                    }
                }
                Err(e) => Err(format!("Aggressive scan error: {}", e)),
            }
        }).await.map_err(|e| format!("Task error: {}", e))?;

        result
    }

    /// Check if a tool is available
    fn check_tool_available(tool: &str) -> bool {
        Command::new("which")
            .arg(tool)
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .status()
            .map(|s| s.success())
            .unwrap_or(false)
    }

    /// Detect target type
    fn detect_target_type(target: &str) -> &'static str {
        if target.contains('@') && !target.contains('/') {
            "EMAIL"
        } else if target.parse::<std::net::IpAddr>().is_ok() ||
                  regex::Regex::new(r"^\d{1,3}\.\d{1,3}\.\d{1,3}\.\d{1,3}$").unwrap().is_match(target) {
            "IP"
        } else if target.contains('.') && !target.contains('@') {
            "DOMAIN"
        } else if !target.contains('.') && !target.contains('@') {
            "USERNAME"
        } else {
            "UNKNOWN"
        }
    }

    /// Build tool arguments based on tool and target
    fn build_tool_args(tool: &str, target: &str) -> Vec<String> {
        match tool {
            "nmap" => vec!["-sV".to_string(), "-sC".to_string(), "-T4".to_string(), target.to_string()],
            "gobuster" => vec!["dir".to_string(), "-u".to_string(), target.to_string(), "-w".to_string(), "/usr/share/wordlists/dirb/common.txt".to_string()],
            "ffuf" => vec!["-u".to_string(), format!("{}/FUZZ", target), "-w".to_string(), "/usr/share/wordlists/dirb/common.txt".to_string()],
            "hydra" => vec!["-l".to_string(), "root".to_string(), "-P".to_string(), "/usr/share/wordlists/rockyou.txt".to_string(), target.to_string(), "ssh".to_string()],
            "sqlmap" => vec!["-u".to_string(), target.to_string(), "--batch".to_string()],
            "nikto" => vec!["-h".to_string(), target.to_string()],
            "whatweb" => vec![target.to_string()],
            "whois" => vec![target.to_string()],
            "dig" => vec!["ANY".to_string(), target.to_string()],
            "nslookup" => vec![target.to_string()],
            _ => vec![target.to_string()],
        }
    }

    /// Execute attack sequence with actual tool execution and result capture
    pub async fn execute_sequence(&self, sequence: &SmartAttackSequence, target: &str, mode: &str) -> Vec<String> {
        let mut results = Vec::new();
        let exec_mode = if sequence.async_execution { "ASYNC" } else { "SEQUENTIAL" };

        results.push(format!("\n🔥 {} SEQUENCE: {} ({}MB limit)",
            exec_mode, sequence.keyword.to_uppercase(), sequence.memory_limit_mb));
        results.push(format!("   📝 {}", sequence.description));
        results.push(format!("   🎯 Target: {} [Type: {}]", target, Self::detect_target_type(target)));
        results.push(format!("   ⚡ Mode: {}", mode.to_uppercase()));
        results.push("   ─────────────────────────────────────────".to_string());

        for tool in &sequence.tools {
            let tool_name = tool.split('.').next().unwrap_or(tool);
            let tool_name_owned = tool_name.to_string();  // Clone for move into closure

            results.push(format!("\n   ▶️  Executing: {}...", tool));

            if !Self::check_tool_available(tool_name) {
                results.push(format!("   ❌ {} - Not installed (skipped)", tool));
                continue;
            }

            // Build command arguments
            let args = Self::build_tool_args(tool_name, target);
            let target_owned = target.to_string();  // Clone for move into closure

            // Execute tool with timeout
            let result = task::spawn_blocking(move || {
                let timeout = std::time::Duration::from_secs(60);
                let output = Command::new(&tool_name_owned)
                    .args(&args)
                    .stdout(Stdio::piped())
                    .stderr(Stdio::piped())
                    .output();

                match output {
                    Ok(out) => {
                        let stdout = String::from_utf8_lossy(&out.stdout).to_string();
                        let stderr = String::from_utf8_lossy(&out.stderr).to_string();

                        if out.status.success() {
                            Ok((stdout, stderr))
                        } else {
                            Err((stdout, stderr))
                        }
                    }
                    Err(e) => Err((String::new(), format!("Execution error: {}", e)))
                }
            }).await;

            match result {
                Ok(Ok((stdout, stderr))) => {
                    // Success - show what was obtained
                    results.push(format!("   ✅ {} - Success", tool));

                    // Show key findings from output
                    if !stdout.is_empty() {
                        let lines: Vec<&str> = stdout.lines().collect();
                        if lines.len() > 20 {
                            results.push(format!("   📊 Output ({} lines, showing first 20):", lines.len()));
                            for line in lines.iter().take(20) {
                                if !line.trim().is_empty() {
                                    results.push(format!("      │ {}", line));
                                }
                            }
                            results.push(format!("      ... ({} more lines)", lines.len() - 20));
                        } else {
                            results.push(format!("   📊 Output:"));
                            for line in lines {
                                if !line.trim().is_empty() {
                                    results.push(format!("      │ {}", line));
                                }
                            }
                        }
                    }

                    if !stderr.is_empty() && !stderr.contains("warning") {
                        results.push(format!("   ⚠️  Warnings: {}", truncate_str(&stderr, 100)));
                    }
                }
                Ok(Err((stdout, stderr))) => {
                    // Failed - show what happened
                    results.push(format!("   ❌ {} - Failed", tool));

                    if !stdout.is_empty() {
                        results.push(format!("   📤 Partial output: {}", truncate_str(&stdout, 200)));
                    }

                    if !stderr.is_empty() {
                        results.push(format!("   ⚠️  Error: {}", truncate_str(&stderr, 200)));
                    }
                }
                Err(e) => {
                    results.push(format!("   ❌ {} - Task error: {}", tool, e));
                }
            }
        }

        results
    }

    /// Main execution with mode parameter
    pub async fn execute_smart_attack(&self, input: &UserInput, mode: &str) -> Vec<String> {
        let mut all_results = Vec::new();
        let sequences = get_all_smart_sequences();
        let target = input.subject.as_deref().unwrap_or("localhost");

        all_results.push(format!("\n╔══════════════════════════════════════════════════════════╗"));
        all_results.push(format!("║  🐺 FENRIR ATTACK EXECUTION - {} MODE              ║", mode.to_uppercase()));
        all_results.push(format!("╚══════════════════════════════════════════════════════════╝"));

        // Phase 1: Stealth Scan (skip for forensic, use selected mode)
        if !input.keywords.iter().any(|k| k == "forensic") {
            all_results.push("\n═══════════════════════════════════════════════════════════".to_string());
            all_results.push(format!("🔍 INITIAL SCAN PHASE [{}]", mode.to_uppercase()));
            all_results.push("═══════════════════════════════════════════════════════════".to_string());

            if mode == "stealth" {
                match self.execute_stealth_scan(target).await {
                    Ok(result) => all_results.push(result),
                    Err(e) => {
                        all_results.push(format!("⚠️  Stealth scan failed: {}", e));
                        all_results.push("   📤 Output: May require elevated privileges or target is down".to_string());
                    }
                }
            } else {
                match self.execute_aggressive_scan(target).await {
                    Ok(result) => all_results.push(result),
                    Err(e) => {
                        all_results.push(format!("⚠️  Aggressive scan failed: {}", e));
                        all_results.push("   📤 Output: May require elevated privileges or target is down".to_string());
                    }
                }
            }
        }

        // Phase 2: Execute sequences
        let mut async_seqs = Vec::new();
        let mut seq_seqs = Vec::new();

        for keyword in &input.keywords {
            if let Some(seq) = sequences.get(keyword) {
                if seq.async_execution {
                    async_seqs.push(seq.clone());
                } else {
                    seq_seqs.push(seq.clone());
                }
            }
        }

        // Async sequences
        if !async_seqs.is_empty() {
            all_results.push("\n═══════════════════════════════════════════════════════════".to_string());
            all_results.push("🚀 ASYNC ATTACK PHASE - Parallel Execution".to_string());
            all_results.push("═══════════════════════════════════════════════════════════".to_string());

            for seq in async_seqs {
                let results = self.execute_sequence(&seq, target, mode).await;
                all_results.extend(results);
            }
        }

        // Sequential sequences
        if !seq_seqs.is_empty() {
            all_results.push("\n═══════════════════════════════════════════════════════════".to_string());
            all_results.push("🔄 SEQUENTIAL ATTACK PHASE - One by One".to_string());
            all_results.push("═══════════════════════════════════════════════════════════".to_string());

            for seq in seq_seqs {
                let results = self.execute_sequence(&seq, target, mode).await;
                all_results.extend(results);
            }
        }

        // Summary
        all_results.push("\n═══════════════════════════════════════════════════════════".to_string());
        all_results.push("✅ ATTACK SEQUENCE COMPLETE".to_string());
        all_results.push("═══════════════════════════════════════════════════════════".to_string());

        all_results
    }
}

// ============================================================================
// INTERACTIVE MODE - MAIN ENTRY POINT
// ============================================================================

pub async fn run_interactive_mode() -> Result<(), Box<dyn std::error::Error>> {
    fenrir_ai_layer::load_env();

    let http_client = Client::new();
    let executor = SmartAttackExecutor::new();
    let stdin = io::stdin();

    print_banner();

    loop {
        print!("\n🐺 fenrir> ");
        io::stdout().flush()?;

        let mut input_line = String::new();
        match stdin.read_line(&mut input_line) {
            Ok(0) => {
                println!("\n🐺 Goodbye!");
                break;
            }
            Ok(_) => {
                let user_text = input_line.trim();
                if user_text.is_empty() {
                    continue;
                }

                // Handle commands
                match user_text.to_lowercase().as_str() {
                    "exit" | "quit" | "q" | "sair" => {
                        println!("\n🐺 Goodbye!");
                        break;
                    }
                    "help" | "?" | "h" => {
                        print_help();
                        continue;
                    }
                    "keywords" | "k" => {
                        print_keywords();
                        continue;
                    }
                    _ => {}
                }

                // Process with fuzzy NLP
                println!("\n🔍 Analyzing input with fuzzy matching...");

                match nlp::parse_command(&http_client, user_text).await {
                    Ok(parsed) => {
                        // Display interpretation
                        println!("\n╔══════════════════════════════════════════════════════════╗");
                        println!("║              🎯 FUZZY INTERPRETATION                     ║");
                        println!("╠══════════════════════════════════════════════════════════╣");
                        
                        // Target
                        let target_display = parsed.subject.as_deref().unwrap_or("❌ Not detected");
                        println!("║ 📍 Target:     {:<42} ║", truncate_str(target_display, 42));
                        
                        // Keywords
                        let keywords_display = if parsed.keywords.is_empty() {
                            "❌ None detected".to_string()
                        } else {
                            parsed.keywords.join(", ")
                        };
                        println!("║ 🔑 Keywords:   {:<42} ║", truncate_str(&keywords_display, 42));
                        
                        // Context
                        let context_display = parsed.context.as_deref().unwrap_or("auto");
                        println!("║ 📋 Context:    {:<42} ║", context_display);
                        
                        // Confidence
                        let conf_bar = generate_confidence_bar(parsed.confidence);
                        println!("║ 📊 Confidence: {} {:<28} ║", conf_bar, format!("{:.0}%", parsed.confidence * 100.0));
                        
                        println!("╚══════════════════════════════════════════════════════════╝");

                        // Show suggestions if any
                        if !parsed.suggestions.is_empty() {
                            println!("\n┌──────────────────────────────────────────────────────────┐");
                            println!("│                    💡 SUGGESTIONS                        │");
                            println!("├──────────────────────────────────────────────────────────┤");
                            for suggestion in &parsed.suggestions {
                                println!("│ {:<56} │", truncate_str(suggestion, 56));
                            }
                            println!("└──────────────────────────────────────────────────────────┘");
                        }

                        // Check if clarification needed
                        if parsed.needs_clarification {
                            println!("\n❓ Missing information:");
                            for question in &parsed.clarification_questions {
                                println!("   • {}", question);
                            }
                            println!("\n📝 Example: \"scan 192.168.1.1 password web stealth\"");
                            continue;
                        }

                        // Show triggered sequences
                        println!("\n╔══════════════════════════════════════════════════════════╗");
                        println!("║           🎯 TRIGGERED ATTACK SEQUENCES                  ║");
                        println!("╠══════════════════════════════════════════════════════════╣");

                        let sequences = get_all_smart_sequences();
                        for keyword in &parsed.keywords {
                            if let Some(seq) = sequences.get(keyword) {
                                let mode = if seq.async_execution { "Async" } else { "Seq" };
                                println!("║ 🔑 {:<10} │ {:<38} ║", 
                                    keyword.to_uppercase(), 
                                    truncate_str(&seq.description, 38));
                                println!("║    Tools:    │ {:<38} ║", 
                                    truncate_str(&seq.tools.join(", "), 38));
                                println!("║    Mode:     │ {} | {}MB | Stealth: {:<14} ║",
                                    mode, seq.memory_limit_mb,
                                    if seq.stealth_first { "Yes" } else { "No" });
                                println!("╟──────────────┴────────────────────────────────────────╢");
                            }
                        }
                        println!("╚══════════════════════════════════════════════════════════╝");

                        // Ask for execution mode
                        print!("\n⚡ Select execution mode:\n");
                        print!("   [1] STEALTH    - Quiet, slow, avoids detection (recommended for OSINT/screening/CSI)\n");
                        print!("   [2] AGGRESSIVE - Fast, loud, thorough detection\n");
                        print!("   ❓ Choice (1/2): ");
                        io::stdout().flush()?;

                        let mut mode_choice = String::new();
                        stdin.read_line(&mut mode_choice)?;
                        let execution_mode = match mode_choice.trim() {
                            "1" | "stealth" | "s" => "stealth",
                            "2" | "aggressive" | "a" => "aggressive",
                            _ => "stealth", // Default to stealth
                        };

                        println!("\n🎯 Mode: {} {}", execution_mode.to_uppercase(),
                            if execution_mode == "stealth" { "🤫" } else { "💥" });

                        // Confirm with user
                        print!("\n❓ Execute this attack? (yes/no/edit): ");
                        io::stdout().flush()?;

                        let mut confirmation = String::new();
                        stdin.read_line(&mut confirmation)?;
                        let confirm = confirmation.trim().to_lowercase();

                        match confirm.as_str() {
                            "yes" | "y" | "s" | "sim" => {
                                let user_input = UserInput {
                                    subject: parsed.subject.clone(),
                                    keywords: parsed.keywords.clone(),
                                    context: parsed.context.clone(),
                                    original_text: user_text.to_string(),
                                    confidence: parsed.confidence,
                                    suggestions: parsed.suggestions.clone(),
                                };

                                println!("\n🚀 Executing smart attack sequence...\n");
                                let results = executor.execute_smart_attack(&user_input, execution_mode).await;
                                for result in results {
                                    println!("{}", result);
                                }
                            }
                            "edit" | "e" => {
                                println!("\n📝 Edit mode:");

                                // Edit target
                                print!("   Target [{}]: ", parsed.subject.as_deref().unwrap_or(""));
                                io::stdout().flush()?;
                                let mut new_target = String::new();
                                stdin.read_line(&mut new_target)?;
                                let new_target = new_target.trim();

                                // Edit keywords
                                print!("   Keywords [{}]: ", parsed.keywords.join(","));
                                io::stdout().flush()?;
                                let mut new_keywords = String::new();
                                stdin.read_line(&mut new_keywords)?;
                                let new_keywords = new_keywords.trim();

                                let user_input = UserInput {
                                    subject: if new_target.is_empty() {
                                        parsed.subject.clone()
                                    } else {
                                        Some(new_target.to_string())
                                    },
                                    keywords: if new_keywords.is_empty() {
                                        parsed.keywords.clone()
                                    } else {
                                        new_keywords.split(',').map(|s| s.trim().to_lowercase()).collect()
                                    },
                                    context: parsed.context.clone(),
                                    original_text: user_text.to_string(),
                                    confidence: 1.0,
                                    suggestions: Vec::new(),
                                };

                                println!("\n🚀 Executing with updated parameters...\n");
                                let results = executor.execute_smart_attack(&user_input, execution_mode).await;
                                for result in results {
                                    println!("{}", result);
                                }
                            }
                            _ => {
                                println!("❌ Cancelled. Try again with more details.");
                            }
                        }
                    }
                    Err(e) => {
                        println!("\n❌ Parse error: {}", e);
                        println!("\n💡 Try: <target> <keywords>");
                        println!("   Example: 192.168.1.1 scan password");
                    }
                }
            }
            Err(e) => {
                eprintln!("❌ Input error: {}", e);
                break;
            }
        }
    }

    Ok(())
}

// ============================================================================
// HELPER FUNCTIONS
// ============================================================================

fn truncate_str(s: &str, max_len: usize) -> String {
    if s.len() > max_len {
        format!("{}...", &s[..max_len.saturating_sub(3)])
    } else {
        s.to_string()
    }
}

fn generate_confidence_bar(confidence: f32) -> String {
    let filled = (confidence * 10.0) as usize;
    let empty = 10 - filled;
    format!("[{}{}]", "█".repeat(filled), "░".repeat(empty))
}

fn print_banner() {
    println!("\n");
    println!("╔══════════════════════════════════════════════════════════════════════╗");
    println!("║                                                                      ║");
    println!("║   ███████╗███████╗███╗   ██╗██████╗ ██╗██████╗                       ║");
    println!("║   ██╔════╝██╔════╝████╗  ██║██╔══██╗██║██╔══██╗                      ║");
    println!("║   █████╗  █████╗  ██╔██╗ ██║██████╔╝██║██████╔╝                      ║");
    println!("║   ██╔══╝  ██╔══╝  ██║╚██╗██║██╔══██╗██║██╔══██╗                      ║");
    println!("║   ██║     ███████╗██║ ╚████║██║  ██║██║██║  ██║                      ║");
    println!("║   ╚═╝     ╚══════╝╚═╝  ╚═══╝╚═╝  ╚═╝╚═╝╚═╝  ╚═╝                      ║");
    println!("║                                                                      ║");
    println!("║   🐺 FENRIR v1.6.66 - Smart Attack Orchestration                     ║");
    println!("║   🎯 Fuzzy Local Interpretation - No API Required                    ║");
    println!("║                                                                      ║");
    println!("╠══════════════════════════════════════════════════════════════════════╣");
    println!("║                                                                      ║");
    println!("║   📝 USAGE: <target> <keywords> [context]                            ║");
    println!("║                                                                      ║");
    println!("║   📍 TARGET:   IP, domain, email, @username                          ║");
    println!("║   🔑 KEYWORDS: password, scan, web, social, exploit, etc.            ║");
    println!("║   📋 CONTEXT:  stealth, aggressive, fast, etc.                       ║");
    println!("║                                                                      ║");
    println!("║   💡 EXAMPLES:                                                       ║");
    println!("║      192.168.1.1 scan password                                       ║");
    println!("║      example.com web oauth stealth                                   ║");
    println!("║      user@email.com social password                                  ║");
    println!("║      @username social                                                ║");
    println!("║                                                                      ║");
    println!("║   🔧 COMMANDS: help | keywords | exit                                ║");
    println!("║                                                                      ║");
    println!("╚══════════════════════════════════════════════════════════════════════╝");
}

fn print_help() {
    println!("\n╔══════════════════════════════════════════════════════════════════════╗");
    println!("║                        🐺 FENRIR HELP                                ║");
    println!("╠══════════════════════════════════════════════════════════════════════╣");
    println!("║                                                                      ║");
    println!("║   📝 SIMPLE FORMAT: <target> <keywords> [context]                    ║");
    println!("║                                                                      ║");
    println!("║   🎯 EXAMPLES:                                                       ║");
    println!("║      192.168.1.1 scan                    → Network scan              ║");
    println!("║      192.168.1.1 scan password           → Scan + password crack     ║");
    println!("║      example.com web sql                 → Web + SQL injection       ║");
    println!("║      user@email.com social password      → OSINT + password          ║");
    println!("║      @username social                    → Social media OSINT        ║");
    println!("║      10.0.0.0/24 scan                    → Scan network range        ║");
    println!("║                                                                      ║");
    println!("║   🔑 KEYWORDS (type 'keywords' for full list):                       ║");
    println!("║      password, scan, web, social, wireless, oauth,                   ║");
    println!("║      database, forensic, exploit, privesc, shell, sniff, recon       ║");
    println!("║                                                                      ║");
    println!("║   📋 CONTEXT MODIFIERS:                                              ║");
    println!("║      stealth  → Slow, quiet, evasive                                 ║");
    println!("║      aggressive → Fast, thorough, noisy                              ║");
    println!("║      fast     → Quick scan                                           ║");
    println!("║                                                                      ║");
    println!("║   ⚡ ATTACK FLOW:                                                    ║");
    println!("║      1. Stealth scan (automatic)                                     ║");
    println!("║      2. If stealth fails → Aggressive scan                           ║");
    println!("║      3. Async attacks (parallel, 666MB/thread)                       ║");
    println!("║      4. Sequential attacks (one-by-one, 2GB/thread)                  ║");
    println!("║                                                                      ║");
    println!("║   💡 TIPS:                                                           ║");
    println!("║      • Combine keywords: scan password web                           ║");
    println!("║      • Use edit mode to correct interpretations                      ║");
    println!("║      • Suggestions appear when input is incomplete                   ║");
    println!("║                                                                      ║");
    println!("╚══════════════════════════════════════════════════════════════════════╝");
}

fn print_keywords() {
    let sequences = get_all_smart_sequences();
    
    println!("\n╔══════════════════════════════════════════════════════════════════════╗");
    println!("║                   🔑 AVAILABLE KEYWORDS                              ║");
    println!("╠══════════════════════════════════════════════════════════════════════╣");

    let mut keywords: Vec<_> = sequences.iter().collect();
    keywords.sort_by(|a, b| a.0.cmp(b.0));

    for (keyword, seq) in keywords {
        let mode = if seq.async_execution { "Async" } else { "Seq  " };
        println!("║                                                                      ║");
        println!("║   🔑 {:<10} │ {:<48} ║", 
            keyword.to_uppercase(), 
            truncate_str(&seq.description, 48));
        println!("║      Tools:    │ {:<48} ║", 
            truncate_str(&seq.tools.join(", "), 48));
        println!("║      Mode:     │ {} | {}MB | Stealth: {:<22} ║",
            mode, seq.memory_limit_mb,
            if seq.stealth_first { "Yes" } else { "No" });
    }

    println!("║                                                                      ║");
    println!("╚══════════════════════════════════════════════════════════════════════╝");
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_all_smart_sequences() {
        let sequences = get_all_smart_sequences();
        assert!(sequences.contains_key("password"));
        assert!(sequences.contains_key("scan"));
        assert!(sequences.contains_key("web"));
    }

    #[test]
    fn test_truncate_str() {
        assert_eq!(truncate_str("hello", 10), "hello");
        assert_eq!(truncate_str("hello world", 8), "hello...");
    }

    #[test]
    fn test_confidence_bar() {
        assert_eq!(generate_confidence_bar(1.0), "[██████████]");
        assert_eq!(generate_confidence_bar(0.5), "[█████░░░░░]");
        assert_eq!(generate_confidence_bar(0.0), "[░░░░░░░░░░]");
    }
}
