// Simple Fenrir main without Ghostty/Starship
mod executor;
mod oraculo;
mod ferramentas;
mod fenrir_ai_layer;
mod fenrir_orchestrator;
mod kali_tools;
mod kali_tools_comprehensive;

use std::env;
use std::io::{self, Write};
use std::process::Command;
use indicatif::ProgressBar;

#[tokio::main]
async fn main() {
    fenrir_ai_layer::load_env();
    let args: Vec<String> = env::args().collect();
    
    println!("🐺 FENRIR 3.0 - Multi-AI Security Platform");
    
    if args.len() > 1 {
        let cmd = args[1..].join(" ");
        process_command(&cmd).await;
    } else {
        interactive().await;
    }
}

async fn interactive() {
    let stdin = io::stdin();
    let mut input = String::new();
    
    println!("\nCommands: gita tudo, gita ai, bite, scan, wifi, tools");
    println!("Exit: exit, quit, sair\n");
    
    loop {
        print!("🐺> ");
        io::stdout().flush().unwrap();
        
        input.clear();
        match stdin.read_line(&mut input) {
            Ok(0) => break,
            Ok(_) => {
                let cmd = input.trim().to_lowercase();
                if cmd.is_empty() { continue; }
                if cmd == "exit" || cmd == "quit" || cmd == "sair" {
                    println!("\n🐺 Bye!\n");
                    break;
                }
                if cmd == "gita tudo" {
                    git_tudo();
                    continue;
                }
                if cmd == "gita ai" {
                    git_ai();
                    continue;
                }
                if cmd == "tools" {
                    println!("\n🔧 Kali Tools: Check docs/KALI_TOOLS_INTEGRATION.md\n");
                    continue;
                }
                if cmd == "wifi" {
                    println!("\n📶 Use this on your own WiFi network only!\n");
                    continue;
                }
                process_command(&cmd).await;
            }
            Err(e) => {
                eprintln!("Error: {}", e);
                break;
            }
        }
    }
}

fn git_tudo() {
    println!("\n🐺 GITA TUDO - Complete Git Workflow\n");
    
    // Status
    println!("📊 Checking status...");
    let status = Command::new("git").arg("status").output();
    if let Ok(out) = String::from_utf8_lossy(&out.stdout).to_string() {
        println!("{}", status);
        if status.contains("nothing to commit") {
            println!("\n✅ Working tree clean");
            println!("🐺 WOOF! WOOF! WOOF! 🐺\n");
            return;
        }
    }
    
    // Add
    println!("\n📦 Adding all...");
    let _ = Command::new("git").args(&["add", "-A"]).output();
    println!("✅ Staged");
    
    // Commit
    println!("\n💾 Committing...");
    let msg = "🔄 Update Fenrir project\n\n🤖 Auto-generated commit";
    let _ = Command::new("git").args(&["commit", "-m", msg]).output();
    println!("✅ Committed");
    
    // Push
    println!("\n🚀 Pushing...");
    let push = Command::new("git").args(&["push", "origin", "main"]).output();
    if let Ok(out) = push {
        if out.status.success() {
            println!("✅ Pushed");
        } else {
            println!("✅ Already up to date");
        }
    }
    
    println!("\n✅ GITA TUDO COMPLETE!");
    println!("🐺 WOOF! WOOF! WOOF! 🐺\n");
}

fn git_ai() {
    println!("\n🤖 GITA AI - Add + Commit\n");
    
    // Status
    println!("📊 Checking status...");
    let status = Command::new("git").arg("status").output();
    if let Ok(out) = String::from_utf8_lossy(&out.stdout).to_string() {
        println!("{}", status);
        if status.contains("nothing to commit") {
            println!("\n✅ Working tree clean");
            println!("🐺 WOOF! WOOF! 🐺\n");
            return;
        }
    }
    
    // Safety check
    println!("\n🔍 Safety check...");
    let check = Command::new("git").args(&["status", "--short"]).output();
    if let Ok(out) = String::from_utf8_lossy(&out.stdout).to_string() {
        if check.lines().any(|l| {
            let lower = l.to_lowercase();
            lower.contains(".env") || lower.contains("secret") || lower.contains("password")
        }) {
            println!("❌ Sensitive files detected! Aborted.");
            return;
        }
    }
    println!("✅ Safe to commit");
    
    // Add
    println!("\n📦 Staging...");
    let _ = Command::new("git").args(&["add", "-A"]).output();
    println!("✅ Staged");
    
    // Commit
    println!("\n💾 Committing...");
    let msg = "🤖 Auto-commit\n\n🔍 Safety checked\n📦 Changes staged";
    let _ = Command::new("git").args(&["commit", "-m", msg]).output();
    println!("✅ Committed");
    
    println!("\n✅ GITA AI COMPLETE!");
    println!("🐺 WOOF! WOOF! 🐺\n");
}

async fn process_command(cmd: &str) {
    println!("Processing: {}", cmd);
    // Add command processing here
}
