//! # Comando About
//! 
//! Exibe informações sobre o projeto Fenrir, incluindo versão,
//! autores, links e um pouco do lore mitológico.

use color_eyre::Result;
use colored::Colorize;

use crate::ui;
use crate::VERSION;

/// Lore do Fenrir
const FENRIR_LORE: &str = r#"
Na mitologia nórdica, Fenrir (também conhecido como Fenrisúlfr) é um lobo 
monstruoso, filho de Loki e da gigante Angrboða. Os deuses, temendo seu 
poder crescente, tentaram acorrentá-lo três vezes. Apenas Gleipnir, uma 
corrente mágica feita pelos anões, conseguiu contê-lo - mas ao custo da 
mão do deus Týr.

Durante o Ragnarök, o crepúsculo dos deuses, Fenrir se libertará de suas 
correntes e devorará Odin, o pai de todos. Seu destino é ser morto por 
Víðarr, filho de Odin, que vingará seu pai.

Fenrir representa a força indomável, o caos necessário para a renovação,
e a inevitabilidade do destino. Aqui, ele devora suas ferramentas de CLI
e as orquestra com o poder de um deus."#;

/// Executa o comando about
pub fn execute() -> Result<()> {
    // Banner especial para about
    ui::print_banner();
    
    println!();
    ui::section("Sobre o Projeto");
    
    // Informações básicas
    println!();
    ui::list_item("Nome", "Fenrir CLI");
    ui::list_item("Versão", VERSION);
    ui::list_item("Linguagem", "Rust 🦀");
    ui::list_item("Licença", "MIT");
    
    println!();
    ui::list_item("Repositório", "https://github.com/peluche/fenrir");
    ui::list_item("Documentação", "https://github.com/peluche/fenrir/docs");
    ui::list_item("Issues", "https://github.com/peluche/fenrir/issues");
    
    // Descrição
    ui::section("Descrição");
    println!();
    println!(
        "  {}",
        "Fenrir é uma ferramenta de orquestração para pentest e segurança.".white()
    );
    println!(
        "  {}",
        "Desenvolvida em Rust para máxima performance e segurança de memória.".dimmed()
    );
    
    // Funcionalidades
    ui::section("Funcionalidades");
    println!();
    
    let features = [
        ("🔍", "Varredura de portas e serviços"),
        ("🛡️", "Guardrails de segurança configuráveis"),
        ("🎯", "Orquestração de ferramentas externas"),
        ("📊", "Relatórios detalhados"),
        ("⚡", "Alto desempenho com async/await"),
        ("🔒", "Segurança por design"),
    ];
    
    for (icon, desc) in features {
        println!("    {} {}", icon, desc);
    }
    
    // Lore
    ui::section("Lore: O Lobo que Devora os Deuses");
    
    for line in FENRIR_LORE.lines() {
        if line.trim().is_empty() {
            println!();
        } else {
            println!("  {}", line.italic().dimmed());
        }
    }
    
    // Arte ASCII do lobo menor
    println!();
    print_wolf_art();
    
    // Créditos
    ui::section("Créditos");
    println!();
    println!("    {} Desenvolvido com {} por Fenrir Team", "▸".bright_magenta(), "❤".bright_red());
    println!("    {} Inspirado na mitologia nórdica", "▸".bright_magenta());
    println!("    {} Powered by Rust 🦀", "▸".bright_magenta());
    
    println!();
    ui::print_separator();
    println!();
    println!(
        "  {}",
        "\"Na cadeia do destino, até os deuses são presas.\"".italic().bright_cyan()
    );
    println!("  {}", "- Völuspá (A Profecia da Vidente)".dimmed());
    println!();

    Ok(())
}

/// Arte ASCII menor do lobo
fn print_wolf_art() {
    let wolf = r#"
                        ▄▄▄
                     ▄▓███▓▄
                   ▄██▀   ▀██▄
                  ▓█▀ ▄█▄ ▄ ▀█▓
                 ▐█▌ ████ █▌ █▌
                 ▓█  ▀▀▀  ▀  █▓
                 ▓██▄     ▄██▓
                  ▀███████████▀
                    ▀▀▀▀▀▀▀▀
    "#;
    
    for line in wolf.lines() {
        println!("{}", line.bright_yellow());
    }
}
