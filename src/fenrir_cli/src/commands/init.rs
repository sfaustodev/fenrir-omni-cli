//! # Comando Init
//! 
//! Inicializa a configuração padrão do Fenrir.
//! Cria o arquivo de configuração com valores seguros.

use color_eyre::Result;
use colored::Colorize;
use std::fs;

use crate::config::{Config, CONFIG_TEMPLATE};
use crate::ui;

/// Executa o comando init
pub fn execute(force: bool) -> Result<()> {
    ui::section("Fenrir Init - Inicialização");
    println!();
    
    let config_path = Config::default_path()?;
    
    ui::status(&format!(
        "Verificando configuração em {}",
        config_path.display().to_string().bright_cyan()
    ));
    
    // Verifica se já existe
    if config_path.exists() && !force {
        ui::warning("Arquivo de configuração já existe.");
        println!();
        ui::info(&format!(
            "Use {} para recriar.",
            "fenrir init --force".bright_yellow()
        ));
        ui::info(&format!(
            "Localização: {}",
            config_path.display().to_string().dimmed()
        ));
        return Ok(());
    }
    
    // Cria diretório se necessário
    if let Some(parent) = config_path.parent() {
        if !parent.exists() {
            ui::status(&format!("Criando diretório {}", parent.display()));
            fs::create_dir_all(parent)?;
            ui::success("Diretório criado.");
        }
    }
    
    // Cria arquivo de configuração
    ui::status("Criando arquivo de configuração...");
    
    // Usa o template com comentários
    let content = format!(
        "# Fenrir CLI - Arquivo de Configuração\n\
         # Criado em: {}\n\
         # Localização: {}\n\
         #\n\
         # Este arquivo controla o comportamento do Fenrir CLI.\n\
         # Edite com cuidado - algumas opções podem afetar a segurança.\n\
         #\n\
         # Documentação: https://github.com/peluche/fenrir/docs/config.md\n\
         \n{}",
        chrono_lite_now(),
        config_path.display(),
        CONFIG_TEMPLATE.trim_start_matches("# Fenrir CLI - Arquivo de Configuração\n")
    );
    
    fs::write(&config_path, content)?;
    
    ui::success("Configuração criada com sucesso!");
    println!();
    
    // Mostra resumo
    ui::section("Configuração Inicial");
    println!();
    
    let defaults = [
        ("Anti-pedofilia", "✓", "ativo"),
        ("Anti-vazamento", "✓", "ativo"),
        ("Pentest agressivo", "✗", "desativado"),
        ("Respeitar robots.txt", "✓", "ativo"),
        ("Log de auditoria", "✓", "ativo"),
        ("Proteger infra crítica", "✓", "ativo"),
    ];
    
    for (name, icon, status) in defaults {
        let colored_icon = if icon == "✓" {
            icon.bright_green()
        } else {
            icon.bright_red()
        };
        println!("    {} {} {}", colored_icon, name.bright_white(), status.dimmed());
    }
    
    println!();
    ui::print_separator();
    
    // Próximos passos
    println!();
    ui::info("Próximos passos:");
    println!();
    println!(
        "    {} Editar configuração:",
        "1.".bright_cyan()
    );
    println!(
        "       {} nano {}",
        "$".dimmed(),
        config_path.display().to_string().bright_yellow()
    );
    println!();
    println!(
        "    {} Verificar regras:",
        "2.".bright_cyan()
    );
    println!(
        "       {} fenrir rules --list",
        "$".dimmed()
    );
    println!();
    println!(
        "    {} Executar primeiro scan:",
        "3.".bright_cyan()
    );
    println!(
        "       {} fenrir scan --target exemplo.com",
        "$".dimmed()
    );
    println!();
    
    ui::success("Fenrir está pronto para uso!");
    
    Ok(())
}

/// Retorna data/hora atual formatada (sem dependência de chrono)
fn chrono_lite_now() -> String {
    use std::time::{SystemTime, UNIX_EPOCH};
    
    let duration = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default();
    
    let secs = duration.as_secs();
    
    // Cálculo simplificado de data (aproximado)
    let days = secs / 86400;
    let years = 1970 + days / 365;
    let remaining_days = days % 365;
    let month = remaining_days / 30 + 1;
    let day = remaining_days % 30 + 1;
    
    let hours = (secs % 86400) / 3600;
    let minutes = (secs % 3600) / 60;
    
    format!(
        "{:04}-{:02}-{:02} {:02}:{:02} UTC",
        years, month, day, hours, minutes
    )
}
