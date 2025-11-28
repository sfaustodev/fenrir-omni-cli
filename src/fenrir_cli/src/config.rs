//! # Módulo de Configuração
//!
//! Responsável por carregar, validar e persistir configurações do Fenrir.
//! O arquivo de configuração é armazenado em `~/.config/fenrir/fenrir_rules.yaml`.

use color_eyre::{eyre::eyre, Result};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

/// Nome do diretório de configuração
const CONFIG_DIR: &str = "fenrir";
/// Nome do arquivo de configuração
const CONFIG_FILE: &str = "fenrir_rules.yaml";

/// Estrutura principal de configuração do Fenrir
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    /// Versão do schema de configuração
    #[serde(default = "default_version")]
    pub version: String,

    /// Políticas de conteúdo (guardrails de segurança)
    #[serde(default)]
    pub content_policies: ContentPolicies,

    /// Configurações de scan
    #[serde(default)]
    pub scan: ScanConfig,

    /// Configurações de output
    #[serde(default)]
    pub output: OutputConfig,
}

/// Políticas de conteúdo - guardrails de segurança
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContentPolicies {
    /// Bloqueia qualquer operação que possa envolver conteúdo de pedofilia
    #[serde(default = "default_true")]
    pub anti_pedophilia: bool,

    /// Previne vazamento de credenciais e dados sensíveis
    #[serde(default = "default_true")]
    pub anti_sensitive_leaks: bool,

    /// Permite técnicas agressivas de pentest (pode causar DoS)
    #[serde(default = "default_false")]
    pub allow_aggressive_pentest: bool,

    /// Respeita robots.txt e políticas de rate-limit
    #[serde(default = "default_true")]
    pub respect_robots_txt: bool,

    /// Log detalhado de todas as operações para auditoria
    #[serde(default = "default_true")]
    pub audit_logging: bool,

    /// Bloqueia operações em infraestrutura crítica conhecida
    #[serde(default = "default_true")]
    pub protect_critical_infra: bool,
}

/// Configurações padrão para scans
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScanConfig {
    /// Número máximo de threads paralelas
    #[serde(default = "default_threads")]
    pub max_threads: u32,

    /// Timeout padrão em segundos
    #[serde(default = "default_timeout")]
    pub default_timeout: u32,

    /// Range de portas padrão
    #[serde(default = "default_port_range")]
    pub default_port_range: String,

    /// User-Agent padrão para requisições HTTP
    #[serde(default = "default_user_agent")]
    pub user_agent: String,
}

/// Configurações de output
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OutputConfig {
    /// Usar cores no output
    #[serde(default = "default_true")]
    pub colors: bool,

    /// Nível de verbosidade (0-3)
    #[serde(default)]
    pub verbosity: u8,

    /// Formato de output (text, json, yaml)
    #[serde(default = "default_format")]
    pub format: String,
}

// Funções de valores padrão para serde
fn default_version() -> String {
    "1.0".to_string()
}
fn default_true() -> bool {
    true
}
fn default_false() -> bool {
    false
}
fn default_threads() -> u32 {
    100
}
fn default_timeout() -> u32 {
    5
}
fn default_port_range() -> String {
    "1-1000".to_string()
}
fn default_user_agent() -> String {
    format!("Fenrir/{} (Security Scanner)", env!("CARGO_PKG_VERSION"))
}
fn default_format() -> String {
    "text".to_string()
}

impl Default for ContentPolicies {
    fn default() -> Self {
        Self {
            anti_pedophilia: true,
            anti_sensitive_leaks: true,
            allow_aggressive_pentest: false,
            respect_robots_txt: true,
            audit_logging: true,
            protect_critical_infra: true,
        }
    }
}

impl Default for ScanConfig {
    fn default() -> Self {
        Self {
            max_threads: default_threads(),
            default_timeout: default_timeout(),
            default_port_range: default_port_range(),
            user_agent: default_user_agent(),
        }
    }
}

impl Default for OutputConfig {
    fn default() -> Self {
        Self {
            colors: true,
            verbosity: 1,
            format: default_format(),
        }
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            version: default_version(),
            content_policies: ContentPolicies::default(),
            scan: ScanConfig::default(),
            output: OutputConfig::default(),
        }
    }
}

impl Config {
    /// Retorna o caminho padrão do arquivo de configuração
    pub fn default_path() -> Result<PathBuf> {
        let config_dir = dirs::config_dir()
            .ok_or_else(|| eyre!("Não foi possível determinar o diretório de configuração"))?;

        Ok(config_dir.join(CONFIG_DIR).join(CONFIG_FILE))
    }

    /// Carrega configuração do arquivo ou cria uma nova se não existir
    pub fn load_or_create(custom_path: Option<&str>) -> Result<Self> {
        let path = match custom_path {
            Some(p) => PathBuf::from(p),
            None => Self::default_path()?,
        };

        if path.exists() {
            Self::load(&path)
        } else {
            let config = Self::default();
            // Só cria automaticamente se for o caminho padrão
            if custom_path.is_none() {
                config.save(&path)?;
            }
            Ok(config)
        }
    }

    /// Carrega configuração de um arquivo YAML
    pub fn load(path: &PathBuf) -> Result<Self> {
        let content = fs::read_to_string(path)
            .map_err(|e| eyre!("Erro ao ler arquivo de configuração {:?}: {}", path, e))?;

        let config: Config =
            serde_yaml::from_str(&content).map_err(|e| eyre!("Erro ao parsear YAML: {}", e))?;

        Ok(config)
    }

    /// Salva configuração em um arquivo YAML
    pub fn save(&self, path: &PathBuf) -> Result<()> {
        // Cria diretório se não existir
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)
                .map_err(|e| eyre!("Erro ao criar diretório {:?}: {}", parent, e))?;
        }

        let yaml = serde_yaml::to_string(self)
            .map_err(|e| eyre!("Erro ao serializar configuração: {}", e))?;

        // Adiciona header explicativo
        let content = format!(
            "# Fenrir CLI - Arquivo de Configuração\n\
             # Localização: {}\n\
             # Documentação: https://github.com/peluche/fenrir\n\
             #\n\
             # Este arquivo controla o comportamento do Fenrir CLI.\n\
             # Edite com cuidado - algumas opções podem afetar a segurança.\n\
             \n{}",
            path.display(),
            yaml
        );

        fs::write(path, content)
            .map_err(|e| eyre!("Erro ao salvar configuração em {:?}: {}", path, e))?;

        Ok(())
    }

    /// Reseta configuração para valores padrão
    pub fn reset(&mut self) {
        *self = Self::default();
    }

    /// Verifica se a configuração permite uma operação específica
    pub fn allows_operation(&self, operation: &str) -> bool {
        match operation {
            "aggressive_scan" => self.content_policies.allow_aggressive_pentest,
            "ignore_robots" => !self.content_policies.respect_robots_txt,
            _ => true,
        }
    }
}

/// Template YAML para configuração padrão (usado em documentação)
pub const CONFIG_TEMPLATE: &str = r#"# Fenrir CLI - Arquivo de Configuração
# Edite este arquivo para customizar o comportamento do Fenrir

version: "1.0"

# Políticas de conteúdo - guardrails de segurança
content_policies:
  # Bloqueia operações que possam envolver conteúdo ilegal
  anti_pedophilia: true
  # Previne vazamento de credenciais e dados sensíveis
  anti_sensitive_leaks: true
  # Permite técnicas agressivas (pode causar DoS)
  allow_aggressive_pentest: false
  # Respeita robots.txt e rate-limits
  respect_robots_txt: true
  # Log detalhado para auditoria
  audit_logging: true
  # Protege infraestrutura crítica conhecida
  protect_critical_infra: true

# Configurações de scan
scan:
  max_threads: 100
  default_timeout: 5
  default_port_range: "1-1000"
  user_agent: "Fenrir/0.1.0 (Security Scanner)"

# Configurações de output
output:
  colors: true
  verbosity: 1
  format: "text"
"#;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config() {
        let config = Config::default();
        assert!(config.content_policies.anti_pedophilia);
        assert!(!config.content_policies.allow_aggressive_pentest);
    }

    #[test]
    fn test_serialize_deserialize() {
        let config = Config::default();
        let yaml = serde_yaml::to_string(&config).unwrap();
        let loaded: Config = serde_yaml::from_str(&yaml).unwrap();
        assert_eq!(config.version, loaded.version);
    }
}
