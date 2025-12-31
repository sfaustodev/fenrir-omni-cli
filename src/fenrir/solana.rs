use crate::circuit_breaker::CircuitBreaker;
use crate::confirm;
use crate::metrics;
use crate::net;
use once_cell::sync::Lazy;
use solana_client::rpc_client::RpcClient;
use solana_sdk::commitment_config::CommitmentConfig;
use solana_sdk::signature::{read_keypair_file, write_keypair_file, Keypair, Signer};
use solana_sdk::system_instruction;
use solana_sdk::transaction::Transaction;
use std::path::Path;
use std::sync::Mutex;
use std::time::Duration;

static BREAKER: Lazy<Mutex<CircuitBreaker>> =
    Lazy::new(|| Mutex::new(CircuitBreaker::new(3, Duration::from_secs(15))));

/// Cria cliente Solana.
pub fn rpc_client(url: &str) -> RpcClient {
    RpcClient::new_with_commitment(url.to_string(), CommitmentConfig::confirmed())
}

/// Carrega keypair.
pub fn load_keypair(path: &Path) -> anyhow::Result<Keypair> {
    read_keypair_file(path).map_err(|e| anyhow::anyhow!(e))
}

/// Gera novo keypair.
pub fn generate_keypair(path: &Path) -> anyhow::Result<Keypair> {
    let keypair = Keypair::new();
    write_keypair_file(&keypair, path)?;
    Ok(keypair)
}

/// Busca saldo.
pub fn balance(client: &RpcClient, pubkey: &str) -> anyhow::Result<u64> {
    let allowed = BREAKER.lock().unwrap().allow();
    if !allowed {
        anyhow::bail!("circuito aberto para RPC");
    }
    match client.get_balance(&pubkey.parse()?) {
        Ok(balance) => {
            metrics::record_rpc_success();
            BREAKER.lock().unwrap().success();
            Ok(balance)
        }
        Err(err) => {
            metrics::record_rpc_failure();
            BREAKER.lock().unwrap().failure();
            Err(err.into())
        }
    }
}

/// Envia transferência.
pub fn transfer(
    client: &RpcClient,
    keypair: &Keypair,
    to: &str,
    lamports: u64,
) -> anyhow::Result<String> {
    let allowed = BREAKER.lock().unwrap().allow();
    if !allowed {
        anyhow::bail!("circuito aberto para RPC");
    }
    if !confirm::confirm(&format!(
        "Confirma transferir {} lamports para {}",
        lamports, to
    ))? {
        anyhow::bail!("ação cancelada");
    }
    let to_pubkey = to.parse()?;
    let ix = system_instruction::transfer(&keypair.pubkey(), &to_pubkey, lamports);
    let blockhash = client.get_latest_blockhash()?;
    let tx = Transaction::new_signed_with_payer(
        &[ix],
        Some(&keypair.pubkey()),
        &[keypair],
        blockhash,
    );
    match client.send_and_confirm_transaction(&tx) {
        Ok(sig) => {
            metrics::record_rpc_success();
            BREAKER.lock().unwrap().success();
            Ok(sig.to_string())
        }
        Err(err) => {
            metrics::record_rpc_failure();
            BREAKER.lock().unwrap().failure();
            Err(err.into())
        }
    }
}

/// Ping WebSocket Solana.
pub async fn ws_ping(ws_url: &str) -> anyhow::Result<String> {
    net::ws_ping(ws_url).await
}
