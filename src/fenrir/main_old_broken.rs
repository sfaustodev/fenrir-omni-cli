// --- ARQUIVOS DE MÓDULO ---
// FENRIR MCP 3.0 - MULTI-AI ORCHESTRATION SYSTEM
mod executor;
mod oraculo;
mod ferramentas;
mod terminal;
mod starship;
mod fenrir_ai_layer;
mod fenrir_orchestrator;
mod kali_tools;
mod kali_tools_comprehensive;

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
use kali_tools::{bite, scan, BiteConfig, BiteIntensity, ScanConfig, ScanType, ScanDepth, ScanOutput, get_available_tools};
use kali_tools_comprehensive::{FenrirOrchestrationEngine, BreachDetector, DecisionLogger, KaliTool};

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
                if trimado.starts_with("bite ") || trimado.starts_with("morder ") {
                    let parts: Vec<&str> = trimado.split_whitespace().collect();
                    if parts.len() < 2 {
                        println!("\n🐺 BITE (MORDER) - Advanced Penetration Testing\n");
                        println!("Usage: bite <target> [options]");
                        println!("  or:  morder <alvo> [opções]\n");
                        println!("Examples:");
                        println!("  bite 192.168.1.100            # Basic pentest");
                        println!("  bite example.com --aggressive  # Aggressive mode");
                        println!("  bite 10.0.0.1 --godmode        # Maximum intensity\n");
                        println!("⚠️  For authorized bug bounty and security testing only");
                    } else {
                        let target = parts[1];
                        let intensity = if parts.iter().any(|&p| p == "--godmode") {
                            BiteIntensity::GodMode
                        } else if parts.iter().any(|&p| p == "--aggressive") {
                            BiteIntensity::Aggressive
                        } else if parts.iter().any(|&p| p == "--cautious") {
                            BiteIntensity::Cautious
                        } else {
                            BiteIntensity::Passive
                        };

                        let config = BiteConfig {
                            target: target.to_string(),
                            tools: vec![],
                            intensity,
                            categories: vec![],
                            auto_exploit: parts.iter().any(|&p| p == "--exploit"),
                            report_path: Some(format!("fenrir_bite_{}.md", target.replace(".", "_"))),
                        };

                        println!("\n🐺 Executing BITE (MORDER) penetration test...");
                        match bite(target, config).await {
                            Ok(result) => {
                                println!("\n{}", result.report);
                                last_command_status = 0;
                            }
                            Err(e) => {
                                eprintln!("\n❌ Bite failed: {}", e);
                                last_command_status = 1;
                            }
                        }
                    }
                    continue;
                }
                if trimado.starts_with("scan ") {
                    let parts: Vec<&str> = trimado.split_whitespace().collect();
                    if parts.len() < 2 {
                        println!("\n🔍 FENRIR SCAN - Security Assessment Planning\n");
                        println!("Usage: scan <target> [options]\n");
                        println!("Examples:");
                        println!("  scan 192.168.1.100              # Quick scan");
                        println!("  scan example.com --comprehensive # Full assessment");
                        println!("  scan 10.0.0.1 --stealth          # Stealth mode\n");
                        println!("📊 Creates security plan without exploiting");
                    } else {
                        let target = parts[1];
                        let scan_type = if parts.iter().any(|&p| p == "--stealth") {
                            ScanType::Stealth
                        } else if parts.iter().any(|&p| p == "--comprehensive") {
                            ScanType::Comprehensive
                        } else {
                            ScanType::Quick
                        };

                        let depth = if parts.iter().any(|&p| p == "--deep") {
                            ScanDepth::Deep
                        } else if parts.iter().any(|&p| p == "--exhaustive") {
                            ScanDepth::Exhaustive
                        } else {
                            ScanDepth::Surface
                        };

                        let config = ScanConfig {
                            target: target.to_string(),
                            scan_type,
                            depth,
                            output_format: ScanOutput::Terminal,
                        };

                        println!("\n🔍 Executing SCAN security assessment...");
                        match scan(target, config).await {
                            Ok(result) => {
                                println!("\n📊 SCAN RESULTS:\n");
                                println!("Target: {}", result.target);
                                println!("Risk Score: {}/100", result.risk_score);
                                println!("\nOpen Ports:");
                                for port in &result.open_ports {
                                    println!("  • {}/{} ({}) - {}",
                                        port.port,
                                        port.protocol,
                                        port.state,
                                        port.service.as_ref().unwrap_or(&"unknown".to_string())
                                    );
                                }
                                println!("\nRecommendations:");
                                for rec in &result.recommendations {
                                    println!("  {}", rec);
                                }
                                println!("\n{}\n", result.security_plan);
                                last_command_status = 0;
                            }
                            Err(e) => {
                                eprintln!("\n❌ Scan failed: {}", e);
                                last_command_status = 1;
                            }
                        }
                    }
                    continue;
                }
                if trimado == "tools" || trimado == "kali" {
                    println!("\n🔧 FENRIR KALI TOOLS INTEGRATION\n");
                    let available = get_available_tools();
                    println!("Available Tools: {}/{}", available.len(), kali_tools::get_kali_tools().len());
                    println!("\nAvailable Tools:");
                    for tool in &available {
                        println!("  • {} ({:?})", tool.name, tool.category);
                    }
                    println!("\nUse 'bite' or 'scan' to utilize these tools");
                    last_command_status = 0;
                    continue;
                }
                if trimado == "orchestrate" || trimado.starts_with("orchestrate ") {
                    let target = if trimado.starts_with("orchestrate ") {
                        let parts: Vec<&str> = trimado.split_whitespace().collect();
                        if parts.len() >= 2 {
                            parts[1].to_string()
                        } else {
                            "127.0.0.1".to_string()
                        }
                    } else {
                        "127.0.0.1".to_string()
                    };

                    println!("\n🐺 FENRIR ORCHESTRATION ENGINE - Sequential Attack Mode");
                    println!("🎯 Target: {}", target);

                    let mut engine = FenrirOrchestrationEngine::new(target.clone());
                    match engine.run_sequential_attack().await {
                        Ok(_) => println!("\n✅ Orchestration complete"),
                        Err(e) => eprintln!("\n❌ Orchestration failed: {}", e),
                    }

                    let report = engine.generate_ethical_report().await;

                    let report_file = format!("fenrir_ethical_report_{}.md",
                        target.replace(".", "_").replace("/", "_"));
                    if let Ok(_) = std::fs::write(&report_file, report) {
                        println!("📄 Ethical Analysis Final Report: {}", report_file);
                    }

                    last_command_status = 0;
                    continue;
                }
                if trimado == "wifi" || trimado.starts_with("wifi ") {
                    println!("\n📶 FENRIR WIFI GATEWAY PASSWORD RECOVERY\n");

                    // Get current gateway
                    println!("🔍 Detecting WiFi gateway...");

                    // Try macOS first
                    #[cfg(target_os = "macos")]
                    {
                        use std::process::Command;
                        let gateway_output = Command::new("route")
                            .args(&["-n", "get", "default"])
                            .output();

                        if let Ok(output) = gateway_output {
                            let gateway_info = String::from_utf8_lossy(&output.stdout);
                            println!("{}", gateway_info);

                            // Extract gateway IP
                            if let Some(gateway_line) = gateway_info.lines().find(|l| l.contains("gateway")) {
                                let gateway_ip = gateway_line.split(":").last().unwrap_or("").trim();
                                println!("\n🎯 Gateway IP: {}", gateway_ip);

                                // Try to get WiFi password
                                println!("\n🔐 Attempting to retrieve WiFi credentials...");

                                let wifi_output = Command::new("security")
                                    .args(&["find-generic-password", "-wa", "WiFi"])
                                    .output();

                                if let Ok(wifi_result) = wifi_output {
                                    let wifi_pass = String::from_utf8_lossy(&wifi_result.stdout).trim().to_string();
                                    if !wifi_pass.is_empty() {
                                        println!("✅ WiFi Password found: {}", wifi_pass);

                                        // Save to breach detector for reporting
                                        let mut detector = BreachDetector::new();
                                        detector.sensitive_data.push(
                                            kali_tools_comprehensive::SensitiveData {
                                                data_id: uuid::Uuid::new_v4().to_string(),
                                                data_type: kali_tools_comprehensive::SensitiveDataType::Password,
                                                content: format!("Gateway WiFi Password: {}", wifi_pass),
                                                file_path: None,
                                                url: None,
                                                confidence: 1.0,
                                                timestamp: chrono::Utc::now(),
                                            }
                                        );

                                        // Log the discovery
                                        println!("\n📊 CREDENTIAL RECOVERY SUMMARY:");
                                        println!("  Gateway IP: {}", gateway_ip);
                                        println!("  WiFi Password: {}", wifi_pass);
                                        println!("  Recovery Method: macOS Keychain");
                                    } else {
                                        println!("⚠️  WiFi password not found in keychain");
                                    }
                                }
                            }
                        }
                    }

                    // Try Linux
                    #[cfg(target_os = "linux")]
                    {
                        use std::process::Command;
                        let gateway_output = Command::new("ip")
                            .args(&["route", "show", "default"])
                            .output();

                        if let Ok(output) = gateway_output {
                            let gateway_info = String::from_utf8_lossy(&output.stdout);
                            println!("{}", gateway_info);

                            // Extract gateway IP
                            if let Some(gateway_line) = gateway_info.lines().next() {
                                let parts: Vec<&str> = gateway_line.split_whitespace().collect();
                                if parts.len() >= 3 {
                                    let gateway_ip = parts[2];
                                    println!("\n🎯 Gateway IP: {}", gateway_ip);
                                }
                            }
                        }

                        // Try to get WiFi password from NetworkManager
                        println!("\n🔐 Checking NetworkManager connections...");
                        let nmcli_output = Command::new("nmcli")
                            .args(&["-t", "-f", "NAME,TYPE", "connection", "show"])
                            .output();

                        if let Ok(nm_result) = nmcli_output {
                            let connections = String::from_utf8_lossy(&nmcli_result.stdout);
                            println!("Available WiFi connections:");
                            for line in connections.lines() {
                                if line.contains(":802-11") {
                                    let ssid = line.split(':').next().unwrap_or("");
                                    println!("  • {}", ssid);
                                }
                            }
                        }
                    }

                    println!("\n⚠️  NOTE: These credentials are from YOUR current network.");
                    println!("   Only use this on networks you own or are authorized to test.\n");

                    last_command_status = 0;
                    continue;
                }

                // Limpar área de entrada antes de processar
                let _ = fenrir_terminal.clear_input_area();

                // Se não for comando especial, é pro Oráculo!
                processar_solicitacao(&trimado, pb).await;

                // Simular status do comando (no mundo real, viria do comando executado)
                last_command_status = 0; // Sucesso
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
                if trimado.starts_with("bite ") || trimado.starts_with("morder ") {
                    let parts: Vec<&str> = trimado.split_whitespace().collect();
                    if parts.len() < 2 {
                        println!("\n🐺 BITE (MORDER) - Advanced Penetration Testing\n");
                        println!("Usage: bite <target> [options]");
                        println!("  or:  morder <alvo> [opções]\n");
                        println!("Examples:");
                        println!("  bite 192.168.1.100            # Basic pentest");
                        println!("  bite example.com --aggressive  # Aggressive mode");
                        println!("  bite 10.0.0.1 --godmode        # Maximum intensity\n");
                        println!("⚠️  For authorized bug bounty and security testing only");
                    } else {
                        let target = parts[1];
                        let intensity = if parts.iter().any(|&p| p == "--godmode") {
                            BiteIntensity::GodMode
                        } else if parts.iter().any(|&p| p == "--aggressive") {
                            BiteIntensity::Aggressive
                        } else if parts.iter().any(|&p| p == "--cautious") {
                            BiteIntensity::Cautious
                        } else {
                            BiteIntensity::Passive
                        };

                        let config = BiteConfig {
                            target: target.to_string(),
                            tools: vec![],
                            intensity,
                            categories: vec![],
                            auto_exploit: parts.iter().any(|&p| p == "--exploit"),
                            report_path: Some(format!("fenrir_bite_{}.md", target.replace(".", "_"))),
                        };

                        println!("\n🐺 Executing BITE (MORDER) penetration test...");
                        match bite(target, config).await {
                            Ok(result) => {
                                println!("\n{}", result.report);
                                last_command_status = 0;
                            }
                            Err(e) => {
                                eprintln!("\n❌ Bite failed: {}", e);
                                last_command_status = 1;
                            }
                        }
                    }
                    continue;
                }
                if trimado.starts_with("scan ") {
                    let parts: Vec<&str> = trimado.split_whitespace().collect();
                    if parts.len() < 2 {
                        println!("\n🔍 FENRIR SCAN - Security Assessment Planning\n");
                        println!("Usage: scan <target> [options]\n");
                        println!("Examples:");
                        println!("  scan 192.168.1.100              # Quick scan");
                        println!("  scan example.com --comprehensive # Full assessment");
                        println!("  scan 10.0.0.1 --stealth          # Stealth mode\n");
                        println!("📊 Creates security plan without exploiting");
                    } else {
                        let target = parts[1];
                        let scan_type = if parts.iter().any(|&p| p == "--stealth") {
                            ScanType::Stealth
                        } else if parts.iter().any(|&p| p == "--comprehensive") {
                            ScanType::Comprehensive
                        } else {
                            ScanType::Quick
                        };

                        let depth = if parts.iter().any(|&p| p == "--deep") {
                            ScanDepth::Deep
                        } else if parts.iter().any(|&p| p == "--exhaustive") {
                            ScanDepth::Exhaustive
                        } else {
                            ScanDepth::Surface
                        };

                        let config = ScanConfig {
                            target: target.to_string(),
                            scan_type,
                            depth,
                            output_format: ScanOutput::Terminal,
                        };

                        println!("\n🔍 Executing SCAN security assessment...");
                        match scan(target, config).await {
                            Ok(result) => {
                                println!("\n📊 SCAN RESULTS:\n");
                                println!("Target: {}", result.target);
                                println!("Risk Score: {}/100", result.risk_score);
                                println!("\nOpen Ports:");
                                for port in &result.open_ports {
                                    println!("  • {}/{} ({}) - {}",
                                        port.port,
                                        port.protocol,
                                        port.state,
                                        port.service.as_ref().unwrap_or(&"unknown".to_string())
                                    );
                                }
                                println!("\nRecommendations:");
                                for rec in &result.recommendations {
                                    println!("  {}", rec);
                                }
                                println!("\n{}\n", result.security_plan);
                                last_command_status = 0;
                            }
                            Err(e) => {
                                eprintln!("\n❌ Scan failed: {}", e);
                                last_command_status = 1;
                            }
                        }
                    }
                    continue;
                }
                if trimado == "tools" || trimado == "kali" {
                    println!("\n🔧 FENRIR KALI TOOLS INTEGRATION\n");
                    let available = get_available_tools();
                    println!("Available Tools: {}/{}", available.len(), kali_tools::get_kali_tools().len());
                    println!("\nAvailable Tools:");
                    for tool in &available {
                        println!("  • {} ({:?})", tool.name, tool.category);
                    }
                    println!("\nUse 'bite' or 'scan' to utilize these tools");
                    last_command_status = 0;
                    continue;
                }
                if trimado == "wifi" {
                    println!("\n📶 FENRIR WIFI GATEWAY PASSWORD RECOVERY\n");

                    // Get current gateway
                    println!("🔍 Detecting WiFi gateway...");

                    // Try macOS first
                    #[cfg(target_os = "macos")]
                    {
                        use std::process::Command;
                        let gateway_output = Command::new("route")
                            .args(&["-n", "get", "default"])
                            .output();

                        if let Ok(output) = gateway_output {
                            let gateway_info = String::from_utf8_lossy(&output.stdout);
                            println!("{}", gateway_info);

                            // Extract gateway IP
                            if let Some(gateway_line) = gateway_info.lines().find(|l| l.contains("gateway")) {
                                let gateway_ip = gateway_line.split(":").last().unwrap_or("").trim();
                                println!("\n🎯 Gateway IP: {}", gateway_ip);

                                // Try to get WiFi password
                                println!("\n🔐 Attempting to retrieve WiFi credentials...");

                                let wifi_output = Command::new("security")
                                    .args(&["find-generic-password", "-wa", "WiFi"])
                                    .output();

                                if let Ok(wifi_result) = wifi_output {
                                    let wifi_pass = String::from_utf8_lossy(&wifi_result.stdout).trim().to_string();
                                    if !wifi_pass.is_empty() {
                                        println!("✅ WiFi Password found: {}", wifi_pass);

                                        // Log the discovery
                                        println!("\n📊 CREDENTIAL RECOVERY SUMMARY:");
                                        println!("  Gateway IP: {}", gateway_ip);
                                        println!("  WiFi Password: {}", wifi_pass);
                                        println!("  Recovery Method: macOS Keychain");
                                    } else {
                                        println!("⚠️  WiFi password not found in keychain");
                                    }
                                }
                            }
                        }
                    }

                    // Try Linux
                    #[cfg(target_os = "linux")]
                    {
                        use std::process::Command;
                        let gateway_output = Command::new("ip")
                            .args(&["route", "show", "default"])
                            .output();

                        if let Ok(output) = gateway_output {
                            let gateway_info = String::from_utf8_lossy(&output.stdout);
                            println!("{}", gateway_info);
                        }
                    }

                    println!("\n⚠️  NOTE: These credentials are from YOUR current network.");
                    println!("   Only use this on networks you own or are authorized to test.\n");

                    last_command_status = 0;
                    continue;
                }
                if trimado == "orchestrate" || trimado.starts_with("orchestrate ") {
                    let target = if trimado.starts_with("orchestrate ") {
                        let parts: Vec<&str> = trimado.split_whitespace().collect();
                        if parts.len() >= 2 {
                            parts[1].to_string()
                        } else {
                            "127.0.0.1".to_string()
                        }
                    } else {
                        "127.0.0.1".to_string()
                    };

                    println!("\n🐺 FENRIR ORCHESTRATION ENGINE - Sequential Attack Mode");
                    println!("🎯 Target: {}", target);

                    let mut engine = FenrirOrchestrationEngine::new(target.clone());
                    match engine.run_sequential_attack().await {
                        Ok(_) => println!("\n✅ Orchestration complete"),
                        Err(e) => eprintln!("\n❌ Orchestration failed: {}", e),
                    }

                    let report = engine.generate_ethical_report().await;

                    let report_file = format!("fenrir_ethical_report_{}.md",
                        target.replace(".", "_").replace("/", "_"));
                    if let Ok(_) = std::fs::write(&report_file, report) {
                        println!("📄 Ethical Analysis Final Report: {}", report_file);
                    }

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