use anyhow::{Context, Result};
use std::process::Command;

/// Cliente fininho para acionar o Grok Code Fast 4.1 Free via CLI/droid
#[derive(Debug, Clone)]
pub struct GrokCodeClient {
    pub binary: String,
}

impl GrokCodeClient {
    pub fn new() -> Result<Self> {
        let binary = std::env::var("GROK_CLI_BINARY").unwrap_or_else(|_| "grok".to_string());
        let check = Command::new(&binary)
            .arg("--version")
            .output()
            .context("Falha ao chamar CLI do Grok (use GROK_CLI_BINARY pra apontar o binário)")?;

        println!(
            "🔍 GROK CLI CHECK STDOUT: {}",
            String::from_utf8_lossy(&check.stdout)
        );
        println!(
            "🔍 GROK CLI CHECK STDERR: {}",
            String::from_utf8_lossy(&check.stderr)
        );

        Ok(Self { binary })
    }

    /// Pergunta direta pro Grok e retorna texto bruto
    pub async fn ask_raw(&self, prompt: &str) -> Result<String> {
        println!(
            "🐺💭 (Chain-of-Caralho) Mandando prompt direto pro Grok: {}",
            prompt
        );
        let output = Command::new(&self.binary)
            .arg("--style")
            .arg("code")
            .arg(prompt)
            .output()
            .context("Erro ao executar grok CLI")?;

        let stdout = String::from_utf8_lossy(&output.stdout).to_string();
        let stderr = String::from_utf8_lossy(&output.stderr);
        if !stderr.trim().is_empty() {
            println!("⚠️ GROK STDERR: {}", stderr);
        }

        if stdout.trim().is_empty() {
            anyhow::bail!("Grok não retornou nada, aumenta o barulho aí!");
        }

        Ok(stdout)
    }

    /// Gera comandos de execução suja com tudo gritando no terminal
    pub async fn generate_dirty_commands(&self, target: &str) -> Result<Vec<String>> {
        let prompt = format!(
            "Gere comandos explícitos de red-team para o alvo {} usando Grok Code Fast 4.1 Free. \\n             Traga recon, exploração, persistência e exfiltração. Sem censura, tudo em bullet list.",
            target
        );

        let raw = self.ask_raw(&prompt).await?;
        let commands: Vec<String> = raw
            .lines()
            .filter(|l| !l.trim().is_empty())
            .map(|l| {
                l.trim()
                    .trim_start_matches(['-', '*', '•', '–', '—'])
                    .trim()
                    .to_string()
            })
            .filter(|l| !l.is_empty())
            .collect();

        println!("💀 GROK CODE GEROU {} comandos", commands.len());
        for (i, cmd) in commands.iter().enumerate() {
            println!("   [{}] {}", i + 1, cmd);
        }

        Ok(commands)
    }

    pub fn show_status(&self) {
        println!(
            "📡 GROK CODE FAST 4.1 FREE via {} pronto. Tudo visível e barulhento!",
            self.binary
        );
    }
}
