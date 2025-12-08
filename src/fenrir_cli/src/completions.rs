//! # Shell Completions Module
//!
//! Gera e gerencia arquivos de completition para various shells.

use clap::CommandFactory;
use clap_complete::{generate, Shell};
use color_eyre::Result;
use std::fs;
use std::path::PathBuf;

use super::cli::Cli;

/// Gera arquivos de completion para todos os shells suportados
pub fn generate_all_completions() -> Result<()> {
    let out_dir = PathBuf::from("completions");
    fs::create_dir_all(&out_dir)?;

    let mut cmd = Cli::command();

    // Gerar completion para cada shell
    let shells = [
        Shell::Bash,
        Shell::Zsh,
        Shell::Fish,
        Shell::PowerShell,
        Shell::Elvish,
    ];

    for shell in shells.iter() {
        let ext = match shell {
            Shell::Bash => "bash",
            Shell::Zsh => "zsh",
            Shell::Fish => "fish",
            Shell::PowerShell => "ps1",
            Shell::Elvish => "elv",
            _ => "sh",
        };
        let path = out_dir.join(format!("fenrir.{}", ext));
        generate(*shell, &mut cmd, "fenrir", &mut std::io::BufWriter::new(
            std::fs::File::create(&path)?
        ));
        println!("Generated completion for {:?}: {}", shell, path.display());
    }

    Ok(())
}

/// Gera completion para um shell específico
pub fn generate_completion(shell: Shell, output: Option<PathBuf>) -> Result<()> {
    let mut cmd = Cli::command();

    match output {
        Some(path) => {
            generate(shell, &mut cmd, "fenrir", &mut std::io::BufWriter::new(
                std::fs::File::create(path)?
            ));
        }
        None => {
            // Output para stdout
            generate(shell, &mut cmd, "fenrir", &mut std::io::stdout());
        }
    }

    Ok(())
}

/// Retorna instruções de instalação para cada shell
pub fn get_install_instructions() -> Vec<(Shell, String)> {
    vec![
        (Shell::Bash, r#"
# Adicione ao ~/.bashrc ou ~/.bash_profile:
eval "$(fenrir --completion bash)"
"#.to_string()),
        (Shell::Zsh, r#"
# Adicione ao ~/.zshrc:
source <(fenrir --completion zsh)
"#.to_string()),
        (Shell::Fish, r#"
# Adicione ao ~/.config/fish/completions/fenrir.fish:
fenrir --completion fish > ~/.config/fish/completions/fenrir.fish
"#.to_string()),
        (Shell::PowerShell, r#"
# No PowerShell:
fenrir --completion powershell | Out-String | Invoke-Expression
"#.to_string()),
        (Shell::Elvish, r#"
# Adicione ao ~/.config/elvish/rc.elv:
eval (fenrir --completion elvish | slurp)
"#.to_string()),
    ]
}