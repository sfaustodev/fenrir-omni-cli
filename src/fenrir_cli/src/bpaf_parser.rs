//! # BPAF Parser Module
//!
//! Parser high-performance usando BPAF para comandos críticos de performance.

use bpaf::{argument, construct, long, short, Parser};
use std::net::{IpAddr, SocketAddr};
use std::num::NonZeroU32;
use std::path::PathBuf;
use std::time::Duration;

/// Estrutura para comandos de scan high-performance usando BPAF
#[derive(Debug, Clone)]
pub struct ScanCommand {
    /// Alvo para scan (IP, hostname, CIDR, ou arquivo com targets)
    pub target: String,

    /// Range de portas (ex: 80,443,8080 ou 1-1000)
    pub ports: String,

    /// Tipo de scan (quick=TOP1000, full=65535, custom=speed/stealth)
    pub scan_type: ScanType,

    /// Timeout por conexão em ms
    pub timeout: Duration,

    /// Número de threads paralelas (combinado com rate limiting)
    pub threads: NonZeroU32,

    /// Rate limit (packets/second)
    pub rate_limit: NonZeroU32,

    /// Output format (json, yaml, csv, table)
    pub output: OutputFormat,

    /// Arquivo de saída
    pub output_file: Option<PathBuf>,

    /// Scan速度快 vs深度
    pub fast_mode: bool,

    /// Modo stealth (evita IDS/IPS)
    pub stealth_mode: bool,

    /// Verbose output
    pub verbose: bool,

    /// Modo quiet (sem output exceto erros)
    pub quiet: bool,

    /// Não resolve DNS
    pub no_dns: bool,

    /// Service detection aggressivo
    pub aggressive: bool,

    /// Scan específico de versão
    pub version_detection: bool,

    /// Scan de scripts NSE
    pub scripts: Option<String>,
}

/// Tipos de scan suportados
#[derive(Debug, Clone)]
pub enum ScanType {
    Quick,
    Full,
    Custom,
    Stealth,
    Fast,
}

/// Formatos de output
#[derive(Debug, Clone)]
pub enum OutputFormat {
    Json,
    Yaml,
    Csv,
    Table,
    Text,
    Xml,
}

/// Parser combinatório avançado para workflows complexos
#[derive(Debug, Clone)]
pub struct WorkflowCommand {
    /// Workflow file path
    pub workflow: PathBuf,

    /// Variables para substituição no workflow
    pub variables: Vec<String>,

    /// Parallel execution
    pub parallel: NonZeroU32,

    /// Continue on error
    pub continue_on_error: bool,

    /// Dry run (só mostra commands, não executa)
    pub dry_run: bool,

    /// Log level
    pub log_level: LogLevel,

    /// Output format
    pub output_format: OutputFormat,
}

/// Log levels
#[derive(Debug, Clone)]
pub enum LogLevel {
    Error,
    Warn,
    Info,
    Debug,
    Trace,
}

impl ScanCommand {
    /// Parser principal para o comando scan
    pub fn scan_parser() -> impl Parser<Self> {
        // BPAF parser construction will be done manually
        construct_scan_parser()
    }

    /// Valida e converte o comando para execução
    pub fn validate(&self) -> Result<(), String> {
        // Validar target
        if self.target.is_empty() {
            return Err("Target cannot be empty".to_string());
        }

        // Validar ports format
        if !self.ports.is_empty() && !is_valid_port_range(&self.ports) {
            return Err("Invalid port range format".to_string());
        }

        // Validar conflicts
        if self.fast_mode && self.stealth_mode {
            return Err("Cannot use both fast and stealth modes".to_string());
        }

        if self.quiet && self.verbose {
            return Err("Cannot use both quiet and verbose modes".to_string());
        }

        // Validar threads vs rate limit
        if self.threads.get() > 1000 {
            return Err("Thread count too high (>1000)".to_string());
        }

        Ok(())
    }

    /// Get parsed socket addresses from ports and target
    pub fn get_scan_targets(&self) -> Result<Vec<SocketAddr>, String> {
        let mut targets = Vec::new();

        // Parse target as IP or resolve hostname
        let ip: IpAddr = match self.target.parse() {
            Ok(ip) => ip,
            Err(_) => {
                // Try to resolve hostname
                match (self.target.clone() + ":80").parse::<SocketAddr>() {
                    Ok(addr) => addr.ip(),
                    Err(_) => return Err(format!("Cannot resolve target: {}", self.target)),
                }
            }
        };

        // Parse ports
        for port_str in self.ports.split(',') {
            let port_str = port_str.trim();

            if port_str.contains('-') {
                // Port range
                let parts: Vec<&str> = port_str.split('-').collect();
                if parts.len() != 2 {
                    continue;
                }

                if let (Ok(start), Ok(end)) = (parts[0].parse::<u16>(), parts[1].parse::<u16>()) {
                    for port in start..=end {
                        targets.push(SocketAddr::new(ip, port));
                    }
                }
            } else if let Ok(port) = port_str.parse::<u16>() {
                targets.push(SocketAddr::new(ip, port));
            }
        }

        if targets.is_empty() {
            return Err("No valid ports found".to_string());
        }

        Ok(targets)
    }
}

impl WorkflowCommand {
    /// Parser para comandos de workflow
    pub fn workflow_parser() -> impl Parser<Self> {
        // BPAF parser construction will be done manually
        construct_workflow_parser()
    }

    /// Parse variables from KEY=VALUE format
    pub fn parse_variables(&self) -> std::collections::HashMap<String, String> {
        let mut vars = std::collections::HashMap::new();

        for var in &self.variables {
            if let Some((key, value)) = var.split_once('=') {
                vars.insert(key.to_string(), value.to_string());
            }
        }

        vars
    }
}

/// Helper function to validate port range format
fn is_valid_port_range(ports: &str) -> bool {
    if ports.is_empty() {
        return false;
    }

    for part in ports.split(',') {
        let part = part.trim();

        if part.contains('-') {
            let range_parts: Vec<&str> = part.split('-').collect();
            if range_parts.len() != 2 {
                return false;
            }

            let start: Result<u16, _> = range_parts[0].parse();
            let end: Result<u16, _> = range_parts[1].parse();

            if let (Ok(start), Ok(end)) = (start, end) {
                if start == 0 || end == 0 || start > end {
                    return false;
                }
            } else {
                return false;
            }
        } else if let Ok(port) = part.parse::<u16>() {
            if port == 0 {
                return false;
            }
        } else {
            return false;
        }
    }

    true
}

/// Construct BPAF parser for ScanCommand manually
fn construct_scan_parser() -> impl Parser<ScanCommand> {
    let target = long("target")
        .short('t')
        .argument("TARGET")
        .fallback("127.0.0.1".to_string());

    let ports = long("ports")
        .short('p')
        .argument("PORTS")
        .fallback("1-1000".to_string());

    let scan_type = long("scan-type")
        .argument("TYPE")
        .fallback(ScanType::Quick);

    let timeout = long("timeout")
        .short('T')
        .argument("MS")
        .fallback(Duration::from_secs(5));

    let threads = long("threads")
        .short('j')
        .argument("N")
        .fallback(NonZeroU32::new(100).unwrap());

    let rate_limit = long("rate-limit")
        .short('r')
        .argument("RATE")
        .fallback(NonZeroU32::new(1000).unwrap());

    let output = long("output")
        .short('o')
        .argument("FORMAT")
        .fallback(OutputFormat::Json);

    let output_file = long("output-file")
        .short('f')
        .argument("FILE")
        .optional();

    let fast_mode = long("fast").switch();
    let stealth_mode = long("stealth").switch();
    let verbose = long("verbose").short('v').switch();
    let quiet = long("quiet").short('q').switch();
    let no_dns = long("no-dns").switch();
    let aggressive = long("aggressive").short('a').switch();
    let version_detection = long("version-detection").short('v').switch();
    let scripts = long("script").argument("SCRIPTS").optional();

    construct!(ScanCommand {
        target,
        ports,
        scan_type,
        timeout,
        threads,
        rate_limit,
        output,
        output_file,
        fast_mode,
        stealth_mode,
        verbose,
        quiet,
        no_dns,
        aggressive,
        version_detection,
        scripts
    })
}

/// Construct BPAF parser for WorkflowCommand manually
fn construct_workflow_parser() -> impl Parser<WorkflowCommand> {
    let workflow = argument("WORKFLOW");

    let variables = long("var")
        .short('v')
        .argument("KEY=VALUE")
        .many();

    let parallel = long("parallel")
        .argument("N")
        .fallback(NonZeroU32::new(1).unwrap());

    let continue_on_error = long("continue-on-error").switch();
    let dry_run = long("dry-run").switch();

    let log_level = long("log-level")
        .argument("LEVEL")
        .fallback(LogLevel::Info);

    let output_format = long("output-format")
        .argument("FORMAT")
        .fallback(OutputFormat::Json);

    construct!(WorkflowCommand {
        workflow,
        variables,
        parallel,
        continue_on_error,
        dry_run,
        log_level,
        output_format
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_scan_command_validation() {
        let cmd = ScanCommand {
            target: "192.168.1.1".to_string(),
            ports: "80,443,8080".to_string(),
            scan_type: ScanType::Quick,
            timeout: Duration::from_secs(5),
            threads: NonZeroU32::new(100).unwrap(),
            rate_limit: NonZeroU32::new(1000).unwrap(),
            output: OutputFormat::Json,
            output_file: None,
            fast_mode: false,
            stealth_mode: false,
            verbose: false,
            quiet: false,
            no_dns: false,
            aggressive: false,
            version_detection: false,
            scripts: None,
        };

        assert!(cmd.validate().is_ok());
    }

    #[test]
    fn test_workflow_variables_parsing() {
        let cmd = WorkflowCommand {
            workflow: PathBuf::from("test.yaml"),
            variables: vec!["TARGET=192.168.1.1".to_string(), "PORT=80".to_string()],
            parallel: NonZeroU32::new(1).unwrap(),
            continue_on_error: false,
            dry_run: false,
            log_level: LogLevel::Info,
            output_format: OutputFormat::Json,
        };

        let vars = cmd.parse_variables();
        assert_eq!(vars.get("TARGET"), Some(&"192.168.1.1".to_string()));
        assert_eq!(vars.get("PORT"), Some(&"80".to_string()));
        assert_eq!(vars.len(), 2);
    }
}