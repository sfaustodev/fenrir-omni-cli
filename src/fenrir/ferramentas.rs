// --- MÓDULO FERRAMENTAS ---
// Utilidades e ferramentas auxiliares para o Fenrir

use std::process::Command;
use std::fs;

pub fn verificar_dependencias() -> Vec<String> {
    let mut faltantes = Vec::new();

    // Verificar ferramentas essenciais
    let ferramentas = vec!["git", "code", "node", "python3", "cargo"];

    for ferramenta in ferramentas {
        if !Command::new("which")
            .arg(ferramenta)
            .output()
            .map(|output| output.status.success())
            .unwrap_or(false)
        {
            faltantes.push(ferramenta.to_string());
        }
    }

    faltantes
}

pub fn criar_estrutura_fenrir() -> std::io::Result<()> {
    let home_dir = dirs::home_dir().unwrap_or_default();
    let fenrir_dir = home_dir.join(".fenrir");

    if !fenrir_dir.exists() {
        fs::create_dir_all(&fenrir_dir)?;
        fs::create_dir_all(fenrir_dir.join("logs"))?;
        fs::create_dir_all(fenrir_dir.join("config"))?;
        fs::create_dir_all(fenrir_dir.join("cache"))?;
    }

    Ok(())
}

pub fn mostrar_info_sistema() {
    println!("🐺 INFO DO SISTEMA FENRIR:");

    // SO
    if cfg!(target_os = "macos") {
        println!("   🍎 macOS");
    } else if cfg!(target_os = "linux") {
        println!("   🐧 Linux");
    } else if cfg!(target_os = "windows") {
        println!("   🪟 Windows");
    }

    // Arquitetura
    if cfg!(target_arch = "x86_64") {
        println!("   🏗️  x86_64");
    } else if cfg!(target_arch = "aarch64") {
        println!("   🏗️  ARM64");
    }

    // Verificar Ghostty
    if Command::new("which").arg("ghostty").output().map(|o| o.status.success()).unwrap_or(false) {
        println!("   🎯 Ghostty: ✅ Instalado");
    } else {
        println!("   🎯 Ghostty: ❌ Não encontrado");
    }

    println!();
}