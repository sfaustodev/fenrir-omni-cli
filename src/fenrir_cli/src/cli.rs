//! # Módulo CLI
//!
//! Define a estrutura de comandos e subcomandos usando `clap`.
//! Responsável por parsear argumentos e rotear para os handlers apropriados.

use clap::{Parser, Subcommand};
use color_eyre::Result;

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

    #[command(subcommand)]
    pub command: Option<Commands>,
}

/// Subcomandos disponíveis no Fenrir
#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Executa varredura de segurança em um alvo
    Scan {
        /// Alvo da varredura (IP, hostname ou URL)
        #[arg(short, long)]
        target: String,

        /// Range de portas para escanear (ex: 1-1000, 80,443,8080)
        #[arg(short, long, default_value = "1-1000")]
        port_range: String,

        /// Tipo de scan (quick, full, stealth)
        #[arg(long, default_value = "quick")]
        scan_type: String,

        /// Timeout em segundos para cada conexão
        #[arg(long, default_value = "5")]
        timeout: u32,

        /// Número de threads paralelas
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
}

/// Função principal que executa o CLI
pub fn run() -> Result<()> {
    let cli = Cli::parse();

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
        Some(cmd) => execute_command(cmd, &config, cli.verbose),
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
    }
}
