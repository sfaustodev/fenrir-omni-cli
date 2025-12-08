//! # Interface interativa estilo Huh?
//!
//! Integra uma experiência de formulário inspirada no projeto
//! https://github.com/sfaustodev/huh para coletar parâmetros e
//! disparar o fluxo de scan do Fenrir.

use color_eyre::Result;
use inquire::{Confirm, CustomType, Select, Text};

use crate::config::Config;
use crate::ui;

/// Executa o assistente interativo que coleta dados e chama o scan
pub fn execute(config: &Config, verbose_default: bool) -> Result<()> {
    ui::print_header();
    ui::section("Huh? front-end");
    ui::info("Responda ao formulário abaixo para montar o comando de scan.");

    let target = Text::new("Alvo (IP/hostname/URL)")
        .with_help_message("Ex: 10.10.10.10 ou https://alvo.com")
        .prompt()?;

    let port_range = Text::new("Portas para escanear")
        .with_default("1-1000")
        .with_help_message("Intervalos (1-1024) ou listas (80,443,8080)")
        .prompt()?;

    let scan_type = Select::new(
        "Tipo de scan",
        vec![
            "quick".to_string(),
            "full".to_string(),
            "stealth".to_string(),
            "aggressive".to_string(),
        ],
    )
    .with_help_message("Escolha o perfil de agressividade do scan")
    .prompt()?;

    let timeout = CustomType::<u32>::new("Timeout por conexão (s)")
        .with_default(5)
        .with_error_message("Digite um número inteiro válido")
        .prompt()?;

    let threads = CustomType::<u32>::new("Threads paralelas")
        .with_default(100)
        .with_error_message("Digite um número inteiro válido")
        .prompt()?;

    let verbose = Confirm::new("Ativar modo verboso?")
        .with_default(verbose_default)
        .with_help_message("Mostra logs extras durante a execução")
        .prompt()?;

    println!();
    ui::section("Resumo do formulário Huh?");
    ui::list_item("Alvo", &target);
    ui::list_item("Portas", &port_range);
    ui::list_item("Tipo", &scan_type);
    ui::list_item("Timeout", &format!("{}s", timeout));
    ui::list_item("Threads", &threads.to_string());
    ui::list_bool("Verboso", verbose);

    println!();
    ui::info("Disparando scan com os parâmetros informados...");

    // Reaproveita o fluxo existente de scan
    super::scan::execute(
        &target,
        &port_range,
        &scan_type,
        timeout,
        threads,
        config,
        verbose,
    )
}
