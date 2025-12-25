//! # Módulo CLI
//!
//! Define a estrutura de comandos e subcomandos usando `clap`.
//! Responsável por parsear argumentos e rotear para os handlers apropriados.

use clap::{Parser, Subcommand};
use clap_complete::Shell;
use color_eyre::Result;
use std::path::PathBuf;

use crate::commands;
use crate::config::Config;
use crate::ui;

/// Fenrir CLI - Devorador de CLIs, orquestrador de ferramentas
#[derive(Parser, Debug)]
#[command(
    name = "fenrir",
    author = "Fenrir Team",
    version,
    about = "🐺 Fenrir CLI – devorador de CLIs, orquestrador de ferramentas.",
    long_about = "Fenrir é uma ferramenta de orquestração para pentest e segurança.\nNa mitologia nórdica, Fenrir é o lobo que devora os deuses.\nAqui, ele devora suas ferramentas de CLI e as orquestra com maestria."
)]
pub struct Cli {
    /// Ativa modo verboso (mais detalhes na saída)
    #[arg(short, long, global = true)]
    pub verbose: bool,

    /// Desativa cores na saída
    #[arg(long, global = true)]
    pub no_color: bool,

    /// Caminho customizado para arquivo de configuração
    #[arg(short, long, global = true)]
    pub config: Option<String>,

    /// Gera script de completion para o shell especificado
    #[arg(long = "completion", value_enum, global = true, exclusive = true)]
    pub completion: Option<Shell>,

    /// Caminho de saída para o script de completion
    #[arg(long, global = true)]
    pub completion_output: Option<PathBuf>,

    #[command(subcommand)]
    pub command: Option<Commands>,
}

/// Subcomandos disponíveis no Fenrir
#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Executa varredura de segurança em um alvo
    Scan {
        /// Alvo da varredura (IP, hostname ou URL)
        #[arg(short = 't', long, value_parser = crate::validators::target_validator())]
        target: String,

        /// Range de portas para escanear (ex: 1-1000, 80,443,8080)
        #[arg(short = 'p', long, default_value = "1-1000", value_parser = crate::validators::port_range_validator())]
        port_range: String,

        /// Tipo de scan (quick, full, stealth)
        #[arg(long, default_value = "quick", value_parser = crate::validators::scan_type_validator())]
        scan_type: String,

        /// Timeout em segundos para cada conexão (1-300)
        #[arg(long, default_value = "5")]
        timeout: u32,

        /// Número de threads paralelas (1-1000)
        #[arg(long, default_value = "100")]
        threads: u32,
    },

    /// Exibe e gerencia regras de segurança (guardrails)
    Rules {
        /// Lista todas as regras ativas
        #[arg(short, long)]
        list: bool,

        /// Recarrega as regras do arquivo de configuração
        #[arg(short, long)]
        reload: bool,
    },

    /// Informações sobre o projeto Fenrir
    About,

    /// Inicializa configuração padrão
    Init {
        /// Força recriação mesmo se já existir
        #[arg(short, long)]
        force: bool,
    },

    /// Automação de workflow Git + GitHub PR (status -> add -> commit -> push -> pr)
    Gitar {
        /// Mensagem de commit (opcional, se não informado usa default)
        #[arg(short, long)]
        message: Option<String>,
    },

    /// Interface interativa estilo Huh? para montar comandos
    Huh,

    /// Natural language mode - describe what you want in plain English/Portuguese
    Ai {
        /// Natural language request (if not provided, enters interactive mode)
        #[arg(short = 'p', long)]
        prompt: Option<String>,
    },
}

/// Função principal que executa o CLI
pub fn run() -> Result<()> {
    let cli = Cli::parse();

    // Handle completion generation
    if let Some(shell) = cli.completion {
        return crate::completions::generate_completion(shell, cli.completion_output);
    }

    // Configura estado global de cores
    if cli.no_color {
        colored::control::set_override(false);
    }

    // Carrega ou cria configuração
    let config = Config::load_or_create(cli.config.as_deref())?;

    // Se nenhum subcomando foi passado, mostra header + ajuda
    match cli.command {
        None => {
            ui::print_header();
            ui::print_available_commands();
            Ok(())
        }
        Some(cmd) => {
            // AI command needs async runtime
            if matches!(cmd, Commands::Ai { .. }) {
                tokio::runtime::Runtime::new()?.block_on(execute_command_async(cmd, &config, cli.verbose))
            } else {
                execute_command(cmd, &config, cli.verbose)
            }
        }
    }
}

/// Executa o subcomando selecionado
fn execute_command(cmd: Commands, config: &Config, verbose: bool) -> Result<()> {
    match cmd {
        Commands::Scan {
            target,
            port_range,
            scan_type,
            timeout,
            threads,
        } => commands::scan::execute(
            &target,
            &port_range,
            &scan_type,
            timeout,
            threads,
            config,
            verbose,
        ),
        Commands::Rules { list, reload } => commands::rules::execute(list, reload, config),
        Commands::About => commands::about::execute(),
        Commands::Init { force } => commands::init::execute(force),
        Commands::Gitar { message } => commands::gitar::execute(message, verbose),
        Commands::Huh => commands::huh_ui::execute(config, verbose),
        Commands::Ai { .. } => unreachable!("AI command should be handled by execute_command_async"),
    }
}

/// Executa comandos assíncronos (AI)
async fn execute_command_async(cmd: Commands, _config: &Config, _verbose: bool) -> Result<()> {
    match cmd {
        Commands::Ai { prompt } => {
            if let Some(prompt_text) = prompt {
                let result = crate::ai::process_natural_input(&prompt_text).await?;
                crate::ai::display_natural_result(&result);
                Ok(())
            } else {
                crate::ai::interactive_mode().await
            }
        }
        _ => unreachable!("Non-async commands should not use execute_command_async"),
    }
}
