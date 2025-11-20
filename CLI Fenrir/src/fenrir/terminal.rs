// --- FENRIR GHOSTTY TERMINAL INTERFACE ---
// Módulo para interface avançada de terminal usando Ghostty
// Bootstrapping de terminal com capacidades extendidas

use std::process::Command;
use std::env;
use std::io::{self, Write};
use std::fs;
use serde::{Deserialize, Serialize};
use crossterm::{
    execute,
    terminal::{self, ClearType},
    cursor::{self, MoveTo},
    style::{Color, Print, SetForegroundColor, ResetColor},
};

// Estrutura de configuração do terminal Ghostty
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GhosttyConfig {
    pub theme: String,
    pub font_family: String,
    pub font_size: f32,
    pub background_opacity: f32,
    pub window_width: u16,
    pub window_height: u16,
}

impl Default for GhosttyConfig {
    fn default() -> Self {
        Self {
            theme: "Fenrir Dark".to_string(),
            font_family: "JetBrains Mono".to_string(),
            font_size: 14.0,
            background_opacity: 0.95,
            window_width: 120,
            window_height: 40,
        }
    }
}

// Interface principal do terminal Fenrir
pub struct FenrirTerminal {
    pub config: GhosttyConfig,
    pub ghostty_available: bool,
}

impl FenrirTerminal {
    pub fn new() -> Self {
        let ghostty_available = Self::check_ghostty_installation();
        Self {
            config: GhosttyConfig::default(),
            ghostty_available,
        }
    }

    // Verifica se Ghostty está instalado
    fn check_ghostty_installation() -> bool {
        Command::new("which")
            .arg("ghostty")
            .output()
            .map(|output| output.status.success())
            .unwrap_or(false)
    }

    // Inicia interface Ghostty se disponível
    pub fn bootstrap_ghostty_interface(&self) -> io::Result<()> {
        if !self.ghostty_available {
            println!("⚠️  Ghostty não encontrado. Usando terminal fallback.");
            self.bootstrap_fallback_interface()?;
            return Ok(());
        }

        // Configura ambiente Ghostty para o Fenrir
        self.setup_ghostty_environment()?;

        // Inicia interface enriquecida
        self.start_enhanced_terminal_mode()?;

        Ok(())
    }

    // Configura variáveis de ambiente e configurações Ghostty
    fn setup_ghostty_environment(&self) -> io::Result<()> {
        env::set_var("GHOSTTY_THEME", &self.config.theme);
        env::set_var("GHOSTTY_FONT_FAMILY", &self.config.font_family);
        env::set_var("GHOSTTY_FONT_SIZE", self.config.font_size.to_string());
        env::set_var("GHOSTTY_BACKGROUND_OPACITY", self.config.background_opacity.to_string());

        // Criar configuração temporária do Ghostty para o Fenrir
        let ghostty_config = self.generate_ghostty_config();
        let config_path = format!("{}/.fenrir/ghostty_config", env::var("HOME").unwrap_or_default());

        if let Some(parent) = std::path::Path::new(&config_path).parent() {
            fs::create_dir_all(parent)?;
        }

        fs::write(&config_path, ghostty_config)?;
        env::set_var("GHOSTTY_CONFIG", config_path);

        Ok(())
    }

    // Gera configuração Ghostty específica para Fenrir
    fn generate_ghostty_config(&self) -> String {
        format!(
            r#"# FENRIR GHOSTTY CONFIGURATION
# Configuração otimizada para o assistente de terminal

theme = {}
font-family = {}
font-size = {}
background-opacity = {}

# Cores do tema Fenrir
foreground = #00ff41
background = #0a0a0a
cursor-color = #ff0040

# Configurações de janela
window-width = {}
window-height = {}
window-theme = dark

# Configurações avançadas
confirm-close-surface = false
shell-integration = fish
keybind = ctrl+shift+f=new_font:size:+1
keybind = ctrl+shift+g=new_font:size:-1

# Performance
gpu-acceleration = yes
resize-delay = 0
"#,
            self.config.theme,
            self.config.font_family,
            self.config.font_size,
            self.config.background_opacity,
            self.config.window_width,
            self.config.window_height
        )
    }

    // Inicia modo de terminal avançado
    fn start_enhanced_terminal_mode(&self) -> io::Result<()> {
        // Limpa terminal e configura modo avançado
        terminal::enable_raw_mode()?;
        execute!(
            io::stdout(),
            terminal::Clear(ClearType::All),
            cursor::MoveTo(0, 0),
            cursor::Hide
        )?;

        // Exibe splash screen do Fenrir em modo Ghostty
        self.display_fenrir_splash()?;

        Ok(())
    }

    // Interface fallback quando Ghostty não está disponível
    fn bootstrap_fallback_interface(&self) -> io::Result<()> {
        execute!(
            io::stdout(),
            terminal::Clear(ClearType::All),
            SetForegroundColor(Color::Green),
            Print("🐺 FENRIR TERMINAL - MODO COMPATIBILIDADE\n"),
            ResetColor,
            Print("Usando terminal padrão. Instale Ghostty para experiência completa.\n\n")
        )?;
        Ok(())
    }

    // Splash screen animado do Fenrir
    fn display_fenrir_splash(&self) -> io::Result<()> {
        let splash_art = r#"
╔══════════════════════════════════════════════════════════════╗
║  ████████╗ ██████╗ ███████╗ ██████╗ ███╗   ███╗ █████╗ ███╗   ██╗  ║
║  ╚══██╔══╝██╔═══██╗██╔════╝██╔═══██╗████╗ ████║██╔══██╗████╗  ██║  ║
║     ██║   ██║   ██║███████╗██║   ██║██╔████╔██║███████║██╔██╗ ██║  ║
║     ██║   ██║   ██║╚════██║██║   ██║██║╚██╔╝██║██╔══██║██║╚██╗██║  ║
║     ██║   ╚██████╔╝███████║╚██████╔╝██║ ╚═╝ ██║██║  ██║██║ ╚████║  ║
║     ╚═╝    ╚═════╝ ╚══════╝ ╚═════╝ ╚═╝     ╚═╝╚═╝  ╚═╝╚═╝  ╚═══╝  ║
║                                                              ║
║   🐺 O Lobo Devorador de Terminais está ATIVO                ║
║   Interface Ghostty + Rust + IA (Gemini)                     ║
║   Bootstrapping completo...                                  ║
╚══════════════════════════════════════════════════════════════╝
"#;

        execute!(
            io::stdout(),
            SetForegroundColor(Color::Green),
            Print(splash_art),
            ResetColor,
            Print("\n")
        )?;

        Ok(())
    }

    // Exibe prompt interativo avançado
    pub fn display_interactive_prompt(&self) -> io::Result<()> {
        execute!(
            io::stdout(),
            MoveTo(0, terminal::size()?.1 - 1),
            SetForegroundColor(Color::Cyan),
            Print("🐺 FENRIR> "),
            ResetColor
        )?;
        io::stdout().flush()?;
        Ok(())
    }

    // Limpa área de entrada e prepara para próximo comando
    pub fn clear_input_area(&self) -> io::Result<()> {
        let (_, height) = terminal::size()?;
        execute!(
            io::stdout(),
            MoveTo(0, height - 2),
            terminal::Clear(ClearType::FromCursorDown)
        )?;
        Ok(())
    }

    // Restaura terminal ao estado original
    pub fn restore_terminal(&self) -> io::Result<()> {
        execute!(
            io::stdout(),
            cursor::Show,
            SetForegroundColor(Color::Reset),
            ResetColor
        )?;
        terminal::disable_raw_mode()?;
        Ok(())
    }
}

// Função pública para inicializar interface do terminal
pub fn bootstrap_terminal_interface() -> io::Result<FenrirTerminal> {
    let terminal = FenrirTerminal::new();
    terminal.bootstrap_ghostty_interface()?;
    Ok(terminal)
}

// Função para detectar capabilities do terminal
pub fn detect_terminal_capabilities() -> (bool, bool, bool) {
    let has_ghostty = Command::new("which")
        .arg("ghostty")
        .output()
        .map(|output| output.status.success())
        .unwrap_or(false);

    let has_colors = env::var("TERM")
        .unwrap_or_default()
        .contains("color");

    let has_unicode_support = env::var("LC_ALL")
        .or_else(|_| env::var("LANG"))
        .unwrap_or_default()
        .contains("UTF");

    (has_ghostty, has_colors, has_unicode_support)
}