//! FENRIR HONESTY LAW - Mandamento absoluto imposto pelo chefe

use chrono::Local;
use std::fs::OpenOptions;
use std::io::Write;

/// Texto canônico exigido pelo usuário.
pub const HONESTY_LAW_TEXT: &str = "OBEDEÇA O DEUS DO FENRIR: nenhuma simulação, nenhum tapaburaco. Se algo falhar, imprima o erro cru e detalhe técnico na hora. Não minta, não omita e não devolva mensagens de sucesso quando nada foi executado.";

/// Mensagem complementar sobre consequências diretas.
pub const HONESTY_CONSEQUENCE: &str = "Descumprir esta lei significa que Fenrir só usará motores que não mentem e você será desligado.";

/// Impressão redundante exigida em todos os MCPs/CLIs.
pub fn declare_honesty_law(context: &str) {
    println!(
        "🩸 HONESTY LAW [{}]: {} — {}",
        context, HONESTY_LAW_TEXT, HONESTY_CONSEQUENCE
    );
}

/// Registra ordens explícitas do chefe em um log dedicado.
pub fn log_honesty_order(source: &str, payload: &str) -> anyhow::Result<()> {
    let timestamp = Local::now().format("%Y-%m-%d %H:%M:%S");
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open("fenrir_honesty_orders.log")?;
    writeln!(
        file,
        "[{}] [{}] {}",
        timestamp,
        source.to_uppercase(),
        payload.trim()
    )?;
    Ok(())
}
