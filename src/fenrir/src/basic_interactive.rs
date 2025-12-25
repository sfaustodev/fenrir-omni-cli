// 🔥 FENRIR BASIC INTERACTIVE - O básico que funcionava
// Com smart API integration - bash HTTP functions

use crate::natural_request;
use crate::operations::FenrirOperations;
use crate::security_protection::{get_security, validate_command};
use crate::smart_api::get_smart_api_client;
use crate::venz_agent::get_venz_agent;
use anyhow::Result;
use indicatif::ProgressBar;
use std::io::{self, Write};

pub struct BasicInteractive {
    pub operations: FenrirOperations,
    pub should_continue: bool,
}

impl BasicInteractive {
    pub fn new(operations: FenrirOperations) -> Self {
        Self {
            operations,
            should_continue: true,
        }
    }

    /// 🎯 MODO INTERATIVO BÁSICO
    pub async fn start_interactive_mode(&mut self) -> Result<()> {
        // Banner minimalista
        println!();
        let banner = r#"
██   ██ ██    ██ ██████  ███████ ██████
 ██ ██  ██    ██ ██   ██ ██      ██   ██
  ███   ██    ██ ██████  █████   ██████
 ██ ██  ██    ██ ██      ██      ██   ██
██   ██  ██████  ██      ███████ ██   ██
"#;
        for line in banner.lines() {
            if !line.trim().is_empty() {
                println!("{}", line.bright_red().bold());
            }
        }
        println!();

        loop {
            if !self.should_continue {
                break;
            }

            print!("🔥 fenrir> ");
            io::stdout().flush()?;

            let mut input = String::new();
            match io::stdin().read_line(&mut input) {
                Ok(0) => break,
                Ok(_) => {
                    let input = input.trim();

                    let parts: Vec<&str> = input.split_whitespace().collect();
                    if parts.is_empty() {
                        continue;
                    }

                    let command = parts[0];
                    let args = &parts[1..];

                    match command.to_lowercase().as_str() {
                        "sair" | "exit" | "quit" => {
                            println!("🐺 FENRIR encerrando. Até a próxima!");
                            break;
                        }
                        "security" => {
                            let security = get_security();
                            security.show_operation_log();
                        }
                        "ai" | "ask" => {
                            self.handle_smart_api(args).await?;
                        }
                        "gemini" => {
                            self.handle_gemini(args).await?;
                        }
                        "grok" | "xai" => {
                            self.handle_grok(args).await?;
                        }
                        "zai" => {
                            self.handle_zai(args).await?;
                        }
                        "qwen" => {
                            self.handle_qwen(args).await?;
                        }
                        "morder" => {
                            self.handle_morder(args).await?;
                        }
                        "rosnar" => {
                            self.handle_rosnar(args).await?;
                        }
                        "devorar" => {
                            self.handle_devorar(args).await?;
                        }
                        "venz" => {
                            self.handle_venz(args).await?;
                        }
                        "debug" => {
                            let security = get_security();
                            security.enable_debug_mode();
                            println!("⚠️ DEBUG MODE ATIVADO - Proteções relaxadas");
                        }
                        _ => {
                            self.handle_natural_language(input).await?;
                        }
                    }
                }
                Err(e) => {
                    println!("❌ Erro na entrada: {}", e);
                    break;
                }
            }
        }

        Ok(())
    }

    async fn handle_natural_language(&mut self, input: &str) -> Result<()> {
        let pb = ProgressBar::new_spinner();
        if let Err(e) = natural_request::process_natural_request(input, Some(&pb)).await {
            pb.finish_with_message("❌ Falha no pipeline natural.");
            eprintln!("❌ Erro no pipeline natural: {}", e);
        } else {
            pb.finish_with_message("✅ Pipeline natural concluído.");
        }
        Ok(())
    }

    /// 📍 COMANDO MORDER
    async fn handle_morder(&mut self, args: &[&str]) -> Result<()> {
        let target = args.join(" ");
        println!("🔥 MORDER: {}", target);

        // Venz cuida da invasão
        let venz_agent = get_venz_agent();
        let results = venz_agent.execute_task("scan", Some(&target)).await?;

        println!("✅ MORDER concluída:");
        for result in results {
            if !result.trim().is_empty() {
                println!("   {}", result);
            }
        }

        Ok(())
    }

    /// 🔍 COMANDO ROSNAR - COM PROTEÇÃO
    async fn handle_rosnar(&mut self, args: &[&str]) -> Result<()> {
        let target = args.join(" ");
        println!("🔍 ROSNAR: {}", target);

        // Validar segurança primeiro
        validate_command("rosnar", args)?;

        // Se passou na validação, Venz executa
        let venz_agent = get_venz_agent();
        let results = venz_agent.execute_task("recon", Some(&target)).await?;

        println!("✅ ROSNAR concluída:");
        for result in results {
            if !result.trim().is_empty() {
                println!("   {}", result);
            }
        }

        Ok(())
    }

    /// 🍽️ COMANDO DEVORAR
    async fn handle_devorar(&mut self, args: &[&str]) -> Result<()> {
        let target = args.join(" ");
        println!("🍽️ DEVORAR: {}", target);

        // Venz faz engenharia reversa completa
        let venz_agent = get_venz_agent();
        let results = venz_agent.execute_task("leak", Some(&target)).await?;

        println!("✅ DEVORAR concluída:");
        for result in results {
            if !result.trim().is_empty() {
                println!("   {}", result);
            }
        }

        Ok(())
    }

    /// 🥷 COMANDO VENZ - SEM CENSURA
    async fn handle_venz(&mut self, args: &[&str]) -> Result<()> {
        let command = args[0];
        let target = if args.len() > 1 {
            Some(args[1..].join(" "))
        } else {
            None
        };

        println!("🥷 VENZ: {} {:?}", command, target);

        // Venz executa SEM RESTRIÇÕES
        let venz_agent = get_venz_agent();
        let results = venz_agent.execute_task(command, target.as_deref()).await?;

        println!("✅ VENZ concluído:");
        for result in results {
            if !result.trim().is_empty() {
                println!("   {}", result);
            }
        }

        venz_agent.show_operation_log();

        Ok(())
    }

    /// 🤖 COMANDO AI - Smart routing
    async fn handle_smart_api(&mut self, args: &[&str]) -> Result<()> {
        let prompt = args.join(" ");
        let api_client = get_smart_api_client();

        println!("🤖 Smart AI Routing...");
        match api_client.smart_call(&prompt) {
            Ok(response) => {
                println!("✅ AI Response:");
                println!("{}", response);
            }
            Err(e) => {
                println!("❌ AI Error: {}", e);
            }
        }

        Ok(())
    }

    /// 💎 COMANDO GEMINI
    async fn handle_gemini(&mut self, args: &[&str]) -> Result<()> {
        let prompt = args.join(" ");
        let api_client = get_smart_api_client();

        println!("💎 Calling Gemini...");
        match api_client.call_gemini(&prompt) {
            Ok(response) => {
                println!("✅ Gemini Response:");
                println!("{}", response);
            }
            Err(e) => {
                println!("❌ Gemini Error: {}", e);
            }
        }

        Ok(())
    }

    /// 🔥 COMANDO GROK
    async fn handle_grok(&mut self, args: &[&str]) -> Result<()> {
        let prompt = args.join(" ");
        let api_client = get_smart_api_client();

        println!("🔥 Calling Grok...");
        match api_client.call_grok(&prompt) {
            Ok(response) => {
                println!("✅ Grok Response:");
                println!("{}", response);
            }
            Err(e) => {
                println!("❌ Grok Error: {}", e);
            }
        }

        Ok(())
    }

    /// ⚡ COMANDO ZAI
    async fn handle_zai(&mut self, args: &[&str]) -> Result<()> {
        let prompt = args.join(" ");
        let api_client = get_smart_api_client();

        println!("⚡ Calling ZAI (GLM 4.7)...");
        match api_client.call_zai(&prompt) {
            Ok(response) => {
                println!("✅ ZAI Response:");
                println!("{}", response);
            }
            Err(e) => {
                println!("❌ ZAI Error: {}", e);
            }
        }

        Ok(())
    }

    /// 🌟 COMANDO QWEN
    async fn handle_qwen(&mut self, args: &[&str]) -> Result<()> {
        let prompt = args.join(" ");
        let api_client = get_smart_api_client();

        println!("🌟 Calling Qwen...");
        match api_client.call_qwen(&prompt) {
            Ok(response) => {
                println!("✅ Qwen Response:");
                println!("{}", response);
            }
            Err(e) => {
                println!("❌ Qwen Error: {}", e);
            }
        }

        Ok(())
    }
}

/// 🔥 FUNÇÃO INTERATIVA GLOBAL
pub async fn start_basic_interactive_mode(operations: FenrirOperations) -> Result<()> {
    let mut interactive = BasicInteractive::new(operations);
    interactive.start_interactive_mode().await
}
