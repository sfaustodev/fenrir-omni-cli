// --- FENRIR STARSHIP MODULE ---
// Implementação customizada do Starship para o Fenrir Terminal
// Prompt ultra-rápido e infinitamente customizável para o Lobo Devorador

use std::env;
use std::process::Command;
use std::path::{Path, PathBuf};
use std::time::{Duration, SystemTime};
use serde::{Deserialize, Serialize};
use nu_ansi_term::{Color, Style};

// Estrutura principal do Fenrir-Starship
#[derive(Debug, Clone)]
pub struct FenrirStarship {
    pub config: StarshipConfig,
    pub context: StarshipContext,
}

// Configuração do Starship Fenrir
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StarshipConfig {
    pub format: String,
    pub scan_timeout: u64,
    pub add_newline: bool,
    pub modules: Vec<String>,
    pub character: CharacterConfig,
    pub git_branch: GitBranchConfig,
    pub directory: DirectoryConfig,
    pub nodejs: NodeJsConfig,
    pub rust: RustConfig,
    pub python: PythonConfig,
    pub docker_context: DockerConfig,
    pub cmd_duration: CmdDurationConfig,
    pub time: TimeConfig,
    pub fenrir: FenrirModuleConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CharacterConfig {
    pub success_symbol: String,
    pub error_symbol: String,
    pub vicmd_symbol: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GitBranchConfig {
    pub format: String,
    pub symbol: String,
    pub truncation_length: usize,
    pub truncation_symbol: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DirectoryConfig {
    pub truncation_length: usize,
    pub truncate_to_repo: bool,
    pub format: String,
    pub repo_root_style: String,
    pub style: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NodeJsConfig {
    pub format: String,
    pub version_format: String,
    pub symbol: String,
    pub detect_extensions: Vec<String>,
    pub detect_files: Vec<String>,
    pub detect_folders: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RustConfig {
    pub format: String,
    pub version_format: String,
    pub symbol: String,
    pub detect_extensions: Vec<String>,
    pub detect_files: Vec<String>,
    pub detect_folders: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PythonConfig {
    pub format: String,
    pub version_format: String,
    pub symbol: String,
    pub detect_extensions: Vec<String>,
    pub detect_files: Vec<String>,
    pub detect_folders: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DockerConfig {
    pub format: String,
    pub symbol: String,
    pub only_with_files: bool,
    pub detect_extensions: Vec<String>,
    pub detect_files: Vec<String>,
    pub detect_folders: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CmdDurationConfig {
    pub min_time: u64,
    pub format: String,
    pub show_milliseconds: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimeConfig {
    pub format: String,
    pub style: String,
    pub utc_time_offset: String,
    pub disabled: bool,
    pub time_range: String,
}

// Configuração exclusiva do módulo Fenrir
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FenrirModuleConfig {
    pub format: String,
    pub symbol: String,
    pub style: String,
    pub show_ghost: bool,
    pub show_mode: bool,
    pub show_terminal: bool,
}

// Contexto do Starship Fenrir
#[derive(Debug, Clone)]
pub struct StarshipContext {
    pub current_dir: PathBuf,
    pub git_repo: Option<GitInfo>,
    pub environment: EnvironmentInfo,
    pub system_info: SystemInfo,
    pub fenrir_mode: FenrirMode,
}

#[derive(Debug, Clone)]
pub struct GitInfo {
    pub branch: String,
    pub commit_hash: String,
    pub status: GitStatus,
    pub ahead: usize,
    pub behind: usize,
}

#[derive(Debug, Clone)]
pub enum GitStatus {
    Clean,
    Modified,
    Staged,
    Conflicted,
}

#[derive(Debug, Clone)]
pub struct EnvironmentInfo {
    pub user: String,
    pub hostname: String,
    pub shell: String,
    pub terminal: String,
}

#[derive(Debug, Clone)]
pub struct SystemInfo {
    pub os: String,
    pub arch: String,
    pub uptime: Duration,
    pub load_average: f32,
}

#[derive(Debug, Clone)]
pub enum FenrirMode {
    Interactive,
    Command,
    GodMode,
    Silent,
}

// Módulos do Fenrir-Starship
#[derive(Debug, Clone)]
pub enum FenrirModule {
    Character,
    Directory,
    GitBranch,
    GitStatus,
    NodeJs,
    Rust,
    Python,
    Docker,
    CmdDuration,
    Time,
    Fenrir,
    Custom(String),
}

impl FenrirStarship {
    // Criar nova instância do Fenrir-Starship
    pub fn new() -> Self {
        let config = Self::load_config();
        let context = Self::create_context();

        Self {
            config,
            context,
        }
    }

    // Carregar configuração (com valores defaults otimizados para Fenrir)
    fn load_config() -> StarshipConfig {
        StarshipConfig {
            format: "$all".to_string(),
            scan_timeout: 30,
            add_newline: true,
            modules: vec![
                "fenrir".to_string(),
                "character".to_string(),
                "directory".to_string(),
                "git_branch".to_string(),
                "git_status".to_string(),
                "nodejs".to_string(),
                "rust".to_string(),
                "python".to_string(),
                "docker_context".to_string(),
                "cmd_duration".to_string(),
                "time".to_string(),
            ],
            character: CharacterConfig {
                success_symbol: "[➜](bold green)".to_string(),
                error_symbol: "[➜](bold red)".to_string(),
                vicmd_symbol: "[❮](bold green)".to_string(),
            },
            git_branch: GitBranchConfig {
                format: "on [$symbol$branch]($style) ".to_string(),
                symbol: "".to_string(),
                truncation_length: 20,
                truncation_symbol: "...".to_string(),
            },
            directory: DirectoryConfig {
                truncation_length: 3,
                truncate_to_repo: true,
                format: "[$path]($style)[$read_only]($read_only_style) ".to_string(),
                repo_root_style: "bold purple".to_string(),
                style: "bold cyan".to_string(),
            },
            nodejs: NodeJsConfig {
                format: "via [${symbol}${version}]($style) ".to_string(),
                version_format: "v${raw}".to_string(),
                symbol: "🦕 ".to_string(),
                detect_extensions: vec!["js".to_string(), "mjs".to_string(), "cjs".to_string()],
                detect_files: vec!["package.json".to_string(), ".node-version".to_string()],
                detect_folders: vec!["node_modules".to_string()],
            },
            rust: RustConfig {
                format: "via [${symbol}${version}]($style) ".to_string(),
                version_format: "v${raw}".to_string(),
                symbol: "🦀 ".to_string(),
                detect_extensions: vec!["rs".to_string()],
                detect_files: vec!["Cargo.toml".to_string(), "Cargo.lock".to_string()],
                detect_folders: vec![],
            },
            python: PythonConfig {
                format: "via [${symbol}${version}]($style) ".to_string(),
                version_format: "v${raw}".to_string(),
                symbol: "🐍 ".to_string(),
                detect_extensions: vec!["py".to_string()],
                detect_files: vec!["requirements.txt".to_string(), "pyproject.toml".to_string(), "Pipfile".to_string()],
                detect_folders: vec![".venv".to_string(), "__pycache__".to_string()],
            },
            docker_context: DockerConfig {
                format: "via [${symbol}${context}]($style) ".to_string(),
                symbol: "🐳 ".to_string(),
                only_with_files: true,
                detect_extensions: vec!["Dockerfile".to_string()],
                detect_files: vec!["docker-compose.yml".to_string(), "docker-compose.yaml".to_string(), "Dockerfile".to_string()],
                detect_folders: vec![".docker".to_string()],
            },
            cmd_duration: CmdDurationConfig {
                min_time: 2000,
                format: "took [$duration]($style) ".to_string(),
                show_milliseconds: false,
            },
            time: TimeConfig {
                format: "at [$time]($style) ".to_string(),
                style: "bold yellow".to_string(),
                utc_time_offset: "local".to_string(),
                disabled: true,
                time_range: "-".to_string(),
            },
            fenrir: FenrirModuleConfig {
                format: "[$symbol$mode]($style) ".to_string(),
                symbol: "🐺 ".to_string(),
                style: "bold white".to_string(),
                show_ghost: true,
                show_mode: true,
                show_terminal: true,
            },
        }
    }

    // Criar contexto com informações do ambiente
    fn create_context() -> StarshipContext {
        let current_dir = env::current_dir().unwrap_or_default();
        let git_repo = Self::detect_git_info(&current_dir);
        let environment = Self::get_environment_info();
        let system_info = Self::get_system_info();
        let fenrir_mode = FenrirMode::Interactive;

        StarshipContext {
            current_dir,
            git_repo,
            environment,
            system_info,
            fenrir_mode,
        }
    }

    // Detectar informações do repositório Git
    fn detect_git_info(dir: &Path) -> Option<GitInfo> {
        let output = Command::new("git")
            .args(&["-C", dir.to_str().unwrap_or("."), "branch", "--show-current"])
            .output()
            .ok()?;

        if !output.status.success() {
            return None;
        }

        let branch = String::from_utf8_lossy(&output.stdout).trim().to_string();

        // Detectar status do Git
        let status_output = Command::new("git")
            .args(&["-C", dir.to_str().unwrap_or("."), "status", "--porcelain"])
            .output()
            .ok()?;

        let git_status = if status_output.stdout.is_empty() {
            GitStatus::Clean
        } else {
            GitStatus::Modified
        };

        Some(GitInfo {
            branch,
            commit_hash: "HEAD".to_string(),
            status: git_status,
            ahead: 0,
            behind: 0,
        })
    }

    // Obter informações do ambiente
    fn get_environment_info() -> EnvironmentInfo {
        EnvironmentInfo {
            user: env::var("USER").unwrap_or_default(),
            hostname: env::var("HOSTNAME").unwrap_or_else(|_| {
                Command::new("hostname").output()
                    .map(|o| String::from_utf8_lossy(&o.stdout).trim().to_string())
                    .unwrap_or_default()
            }),
            shell: env::var("SHELL").unwrap_or_default(),
            terminal: env::var("TERM").unwrap_or_default(),
        }
    }

    // Obter informações do sistema
    fn get_system_info() -> SystemInfo {
        SystemInfo {
            os: if cfg!(target_os = "macos") { "macOS".to_string() }
                 else if cfg!(target_os = "linux") { "Linux".to_string() }
                 else { "Unknown".to_string() },
            arch: if cfg!(target_arch = "x86_64") { "x86_64".to_string() }
                   else if cfg!(target_arch = "aarch64") { "ARM64".to_string() }
                   else { "Unknown".to_string() },
            uptime: Duration::from_secs(0), // Implementar uptime real
            load_average: 0.0, // Implementar load average
        }
    }

    // Renderizar prompt completo
    pub fn render_prompt(&self, last_command_status: i32) -> String {
        let mut prompt = String::new();

        // Adicionar módulo Fenrir
        prompt.push_str(&self.render_fenrir_module());

        // Adicionar módulo de diretório
        prompt.push_str(&self.render_directory_module());

        // Adicionar módulos Git se estiver em um repositório
        if let Some(_) = &self.context.git_repo {
            prompt.push_str(&self.render_git_branch_module());
            prompt.push_str(&self.render_git_status_module());
        }

        // Adicionar módulos de linguagem
        prompt.push_str(&self.render_nodejs_module());
        prompt.push_str(&self.render_rust_module());
        prompt.push_str(&self.render_python_module());
        prompt.push_str(&self.render_docker_module());

        // Adicionar módulo de tempo (se habilitado)
        if !self.config.time.disabled {
            prompt.push_str(&self.render_time_module());
        }

        // Adicionar character
        prompt.push_str(&self.render_character_module(last_command_status));

        prompt
    }

    // Renderizar módulo Fenrir (exclusivo)
    fn render_fenrir_module(&self) -> String {
        let config = &self.config.fenrir;
        let style = self.parse_style(&config.style);

        let mut output = String::new();
        output.push_str(&config.symbol);

        if config.show_mode {
            output.push_str(match self.context.fenrir_mode {
                FenrirMode::Interactive => "INTERACTIVE",
                FenrirMode::Command => "COMMAND",
                FenrirMode::GodMode => "🔴 GOD MODE",
                FenrirMode::Silent => "SILENT",
            });
        }

        if config.show_ghost {
            output.push('👻');
        }

        style.paint(output).to_string() + " "
    }

    // Renderizar módulo de diretório
    fn render_directory_module(&self) -> String {
        let config = &self.config.directory;
        let style = self.parse_style(&config.style);

        let dir_path = self.context.current_dir.to_string_lossy();
        let display_path = if config.truncate_to_repo && self.context.git_repo.is_some() {
            // Truncar para repo root
            if let Some(home_dir) = dirs::home_dir() {
                if dir_path.starts_with(&*home_dir.to_string_lossy()) {
                    format!("~{}", &dir_path[home_dir.to_string_lossy().len()..])
                } else {
                    dir_path.to_string()
                }
            } else {
                dir_path.to_string()
            }
        } else {
            dir_path.to_string()
        };

        style.paint(display_path).to_string() + " "
    }

    // Renderizar módulo Git Branch
    fn render_git_branch_module(&self) -> String {
        if let Some(git_info) = &self.context.git_repo {
            let config = &self.config.git_branch;
            let style = Style::default().fg(Color::Purple);

            let branch_name = if git_info.branch.len() > config.truncation_length {
                format!("{}{}",
                    &git_info.branch[..config.truncation_length],
                    config.truncation_symbol
                )
            } else {
                git_info.branch.clone()
            };

            format!("{}on {} ",
                config.symbol,
                style.paint(branch_name)
            )
        } else {
            String::new()
        }
    }

    // Renderizar módulo Git Status
    fn render_git_status_module(&self) -> String {
        if let Some(git_info) = &self.context.git_repo {
            match git_info.status {
                GitStatus::Clean => String::new(),
                GitStatus::Modified => Style::default().fg(Color::Red).paint("✗ ").to_string(),
                GitStatus::Staged => Style::default().fg(Color::Yellow).paint("+ ").to_string(),
                GitStatus::Conflicted => Style::default().fg(Color::Red).paint("✖ ").to_string(),
            }
        } else {
            String::new()
        }
    }

    // Renderizar módulo Node.js
    fn render_nodejs_module(&self) -> String {
        if self.detect_nodejs_project() {
            if let Some(version) = self.get_node_version() {
                return format!("🦕 {} ", version);
            }
        }
        String::new()
    }

    // Renderizar módulo Rust
    fn render_rust_module(&self) -> String {
        if self.detect_rust_project() {
            if let Some(version) = self.get_rust_version() {
                return format!("🦀 {} ", version);
            }
        }
        String::new()
    }

    // Renderizar módulo Python
    fn render_python_module(&self) -> String {
        if self.detect_python_project() {
            if let Some(version) = self.get_python_version() {
                return format!("🐍 {} ", version);
            }
        }
        String::new()
    }

    // Renderizar módulo Docker
    fn render_docker_module(&self) -> String {
        if self.detect_docker_project() {
            if let Ok(context) = self.get_docker_context() {
                return format!("🐳 {} ", context);
            }
        }
        String::new()
    }

    // Renderizar módulo de tempo
    fn render_time_module(&self) -> String {
        let _now = SystemTime::now();
        let style = self.parse_style(&self.config.time.style);

        // Implementar formatação de tempo real
        let time_str = chrono::Local::now()
            .format("%H:%M:%S")
            .to_string();

        format!("at {} ", style.paint(time_str))
    }

    // Renderizar character (prompt final)
    fn render_character_module(&self, last_command_status: i32) -> String {
        let config = &self.config.character;
        let style = if last_command_status == 0 {
            Style::default().fg(Color::Green).bold()
        } else {
            Style::default().fg(Color::Red).bold()
        };

        let symbol = if last_command_status == 0 {
            &config.success_symbol
        } else {
            &config.error_symbol
        };

        style.paint(symbol).to_string() + " "
    }

    // Detectar projeto Node.js
    fn detect_nodejs_project(&self) -> bool {
        self.context.current_dir.join("package.json").exists() ||
        self.context.current_dir.join("node_modules").exists() ||
        self.context.current_dir.join(".node-version").exists()
    }

    // Detectar projeto Rust
    fn detect_rust_project(&self) -> bool {
        self.context.current_dir.join("Cargo.toml").exists() ||
        self.context.current_dir.join("Cargo.lock").exists()
    }

    // Detectar projeto Python
    fn detect_python_project(&self) -> bool {
        self.context.current_dir.join("requirements.txt").exists() ||
        self.context.current_dir.join("pyproject.toml").exists() ||
        self.context.current_dir.join("Pipfile").exists() ||
        self.context.current_dir.join(".venv").exists() ||
        self.context.current_dir.join("__pycache__").exists()
    }

    // Detectar projeto Docker
    fn detect_docker_project(&self) -> bool {
        self.context.current_dir.join("Dockerfile").exists() ||
        self.context.current_dir.join("docker-compose.yml").exists() ||
        self.context.current_dir.join("docker-compose.yaml").exists()
    }

    // Obter versão Node.js
    fn get_node_version(&self) -> Option<String> {
        Command::new("node")
            .arg("--version")
            .output()
            .ok()
            .and_then(|output| {
                if output.status.success() {
                    Some(String::from_utf8_lossy(&output.stdout).trim().to_string())
                } else {
                    None
                }
            })
    }

    // Obter versão Rust
    fn get_rust_version(&self) -> Option<String> {
        Command::new("rustc")
            .arg("--version")
            .output()
            .ok()
            .and_then(|output| {
                if output.status.success() {
                    let version = String::from_utf8_lossy(&output.stdout);
                    version.split_whitespace().nth(1).map(|v| v.to_string())
                } else {
                    None
                }
            })
    }

    // Obter versão Python
    fn get_python_version(&self) -> Option<String> {
        Command::new("python3")
            .arg("--version")
            .output()
            .ok()
            .and_then(|output| {
                if output.status.success() {
                    let version = String::from_utf8_lossy(&output.stdout);
                    version.split_whitespace().nth(1).map(|v| v.to_string())
                } else {
                    None
                }
            })
    }

    // Obter contexto Docker
    fn get_docker_context(&self) -> Result<String, Box<dyn std::error::Error>> {
        let output = Command::new("docker")
            .args(&["context", "show"])
            .output()?;

        if output.status.success() {
            Ok(String::from_utf8_lossy(&output.stdout).trim().to_string())
        } else {
            Err("Failed to get docker context".into())
        }
    }

    // Parse style string para Style object
    fn parse_style(&self, style_str: &str) -> Style {
        // Implementar parsing completo de estilos
        // Por enquanto, usar estilos simples baseados nas cores

        if style_str.contains("green") {
            Style::default().fg(Color::Green)
        } else if style_str.contains("red") {
            Style::default().fg(Color::Red)
        } else if style_str.contains("blue") {
            Style::default().fg(Color::Blue)
        } else if style_str.contains("yellow") {
            Style::default().fg(Color::Yellow)
        } else if style_str.contains("purple") {
            Style::default().fg(Color::Purple)
        } else if style_str.contains("cyan") {
            Style::default().fg(Color::Cyan)
        } else if style_str.contains("white") {
            Style::default().fg(Color::White)
        } else {
            Style::default()
        }
        .bold_if(style_str.contains("bold"))
    }

    // Atualizar contexto (chamado antes de cada renderização)
    pub fn update_context(&mut self) {
        self.context = Self::create_context();
    }

    // Renderizar prompt para terminal específico
    pub fn render_for_terminal(&self, ghostty_available: bool, last_command_status: i32) -> String {
        let mut prompt = self.render_prompt(last_command_status);

        // Adicionar indicadores do terminal Ghostty se disponível
        if ghostty_available {
            prompt = format!("[Ghostty] {}", prompt);
        }

        prompt
    }
}

// Extension trait para Style
trait StyleExt {
    fn bold_if(self, condition: bool) -> Self;
}

impl StyleExt for Style {
    fn bold_if(self, condition: bool) -> Self {
        if condition {
            self.bold()
        } else {
            self
        }
    }
}

// Função pública para inicializar o Fenrir-Starship
pub fn initialize_fenrir_starship() -> FenrirStarship {
    FenrirStarship::new()
}