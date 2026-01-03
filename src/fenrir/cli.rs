use crate::bugbounty;
use crate::confirm;
use crate::health;
use crate::liquidity;
use crate::metrics;
use crate::net;
use crate::osint;
use crate::plugins::PluginRegistry;
use crate::sandbox;
use crate::secrets::SecretStore;
use crate::solana;
use crate::wrapper;
use crate::zcash;
use clap::{Parser, Subcommand};
use std::net::SocketAddr;
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "fenrir", version, about = "🐺 FENRIR OMNI CLI")]
pub struct FenrirCli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Blockchain(BlockchainCmd),
    Secrets(SecretsCmd),
    Metrics(MetricsCmd),
    Health(HealthCmd),
    Wrapper(WrapperCmd),
    BugBounty(BugBountyCmd),
    Osint(OsintCmd),
    Sandbox(SandboxCmd),
    Plugins(PluginCmd),
    Status,
}

#[derive(Subcommand)]
enum BlockchainCmd {
    Solana(SolanaCmd),
    Zcash(ZcashCmd),
    Liquidity(LiquidityCmd),
    Swap(SwapCmd),
    Analyze(AnalyzeCmd),
    Anonymous,
}

#[derive(Subcommand)]
enum SolanaCmd {
    Balance { rpc: String, pubkey: String },
    Transfer { rpc: String, keypair: PathBuf, to: String, lamports: u64 },
    Keygen { output: PathBuf },
    WsPing { ws: String },
}

#[derive(Subcommand)]
enum ZcashCmd {
    Keys { #[arg(long)] generate: bool },
}

#[derive(Subcommand)]
enum LiquidityCmd {
    Jupiter { input: String, output: String, amount: u64 },
    Orca,
}

#[derive(Subcommand)]
enum SwapCmd {
    CrossChain { from: String, to: String, amount: u64 },
}

#[derive(Subcommand)]
enum AnalyzeCmd {
    Solana { rpc: String, pubkey: String },
    Zcash { address: String },
}

#[derive(Subcommand)]
enum SecretsCmd {
    Set { key: String, value: String },
    Get { key: String },
    Delete { key: String },
    List,
}

#[derive(Subcommand)]
enum MetricsCmd {
    Show,
    Serve { addr: String },
}

#[derive(Subcommand)]
enum HealthCmd {
    Check,
    Serve { addr: String },
}

#[derive(Subcommand)]
enum WrapperCmd {
    Generate { tool: String, output: Option<PathBuf> },
}

#[derive(Subcommand)]
enum BugBountyCmd {
    Recon { target: String },
    Report { target: String },
}

#[derive(Subcommand)]
enum OsintCmd {
    TorProbe { url: String },
    Ssh { target: String },
}

#[derive(Subcommand)]
enum SandboxCmd {
    Run { cmd: String },
}

#[derive(Subcommand)]
enum PluginCmd {
    List,
    Load { path: PathBuf },
    Run { name: String, input: String },
}

/// Executa CLI moderna.
pub async fn run_cli() -> anyhow::Result<()> {
    metrics::init_metrics();
    let cli = FenrirCli::parse();
    match cli.command {
        Commands::Blockchain(cmd) => handle_blockchain(cmd).await?,
        Commands::Secrets(cmd) => handle_secrets(cmd)?,
        Commands::Metrics(cmd) => handle_metrics(cmd).await?,
        Commands::Health(cmd) => handle_health(cmd).await?,
        Commands::Wrapper(cmd) => handle_wrapper(cmd)?,
        Commands::BugBounty(cmd) => handle_bugbounty(cmd)?,
        Commands::Osint(cmd) => handle_osint(cmd).await?,
        Commands::Sandbox(cmd) => handle_sandbox(cmd)?,
        Commands::Plugins(cmd) => handle_plugins(cmd)?,
        Commands::Status => {
            let report = health::check();
            println!("🐺 STATUS: {} | uptime {}s", report.status, report.uptime_seconds);
        }
    }
    Ok(())
}

async fn handle_blockchain(cmd: BlockchainCmd) -> anyhow::Result<()> {
    match cmd {
        BlockchainCmd::Solana(cmd) => match cmd {
            SolanaCmd::Balance { rpc, pubkey } => {
                let client = solana::rpc_client(&rpc);
                let balance = solana::balance(&client, &pubkey)?;
                println!("🐺 SOLANA BALANCE {} lamports", balance);
            }
            SolanaCmd::Transfer { rpc, keypair, to, lamports } => {
                let client = solana::rpc_client(&rpc);
                let keypair = solana::load_keypair(&keypair)?;
                let sig = solana::transfer(&client, &keypair, &to, lamports)?;
                println!("🐺 TX SENT {}", sig);
            }
            SolanaCmd::Keygen { output } => {
                solana::generate_keypair(&output)?;
                println!("🐺 KEYPAIR GERADO em {}", output.display());
            }
            SolanaCmd::WsPing { ws } => {
                let response = solana::ws_ping(&ws).await?;
                println!("🐺 WS PING {}", response);
            }
        },
        BlockchainCmd::Zcash(cmd) => match cmd {
            ZcashCmd::Keys { generate } => {
                if !generate {
                    println!("🐺 Use --generate para criar chaves.");
                    return Ok(());
                }
                let keys = zcash::generate_keys()?;
                println!("🐺 ZCASH ADDRESS {}", keys.address);
                println!("🐺 SEED {}", keys.seed);
            }
        },
        BlockchainCmd::Liquidity(cmd) => match cmd {
            LiquidityCmd::Jupiter { input, output, amount } => {
                let quote = liquidity::jupiter_quote(&input, &output, amount).await?;
                println!("🐺 JUPITER OUT {}", quote.outAmount);
            }
            LiquidityCmd::Orca => {
                let pools = liquidity::orca_pools().await?;
                println!("🐺 ORCA POOLS {}", pools.len());
            }
        },
        BlockchainCmd::Swap(cmd) => match cmd {
            SwapCmd::CrossChain { from, to, amount } => {
                if !confirm::confirm(&format!(
                    "Confirma swap cross-chain {} -> {} (amount {})",
                    from, to, amount
                ))? {
                    anyhow::bail!("ação cancelada");
                }
                println!("🐺 Swap stub ativado. Integração real em breve.");
            }
        },
        BlockchainCmd::Analyze(cmd) => match cmd {
            AnalyzeCmd::Solana { rpc, pubkey } => {
                let client = solana::rpc_client(&rpc);
                let balance = solana::balance(&client, &pubkey)?;
                println!("🐺 ANALYZE SOLANA balance {}", balance);
            }
            AnalyzeCmd::Zcash { address } => {
                println!("🐺 ANALYZE ZCASH {} (stub)", address);
            }
        },
        BlockchainCmd::Anonymous => {
            println!("🐺 Feature bloqueada por compliance. Mixer não será implementado.");
        }
    }
    Ok(())
}

fn handle_secrets(cmd: SecretsCmd) -> anyhow::Result<()> {
    let store = SecretStore::new("fenrir");
    match cmd {
        SecretsCmd::Set { key, value } => {
            store.set(&key, &value)?;
            println!("🐺 SEGREDO SALVO {}", key);
        }
        SecretsCmd::Get { key } => {
            let value = store.get(&key)?;
            match value {
                Some(v) => println!("🐺 {} = {}", key, v),
                None => println!("🐺 Segredo não encontrado"),
            }
        }
        SecretsCmd::Delete { key } => {
            store.delete(&key)?;
            println!("🐺 SEGREDO REMOVIDO {}", key);
        }
        SecretsCmd::List => {
            for key in store.list()? {
                println!("🐺 {}", key);
            }
        }
    }
    Ok(())
}

async fn handle_metrics(cmd: MetricsCmd) -> anyhow::Result<()> {
    match cmd {
        MetricsCmd::Show => {
            metrics::init_metrics();
            println!("{}", metrics::export_metrics());
        }
        MetricsCmd::Serve { addr } => {
            let socket: SocketAddr = addr.parse()?;
            println!("🐺 Metrics em http://{}", socket);
            health::serve(socket).await?;
        }
    }
    Ok(())
}

async fn handle_health(cmd: HealthCmd) -> anyhow::Result<()> {
    match cmd {
        HealthCmd::Check => {
            let report = health::check();
            println!("🐺 HEALTH {} uptime {}s", report.status, report.uptime_seconds);
        }
        HealthCmd::Serve { addr } => {
            let socket: SocketAddr = addr.parse()?;
            println!("🐺 Health em http://{}", socket);
            health::serve(socket).await?;
        }
    }
    Ok(())
}

fn handle_wrapper(cmd: WrapperCmd) -> anyhow::Result<()> {
    match cmd {
        WrapperCmd::Generate { tool, output } => {
            let spec = wrapper::generate_wrapper(&tool)?;
            let json = serde_json::to_string_pretty(&spec)?;
            if let Some(path) = output {
                std::fs::write(&path, &json)?;
                println!("🐺 Wrapper salvo em {}", path.display());
            } else {
                println!("{}", json);
            }
        }
    }
    Ok(())
}

fn handle_bugbounty(cmd: BugBountyCmd) -> anyhow::Result<()> {
    match cmd {
        BugBountyCmd::Recon { target } => {
            println!("{}", bugbounty::recon(&target));
        }
        BugBountyCmd::Report { target } => {
            println!("{}", bugbounty::report(&target));
        }
    }
    Ok(())
}

async fn handle_osint(cmd: OsintCmd) -> anyhow::Result<()> {
    match cmd {
        OsintCmd::TorProbe { url } => {
            let result = osint::tor_probe(&url).await?;
            println!("{}", result);
        }
        OsintCmd::Ssh { target } => {
            println!("{}", net::ssh_stub(&target));
        }
    }
    Ok(())
}

fn handle_sandbox(cmd: SandboxCmd) -> anyhow::Result<()> {
    match cmd {
        SandboxCmd::Run { cmd } => {
            sandbox::apply_sandbox()?;
            let status = std::process::Command::new("sh")
                .arg("-c")
                .arg(cmd)
                .status()?;
            println!("🐺 Sandbox exit {}", status);
        }
    }
    Ok(())
}

fn handle_plugins(cmd: PluginCmd) -> anyhow::Result<()> {
    let mut registry = PluginRegistry::new();
    match cmd {
        PluginCmd::List => {
            for (name, description) in registry.list() {
                println!("🐺 {} - {}", name, description);
            }
        }
        PluginCmd::Load { path } => unsafe {
            registry.load_dynamic(path)?;
            println!("🐺 Plugin carregado");
        },
        PluginCmd::Run { name, input } => {
            let output = registry.run(&name, &input)?;
            println!("🐺 Plugin output {}", output);
        }
    }
    Ok(())
}
