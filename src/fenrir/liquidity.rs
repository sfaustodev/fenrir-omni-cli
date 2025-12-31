use crate::http_client;
use serde::Deserialize;

/// Resposta mínima do Jupiter.
#[derive(Debug, Deserialize)]
pub struct JupiterQuote {
    pub outAmount: String,
}

/// Consulta liquidez via Jupiter quote.
pub async fn jupiter_quote(input: &str, output: &str, amount: u64) -> anyhow::Result<JupiterQuote> {
    let client = http_client::shared_client();
    let url = format!(
        "https://quote-api.jup.ag/v6/quote?inputMint={}&outputMint={}&amount={}",
        input, output, amount
    );
    let resp = client.get(url).send().await?;
    let quote = resp.json::<JupiterQuote>().await?;
    Ok(quote)
}

/// Resposta mínima do Orca.
#[derive(Debug, Deserialize)]
pub struct OrcaWhirlpool {
    pub id: String,
}

/// Consulta pools via Orca (stub leve).
pub async fn orca_pools() -> anyhow::Result<Vec<OrcaWhirlpool>> {
    let client = http_client::shared_client();
    let resp = client
        .get("https://api.mainnet.orca.so/v1/whirlpool/list")
        .send()
        .await?;
    let pools = resp.json::<Vec<OrcaWhirlpool>>().await?;
    Ok(pools)
}
