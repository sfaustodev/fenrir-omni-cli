//! # Helpers de cores e estilização
//! 
//! Funções utilitárias para formatação colorida consistente.

use colored::{Color, Colorize, ColoredString};

/// Paleta de cores do Fenrir
pub struct FenrirPalette;

impl FenrirPalette {
    pub const PRIMARY: Color = Color::BrightCyan;
    pub const SECONDARY: Color = Color::BrightMagenta;
    pub const SUCCESS: Color = Color::BrightGreen;
    pub const WARNING: Color = Color::BrightYellow;
    pub const ERROR: Color = Color::BrightRed;
    pub const INFO: Color = Color::BrightBlue;
    pub const MUTED: Color = Color::BrightBlack;
}

/// Extensão para strings com estilos do Fenrir
pub trait FenrirStyle {
    fn fenrir_primary(&self) -> ColoredString;
    fn fenrir_secondary(&self) -> ColoredString;
    fn fenrir_success(&self) -> ColoredString;
    fn fenrir_warning(&self) -> ColoredString;
    fn fenrir_error(&self) -> ColoredString;
    fn fenrir_info(&self) -> ColoredString;
    fn fenrir_muted(&self) -> ColoredString;
    fn fenrir_highlight(&self) -> ColoredString;
}

impl FenrirStyle for str {
    fn fenrir_primary(&self) -> ColoredString {
        self.color(FenrirPalette::PRIMARY).bold()
    }

    fn fenrir_secondary(&self) -> ColoredString {
        self.color(FenrirPalette::SECONDARY)
    }

    fn fenrir_success(&self) -> ColoredString {
        self.color(FenrirPalette::SUCCESS)
    }

    fn fenrir_warning(&self) -> ColoredString {
        self.color(FenrirPalette::WARNING)
    }

    fn fenrir_error(&self) -> ColoredString {
        self.color(FenrirPalette::ERROR)
    }

    fn fenrir_info(&self) -> ColoredString {
        self.color(FenrirPalette::INFO)
    }

    fn fenrir_muted(&self) -> ColoredString {
        self.color(FenrirPalette::MUTED)
    }

    fn fenrir_highlight(&self) -> ColoredString {
        self.on_bright_black().white().bold()
    }
}

impl FenrirStyle for String {
    fn fenrir_primary(&self) -> ColoredString {
        self.as_str().fenrir_primary()
    }

    fn fenrir_secondary(&self) -> ColoredString {
        self.as_str().fenrir_secondary()
    }

    fn fenrir_success(&self) -> ColoredString {
        self.as_str().fenrir_success()
    }

    fn fenrir_warning(&self) -> ColoredString {
        self.as_str().fenrir_warning()
    }

    fn fenrir_error(&self) -> ColoredString {
        self.as_str().fenrir_error()
    }

    fn fenrir_info(&self) -> ColoredString {
        self.as_str().fenrir_info()
    }

    fn fenrir_muted(&self) -> ColoredString {
        self.as_str().fenrir_muted()
    }

    fn fenrir_highlight(&self) -> ColoredString {
        self.as_str().fenrir_highlight()
    }
}

/// Formata um valor com unidade (ex: portas, bytes)
pub fn format_count(count: usize, singular: &str, plural: &str) -> String {
    if count == 1 {
        format!("{} {}", count, singular)
    } else {
        format!("{} {}", count, plural)
    }
}

/// Formata uma barra de progresso simples
pub fn progress_bar(current: usize, total: usize, width: usize) -> String {
    let ratio = current as f64 / total as f64;
    let filled = (ratio * width as f64) as usize;
    let empty = width - filled;
    
    format!(
        "[{}{}] {:.1}%",
        "█".repeat(filled).bright_green(),
        "░".repeat(empty).dimmed(),
        ratio * 100.0
    )
}
