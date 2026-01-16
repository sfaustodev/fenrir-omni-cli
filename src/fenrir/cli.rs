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
use crate::wrapper;
use bpaf::*;
use std::net::SocketAddr;
use std::path::PathBuf;

// Command definitions using bpaf
#[derive(Debug, Clone)]
pub enum Commands {
    Blockchain(BlockchainCmd),
    Secrets(SecretsCmd),
    Metrics(MetricsCmd),
    Health(HealthCmd),
    Wrapper(WrapperCmd),
    BugBounty(BugBountyCmd),
    Osint(OsintCmd),
    Sandbox(SandboxCmd),
    Plugins(PluginCmd),
    Daemon(DaemonCmd),
    Breach,
    Status,
}

#[derive(Debug, Clone)]
pub enum BlockchainCmd {
    Liquidity(LiquidityCmd),
    Swap(SwapCmd),
    Analyze(AnalyzeCmd),
    Anonymous,
}

#[derive(Debug, Clone)]
pub enum LiquidityCmd {
    Jupiter {
        input: String,
        output: String,
        amount: u64,
    },
    Orca,
}

#[derive(Debug, Clone)]
pub enum SwapCmd {
    CrossChain {
        from: String,
        to: String,
        amount: u64,
    },
}

#[derive(Debug, Clone)]
pub enum AnalyzeCmd {
    Stub { target: String },
}

#[derive(Debug, Clone)]
pub enum SecretsCmd {
    Set { key: String, value: String },
    Get { key: String },
    Delete { key: String },
    List,
}

#[derive(Debug, Clone)]
pub enum MetricsCmd {
    Show,
    Serve { addr: String },
}

#[derive(Debug, Clone)]
pub enum HealthCmd {
    Check,
    Serve { addr: String },
}

#[derive(Debug, Clone)]
pub enum WrapperCmd {
    Generate {
        tool: String,
        output: Option<PathBuf>,
    },
}

#[derive(Debug, Clone)]
pub enum BugBountyCmd {
    Recon { target: String },
    Report { target: String },
}

#[derive(Debug, Clone)]
pub enum OsintCmd {
    TorProbe { url: String },
    Ssh { target: String },
}

#[derive(Debug, Clone)]
pub enum SandboxCmd {
    Run { cmd: String },
}

#[derive(Debug, Clone)]
pub enum PluginCmd {
    List,
    Load { path: PathBuf },
    Run { name: String, input: String },
}

#[derive(Debug, Clone)]
pub enum DaemonCmd {
    Start { target: Option<String> },
    Stop,
    Status,
}

// bpaf parser - simplified for now
fn commands_parser() -> impl Parser<Commands> {
    // For now, just support status command to get basic functionality working
    literal("status").map(|_| Commands::Status)
}

/// Executa CLI moderna.
pub async fn run_cli() -> anyhow::Result<()> {
    metrics::init_metrics();

    let parser = commands_parser()
        .to_options()
        .descr("🐺 FENRIR OMNI CLI")
        .version(env!("CARGO_PKG_VERSION"));

    let command = parser.run();

    match command {
        Commands::Status => {
            let report = health::check();
            println!(
                "🐺 STATUS: {} | uptime {}s",
                report.status, report.uptime_seconds
            );
        }
        Commands::Daemon(cmd) => {
            match cmd {
                DaemonCmd::Start { target } => {
                    println!("🐺 Daemon start not implemented in CLI yet - use interactive mode");
                }
                DaemonCmd::Stop => {
                    println!("🐺 Daemon stop not implemented in CLI yet - use interactive mode");
                }
                DaemonCmd::Status => {
                    println!("🐺 Daemon status not implemented in CLI yet - use interactive mode");
                }
            }
        }
        Commands::Breach => {
            println!("🐺 Breach check not implemented in CLI yet - use interactive mode");
        }
        _ => {
            println!("🐺 Command not implemented yet in bpaf parser");
        }
    }
    Ok(())
}

async fn handle_blockchain(cmd: BlockchainCmd) -> anyhow::Result<()> {
    match cmd {
        BlockchainCmd::Liquidity(cmd) => match cmd {
            LiquidityCmd::Jupiter {
                input,
                output,
                amount,
            } => {
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
            AnalyzeCmd::Stub { target } => {
                println!(
                    "🐺 ANALYZE {} (stub - blockchain analysis not implemented)",
                    target
                );
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
            println!(
                "🐺 HEALTH {} uptime {}s",
                report.status, report.uptime_seconds
            );
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
