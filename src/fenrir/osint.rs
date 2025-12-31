use crate::http_client;

/// Stub de integração Tor para OSINT.
pub async fn tor_probe(url: &str) -> anyhow::Result<String> {
    let client = http_client::shared_client();
    let resp = client.get(url).send().await?;
    Ok(format!(
        "🐺 Tor stub: resposta {} (sem proxy real).",
        resp.status()
    ))
}
