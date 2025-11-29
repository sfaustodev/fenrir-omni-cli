//! # Módulo UI
//!
//! Responsável por toda a apresentação visual do Fenrir CLI.
//! Inclui banner ASCII, cores, formatação de output e helpers de UX.

mod banner;
mod colors;

pub use banner::*;
pub use colors::*;

use crate::VERSION;
use colored::Colorize;

/// Imprime o header completo do Fenrir (banner + versão + modo)
pub fn print_header() {
    print_banner();
    println!();
    println!(
        "  {} v{} | {}",
        "Fenrir CLI".bright_cyan().bold(),
        VERSION.yellow(),
        "Modo: CLI interativo em Rust".dimmed()
    );
    println!(
        "  {}",
        "Devorador de CLIs, orquestrador de ferramentas."
            .italic()
            .dimmed()
    );
    println!();
    print_separator();
}

/// Imprime lista de comandos disponíveis
pub fn print_available_commands() {
    println!();
    println!(
        "  {} {}",
        "Uso:".bright_white().bold(),
        "fenrir <COMMAND> [OPTIONS]".cyan()
    );
    println!();
    println!("  {}", "Comandos disponíveis:".bright_white().bold());
    println!();

    let commands = [
        ("scan", "Executa varredura de segurança em um alvo"),
        ("rules", "Exibe e gerencia regras de segurança (guardrails)"),
        ("about", "Informações sobre o projeto Fenrir"),
        ("init", "Inicializa configuração padrão"),
        ("huh", "Interface interativa estilo Huh? para montar scans"),
        ("help", "Mostra esta ajuda ou ajuda de um subcomando"),
    ];

    for (cmd, desc) in commands {
        println!(
            "    {}  {}",
            format!("{:<10}", cmd).bright_green().bold(),
            desc.dimmed()
        );
    }

    println!();
    println!(
        "  {} {}",
        "Dica:".yellow().bold(),
        "Use 'fenrir <command> --help' para mais detalhes.".dimmed()
    );
    println!();
}

/// Imprime uma linha separadora estilizada
pub fn print_separator() {
    println!("  {}", "─".repeat(60).dimmed());
}

/// Imprime mensagem de status com prefixo colorido
pub fn status(msg: &str) {
    println!("  {} {}", "[*]".bright_blue().bold(), msg);
}

/// Imprime mensagem de sucesso
pub fn success(msg: &str) {
    println!("  {} {}", "[+]".bright_green().bold(), msg);
}

/// Imprime mensagem de aviso
pub fn warning(msg: &str) {
    println!("  {} {}", "[!]".bright_yellow().bold(), msg);
}

/// Imprime mensagem de erro
pub fn error(msg: &str) {
    println!("  {} {}", "[-]".bright_red().bold(), msg);
}

/// Imprime mensagem informativa
pub fn info(msg: &str) {
    println!("  {} {}", "[i]".bright_cyan().bold(), msg);
}

/// Imprime um título de seção
pub fn section(title: &str) {
    println!();
    println!("  {} {}", "▸".bright_magenta(), title.bright_white().bold());
    println!("  {}", "─".repeat(40).dimmed());
}

/// Imprime item de lista com indentação
pub fn list_item(key: &str, value: &str) {
    println!("    {} {}", format!("{}:", key).bright_cyan(), value);
}

/// Imprime item de lista booleano com indicador visual
pub fn list_bool(key: &str, value: bool) {
    let indicator = if value {
        "●".bright_green()
    } else {
        "○".bright_red()
    };
    let status = if value { "ativo" } else { "inativo" };
    println!(
        "    {} {} {}",
        indicator,
        format!("{}:", key).bright_cyan(),
        status.dimmed()
    );
}
