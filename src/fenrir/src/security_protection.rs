// 🔒 SECURITY PROTECTION - REGRAS FUNDAMENTAIS
// ROSNAR só em EXTERNO FÍSICO - PROTEÇÃO ABSOLUTA

use std::path::{Path, PathBuf};
//use std::process::Command; // Removido temporariamente
use anyhow::{anyhow, Result};
use std::io::Write;

pub struct SecurityProtection {
    pub debug_mode: bool,
    pub external_drives: Vec<PathBuf>,
    pub operation_log: Vec<String>,
}

impl SecurityProtection {
    pub fn new() -> Self {
        Self {
            debug_mode: false,
            external_drives: vec![],
            operation_log: vec![],
        }
    }

    /// 🔒 VALIDAR SE PODE FAZER ROSNAR
    pub fn can_rosnar(&mut self, target: &str) -> Result<bool> {
        self.log_operation(&format!("VALIDANDO ROSNAR: {}", target));

        // Detectar se é alvo interno
        if self.is_internal_target(target) {
            self.log_operation(&format!("❌ ROSNAR BLOQUEADO - Alvo interno: {}", target));

            if !self.debug_mode {
                println!("🚨 PROTEÇÃO ATIVADA - ROSNAR BLOQUEADO!");
                println!("🔹 Alvo '{}' detectado como INTERNO", target);
                println!("🔹 Regra: ROSNAR só permitido em EXTERNO FÍSICO");
                println!("🔹 Ação: Operação cancelada para proteção");
                println!("⚠️ RECURSO: Se isso for um erro, use --debug-mode para override");
                return Ok(false);
            } else {
                println!("⚠️ DEBUG MODE: ROSNAR em alvo interno PERMITIDO");
            }
        }

        // Detectar se é drive externo físico
        let target_path = Path::new(target);
        if let Some(parent) = target_path.parent() {
            if self.is_external_drive(parent) {
                self.log_operation(&format!("✅ ROSNAR PERMITIDO - Alvo externo: {}", target));
                println!("🔹 ROSNAR autorizado em drive externo físico");
                return Ok(true);
            }
        }

        // Lista de drives externos comuns
        let external_patterns = vec![
            "/Volumes/",   // macOS external drives
            "/media/",     // Linux mounts
            "/mnt/",       // Linux external mounts
            "C:\\Users\\", // Windows C: users (bloqueado)
            "/home/",      // Linux home (bloqueado)
            "D:\\",        // Windows D: drive (potencial externo)
            "E:\\",        // Windows E: drive (externo)
        ];

        let is_externo = external_patterns
            .iter()
            .any(|pattern| target.contains(pattern))
            && !target.contains("Users")
            && !target.contains("home")
            && !target.starts_with("/");

        if is_externo {
            self.log_operation(&format!("✅ ROSNAR PERMITIDO - Padrão externo: {}", target));
            return Ok(true);
        }

        if !self.debug_mode {
            self.log_operation(&format!(
                "❌ ROSNAR BLOQUEADO - Não verificado como externo: {}",
                target
            ));
            println!("🚨 PROTEÇÃO ATIVADA - ROSNAR BLOQUEADO!");
            println!(
                "🔹 Alvo '{}' não verificado como drive externo físico",
                target
            );
            println!("🔹 Para liberar: confirme que é drive externo ou use --debug-mode");
            Ok(false)
        } else {
            self.log_operation(&format!(
                "⚠️ DEBUG MODE: ROSNAR PERMITIDO - Override: {}",
                target
            ));
            Ok(true)
        }
    }

    /// 📁 VERIFICAR SE ALVO É INTERNO
    fn is_internal_target(&self, target: &str) -> bool {
        let target_lower = target.to_lowercase();

        // Paths internos BLOQUEADOS
        let internal_patterns = vec![
            "users",         // Windows/Linux users
            "home",          // Linux home
            "desktop",       // Desktop
            "documents",     // Documents
            "downloads",     // Downloads
            "program files", // Windows Program Files
            "applications",  // macOS Applications
            "library",       // macOS Library
            "system32",      // Windows System32
            "etc",           // Linux /etc
            "var",           // Linux /var
            "usr",           // Linux /usr
            "bin",           // Linux /bin
            "sbin",          // Linux /sbin
            "opt",           // Linux /opt
            "tmp",           // Temp (pode ser perigoso)
            "proc",          // Linux proc
            "sys",           // Linux sys
            "dev",           // Linux dev
        ];

        internal_patterns
            .iter()
            .any(|pattern| target_lower.contains(pattern))
    }

    /// 💿 VERIFICAR SE É DRIVE EXTERNO
    fn is_external_drive(&self, path: &Path) -> bool {
        if let Some(path_str) = path.to_str() {
            path_str.starts_with("/Volumes/") ||  // macOS external
            path_str.starts_with("/media/") ||    // Linux USB
            path_str.starts_with("/mnt/") ||      // Linux mounts
            path_str.chars().nth(1) == Some(':') && path_str.len() > 3 // Windows D:, E:, etc
        } else {
            false
        }
    }

    /// 🔍 ESCANEAR DRIVES EXTERNOS
    pub fn scan_external_drives(&mut self) {
        self.log_operation("🔍 ESCANEANDO DRIVES EXTERNOS");

        #[cfg(unix)]
        {
            // macOS /Volumes e Linux /media, /mnt
            let external_paths = vec!["/Volumes", "/media", "/mnt"];

            for path in external_paths {
                if Path::new(path).exists() {
                    if let Ok(entries) = std::fs::read_dir(path) {
                        for entry in entries.flatten() {
                            if entry.file_type().map(|ft| ft.is_dir()).unwrap_or(false) {
                                self.external_drives.push(entry.path());
                                self.log_operation(&format!(
                                    "📁 Drive externo detectado: {}",
                                    entry.path().display()
                                ));
                            }
                        }
                    }
                }
            }
        }

        #[cfg(windows)]
        {
            // Windows drives A-Z
            for drive_letter in b'A'..=b'Z' {
                let drive_path = format!("{}:\\", drive_letter as char);
                if Path::new(&drive_path).exists() && drive_letter != b'C' {
                    self.external_drives.push(PathBuf::from(&drive_path));
                    self.log_operation(&format!("📁 Drive externo detectado: {}", drive_path));
                }
            }
        }

        println!(
            "🔍 {} drives externos detectados",
            self.external_drives.len()
        );
    }

    /// 📝 REGISTRAR OPERAÇÃO
    fn log_operation(&mut self, operation: &str) {
        let timestamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();

        let log_entry = format!("[{}] {}", timestamp, operation);
        self.operation_log.push(log_entry.clone());

        println!("📝 {}", log_entry);
    }

    /// ⚠️ ATIVAR DEBUG MODE
    pub fn enable_debug_mode(&mut self) {
        self.debug_mode = true;
        self.log_operation("⚠️ DEBUG MODE ATIVADO - Proteções relaxadas");
        println!("⚠️ DEBUG MODE ATIVADO!");
        println!("🔹 Proteções anti-rosnar relaxadas");
        println!("🔹 Use com CUIDADO MÁXIMO");
        println!("🔹 Você é responsável pelas consequências");
    }

    /// 🔒 DESATIVAR DEBUG MODE
    pub fn disable_debug_mode(&mut self) {
        self.debug_mode = false;
        self.log_operation("🔒 DEBUG MODE DESATIVADO - Proteções normais");
        println!("🔒 DEBUG MODE DESATIVADO");
        println!("🔹 Proteções anti-rosnar ATIVADAS");
    }

    /// 📊 MOSTRAR LOG DE OPERAÇÕES
    pub fn show_operation_log(&self) {
        println!("\n📊 LOG DE OPERAÇÕES DE SEGURANÇA");
        println!("{}", "═".repeat(50));

        for entry in &self.operation_log {
            println!("{}", entry);
        }

        println!("{}", "═".repeat(50));
        println!("📁 Total drives externos: {}", self.external_drives.len());
        println!(
            "⚠️ Debug Mode: {}",
            if self.debug_mode {
                "ATIVADO"
            } else {
                "Desativado"
            }
        );
    }

    /// 🗑️ LIMPAR LOG
    pub fn clear_log(&mut self) {
        self.operation_log.clear();
        self.log_operation("📝 Log limpo");
    }

    /// 🚨 VERIFICAÇÃO DE DUPLICIDADE - EVITAR ROSNAR DUPLICADO
    pub fn check_duplicate_rosnar(&mut self, target: &str) -> bool {
        let recent_logs: Vec<String> = self
            .operation_log
            .iter()
            .filter(|log| log.contains("ROSNAR") && log.contains(target))
            .cloned()
            .collect();

        let count = recent_logs.len();

        if count > 1 {
            println!("🚨 ALERTA: ROSNAR duplicado detectado!");
            println!("🔹 Alvo: {}", target);
            println!("🔹 Execuções anteriores: {}", count - 1);
            println!("🔹 Regra: Evite operações duplicadas");
            println!("⚠️ Continue apenas se tiver certeza absoluta");

            // Log após usar os dados
            self.log_operation(&format!("🚨 DUPLICATE ROSNAR DETECTED: {}", target));
            return true;
        }

        false
    }
}

/// 🔒 FUNÇÃO GLOBAL DE VERIFICAÇÃO
static mut SECURITY: Option<SecurityProtection> = None;

pub fn get_security() -> &'static mut SecurityProtection {
    unsafe {
        if SECURITY.is_none() {
            SECURITY = Some(SecurityProtection::new());
        }
        SECURITY.as_mut().unwrap()
    }
}

/// 🔒 VALIDADOR GERAL PARA COMANDOS
pub fn validate_command(cmd: &str, args: &[&str]) -> Result<()> {
    let security = get_security();

    // Se for comando ROSNAR
    if cmd.to_lowercase().contains("rosnar")
        || args.iter().any(|arg| arg.to_lowercase().contains("rosnar"))
    {
        // Encontrar o alvo
        let target = args
            .iter()
            .find(|arg| !arg.starts_with("-") && !arg.to_lowercase().contains("rosnar"))
            .unwrap_or(&"");

        if !target.is_empty() {
            if !security.can_rosnar(target)? {
                return Err(anyhow!("❌ ROSNAR BLOQUEADO por proteção de segurança"));
            }

            // Verificar duplicatas
            if security.check_duplicate_rosnar(target) {
                println!("⚠️ ROSNAR duplicado detectado! Confirme para continuar:");

                // Em modo interativo, pedir confirmação
                print!("🎯 Continuar ROSNAR duplicado? (s/n): ");
                std::io::stdout().flush().unwrap();

                let mut response = String::new();
                std::io::stdin().read_line(&mut response).unwrap();

                if !response.trim().to_lowercase().starts_with('s') {
                    return Err(anyhow!("❌ ROSNAR cancelado pelo usuário"));
                }
            }
        }
    }

    Ok(())
}

/// 🔥 VALIDAÇÃO EXPRESSA DO CHEFE
pub fn executive_validation(command: &str, target: &str) -> Result<()> {
    println!("\n🚨 EXECUTIVE VALIDATION REQUESTED");
    println!("🔹 Comando: {}", command);
    println!("🔹 Alvo: {}", target);

    // Em produção, isso pediria confirmação do CHEFE
    println!("⚠️ Esta é uma operação de ALTO RISCO");

    println!("🎯 CHEFE - Confirmar operação de alto risco? (s/n): ");
    std::io::stdout().flush().unwrap();

    let mut response = String::new();
    std::io::stdin().read_line(&mut response).unwrap();

    if !response.trim().to_lowercase().starts_with('s') {
        return Err(anyhow!("❌ Operação CANCELADA pelo CHEFE"));
    }

    println!("✅ EXECUTIVE APPROVAL GRANTED");
    Ok(())
}
