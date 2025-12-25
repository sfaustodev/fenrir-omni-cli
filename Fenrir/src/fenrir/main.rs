// --- ARQUIVOS DE MÓDULO ---
// FENRIR MCP 2.0 - MULTI-AI ORCHESTRATION SYSTEM
mod executor;
mod oraculo;
mod ferramentas;
mod terminal;
mod starship;
mod fenrir_ai_layer;
mod fenrir_orchestrator;

// --- IMPORTS (use) ---
// Agora a gente chama as funções dos *nossos* módulos.



use indicatif::{ProgressBar, ProgressStyle};
use std::env;
use std::io::{self, Write};
use std::time::Duration;
use terminal::{bootstrap_terminal_interface, detect_terminal_capabilities};
use executor::{ask_for_confirmation, handle_execute_command, handle_open_editor, log_task, FenrirTask};
use starship::{initialize_fenrir_starship, FenrirStarship};
use fenrir_orchestrator::FenrirOrchestrator;

#[tokio::main]
async fn main() {
    // Load environment variables from .env file
    fenrir_ai_layer::load_env();

    let args: Vec<String> = env::args().collect();
    let pb = ProgressBar::new_spinner(); // Spinner pra gente ver rodando

    // Detectar capabilities do terminal
    let (has_ghostty, _has_colors, _has_unicode) = detect_terminal_capabilities();

    // Inicializar Fenrir-Starship
    let mut fenrir_starship = initialize_fenrir_starship();

    if has_ghostty {
        println!("🐺 Ghostty + Starship detectados! Inicializando interface divina...");
    } else {
        println!("⚠️  Ghostty não encontrado. Use 'brew install --cask ghostty' para experiência completa.");
        println!("🌟 Starship Fenrir carregado mesmo assim!");
    }

    if args.len() > 1 {
        // Modo "um comando e vaza"
        let consulta_completa = args[1..].join(" ");
        processar_solicitacao(&consulta_completa, &pb).await;
    } else {
        // Modo interativo com interface Ghostty + Starship
        match bootstrap_terminal_interface() {
            Ok(fenrir_terminal) => {
                println!("🚀 Interface Ghostty + Starship Fenrir inicializada com sucesso!");
                interativo(&pb, &fenrir_terminal, &mut fenrir_starship).await;

                // Restaurar terminal ao sair
                let _ = fenrir_terminal.restore_terminal();
            }
            Err(e) => {
                eprintln!("⚠️  Falha ao inicializar interface Ghostty: {}", e);
                println!("🐺 Modo interativo Fenrir-Starship fallback.");
                println!("🌟 Prompt Starship ativado!");
                interativo_fallback(&pb, &mut fenrir_starship).await;
            }
        }
    }
}

// Modo interativo com interface Ghostty + Starship avançada
async fn interativo(pb: &ProgressBar, fenrir_terminal: &terminal::FenrirTerminal, fenrir_starship: &mut FenrirStarship) {
    let stdin = io::stdin();
    let mut input_buffer = String::new();
    let mut last_command_status = 0;

    loop {
        // Atualizar contexto do Starship
        fenrir_starship.update_context();

        // Renderizar prompt Starship personalizado
        let prompt = fenrir_starship.render_for_terminal(fenrir_terminal.ghostty_available, last_command_status);
        print!("{}", prompt);
        io::stdout().flush().unwrap();

        input_buffer.clear();
        match stdin.read_line(&mut input_buffer) {
            Ok(0) => break, // Fim da entrada (Ctrl+D)
            Ok(_) => {
                let trimado = input_buffer.trim().to_lowercase();
                if trimado.is_empty() {
                    continue;
                }
                if trimado == "sair" || trimado == "exit" || trimado == "quit" {
                    println!("\n🐺 Falou, parceiro! O Lobo está descansando...");
                    break;
                }
                if trimado == "ghostty" {
                    println!("\n🎯 Ghostty Status: {}",
                        if fenrir_terminal.ghostty_available { "ATIVO ✅" } else { "NÃO DISPONÍVEL ❌" });
                    last_command_status = 0;
                    continue;
                }
                if trimado == "status" {
                    println!("\n📊 STATUS DO FENRIR-STARSHIP:");
                    println!("   🐺 Interface: Ghostty {}",
                        if fenrir_terminal.ghostty_available { "✅" } else { "❌" });
                    println!("   🌟 Starship: ATIVO ✅");
                    println!("   🎨 Tema: {}", fenrir_terminal.config.theme);
                    println!("   🔤 Fonte: {} ({:.1}px)",
                        fenrir_terminal.config.font_family,
                        fenrir_terminal.config.font_size);
                    last_command_status = 0;
                    continue;
                }
                if trimado == "starship" {
                    println!("\n🌟 FENRIR-STARSHIP CONFIGURATION:");
                    println!("   🎯 Formato: {}", fenrir_starship.config.format);
                    println!("   📦 Módulos: {:?}", fenrir_starship.config.modules);
                    println!("   🐺 Símbolo Fenrir: {}", fenrir_starship.config.fenrir.symbol);
                    last_command_status = 0;
                    continue;
                }
                if trimado == "godmode" {
                    println!("\n🔴 FENRIR GOD MODE ATIVADO!");
                    println!("💀 Poders divinos concedidos ao Lobo Devorador!");
                    last_command_status = 0;
                    continue;
                }

                // Limpar área de entrada antes de processar
                let _ = fenrir_terminal.clear_input_area();

                // Se não for comando especial, é pro Oráculo!
                processar_solicitacao(&trimado, pb).await;

                // Simular status do comando (no mundo real, viria do comando executado)
                last_command_status = 0; // Sucesso

                // Pausa antes do próximo prompt
                println!("\n⏳ Pressione Enter para continuar...");
                let _ = io::stdin().read_line(&mut String::new());
            }
            Err(e) => {
                eprintln!("❌ Erro lendo entrada: {}", e);
                last_command_status = 1; // Erro
                break;
            }
        }
    }
}

// Modo interativo fallback quando Ghostty falha (mas Starship funciona!)
async fn interativo_fallback(pb: &ProgressBar, fenrir_starship: &mut FenrirStarship) {
    let stdin = io::stdin();
    let mut input_buffer = String::new();
    let mut last_command_status = 0;

    println!("🌟 Iniciando modo Starship-only...");

    loop {
        // Atualizar contexto do Starship
        fenrir_starship.update_context();

        // Renderizar prompt Starship (sem terminal Ghostty)
        let prompt = fenrir_starship.render_prompt(last_command_status);
        print!("{}", prompt);
        io::stdout().flush().unwrap();

        input_buffer.clear();
        match stdin.read_line(&mut input_buffer) {
            Ok(0) => break,
            Ok(_) => {
                let trimado = input_buffer.trim().to_lowercase();
                if trimado.is_empty() {
                    continue;
                }
                if trimado == "sair" || trimado == "exit" {
                    println!("\n🐺 Falou, parceiro! O Lobo está descansando...");
                    break;
                }
                if trimado == "starship" {
                    println!("\n🌟 FENRIR-STARSHIP MODO FALLBACK:");
                    println!("   ✅ Starship: ATIVO (modo standalone)");
                    println!("   ❌ Ghostty: NÃO DISPONÍVEL");
                    println!("   🐺 Modo: Fenrir-Starship puro");
                    last_command_status = 0;
                    continue;
                }
                if trimado == "godmode" {
                    println!("\n🔴 FENRIR-STARSHIP GOD MODE!");
                    println!("💀 Poderes do Starship intensificados!");
                    last_command_status = 0;
                    continue;
                }

                processar_solicitacao(&trimado, pb).await;
                last_command_status = 0; // Sucesso simulado
            }
            Err(e) => {
                eprintln!("❌ Erro lendo entrada: {}", e);
                last_command_status = 1; // Erro
                break;
            }
        }
    }
}

// --- O CÉREBRO DO FENRIR MCP 2.0 ---
// MULTI-AI ORCHESTRATION SYSTEM
async fn processar_solicitacao(consulta: &str, pb: &ProgressBar) {
    pb.set_style(
        ProgressStyle::default_spinner()
            .tick_strings(&["VAI", "CORNO!", "PENSE", "DESGRAÇA!", "...", "VAI", "LOGO", "CARALHO!", "(ノ°Д°）ノ", "┻━┻", "...", "VAI", "CORNO!"])
            .template("{spinner:.bold.yellow} {msg}")
            .unwrap(),
    );
    pb.set_message("🐺 Fenrir MCP 2.0 - Multi-AI Orchestration...");
    pb.enable_steady_tick(Duration::from_millis(150));

    // Initialize orchestrator
    let orchestrator = FenrirOrchestrator::new();

    // Read FENRIR_MCP.md (permanent system prompt)
    println!("📚 Reading FENRIR_MCP.md system prompt...");

    // Process through multi-AI hierarchy
    let resultado = orchestrator.process_input(consulta.to_string()).await;

    // Display result
    pb.finish_with_message("✅ Fenrir MCP Complete!");
    println!("\n{}", resultado);
}