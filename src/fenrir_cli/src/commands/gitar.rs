//! Comando 'gitar' - Automação de workflow Git + GitHub PR
//!
//! Sequência:
//! 1. git status
//! 2. git add -A
//! 3. git commit -m "mensagem"
//! 4. git push
//! 5. gh pr create --fill (ou abre browser)

use crate::ui;
use color_eyre::eyre::{Context, Result};
use std::process::Command;

pub fn execute(message: Option<String>, verbose: bool) -> Result<()> {
    ui::section("🎸 Gitar - Git Workflow Automation");

    // 1. Git Status
    ui::info("Executando 'git status'...");
    run_command("git", &["status"], verbose)?;

    // 2. Git Add -A
    ui::info("Executando 'git add -A'...");
    run_command("git", &["add", "-A"], verbose)?;

    // 3. Git Commit
    let msg = match message {
        Some(m) => m,
        None => {
            // Se não passou mensagem, usa uma default genérica ou pergunta (aqui default)
            "chore: update via fenrir gitar".to_string()
        }
    };
    
    ui::info(&format!("Executando 'git commit -m \"{}\"...'", msg));
    match run_command("git", &["commit", "-m", &msg], verbose) {
        Ok(_) => ui::success("Commit realizado com sucesso!"),
        Err(e) => {
            // Pode falhar se não houver nada para commitar, mas seguimos
            ui::warning(&format!("Commit retornou erro (talvez nada para commitar?): {}", e));
        }
    }

    // 4. Git Push
    ui::info("Executando 'git push'...");
    // Tenta push simples primeiro. Se falhar (upstream unset), tenta setup
    if let Err(_) = run_command("git", &["push"], verbose) {
         ui::warning("Push falhou. Tentando 'git push --set-upstream origin HEAD'...");
         // Obter branch atual
         let branch = get_current_branch()?;
         run_command("git", &["push", "--set-upstream", "origin", &branch], verbose)
            .context("Falha ao realizar git push upstream")?;
    }
    ui::success("Push realizado!");

    // 5. GH PR Create
    ui::info("Tentando criar PR via GH CLI ('gh pr create --fill')...");
    match run_command("gh", &["pr", "create", "--fill"], verbose) {
        Ok(_) => ui::success("PR criado com sucesso!"),
        Err(_) => {
            ui::warning("Falha ao criar PR automático. Tentando abrir web...");
            if let Err(e) = run_command("gh", &["pr", "create", "--web"], verbose) {
                 ui::error(&format!("Não foi possível criar o PR: {}", e));
                 ui::info("Dica: Instale o GitHub CLI (brew install gh) e autentique (gh auth login).");
            }
        }
    }

    Ok(())
}

fn run_command(cmd: &str, args: &[&str], verbose: bool) -> Result<()> {
    if verbose {
        println!("> {} {}", cmd, args.join(" "));
    }

    let status = Command::new(cmd)
        .args(args)
        .status()
        .context(format!("Falha ao executar {}", cmd))?;

    if status.success() {
        Ok(())
    } else {
        Err(color_eyre::eyre::eyre!("Comando falhou com código {:?}", status.code()))
    }
}

fn get_current_branch() -> Result<String> {
    let output = Command::new("git")
        .args(&["rev-parse", "--abbrev-ref", "HEAD"])
        .output()
        .context("Falha ao obter branch atual")?;
        
    Ok(String::from_utf8_lossy(&output.stdout).trim().to_string())
}
