//! # Fenrir CLI
//!
//! Ponto de entrada principal da aplicação Fenrir.
//! Inicializa o sistema de erros, logging e delega para o módulo CLI.

mod cli;
mod commands;
mod completions;
mod config;
mod core;
mod hybrid_parser;
mod ui;
mod validators;

use color_eyre::Result;
use tracing_subscriber::{fmt, prelude::*, EnvFilter};

/// Versão da aplicação (extraída do Cargo.toml)
pub const VERSION: &str = env!("CARGO_PKG_VERSION");
pub const PKG_NAME: &str = env!("CARGO_PKG_NAME");

fn main() -> Result<()> {
    // Inicializa color-eyre para stack traces bonitos
    color_eyre::install()?;

    // Configura tracing/logging com filtro por variável de ambiente
    tracing_subscriber::registry()
        .with(fmt::layer().with_target(false))
        .with(EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info")))
        .init();

    // Delega para o módulo CLI processar argumentos e executar comandos
    cli::run()
}
