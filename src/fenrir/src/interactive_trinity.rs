// 🔥 FENRIR TRINITY IA - MODO INTERATIVO AVANÇADO
// Chain of Thoughts com coordenação Gemini + Claude + Grok

use crate::grok_coordinator::TrinityCoordinator;
use crate::operations::FenrirOperations;
use anyhow::Result;
use std::io::{self, Write};

pub struct InteractiveTrinity {
    pub coordinator: TrinityCoordinator,
    pub operations: FenrirOperations,
}

impl InteractiveTrinity {
    pub fn new() -> Result<Self> {
        let coordinator = TrinityCoordinator::new().map_err(|e| {
            eprintln!("❌ Erro ao inicializar coordenador Trinity: {}", e);
            eprintln!("💡 Verifique se a variável de ambiente $GEMINI_API_KEY está configurada");
            e
        })?;

        let operations = FenrirOperations::new()?;

        Ok(Self {
            coordinator,
            operations,
        })
    }

    /// 🧠 MODO INTERATIVO TRINDITY - Chain of Thoughts completo
    pub async fn start_interactive_mode(&self) -> Result<()> {
        println!("\n🔴🔴🔴 FENRIR TRINITY IA - MODO INTERATIVO AVANÇADO 🔴🔴🔴");
        println!("💀 Coordenação: Gemini (contexto) + Claude (complexidade) + Grok (real-time)");
        println!("🚀 Digite 'ajuda' para comandos ou 'sair' para encerrar");
        println!("");

        loop {
            print!("🐺🧠🔥 Trinity> ");
            io::stdout().flush()?;

            let mut input = String::new();
            match io::stdin().read_line(&mut input) {
                Ok(0) => break, // Ctrl+D
                Ok(_) => {
                    let input = input.trim();
                    if input.is_empty() {
                        continue;
                    }

                    match input.to_lowercase().as_str() {
                        "sair" | "exit" | "quit" => {
                            println!("\n🐺 Trinity IA encerrando... O Lobo Devorador descansa.");
                            break;
                        }
                        "ajuda" | "help" => {
                            self.show_help();
                            continue;
                        }
                        "status" => {
                            self.show_status();
                            continue;
                        }
                        "modo" => {
                            self.show_coordination_mode();
                            continue;
                        }
                        "test" => {
                            self.run_test_scenario().await?;
                            continue;
                        }
                        _ => {
                            // Processar comando via Chain of Thoughts
                            if let Err(e) = self.process_with_chain_of_thoughts(input).await {
                                eprintln!("❌ Erro no processamento: {}", e);
                            }
                        }
                    }
                }
                Err(e) => {
                    eprintln!("❌ Erro lendo input: {}", e);
                    break;
                }
            }
        }

        Ok(())
    }

    /// 🧠 CHAIN OF THOUGHTS - Processamento completo
    async fn process_with_chain_of_thoughts(&self, input: &str) -> Result<()> {
        println!("\n🔥 INICIANDO CHAIN OF THOUGHTS PARA: {}", input);
        println!("");

        // Processar através do coordenador Trinity
        self.coordinator.process_interactive_request(input).await?;

        Ok(())
    }

    /// 📋 MENU DE AJUDA
    fn show_help(&self) {
        println!("\n📋 FENRIR TRINITY IA - COMANDOS DISPONÍVEIS:");
        println!("");
        println!("🎯 COMANDOS OPERACIONAIS:");
        println!("  morder <alvo>      - Ataque externo brutal");
        println!("  rosnar [alvo]      - Scan defensivo interno");
        println!("  devorar <alvo>    - Engenharia reversa completa");
        println!("  godmode           - Ativar poderes divinos");
        println!("");
        println!("🧠 COMANDOS TRINITY:");
        println!("  ajuda             - Mostrar este menu");
        println!("  status            - Status da coordenação IA");
        println!("  modo              - Modo de coordenação atual");
        println!("  test              - Executar cenário de teste");
        println!("  sair              - Encerrar Trinity IA");
        println!("");
        println!("🔥 EXEMPLOS:");
        println!("  morder bitcoin2000");
        println!("  rosnar sistema");
        println!("  devorar explorer.exe");
        println!("  godmode");
        println!("");
    }

    /// 📊 STATUS DA COORDENAÇÃO
    fn show_status(&self) {
        println!("\n📊 STATUS FENRIR TRINITY IA:");
        println!("  🧠 Gemini: Contexto e memória ATIVO ✅");
        println!("  🔥 Claude: Complexidade e planejamento ATIVO ✅");
        println!("  🚀 Grok 4.1 Fast: Tempo real e aprovação ATIVO ✅");
        println!("  🔴 FENRIR GOD MODE: Executor final ATIVO ✅");
        println!("  💀 Operações táticas: PRONTAS ✅");
        println!("");
    }

    /// ⚙️ MODO DE COORDENAÇÃO
    fn show_coordination_mode(&self) {
        println!("\n⚙️ MODO DE COORDENAÇÃO ATUAL:");
        println!("  🧠 Chain of Thoughts: ATIVO 🟢");
        println!("  🔥 Consenso Gemini+Grok: OBRIGATÓRIO 🟢");
        println!("  🚀 Aprovação FENRIR: FINAL 🟢");
        println!("  💀 GOD MODE: PERMANENTE 🟢");
        println!("");
    }

    /// 🧪 CENÁRIO DE TESTE
    async fn run_test_scenario(&self) -> Result<()> {
        println!("\n🧪 EXECUTANDO CENÁRIO DE TESTE TRINITY...");
        println!("");

        let test_input = "morder bitcoin2000 com godmode máximo";

        println!("🎯 Input de teste: {}", test_input);
        println!("");

        self.process_with_chain_of_thoughts(test_input).await?;

        println!("\n✅ CENÁRIO DE TESTE CONCLUÍDO!");
        println!("");

        Ok(())
    }

    /// 🔥 MODO DE EMERGÊNCIA - Ataque rápido sem Chain of Thoughts
    pub async fn emergency_mode(&self, input: &str) -> Result<()> {
        println!("\n🚨 MODO DE EMERGÊNCIA FENRIR - EXECUÇÃO DIRETA!");
        println!("💀 Pulando Chain of Thoughts para resposta imediata");
        println!("");

        // Processamento direto baseado no input
        if input.contains("morder") {
            let parts: Vec<&str> = input.split_whitespace().collect();
            if let Some(target) = parts.get(1) {
                println!("🔥 MODO EMERGÊNCIA - MORDER {}", target);
                self.operations.execute_morder(target).await?;
            }
        } else if input.contains("rosnar") {
            println!("🔥 MODO EMERGÊNCIA - ROSNAR SISTEMA");
            self.operations.execute_rosnar(None).await?;
        } else if input.contains("devorar") {
            let parts: Vec<&str> = input.split_whitespace().collect();
            if let Some(target) = parts.get(1) {
                println!("🔥 MODO EMERGÊNCIA - DEVORAR {}", target);
                self.operations.execute_devorar(target).await?;
            }
        }

        println!("💀 EMERGÊNCIA CONCLUÍDA!");
        println!("");

        Ok(())
    }
}
