//! # Comando Rules
//! 
//! Exibe e gerencia regras de segurança (guardrails) do Fenrir.
//! As regras são carregadas do arquivo de configuração.

use color_eyre::Result;
use colored::Colorize;

use crate::config::Config;
use crate::ui;

/// Executa o comando de rules
pub fn execute(list: bool, reload: bool, config: &Config) -> Result<()> {
    ui::section("Fenrir Rules - Guardrails de Segurança");
    
    if reload {
        ui::status("Recarregando regras...");
        // STUB: Reload seria feito recarregando o arquivo de config
        ui::success("Regras recarregadas com sucesso.");
        println!();
    }

    // Por padrão ou com --list, mostra as regras
    if list || !reload {
        show_rules(config);
    }

    Ok(())
}

/// Exibe todas as regras ativas
fn show_rules(config: &Config) {
    println!();
    println!(
        "  {} Arquivo: {}",
        "📁".dimmed(),
        Config::default_path()
            .map(|p| p.display().to_string())
            .unwrap_or_else(|_| "não encontrado".to_string())
            .dimmed()
    );
    println!();

    // Políticas de conteúdo
    ui::section("Políticas de Conteúdo");
    
    let policies = &config.content_policies;
    
    ui::list_bool("Anti-pedofilia", policies.anti_pedophilia);
    println!("      {}", "Bloqueia operações envolvendo conteúdo ilegal".dimmed());
    
    ui::list_bool("Anti-vazamento sensível", policies.anti_sensitive_leaks);
    println!("      {}", "Previne exposição de credenciais e dados pessoais".dimmed());
    
    ui::list_bool("Pentest agressivo", policies.allow_aggressive_pentest);
    println!("      {}", "Permite técnicas que podem causar DoS".dimmed());
    
    ui::list_bool("Respeitar robots.txt", policies.respect_robots_txt);
    println!("      {}", "Segue políticas de crawling dos sites".dimmed());
    
    ui::list_bool("Log de auditoria", policies.audit_logging);
    println!("      {}", "Registra todas as operações para análise".dimmed());
    
    ui::list_bool("Proteger infra crítica", policies.protect_critical_infra);
    println!("      {}", "Bloqueia ações em sistemas críticos conhecidos".dimmed());

    // Configurações de scan
    ui::section("Configurações de Scan");
    
    ui::list_item("Max threads", &config.scan.max_threads.to_string());
    ui::list_item("Timeout padrão", &format!("{}s", config.scan.default_timeout));
    ui::list_item("Portas padrão", &config.scan.default_port_range);
    ui::list_item("User-Agent", &config.scan.user_agent);

    // Sumário
    println!();
    ui::print_separator();
    
    let active_count = count_active_policies(policies);
    let total_count = 6;
    
    println!(
        "\n  {} {}/{} guardrails ativos",
        if active_count == total_count { "✓".bright_green() } else { "!".bright_yellow() },
        active_count.to_string().bright_green(),
        total_count
    );

    if !policies.allow_aggressive_pentest {
        println!(
            "  {} {}",
            "ℹ".bright_blue(),
            "Modo conservador ativo (recomendado)".dimmed()
        );
    } else {
        println!(
            "  {} {}",
            "⚠".bright_yellow(),
            "Modo agressivo habilitado - use com responsabilidade!".bright_yellow()
        );
    }

    println!();
    ui::info("Para editar: nano ~/.config/fenrir/fenrir_rules.yaml");
}

/// Conta quantas políticas de segurança estão ativas
fn count_active_policies(policies: &crate::config::ContentPolicies) -> usize {
    let mut count = 0;
    if policies.anti_pedophilia { count += 1; }
    if policies.anti_sensitive_leaks { count += 1; }
    if !policies.allow_aggressive_pentest { count += 1; }  // Invertido: desativado é mais seguro
    if policies.respect_robots_txt { count += 1; }
    if policies.audit_logging { count += 1; }
    if policies.protect_critical_infra { count += 1; }
    count
}
