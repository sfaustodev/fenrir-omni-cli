use futures::{SinkExt, StreamExt};
use tokio_tungstenite::tungstenite::Message;
use url::Url;

/// Conecta em WebSocket e executa ping simples.
pub async fn ws_ping(url: &str) -> anyhow::Result<String> {
    let url = Url::parse(url)?;
    let (mut stream, _) = tokio_tungstenite::connect_async(url).await?;
    stream.send(Message::Text("ping".into())).await?;
    if let Some(msg) = stream.next().await {
        let text = msg?.to_text()?.to_string();
        return Ok(text);
    }
    anyhow::bail!("sem resposta")
}

/// Stub de SSH.
pub fn ssh_stub(target: &str) -> String {
    format!(
        "🐺 SSH stub ativado para {}. Integração em breve.",
        target
    )
}
