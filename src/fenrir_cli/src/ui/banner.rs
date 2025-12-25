//! # Minimalistic Fenrir Banner
//!
//! Clean, simple banner inspired by Claude's design

use colored::Colorize;

/// Minimalistic banner - just FENRIR
const FENRIR_BANNER: &str = r#"
██   ██ ██    ██ ██████  ███████ ██████
 ██ ██  ██    ██ ██   ██ ██      ██   ██
  ███   ██    ██ ██████  █████   ██████
 ██ ██  ██    ██ ██      ██      ██   ██
██   ██  ██████  ██      ███████ ██   ██
"#;

/// Imprime o banner minimalista do Fenrir
pub fn print_banner() {
    println!();
    for line in FENRIR_BANNER.lines() {
        println!("{}", line.bright_red().bold());
    }
    println!();
}

/// Retorna o banner como string
pub fn get_banner() -> &'static str {
    FENRIR_BANNER
}
