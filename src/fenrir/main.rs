// --- FENRIR 5.0 - SMART ATTACK ORCHESTRATION ---
// Advanced red team tooling with NLP integration and stealth-first approach

mod batch_executor;
mod circuit_breaker;
mod cli;
mod confirm;
mod daemon;
mod executor;
mod fenrir_ai_layer;
mod fenrir_orchestrator;
mod ferramentas;
mod git_automation;
mod health;
mod http_client;
mod interactive;
mod kali_tools;
mod kali_tools_comprehensive;
mod metrics;
mod oraculo;
mod secrets;
// mod solana;
// mod zcash;
mod bugbounty;
mod bugbounty_tools;
mod burp_integration;
mod bounty_tracker;
mod liquidity;
mod net;
mod nlp;
mod osint;
mod plugins;
mod sandbox;
mod wrapper;

use reqwest::Client;
use serde_json::{json, Value};
use std::io::{self, Write};
use std::sync::Arc;
use tokio::sync::Mutex;
use lazy_static::lazy_static;

lazy_static! {
    pub static ref DAEMON: Arc<Mutex<Option<daemon::FenrirDaemon>>> = Arc::new(Mutex::new(None));
}

#[tokio::main]
async fn main() {
    fenrir_ai_layer::load_env();

    // Check for CLI arguments
    if std::env::args().len() > 1 {
        if let Err(err) = cli::run_cli().await {
            eprintln!("❌ {}", err);
        }
        return;
    }

    // Run the new interactive mode
    if let Err(err) = interactive::run_interactive_mode().await {
        eprintln!("❌ Interactive mode error: {}", err);
        
        // Fallback to legacy mode
        println!("\n⚠️  Falling back to legacy mode...\n");
        run_legacy_mode().await;
    }
}

/// Legacy interactive mode for backward compatibility
async fn run_legacy_mode() {
    println!("🐺 FENRIR - Pure Natural Language Security Orchestration (Legacy Mode)");
    println!("🎯 Rule N1: Natural Language Only - Commands Not Allowed");
    println!("💬 Type natural language requests (English/Portuguese/Spanish):");
    println!("   \"scan this website for vulnerabilities\"");
    println!("   \"crack passwords for instagram account\"");
    println!("   \"find oauth2 vulnerabilities in facebook\"");
    println!("   \"start daemon for continuous monitoring\"");
    println!("   \"check for leaked credentials\"");
    println!("   \"perform phishing simulation\"");
    println!("   \"bypass 2fa for this target\"");
    println!("   \"enumerate social media profiles\"");
    println!("   \"scan for viruses on my system\"");
    println!("   \"clean old files not opened for a month\"");
    println!("\n⚠️  Single word commands require: word + target + instructions");
    println!("   Example: \"scan example.com for oauth vulnerabilities\"");
    println!("   Example: \"crack passwords john.doe@gmail.com using social engineering\"");
    println!("\n🔧 Available tool types: OAuth2, Social Networks, Phishing, 2FA, Passwords, Malware, Files");
    println!("\nType 'exit' to quit\n");

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

                // New Paradigm: Smart Attack Orchestration
                println!("🔍 Processing input: {}", user_input);
                match nlp::parse_command(&http_client, user_input).await {
                    Ok(parsed) => {
                        println!("🤖 AI Interpretation:");
                        println!("   📍 Subject: {}", parsed.subject.as_ref().unwrap_or(&"None".to_string()));
                        println!("   🎯 Keywords: {}", parsed.keywords.join(", "));
                        println!("   📋 Context: {}", parsed.context.as_ref().unwrap_or(&"None".to_string()));
                        println!("   📊 Confidence: {:.1}%", parsed.confidence * 100.0);
                        println!("   📝 Plan: {}", parsed.explanation);
                        println!();

                        // Check for clarification needed
                        if parsed.needs_clarification {
                            println!("❓ Clarification needed:");
                            for question in &parsed.clarification_questions {
                                println!("   • {}", question);
                            }
                            println!("   Please provide more details and try again.\n");
                            continue;
                        }

                        // Double-check with user
                        println!("❓ Is this interpretation correct? (yes/no): ");
                        let mut confirmation = String::new();
                        match stdin.read_line(&mut confirmation) {
                            Ok(_) => {
                                let confirm = confirmation.trim().to_lowercase();
                                if confirm != "yes" && confirm != "y" {
                                    println!("❌ Interpretation rejected. Please rephrase your request.\n");
                                    continue;
                                }
                            }
                            Err(_) => {
                                println!("❌ Error reading confirmation. Skipping execution.\n");
                                continue;
                            }
                        }

                        println!("🚀 Executing smart attack sequence...\n");
                        println!("🔧 Command: {}\n", parsed.command);

                        // Check if it's a daemon command
                        if user_input.to_lowercase().contains("daemon") {
                            if user_input.to_lowercase().contains("start") {
                                let target = extract_target_from_nlp(user_input).unwrap_or("localhost".to_string());
                                let config = daemon::DaemonConfig::default();
                                let daemon_instance = daemon::FenrirDaemon::new(target, config);

                                match daemon_instance.start().await {
                                    Ok(()) => {
                                        let mut d = DAEMON.lock().await;
                                        *d = Some(daemon_instance);
                                        println!("✅ Daemon started successfully\n");
                                    }
                                    Err(e) => println!("❌ Failed to start daemon: {}\n", e),
                                }
                            } else if user_input.to_lowercase().contains("stop") {
                                let d = DAEMON.lock().await;
                                if let Some(daemon_instance) = &*d {
                                    match daemon_instance.stop().await {
                                        Ok(()) => println!("✅ Daemon stopped\n"),
                                        Err(e) => println!("❌ Failed to stop daemon: {}\n", e),
                                    }
                                } else {
                                    println!("❌ No daemon is running\n");
                                }
                            } else if user_input.to_lowercase().contains("status") {
                                let d = DAEMON.lock().await;
                                if let Some(daemon_instance) = &*d {
                                    println!("{}", daemon_instance.status().await);
                                } else {
                                    println!("🐺 No daemon is running\n");
                                }
                            }
                        }
                        // Check if it's a breach detection command
                        else if user_input.to_lowercase().contains("breach") || user_input.to_lowercase().contains("security breach") {
                            let d = DAEMON.lock().await;
                            if let Some(daemon_instance) = &*d {
                                let breach_detector = daemon_instance.breach_detector.lock().await;
                                if breach_detector.detected_breaches.is_empty() {
                                    println!("✅ No security breaches detected\n");
                                } else {
                                    println!("🚨 SECURITY BREACHES DETECTED:\n");
                                    for (i, breach) in breach_detector.detected_breaches.iter().enumerate() {
                                        println!("{}. {} - {}", i + 1, format!("{:?}", breach.breach_type), breach.description);
                                        println!("   Severity: {:?}", breach.severity);
                                        if !breach.recommendations.is_empty() {
                                            println!("   Recommendations:");
                                            for rec in &breach.recommendations {
                                                println!("     - {}", rec);
                                            }
                                        }
                                        println!();
                                    }
                                }
                            } else {
                                println!("❌ No daemon is running. Start daemon first to monitor breaches.\n");
                            }
                        }
                        // Check if it's OAuth2 analysis
                        else if user_input.to_lowercase().contains("oauth") {
                            let target = extract_target_from_nlp(user_input).unwrap_or("https://example.com/oauth".to_string());
                            let client_id = extract_client_id_from_nlp(user_input).unwrap_or("test_client".to_string());

                            match osint::crack_oauth2_flow(&target, &client_id).await {
                                Ok(vulnerabilities) => {
                                    if vulnerabilities.is_empty() {
                                        println!("✅ No OAuth2 vulnerabilities found\n");
                                    } else {
                                        println!("🚨 OAUTH2 VULNERABILITIES DETECTED:\n");
                                        for vuln in vulnerabilities {
                                            println!("• {} ({})", vuln.vuln_type, vuln.severity);
                                            println!("  Description: {}", vuln.description);
                                            println!("  PoC: {}\n", vuln.proof_of_concept);
                                        }
                                    }
                                }
                                Err(e) => println!("❌ OAuth2 analysis failed: {}\n", e),
                            }
                        }
                        // Check if it's social network enumeration
                        else if user_input.to_lowercase().contains("social") || user_input.to_lowercase().contains("instagram") ||
                                user_input.to_lowercase().contains("facebook") || user_input.to_lowercase().contains("twitter") {
                            let username = extract_username_from_nlp(user_input).unwrap_or("testuser".to_string());

                            match osint::enumerate_social_networks(&username).await {
                                Ok(profiles) => {
                                    println!("📱 SOCIAL NETWORK ENUMERATION RESULTS:\n");
                                    for (platform, profile) in profiles {
                                        println!("• {}: {}", platform, if profile.exists { "Found" } else { "Not found" });
                                        if profile.exists {
                                            println!("  URL: {}", profile.url);
                                            if let Some(followers) = profile.followers {
                                                println!("  Followers: {}", followers);
                                            }
                                            if let Some(posts) = profile.posts {
                                                println!("  Posts: {}", posts);
                                            }
                                        }
                                        println!();
                                    }
                                }
                                Err(e) => println!("❌ Social enumeration failed: {}\n", e),
                            }
                        }
                        // Check if it's password cracking
                        else if user_input.to_lowercase().contains("password") || user_input.to_lowercase().contains("crack") {
                            let target = extract_target_from_nlp(user_input).unwrap_or("test@example.com".to_string());
                            let platform = extract_platform_from_nlp(user_input).unwrap_or("generic".to_string());

                            match osint::crack_social_passwords(&target, &platform).await {
                                Ok(passwords) => {
                                    println!("🔐 GENERATED PASSWORD CANDIDATES FOR {}:\n", target);
                                    for (i, pwd) in passwords.iter().enumerate() {
                                        println!("{}. {}", i + 1, pwd);
                                    }
                                    println!("\n💡 Use these with password cracking tools like hashcat or john\n");
                                }
                                Err(e) => println!("❌ Password generation failed: {}\n", e),
                            }
                        }
                        // Check if it's phishing simulation
                        else if user_input.to_lowercase().contains("phish") {
                            let target = extract_target_from_nlp(user_input).unwrap_or("example.com".to_string());
                            let template = extract_template_from_nlp(user_input).unwrap_or("bank".to_string());

                            match osint::simulate_phishing_campaign(&target, &template).await {
                                Ok(results) => {
                                    println!("🎣 PHISHING SIMULATION RESULTS:\n");
                                    println!("• Emails sent: {}", results.emails_sent);
                                    println!("• Clicks detected: {}", results.clicks_detected);
                                    println!("• Credentials harvested: {}", results.credentials_harvested);
                                    println!("• Success rate: {:.1}%", results.success_rate * 100.0);
                                    println!("• Template used: {}", results.template_used);
                                    println!("\n💡 Recommendations:");
                                    for rec in &results.recommendations {
                                        println!("  - {}", rec);
                                    }
                                    println!();
                                }
                                Err(e) => println!("❌ Phishing simulation failed: {}\n", e),
                            }
                        }
                        // Check if it's 2FA bypass
                        else if user_input.to_lowercase().contains("2fa") || user_input.to_lowercase().contains("two factor") {
                            let target = extract_target_from_nlp(user_input).unwrap_or("test@example.com".to_string());
                            let method = extract_method_from_nlp(user_input).unwrap_or("sms".to_string());

                            match osint::bypass_2fa(&target, &method).await {
                                Ok(methods) => {
                                    println!("🔓 2FA BYPASS METHODS FOR {} ({}):\n", target, method);
                                    for (i, method_desc) in methods.iter().enumerate() {
                                        println!("{}. {}", i + 1, method_desc);
                                    }
                                    println!("\n⚠️  These are for educational purposes only\n");
                                }
                                Err(e) => println!("❌ 2FA bypass analysis failed: {}\n", e),
                            }
                        }
                        // Check if it's leaked database search
                        else if user_input.to_lowercase().contains("leak") || user_input.to_lowercase().contains("breach") {
                            let query = extract_query_from_nlp(user_input).unwrap_or("test@example.com".to_string());

                            match osint::search_leaked_databases(&query).await {
                                Ok(results) => {
                                    println!("🔍 LEAKED DATABASE SEARCH RESULTS FOR '{}':\n", query);
                                    for result in results {
                                        println!("• Database: {}", result.database);
                                        println!("  Found: {}", if result.found { "YES" } else { "NO" });
                                        if result.found {
                                            for breach in &result.breaches {
                                                println!("  - Date: {}", breach.date);
                                                println!("  - Records: {}", breach.records);
                                                println!("  - Description: {}", breach.description);
                                            }
                                        }
                                        println!();
                                    }
                                }
                                Err(e) => println!("❌ Leak search failed: {}\n", e),
                            }
                        }
                        // Check if it's quick passwords retriever (Phase 6)
                        else if user_input.to_lowercase().contains("password") && user_input.to_lowercase().contains("quick") {
                            let target = extract_target_from_nlp(user_input).unwrap_or("".to_string());
                            let context = extract_context_from_nlp(user_input).unwrap_or("generic".to_string());

                            match osint::quick_passwords_retriever(&target, &context).await {
                                Ok(passwords) => {
                                    println!("🔑 QUICK PASSWORDS RETRIEVER FOR '{}' ({}):\n", target, context);
                                    for (i, pwd) in passwords.iter().enumerate() {
                                        println!("{}. {}", i + 1, pwd);
                                    }
                                    println!("\n💡 Use these for quick testing or as starting points\n");
                                }
                                Err(e) => println!("❌ Quick passwords retrieval failed: {}\n", e),
                            }
                        }
                        // Default: use executor for other commands
                        else {
                            executor::handle_execute_command(&parsed.command);
                        }
                    }
                    Err(e) => {
                        println!("❌ Natural language processing failed: {}\n", e);
                        println!("💡 Try more descriptive requests like:");
                        println!("   \"scan example.com for vulnerabilities\"");
                        println!("   \"find oauth2 issues in facebook login\"");
                        println!("   \"crack passwords for john.doe@gmail.com\"");
                        println!("   \"start daemon monitoring\"");
                        println!("   \"check for leaked credentials\"\n");
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

/// Helper functions for extracting information from natural language input
fn extract_target_from_nlp(input: &str) -> Option<String> {
    // Simple extraction - look for URLs, domains, emails, IPs
    let url_regex = regex::Regex::new(r"https?://[^\s]+").unwrap();
    if let Some(url_match) = url_regex.find(input) {
        return Some(url_match.as_str().to_string());
    }

    let domain_regex = regex::Regex::new(r"\b[a-zA-Z0-9-]+\.[a-zA-Z]{2,}\b").unwrap();
    if let Some(domain_match) = domain_regex.find(input) {
        return Some(domain_match.as_str().to_string());
    }

    let email_regex = regex::Regex::new(r"[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}").unwrap();
    if let Some(email_match) = email_regex.find(input) {
        return Some(email_match.as_str().to_string());
    }

    // Look for words that might be targets
    let words: Vec<&str> = input.split_whitespace().collect();
    for word in words {
        if word.contains("example") || word.contains("test") || word.contains("localhost") {
            return Some(word.to_string());
        }
    }

    None
}

fn extract_client_id_from_nlp(input: &str) -> Option<String> {
    // Look for client_id patterns
    let client_id_regex = regex::Regex::new(r"client[_-]id[:=]\s*([^\s]+)").unwrap();
    if let Some(captures) = client_id_regex.captures(input) {
        if let Some(client_id) = captures.get(1) {
            return Some(client_id.as_str().to_string());
        }
    }

    // Default test client ID
    Some("test_client_id".to_string())
}

fn extract_username_from_nlp(input: &str) -> Option<String> {
    // Look for @ mentions or usernames
    let username_regex = regex::Regex::new(r"@([a-zA-Z0-9_]+)").unwrap();
    if let Some(captures) = username_regex.captures(input) {
        if let Some(username) = captures.get(1) {
            return Some(username.as_str().to_string());
        }
    }

    // Look for email-like patterns
    let email_regex = regex::Regex::new(r"([a-zA-Z0-9._%+-]+)@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}").unwrap();
    if let Some(captures) = email_regex.captures(input) {
        if let Some(username) = captures.get(1) {
            return Some(username.as_str().to_string());
        }
    }

    // Extract from words
    let words: Vec<&str> = input.split_whitespace().collect();
    for word in words {
        if word.len() > 3 && word.chars().all(|c| c.is_alphanumeric() || c == '_' || c == '.') {
            return Some(word.to_string());
        }
    }

    None
}

fn extract_platform_from_nlp(input: &str) -> Option<String> {
    let platforms = vec!["instagram", "facebook", "twitter", "linkedin", "github", "gmail", "yahoo", "hotmail"];

    for platform in platforms {
        if input.to_lowercase().contains(platform) {
            return Some(platform.to_string());
        }
    }

    Some("generic".to_string())
}

fn extract_template_from_nlp(input: &str) -> Option<String> {
    let templates = vec!["bank", "paypal", "amazon", "netflix", "facebook", "google", "microsoft"];

    for template in templates {
        if input.to_lowercase().contains(template) {
            return Some(template.to_string());
        }
    }

    Some("generic".to_string())
}

fn extract_method_from_nlp(input: &str) -> Option<String> {
    if input.to_lowercase().contains("sms") {
        Some("sms".to_string())
    } else if input.to_lowercase().contains("email") {
        Some("email".to_string())
    } else if input.to_lowercase().contains("totp") || input.to_lowercase().contains("app") {
        Some("totp".to_string())
    } else {
        Some("sms".to_string())
    }
}

fn extract_query_from_nlp(input: &str) -> Option<String> {
    // Extract emails, usernames, or search terms
    let email_regex = regex::Regex::new(r"[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}").unwrap();
    if let Some(email_match) = email_regex.find(input) {
        return Some(email_match.as_str().to_string());
    }

    let username_regex = regex::Regex::new(r"@([a-zA-Z0-9_]+)").unwrap();
    if let Some(captures) = username_regex.captures(input) {
        if let Some(username) = captures.get(1) {
            return Some(username.as_str().to_string());
        }
    }

    // Extract quoted terms
    let quote_regex = regex::Regex::new(r#""([^"]+)""#).unwrap();
    if let Some(captures) = quote_regex.captures(input) {
        if let Some(quoted) = captures.get(1) {
            return Some(quoted.as_str().to_string());
        }
    }

    // Default to first non-stop word
    let words: Vec<&str> = input.split_whitespace()
        .filter(|w| w.len() > 3)
        .collect();

    words.first().map(|s| s.to_string())
}

fn extract_context_from_nlp(input: &str) -> Option<String> {
    let contexts = vec!["admin", "administrator", "wifi", "network", "email", "mail", "web", "database", "server"];

    for context in contexts {
        if input.to_lowercase().contains(context) {
            return Some(context.to_string());
        }
    }

    Some("generic".to_string())
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
