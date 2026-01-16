// --- FENRIR 4.0 - DIRECT AI COMMANDS ---
// Simple interface with direct AI calls

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
    static ref DAEMON: Arc<Mutex<Option<daemon::FenrirDaemon>>> = Arc::new(Mutex::new(None));
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

    println!("🐺 FENRIR 4.0 - AI-Powered Command Translation");
    println!("Security Testing Platform\n");

    let stdin = io::stdin();
    let mut input = String::new();
    let http_client = Client::new();

    println!("🎯 Special Commands:");
    println!("  scan <target> [comprehensive]  - Security scan");
    println!("  bite <target> [aggressive]     - Penetration test");
    println!("  batch recon <target>           - Batch reconnaissance");
    println!("  batch vuln <target>            - Batch vulnerability scan");
    println!("  batch passwd <target>          - Batch password attacks");
    println!("  batch full <target>            - Full penetration test suite");
    println!("  daemon start [target]          - Start continuous security monitoring");
    println!("  daemon stop                     - Stop daemon service");
    println!("  daemon status                   - Show daemon status");
    println!("  security breach detected        - Check for detected breaches");
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

                // Daemon commands
                if user_input.starts_with("daemon ") {
                    let parts: Vec<&str> = user_input.split_whitespace().collect();
                    if parts.len() >= 2 {
                        match parts[1] {
                            "start" => {
                                let target = parts.get(2).unwrap_or(&"localhost").to_string();
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
                            }
                            "stop" => {
                                let d = DAEMON.lock().await;
                                if let Some(daemon_instance) = &*d {
                                    match daemon_instance.stop().await {
                                        Ok(()) => println!("✅ Daemon stopped\n"),
                                        Err(e) => println!("❌ Failed to stop daemon: {}\n", e),
                                    }
                                } else {
                                    println!("❌ No daemon is running\n");
                                }
                            }
                            "status" => {
                                let d = DAEMON.lock().await;
                                if let Some(daemon_instance) = &*d {
                                    println!("{}", daemon_instance.status().await);
                                } else {
                                    println!("🐺 No daemon is running\n");
                                }
                            }
                            _ => println!("❌ Unknown daemon command. Use: start, stop, status\n"),
                        }
                    } else {
                        println!("❌ Usage: daemon <start|stop|status> [target]\n");
                    }
                    continue;
                }

                // Security breach command
                if user_input == "security breach detected" {
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
                    continue;
                }

                // Batch commands
                if user_input.starts_with("batch ") {
                    let parts: Vec<&str> = user_input.split_whitespace().collect();
                    if parts.len() >= 3 {
                        let batch_type = parts[1];
                        let target = parts[2];

                        let executor = batch_executor::BatchExecutor::new(target.to_string());

                        match batch_type {
                            "recon" => {
                                let job = batch_executor::create_recon_job(target);
                                match executor.submit_job(job.clone()).await {
                                    Ok(job_id) => {
                                        println!("🔍 Submitted reconnaissance job: {}", job_id);
                                        match executor.execute_job(&job_id).await {
                                            Ok(result) => {
                                                println!(
                                                    "✅ Recon complete: {}/{} tools successful",
                                                    result.successful_tools, result.total_tools
                                                );
                                                println!("{}", result.summary);
                                            }
                                            Err(e) => println!("❌ Recon failed: {}", e),
                                        }
                                    }
                                    Err(e) => println!("❌ Failed to submit job: {}", e),
                                }
                            }
                            "vuln" => {
                                let job = batch_executor::create_vuln_scan_job(target);
                                match executor.submit_job(job.clone()).await {
                                    Ok(job_id) => {
                                        println!("🔎 Submitted vulnerability scan job: {}", job_id);
                                        match executor.execute_job(&job_id).await {
                                            Ok(result) => {
                                                println!(
                                                    "✅ Vuln scan complete: {}/{} tools successful",
                                                    result.successful_tools, result.total_tools
                                                );
                                                println!("{}", result.summary);
                                            }
                                            Err(e) => println!("❌ Vuln scan failed: {}", e),
                                        }
                                    }
                                    Err(e) => println!("❌ Failed to submit job: {}", e),
                                }
                            }
                            "passwd" => {
                                let job = batch_executor::create_password_attack_job(target);
                                match executor.submit_job(job.clone()).await {
                                    Ok(job_id) => {
                                        println!("🔐 Submitted password attack job: {}", job_id);
                                        match executor.execute_job(&job_id).await {
                                            Ok(result) => {
                                                println!("✅ Password attack complete: {}/{} tools successful",
                                                    result.successful_tools, result.total_tools);
                                                println!("{}", result.summary);
                                            }
                                            Err(e) => println!("❌ Password attack failed: {}", e),
                                        }
                                    }
                                    Err(e) => println!("❌ Failed to submit job: {}", e),
                                }
                            }
                            "full" => {
                                let job = batch_executor::create_full_pentest_job(target);
                                match executor.submit_job(job.clone()).await {
                                    Ok(job_id) => {
                                        println!("🎯 Submitted full pentest job: {}", job_id);
                                        match executor.execute_job(&job_id).await {
                                            Ok(result) => {
                                                println!("✅ Full pentest complete: {}/{} tools successful",
                                                    result.successful_tools, result.total_tools);
                                                println!("{}", result.summary);
                                            }
                                            Err(e) => println!("❌ Full pentest failed: {}", e),
                                        }
                                    }
                                    Err(e) => println!("❌ Failed to submit job: {}", e),
                                }
                            }
                            _ => {
                                println!(
                                    "❌ Unknown batch type: {}. Use: recon, vuln, passwd, full",
                                    batch_type
                                );
                            }
                        }
                    } else {
                        println!("❌ Usage: batch <type> <target>");
                        println!("   Types: recon, vuln, passwd, full");
                    }
                    continue;
                }

                // Parse commands with arguments
                let parts: Vec<&str> = user_input.splitn(3, ' ').collect();
                if parts.len() >= 2 {
                    match parts[0] {
                        "batch" => {
                            if parts.len() < 3 {
                                println!("❌ Usage: batch <type> <target>\n");
                                println!("  Types: recon, vuln, passwd, full\n");
                                continue;
                            }
                            let batch_type = parts[1];
                            let target = parts[2];

                            let job = match batch_type {
                                "recon" => batch_executor::create_recon_job(target),
                                "vuln" => batch_executor::create_vuln_scan_job(target),
                                "passwd" => batch_executor::create_password_attack_job(target),
                                "full" => batch_executor::create_full_pentest_job(target),
                                _ => {
                                    println!("❌ Unknown batch type: {}\n", batch_type);
                                    println!("  Available: recon, vuln, passwd, full\n");
                                    continue;
                                }
                            };

                            let executor = batch_executor::BatchExecutor::new(target.to_string());
                            match executor.submit_job(job.clone()).await {
                                Ok(job_id) => {
                                    println!("🔧 Submitted batch job: {}", job.name);
                                    println!("📋 Job ID: {}", job_id);
                                    println!("🎯 Target: {}", target);
                                    println!("🔧 Tools: {}", job.tools.len());
                                    println!("⚙️  Mode: {:?}\n", job.config.mode);

                                    // Execute immediately
                                    match executor.execute_job(&job_id).await {
                                        Ok(result) => {
                                            println!("✅ BATCH EXECUTION COMPLETE");
                                            println!(
                                                "🎯 Successful: {}/{}",
                                                result.successful_tools, result.total_tools
                                            );
                                            println!("❌ Failed: {}", result.failed_tools);
                                            println!(
                                                "⏱️  Total Time: {:.2}s",
                                                result.total_execution_time.as_secs_f64()
                                            );
                                            println!("\n📄 Summary:\n{}\n", result.summary);
                                        }
                                        Err(e) => println!("❌ Batch execution failed: {}\n", e),
                                    }
                                }
                                Err(e) => println!("❌ Failed to submit job: {}\n", e),
                            }
                            continue;
                        }
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

                                    // Suggest next steps
                                    println!("\n💡 NEXT STEPS SUGGESTIONS:");
                                    println!("  • bite {}          - Run penetration test on discovered vulnerabilities", target);
                                    println!("  • batch vuln {}     - Execute comprehensive vulnerability scan", target);
                                    println!("  • batch recon {}    - Gather more intelligence on the target", target);
                                    println!("  • security breach detected - Check for detected security breaches\n");
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
                                    println!(
                                        "💥 Vulnerabilities: {}",
                                        result.vulnerabilities.len()
                                    );
                                    if !result.vulnerabilities.is_empty() {
                                        println!(
                                            "📊 Vulnerabilities:\n{}",
                                            result.vulnerabilities.join("\n")
                                        );
                                    }
                                    println!("📄 Report:\n{}\n", result.report);

                                    // Suggest next steps
                                    println!("\n💡 NEXT STEPS SUGGESTIONS:");
                                    println!("  • batch full {}     - Run complete penetration test suite", target);
                                    println!("  • security breach detected - Check for security breaches found during testing");
                                    println!("  • batch passwd {}    - Attempt password attacks on discovered services", target);
                                    println!("  • scan {} comprehensive - Perform deeper security assessment\n", target);
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
                            match nlp::parse_command(&http_client, user_input).await {
                                Ok(parsed) => {
                                    println!("🤖 {}: {}", parsed.source, parsed.explanation);
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
                    match nlp::parse_command(&http_client, user_input).await {
                        Ok(parsed) => {
                            println!("🤖 {}: {}", parsed.source, parsed.explanation);
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
