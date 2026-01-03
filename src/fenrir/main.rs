pub mod executor;
mod oraculo;
mod ferramentas;
mod fenrir_ai_layer;
mod fenrir_orchestrator;
mod kali_tools;
mod kali_tools_comprehensive;
mod git_automation;
mod cli;
mod confirm;
mod disk_cleanup;
mod http_client;
mod secrets;
mod metrics;
mod health;
mod circuit_breaker;
#[cfg(feature = "crypto")]
mod solana;
#[cfg(feature = "crypto")]
mod zcash;
#[cfg(feature = "crypto")]
mod liquidity;
mod plugins;
mod wrapper;
mod sandbox;
mod bugbounty;
mod osint;
mod net;
mod iphone_pentest;
mod modern_pentest;
mod python_plugins;
mod ethical_protocol;

use std::io::{self, Write};
use git_automation::{gita_ai, gita_tudo};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    println!("🐺 FENRIR OMNI CLI - Advanced Penetration Testing Framework");
    println!("🤖 Protocol 0: Ethical Context Framework Active");
    println!("🔒 Humans make decisions, AI executes tasks");
    println!();

    // Check if arguments provided
    let args: Vec<String> = std::env::args().collect();

    match args.get(1).map(String::as_str) {
        Some("gita") => match args.get(2).map(String::as_str) {
            Some("tudo") => match args.get(3).map(String::as_str) {
                Some("ai") => {
                    println!("🤖 Executing GITA TUDO + AI...");
                    gita_tudo();
                    gita_ai();
                    println!("✅ GITA TUDO AI Complete!");
                }
                _ => {
                    println!("🔄 Executing GITA TUDO...");
                    gita_tudo();
                }
            },
            Some("ai") => {
                println!("🤖 Executing GITA AI...");
                gita_ai();
            }
            _ => {
                println!("❌ Unknown gita command: {}", args.get(2).unwrap_or(&"".to_string()));
            }
        },
        Some(cmd) => {
            println!("❌ Unknown command: {}", cmd);
        }
        None => {
            // Interactive mode
            println!("🔥 Enter interactive mode. Type 'help' for commands.");
            println!("🐺 Available commands:");
            println!("  gita tudo ai - Git automation with AI assistance");
            println!("  gita tudo - Full git automation");
            println!("  gita ai - Safe git automation with AI checks");
            println!("  help - Show this help");
            println!("  exit - Exit Fenrir");
            println!();

            loop {
                print!("fenrir> ");
                io::stdout().flush()?;

                let mut input = String::new();
                io::stdin().read_line(&mut input)?;
                let input = input.trim();

                match input {
                    "exit" | "quit" => break,
                    "help" => {
                        println!("🐺 FENRIR COMMANDS:");
                        println!("  gita tudo ai - Git automation with AI assistance");
                        println!("  gita tudo - Full git automation");
                        println!("  gita ai - Safe git automation with AI checks");
                        println!("  help - Show this help");
                        println!("  exit - Exit Fenrir");
                    }
                    "gita tudo ai" => {
                        println!("🤖 Executing GITA TUDO + AI...");
                        gita_tudo();
                        gita_ai();
                        println!("✅ GITA TUDO AI Complete!");
                    }
                    "gita tudo" => {
                        println!("🔄 Executing GITA TUDO...");
                        gita_tudo();
                    }
                    "gita ai" => {
                        println!("🤖 Executing GITA AI...");
                        gita_ai();
                    }
                    "" => continue,
                    _ => {
                        println!("❌ Unknown command: {}", input);
                        println!("💡 Type 'help' for available commands");
                    }
                }
                println!();
            }

            println!("🐺 Fenrir session ended. Stay ethical!");
            Ok(())
        }
    }
}
