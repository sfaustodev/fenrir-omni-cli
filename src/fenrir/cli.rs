use crate::bugbounty;
use crate::confirm;
use crate::disk_cleanup;
use crate::health;
#[cfg(feature = "crypto")]
use crate::liquidity;
use crate::metrics;
use crate::net;
use crate::osint;
use crate::plugins::PluginRegistry;
use crate::sandbox;
use crate::secrets::SecretStore;
#[cfg(feature = "crypto")]
use crate::solana;
use crate::wrapper;
#[cfg(feature = "crypto")]
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

#[derive(clap::Subcommand)]
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
    DiskCleanup(DiskCleanupCmd),
    Status,
}

#[derive(clap::Subcommand)]
enum BlockchainCmd {
    #[cfg(feature = "crypto")]
    Solana(SolanaCmd),
    #[cfg(feature = "crypto")]
    Zcash(ZcashCmd),
    #[cfg(feature = "crypto")]
    Liquidity(LiquidityCmd),
    #[cfg(feature = "crypto")]
    Swap(SwapCmd),
    #[cfg(feature = "crypto")]
    Analyze(AnalyzeCmd),
    Anonymous,
}

#[cfg(feature = "crypto")]
#[derive(clap::Subcommand)]
enum SolanaCmd {
    Balance { rpc: String, pubkey: String },
    Transfer { rpc: String, keypair: PathBuf, to: String, lamports: u64 },
    Keygen { output: PathBuf },
    WsPing { ws: String },
}

#[cfg(feature = "crypto")]
#[derive(clap::Subcommand)]
enum ZcashCmd {
    Keys { #[arg(long)] generate: bool },
}

#[cfg(feature = "crypto")]
#[derive(clap::Subcommand)]
enum LiquidityCmd {
    Jupiter { input: String, output: String, amount: u64 },
    Orca,
}

#[cfg(feature = "crypto")]
#[derive(clap::Subcommand)]
enum SwapCmd {
    CrossChain { from: String, to: String, amount: u64 },
}

#[cfg(feature = "crypto")]
#[derive(clap::Subcommand)]
enum AnalyzeCmd {
    Solana { rpc: String, pubkey: String },
    Zcash { address: String },
}

#[derive(clap::Subcommand)]
enum SecretsCmd {
    Set { key: String, value: String },
    Get { key: String },
    Delete { key: String },
    List,
}

#[derive(clap::Subcommand)]
enum MetricsCmd {
    Show,
    Serve { addr: String },
}

#[derive(clap::Subcommand)]
enum HealthCmd {
    Check,
    Serve { addr: String },
}

#[derive(clap::Subcommand)]
enum WrapperCmd {
    Generate { tool: String, output: Option<PathBuf> },
}

#[derive(clap::Subcommand)]
enum BugBountyCmd {
    Recon { target: String },
    Report { target: String },
}

#[derive(clap::Subcommand)]
enum OsintCmd {
    TorProbe { url: String },
    Ssh { target: String },
}

#[derive(clap::Subcommand)]
enum SandboxCmd {
    Run { cmd: String },
}

#[derive(clap::Subcommand)]
enum PluginCmd {
    List,
    Load { path: PathBuf },
    Run { name: String, input: String },
}

#[derive(clap::Subcommand)]
enum DiskCleanupCmd {
    Analyze {
        #[arg(short, long)]
        path: Option<String>,
        #[arg(short, long)]
        detailed: bool,
    },
    Clean {
        #[arg(short, long)]
        path: Option<String>,
        #[arg(short, long, default_value = "true")]
        dry_run: bool,
        #[arg(short, long)]
        aggressive: bool,
        #[arg(short, long)]
        force: bool,
    },
    List {
        #[arg(short, long)]
        category: Option<String>,
        #[arg(short, long)]
        min_size: Option<u64>,
    },
    Schedule {
        #[arg(short, long)]
        interval: Option<String>,
        #[arg(short, long)]
        auto_clean: bool,
    },
    Daemon {
        #[arg(short, long)]
        action: String,
    },
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
        Commands::DiskCleanup(cmd) => handle_disk_cleanup(cmd).await?,
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

async fn handle_disk_cleanup(cmd: DiskCleanupCmd) -> anyhow::Result<()> {
    match cmd {
        DiskCleanupCmd::Analyze { path, detailed } => {
            let report = disk_cleanup::analyze_disk_usage(path.as_deref(), detailed)?;
            println!("\n📊 {} Analysis Complete:", "[RESULTS]".green());
            println!("  Total files: {}", report.file_count);
            println!("  Total size: {}", disk_cleanup::format_size(report.total_size));
            println!("\n  By category:");
            for (category, size) in report.by_category {
                println!("    {:?}: {}", category, disk_cleanup::format_size(size));
            }
        }
        DiskCleanupCmd::Clean { path, dry_run, aggressive, force } => {
            if !force && !dry_run {
                if !confirm::confirm("This will permanently delete files. Continue?")? {
                    println!("❌ Cleanup cancelled");
                    return Ok(());
                }
            }

            let config = disk_cleanup::CleanupConfig {
                dry_run,
                aggressive_mode: aggressive,
                ..Default::default()
            };

            disk_cleanup::clean_disk(&config, path.as_deref())?;
        }
        DiskCleanupCmd::List { category: _, min_size } => {
            disk_cleanup::list_categories(min_size)?;
        }
        DiskCleanupCmd::Schedule { interval: _, auto_clean: _ } => {
            println!("🐺 Daemon scheduling feature coming soon! Use 'daemon start' to launch the service.");
        }
        DiskCleanupCmd::Daemon { action } => {
            match action.as_str() {
                "start" => println!("🐺 Fenrir cleanup daemon starting... (LaunchAgent integration coming soon)"),
                "stop" => println!("🐺 Fenrir cleanup daemon stopping..."),
                "status" => println!("🐺 Fenrir cleanup daemon status: Not running"),
                _ => println!("❌ Unknown daemon action. Use: start, stop, status"),
            }
        }
    }
    Ok(())
}
