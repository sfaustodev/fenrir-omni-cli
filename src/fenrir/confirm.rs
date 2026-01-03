use std::io::{self, Write};

/// Confirma ações de risco com o usuário.
pub fn confirm(prompt: &str) -> anyhow::Result<bool> {
    print!("🐺 {} (digite 'sim' para confirmar): ", prompt);
    io::stdout().flush()?;
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    let normalized = input.trim().to_lowercase();
    Ok(matches!(normalized.as_str(), "sim" | "s" | "yes" | "y"))
}
