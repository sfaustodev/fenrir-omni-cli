// 🔥 FENRIR BASIC INTERACTIVE - O básico que funcionava
// Sem IA pra não dar merda - comandos diretos

use crate::natural_request;
use crate::operations::FenrirOperations;
use crate::security_protection::{get_security, validate_command};
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
        println!("\n🔥 FENRIR BASIC MODE - O que realmente funciona");
        println!("💎 Sem IA pra não dar merda - comandos diretos");
        println!("🥷 Venz aguardando ordens sem censura");
        println!("🔒 Proteções anti-rosnar ativas");
        println!("🚨 Express confirmation para operações de risco");
        println!("\n🎯 COMANDOS DISPONÍVEIS:");
        println!("   📍 morder <alvo>      - Ataque externo");
        println!("   🔍 rosnar <alvo>     - Scan defensivo (SÓ EXTERNO)");
        println!("   🍽️ devorar <alvo>   - Engenharia reversa");
        println!("   🥷 venz <comando>     - Venz executa sem censura");
        println!("   🔒 security          - Status de segurança");
        println!("   🚪 sair              - Sair");
        println!("\n⚠️ REGRAS DO CHEFE:");
        println!("   ✅ ROSNAR só em DISCO EXTERNO FÍSICO");
        println!("   ✅ Venz SEM CENSURA = AGENTE MULTI-USO");
        println!("   ✅ Operações de risco = CONFIRMAÇÃO CHEFE");
        println!("");

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
                    if input.is_empty() {
                        continue;
                    }

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
                        "morder" => {
                            if args.is_empty() {
                                println!("❌ Uso: morder <alvo>");
                                continue;
                            }
                            self.handle_morder(args).await?;
                        }
                        "rosnar" => {
                            if args.is_empty() {
                                println!("❌ Uso: rosnar <alvo-externo>");
                                continue;
                            }
                            self.handle_rosnar(args).await?;
                        }
                        "devorar" => {
                            if args.is_empty() {
                                println!("❌ Uso: devorar <alvo>");
                                continue;
                            }
                            self.handle_devorar(args).await?;
                        }
                        "venz" => {
                            if args.is_empty() {
                                println!("❌ Uso: venz <comando> [alvo]");
                                continue;
                            }
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
}

/// 🔥 FUNÇÃO INTERATIVA GLOBAL
pub async fn start_basic_interactive_mode(operations: FenrirOperations) -> Result<()> {
    let mut interactive = BasicInteractive::new(operations);
    interactive.start_interactive_mode().await
}
