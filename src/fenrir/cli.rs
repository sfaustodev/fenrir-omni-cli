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
use std::collections::HashMap;
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
    Demo,
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

// Fuzzy command detection with smart matching
fn fuzzy_command_match(input: &str) -> Option<Commands> {
    let input_lower = input.to_lowercase();
    let input_words: Vec<&str> = input_lower.split_whitespace().collect();

    // Command patterns with fuzzy matching
    let command_patterns: HashMap<&str, Vec<Vec<&str>>> = [
        ("status", vec![vec!["status"], vec!["check", "status"], vec!["system", "status"]]),
        ("demo", vec![vec!["demo"], vec!["demonstrate"], vec!["show", "tools"], vec!["tools", "demo"]]),
        ("daemon", vec![
            vec!["daemon", "start"],
            vec!["start", "daemon"],
            vec!["daemon", "stop"],
            vec!["stop", "daemon"],
            vec!["daemon", "status"],
            vec!["check", "daemon"]
        ]),
        ("breach", vec![vec!["breach"], vec!["security", "breach"], vec!["check", "breach"]]),
        ("health", vec![vec!["health"], vec!["check", "health"], vec!["system", "health"]]),
        ("metrics", vec![vec!["metrics"], vec!["show", "metrics"], vec!["check", "metrics"]]),
        ("secrets", vec![
            vec!["secrets", "list"],
            vec!["list", "secrets"],
            vec!["secrets", "set"],
            vec!["set", "secret"],
            vec!["secrets", "get"],
            vec!["get", "secret"]
        ]),
        ("blockchain", vec![
            vec!["blockchain", "liquidity"],
            vec!["liquidity", "check"],
            vec!["blockchain", "swap"],
            vec!["swap", "crypto"]
        ]),
        ("osint", vec![
            vec!["osint", "tor"],
            vec!["tor", "probe"],
            vec!["osint", "ssh"],
            vec!["ssh", "check"]
        ]),
        ("plugins", vec![
            vec!["plugins", "list"],
            vec!["list", "plugins"],
            vec!["plugins", "load"],
            vec!["load", "plugin"]
        ]),
        ("sandbox", vec![vec!["sandbox"], vec!["run", "sandbox"]]),
        ("wrapper", vec![vec!["wrapper"], vec!["generate", "wrapper"]]),
        ("bugbounty", vec![
            vec!["bugbounty", "recon"],
            vec!["recon", "target"],
            vec!["bugbounty", "report"],
            vec!["report", "bug"]
        ])
    ].into_iter().collect();

    // Find best fuzzy match
    for (cmd_name, patterns) in &command_patterns {
        for pattern in patterns {
            if fuzzy_match_words(&input_words, pattern) {
                return match *cmd_name {
                    "status" => Some(Commands::Status),
                    "demo" => Some(Commands::Demo),
                    "breach" => Some(Commands::Breach),
                    "daemon" => {
                        if input_words.contains(&"start") {
                            Some(Commands::Daemon(DaemonCmd::Start { target: None }))
                        } else if input_words.contains(&"stop") {
                            Some(Commands::Daemon(DaemonCmd::Stop))
                        } else {
                            Some(Commands::Daemon(DaemonCmd::Status))
                        }
                    },
                    "health" => Some(Commands::Health(HealthCmd::Check)),
                    "metrics" => Some(Commands::Metrics(MetricsCmd::Show)),
                    _ => None, // For now, only implement basic commands
                };
            }
        }
    }

    None
}

// Fuzzy word matching function
fn fuzzy_match_words(input_words: &[&str], pattern: &[&str]) -> bool {
    if pattern.is_empty() {
        return input_words.is_empty();
    }

    let mut input_idx = 0;
    let mut pattern_idx = 0;

    while input_idx < input_words.len() && pattern_idx < pattern.len() {
        if input_words[input_idx].contains(pattern[pattern_idx]) ||
           pattern[pattern_idx].contains(input_words[input_idx]) ||
           levenshtein_distance(input_words[input_idx], pattern[pattern_idx]) <= 2 {
            pattern_idx += 1;
        }
        input_idx += 1;
    }

    pattern_idx == pattern.len()
}

// Simple Levenshtein distance for fuzzy matching
fn levenshtein_distance(s1: &str, s2: &str) -> usize {
    let s1_chars: Vec<char> = s1.chars().collect();
    let s2_chars: Vec<char> = s2.chars().collect();

    let len1 = s1_chars.len();
    let len2 = s2_chars.len();

    let mut matrix = vec![vec![0; len2 + 1]; len1 + 1];

    for i in 0..=len1 {
        matrix[i][0] = i;
    }
    for j in 0..=len2 {
        matrix[0][j] = j;
    }

    for i in 1..=len1 {
        for j in 1..=len2 {
            let cost = if s1_chars[i - 1] == s2_chars[j - 1] { 0 } else { 1 };
            matrix[i][j] = (matrix[i - 1][j] + 1)
                .min(matrix[i][j - 1] + 1)
                .min(matrix[i - 1][j - 1] + cost);
        }
    }

    matrix[len1][len2]
}

// bpaf parser with fuzzy fallback
fn commands_parser() -> impl Parser<Commands> {
    // Try exact matches first, then fallback to fuzzy matching
    let exact_parser = literal("status").map(|_| Commands::Status);

    // For fuzzy matching, we'll handle it in the main function
    exact_parser
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
            use crate::daemon::{FenrirDaemon, DaemonConfig};

            match cmd {
                DaemonCmd::Start { target } => {
                    // Handle optional target parameter
                    let target_value = match target {
                        Some(t) => t,
                        None => {
                            println!("⚠️  No target specified");
                            println!("💡 Usage: fenrir daemon start <target>");
                            println!("📝 Example: fenrir daemon start 192.168.1.0/24");
                            return Ok(());
                        }
                    };

                    // Check if daemon is already running
                    let daemon_guard = crate::DAEMON.lock().await;
                    if daemon_guard.is_some() {
                        println!("⚠️  Daemon is already running");
                        println!("💡 Use 'fenrir daemon stop' first to stop the current instance");
                        drop(daemon_guard);

                        // Show status of existing daemon
                        let daemon_guard = crate::DAEMON.lock().await;
                        if let Some(daemon) = daemon_guard.as_ref() {
                            let status = daemon.status().await;
                            println!("{}", status);
                        }
                    } else {
                        drop(daemon_guard);

                        // Create new daemon instance
                        let config = DaemonConfig::default();
                        let daemon = FenrirDaemon::new(target_value.clone(), config);

                        // Start the daemon
                        match daemon.start().await {
                            Ok(_) => {
                                // Store daemon in global state
                                let mut daemon_guard = crate::DAEMON.lock().await;
                                *daemon_guard = Some(daemon);
                                drop(daemon_guard);

                                println!("✅ Daemon started successfully");
                                println!("🌐 Target: {}", target_value);
                                println!("💡 Use 'fenrir daemon status' to check status");
                                println!("💡 Use 'fenrir daemon stop' to stop the daemon");
                            }
                            Err(e) => {
                                eprintln!("❌ Failed to start daemon: {}", e);
                            }
                        }
                    }
                }
                DaemonCmd::Stop => {
                    let mut daemon_guard = crate::DAEMON.lock().await;
                    if let Some(daemon) = daemon_guard.as_ref() {
                        match daemon.stop().await {
                            Ok(_) => {
                                *daemon_guard = None;
                                println!("✅ Daemon stopped successfully");
                            }
                            Err(e) => {
                                eprintln!("❌ Failed to stop daemon: {}", e);
                            }
                        }
                    } else {
                        println!("⚠️  Daemon is not running");
                        println!("💡 Use 'fenrir daemon start <target>' to start the daemon");
                    }
                }
                DaemonCmd::Status => {
                    let daemon_guard = crate::DAEMON.lock().await;
                    if let Some(daemon) = daemon_guard.as_ref() {
                        let status = daemon.status().await;
                        println!("{}", status);
                    } else {
                        println!("🐺 FENRIR DAEMON STATUS");
                        println!("Running: No");
                        println!("\n💡 Use 'fenrir daemon start <target>' to start the daemon");
                    }
                }
            }
        }
        Commands::Breach => {
            println!("🐺 Breach check not implemented in CLI yet - use interactive mode");
        }
        Commands::Demo => {
            if let Err(e) = crate::kali_tools::demonstrate_tools().await {
                println!("❌ Demo failed: {}", e);
            }
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

async fn handle_bugbounty(cmd: BugBountyCmd) -> anyhow::Result<()> {
    match cmd {
        BugBountyCmd::Recon { target } => {
            println!("{}", bugbounty::recon(&target).await);
        }
        BugBountyCmd::Report { target } => {
            println!("{}", bugbounty::report(&target).await);
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
