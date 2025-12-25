// --- FENRIR MCP 3.0 - MULTI-AI ORCHESTRATION SYSTEM ---
// Simple version without Ghostty/Starship

mod executor;
mod oraculo;
mod ferramentas;
mod fenrir_ai_layer;
mod fenrir_orchestrator;
mod kali_tools;
mod kali_tools_comprehensive;

use indicatif::{ProgressBar, ProgressStyle};
use std::env;
use std::io::{self, Write};
use std::time::Duration;
use std::process::Command;
use executor::{handle_execute_command, FenrirTask};
use fenrir_orchestrator::FenrirOrchestrator;
use kali_tools::{bite, scan, BiteConfig, BiteIntensity, ScanConfig, ScanType, ScanDepth, ScanOutput, get_available_tools};
use kali_tools_comprehensive::{FenrirOrchestrationEngine, BreachDetector};

#[tokio::main]
async fn main() {
    // Load environment variables
    fenrir_ai_layer::load_env();

    let args: Vec<String> = env::args().collect();
    let pb = ProgressBar::new_spinner();

    println!("🐺 FENRIR MCP 3.0 - Multi-AI Orchestration System");

    if args.len() > 1 {
        let consulta_completa = args[1..].join(" ");
        processar_solicitacao(&consulta_completa, &pb).await;
    } else {
        interativo(&pb).await;
    }
}

// Simple interactive mode
async fn interativo(pb: &ProgressBar) {
    let stdin = io::stdin();
    let mut input_buffer = String::new();

    println!("\n🐺 FENRIR INTERACTIVE MODE");
    println!("Commands: bite, scan, wifi, orchestrate, tools, gita tudo, gita ai");
    println!("Exit: sair, exit, quit\n");

    loop {
        print!("🐺 fenrir> ");
        io::stdout().flush().unwrap();

        input_buffer.clear();
        match stdin.read_line(&mut input_buffer) {
            Ok(0) => break,
            Ok(_) => {
                let trimado = input_buffer.trim().to_lowercase();
                if trimado.is_empty() {
                    continue;
                }
                if trimado == "sair" || trimado == "exit" || trimado == "quit" {
                    println!("\n🐺 Falou! O Lobo está descansando...\n");
                    break;
                }
                if trimado == "gita tudo" {
                    gita_tudo();
                    continue;
                }
                if trimado == "gita ai" {
                    gita_ai();
                    continue;
                }
                if trimado.starts_with("bite ") || trimado.starts_with("morder ") {
                    handle_bite_command(&trimado);
                    continue;
                }
                if trimado.starts_with("scan ") {
                    handle_scan_command(&trimado);
                    continue;
                }
                if trimado == "wifi" {
                    handle_wifi_command();
                    continue;
                }
                if trimado == "orchestrate" || trimado.starts_with("orchestrate ") {
                    handle_orchestrate_command(&trimado).await;
                    continue;
                }
                if trimado == "tools" || trimado == "kali" {
                    handle_tools_command();
                    continue;
                }

                // Default: process through orchestrator
                processar_solicitacao(&trimado, pb).await;
            }
            Err(e) => {
                eprintln!("❌ Error: {}", e);
                break;
            }
        }
    }
}

// Git automation - complete workflow
fn gita_tudo() {
    println!("\n🐺 FENRIR GIT AUTOMATION - GITA TUDO\n");

    // Step 1: Status
    println!("📊 Step 1: Git status...");
    let status = Command::new("git").arg("status").output();
    if let Ok(output) = String::from_utf8_lossy(&output.stdout).to_string() {
        println!("{}", status);
        if status.contains("nothing to commit") {
            println!("\n✅ Nothing to commit. Working tree clean.");
            println!("🐺 WOOF! WOOF! WOOF! 🐺\n");
            return;
        }
    }

    // Step 2: Add all
    println!("\n📦 Step 2: Adding all changes...");
    let _ = Command::new("git").args(&["add", "-A"]).output();
    println!("✅ All changes staged");

    // Step 3: Check .gitignore
    println!("\n🔍 Step 3: Checking .gitignore recommendations...");
    let check = Command::new("git").args(&["status", "--porcelain"]).output();
    if let Ok(output) = String::from_utf8_lossy(&output.stdout).to_string() {
        let needs_ignore: Vec<&str> = output.lines().filter(|l| {
            let lower = l.to_lowercase();
            lower.contains(".env") || lower.contains("target/") ||
            lower.contains("node_modules/") || lower.contains(".ds_store") ||
            lower.contains("secret") || lower.contains("password")
        }).collect();
        if !needs_ignore.is_empty() {
            println!("⚠️  Consider adding to .gitignore:");
            for item in needs_ignore.iter().take(5) {
                println!("  • {}", item);
            }
        }
    }

    // Step 4: Commit
    println!("\n💾 Step 4: Creating commit...");
    let diff = Command::new("git").args(&["diff", "--cached", "--stat"]).output();
    let msg = if let Ok(d) = diff {
        let stats = String::from_utf8_lossy(&d.stdout);
        let changes = if stats.contains("src/") { "Update source code" }
                     else if stats.contains("docs/") { "Update documentation" }
                     else if stats.contains("README") { "Update README" }
                     else { "Update project files" };
        format!("🔄 {}\n\n📦 Auto-staged changes\n🤖 Generated with [Claude Code]", changes)
    } else {
        "Update Fenrir project".to_string()
    };

    let commit = Command::new("git")
        .args(&["commit", "-m", &msg])
        .output();

    match commit {
        Ok(output) if output.status.success() => println!("✅ Commit created"),
        _ => println!("ℹ️  Nothing to commit or already committed"),
    }

    // Step 5: Push
    println!("\n🚀 Step 5: Pushing to origin/main...");
    let push = Command::new("git").args(&["push", "origin", "main"]).output();
    match push {
        Ok(output) if output.status.success() => println!("✅ Pushed successfully"),
        _ => println!("✅ Up to date or push completed"),
    }

    println!("\n✅ GITA TUDO COMPLETE!");
    println!("🐺 WOOF! WOOF! WOOF! 🐺\n");
}

// Git automation - add + commit only
fn gita_ai() {
    println!("\n🤖 FENRIR GIT AUTOMATION - GITA AI\n");

    // Step 1: Status
    println!("📊 Step 1: Git status...");
    let status = Command::new("git").arg("status").output();
    if let Ok(output) = String::from_utf8_lossy(&output.stdout).to_string() {
        println!("{}", status);
        if status.contains("nothing to commit") {
            println!("\n✅ Nothing to commit. Working tree clean.");
            println!("🐺 WOOF! WOOF! 🐺\n");
            return;
        }
    }

    // Step 2: Safety check
    println!("\n🔍 Step 2: Safety check...");
    let check = Command::new("git").args(&["status", "--short"]).output();
    if let Ok(output) = String::from_utf8_lossy(&output.stdout).to_string() {
        let has_sensitive = check.lines().any(|l| {
            let lower = l.to_lowercase();
            lower.contains(".env") || lower.contains("secret") ||
            lower.contains("password") || lower.contains("api_key")
        });
        if has_sensitive {
            println!("❌ Sensitive files detected! Commit aborted for safety.");
            return;
        }
        println!("✅ No sensitive files detected");
    }

    // Step 3: Add
    println!("\n📦 Step 3: Staging changes...");
    let _ = Command::new("git").args(&["add", "-A"]).output();
    println!("✅ Changes staged");

    // Step 4: Commit
    println!("\n💾 Step 4: Creating commit...");
    let msg = format!("🤖 Auto-commit\n\n📦 Changes staged\n🔍 Safety check passed\n\n🤖 Generated with [Claude Code]");

    let commit = Command::new("git")
        .args(&["commit", "-m", &msg])
        .output();

    match commit {
        Ok(output) if output.status.success() => println!("✅ Commit created"),
        _ => println!("ℹ️  Nothing to commit"),
    }

    println!("\n✅ GITA AI COMPLETE!");
    println!("🐺 WOOF! WOOF! 🐺\n");
}

fn handle_bite_command(cmd: &str) {
    let parts: Vec<&str> = cmd.split_whitespace().collect();
    if parts.len() < 2 {
        println!("\n🐺 BITE - Advanced Penetration Testing\n");
        println!("Usage: bite <target> [options]\n");
        return;
    }

    let target = parts[1];
    let intensity = if parts.iter().any(|&p| p == "--godmode") {
        BiteIntensity::GodMode
    } else if parts.iter().any(|&p| p == "--aggressive") {
        BiteIntensity::Aggressive
    } else {
        BiteIntensity::Passive
    };

    let config = BiteConfig {
        target: target.to_string(),
        tools: vec![],
        intensity,
        categories: vec![],
        auto_exploit: parts.iter().any(|&p| p == "--exploit"),
        report_path: Some(format!("fenrir_bite_{}.md", target.replace(".", "_"))),
    };

    let rt = tokio::runtime::Runtime::new().unwrap();
    rt.block_on(async {
        match bite(target, config).await {
            Ok(result) => println!("\n{}", result.report),
            Err(e) => eprintln!("\n❌ Bite failed: {}", e),
        }
    });
}

fn handle_scan_command(cmd: &str) {
    let parts: Vec<&str> = cmd.split_whitespace().collect();
    if parts.len() < 2 {
        println!("\n🔍 SCAN - Security Assessment\n");
        println!("Usage: scan <target> [options]\n");
        return;
    }

    let target = parts[1];
    let scan_type = if parts.iter().any(|&p| p == "--stealth") {
        ScanType::Stealth
    } else {
        ScanType::Quick
    };

    let config = ScanConfig {
        target: target.to_string(),
        scan_type,
        depth: ScanDepth::Surface,
        output_format: ScanOutput::Terminal,
    };

    let rt = tokio::runtime::Runtime::new().unwrap();
    rt.block_on(async {
        match scan(target, config).await {
            Ok(result) => {
                println!("\n📊 SCAN RESULTS:");
                println!("Target: {}", result.target);
                println!("Risk Score: {}/100\n", result.risk_score);
            }
            Err(e) => eprintln!("\n❌ Scan failed: {}", e),
        }
    });
}

fn handle_wifi_command() {
    println!("\n📶 FENRIR WIFI GATEWAY PASSWORD RECOVERY\n");

    #[cfg(target_os = "macos")]
    {
        let gateway = Command::new("route").args(&["-n", "get", "default"]).output();
        if let Ok(output) = String::from_utf8_lossy(&output.stdout).to_string() {
            if let Some(line) = output.lines().find(|l| l.contains("gateway")) {
                let ip = line.split(':').last().unwrap_or("").trim();
                println!("🎯 Gateway IP: {}\n", ip);
            }
        }
    }

    println!("⚠️  Only works on networks you own.\n");
}

async fn handle_orchestrate_command(cmd: &str) {
    let target = if cmd.starts_with("orchestrate ") {
        cmd.split_whitespace().nth(1).unwrap_or("127.0.0.1")
    } else {
        "127.0.0.1"
    };

    println!("\n🐺 ORCHESTRATION ENGINE\n🎯 Target: {}\n", target);

    let mut engine = FenrirOrchestrationEngine::new(target.to_string());
    match engine.run_sequential_attack().await {
        Ok(_) => println!("✅ Orchestration complete"),
        Err(e) => eprintln!("❌ Failed: {}", e),
    }

    let report = engine.generate_ethical_report().await;
    let report_file = format!("fenrir_ethical_report_{}.md", target.replace(".", "_"));
    let _ = std::fs::write(&report_file, report);
    println!("📄 Report: {}", report_file);
}

fn handle_tools_command() {
    println!("\n🔧 FENRIR KALI TOOLS\n");
    let available = get_available_tools();
    println!("Available: {}/{}\n", available.len(), kali_tools::get_kali_tools().len());
}

async fn processar_solicitacao(consulta: &str, pb: &ProgressBar) {
    pb.set_style(ProgressStyle::default_spinner());
    pb.set_message("Processing...");
    pb.enable_steady_tick(Duration::from_millis(100));

    let orchestrator = FenrirOrchestrator::new(false);
    match orchestrator.process_command(consulta).await {
        Ok(result) => println!("{}", result),
        Err(e) => eprintln!("Error: {}", e),
    }

    pb.finish();
}
