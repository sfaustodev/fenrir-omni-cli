//! # Hybrid Parser Module
//!
//! Interface unificada abstraindo diferentes parsers (clap + custom otimizado).

use clap::Parser;
use color_eyre::Result;
use std::net::{IpAddr, SocketAddr};
use std::time::Duration;

/// Trait para parsers unificados
pub trait FenrirParser<T> {
    fn parse(&self) -> Result<T>;
    fn validate(&self) -> Result<()>;
}

/// Estrutura unificada para comandos de scan
#[derive(Debug, Clone)]
pub struct UnifiedScanCommand {
    pub target: String,
    pub ports: String,
    pub scan_type: ScanType,
    pub timeout: Duration,
    pub threads: u32,
    pub rate_limit: u32,
    pub output: OutputFormat,
    pub output_file: Option<std::path::PathBuf>,
    pub fast_mode: bool,
    pub stealth_mode: bool,
    pub verbose: bool,
    pub quiet: bool,
    pub no_dns: bool,
    pub aggressive: bool,
    pub version_detection: bool,
    pub scripts: Option<String>,
}

/// Tipos de scan suportados
#[derive(Debug, Clone, PartialEq)]
pub enum ScanType {
    Quick,   // Top 1000 ports
    Full,    // All ports
    Custom,  // User-defined
    Stealth, // Evasive scanning
    Fast,    // Top 100 ports
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

/// Parser Híbrido (baseado em clap com otimizações)
#[derive(Debug, Parser)]
pub struct HybridScanParser {
    /// Alvo para scan (IP, hostname, CIDR)
    #[arg(short = 't', long)]
    pub target: String,

    /// Portas (ex: 80,443,8080 ou 1-1000)
    #[arg(short = 'p', long, default_value = "1-1000")]
    pub ports: String,

    /// Tipo de scan (quick, full, stealth, fast)
    #[arg(long, default_value = "quick")]
    pub scan_type: String,

    /// Timeout em ms
    #[arg(long, short = 'T', default_value = "5000")]
    pub timeout_ms: u64,

    /// Threads (1-1000)
    #[arg(long, short = 'j', default_value = "100")]
    pub threads: u32,

    /// Rate limit (packets/sec)
    #[arg(long, short = 'r', default_value = "1000")]
    pub rate_limit: u32,

    /// Output format (json, yaml, csv, table)
    #[arg(long, short = 'o', default_value = "json")]
    pub output: String,

    /// Output file
    #[arg(long, short = 'f')]
    pub output_file: Option<std::path::PathBuf>,

    /// Fast mode (top 100 ports)
    #[arg(long)]
    pub fast: bool,

    /// Stealth mode
    #[arg(long)]
    pub stealth: bool,

    /// Verbose output
    #[arg(long, short = 'v')]
    pub verbose: bool,

    /// Quiet mode
    #[arg(long, short = 'q')]
    pub quiet: bool,

    /// Skip DNS resolution
    #[arg(long)]
    pub no_dns: bool,

    /// Aggressive scan
    #[arg(long, short = 'a')]
    pub aggressive: bool,

    /// Version detection
    #[arg(long)]
    pub version_detection: bool,

    /// Custom scripts
    #[arg(long)]
    pub scripts: Option<String>,
}

impl FenrirParser<UnifiedScanCommand> for HybridScanParser {
    fn parse(&self) -> Result<UnifiedScanCommand> {
        let scan_type = match self.scan_type.to_lowercase().as_str() {
            "quick" => ScanType::Quick,
            "full" => ScanType::Full,
            "custom" => ScanType::Custom,
            "stealth" => ScanType::Stealth,
            "fast" => ScanType::Fast,
            _ => ScanType::Quick,
        };

        let output = match self.output.to_lowercase().as_str() {
            "json" => OutputFormat::Json,
            "yaml" => OutputFormat::Yaml,
            "csv" => OutputFormat::Csv,
            "table" => OutputFormat::Table,
            "text" => OutputFormat::Text,
            "xml" => OutputFormat::Xml,
            _ => OutputFormat::Json,
        };

        Ok(UnifiedScanCommand {
            target: self.target.clone(),
            ports: self.ports.clone(),
            scan_type,
            timeout: Duration::from_millis(self.timeout_ms),
            threads: self.threads,
            rate_limit: self.rate_limit,
            output,
            output_file: self.output_file.clone(),
            fast_mode: self.fast,
            stealth_mode: self.stealth,
            verbose: self.verbose,
            quiet: self.quiet,
            no_dns: self.no_dns,
            aggressive: self.aggressive,
            version_detection: self.version_detection,
            scripts: self.scripts.clone(),
        })
    }

    fn validate(&self) -> Result<()> {
        // Validar target
        if self.target.is_empty() {
            return Err(color_eyre::eyre::eyre!("Target cannot be empty"));
        }

        // Validar que não há conflitos
        if self.fast && self.stealth {
            return Err(color_eyre::eyre::eyre!("Cannot use both fast and stealth modes"));
        }

        if self.verbose && self.quiet {
            return Err(color_eyre::eyre::eyre!("Cannot use both verbose and quiet modes"));
        }

        // Validar limits
        if self.threads > 1000 {
            return Err(color_eyre::eyre::eyre!("Thread count too high (max: 1000)"));
        }

        if self.rate_limit > 10000 {
            return Err(color_eyre::eyre::eyre!("Rate limit too high (max: 10000)"));
        }

        // Validar ports format
        if !is_valid_port_range(&self.ports) {
            return Err(color_eyre::eyre::eyre!("Invalid port range format"));
        }

        Ok(())
    }
}

impl UnifiedScanCommand {
    /// Get parsed socket addresses from ports and target
    pub fn get_scan_targets(&self) -> Result<Vec<SocketAddr>> {
        let mut targets = Vec::new();

        // Parse target as IP or resolve hostname
        let ip: IpAddr = match self.target.parse() {
            Ok(ip) => ip,
            Err(_) => {
                // Try to resolve hostname
                use std::net::ToSocketAddrs;
                match (self.target.clone() + ":80").to_socket_addrs() {
                    Ok(mut addrs) => addrs.next()
                        .ok_or_else(|| color_eyre::eyre::eyre!("Cannot resolve target: {}", self.target))?
                        .ip(),
                    Err(_) => return Err(color_eyre::eyre::eyre!("Cannot resolve target: {}", self.target)),
                }
            }
        };

        // Parse ports efficiently
        for port_str in self.ports.split(',') {
            let port_str = port_str.trim();

            if port_str.contains('-') {
                // Port range
                let range_parts: Vec<&str> = port_str.split('-').collect();
                if range_parts.len() != 2 {
                    continue;
                }

                if let (Ok(start), Ok(end)) = (range_parts[0].parse::<u16>(), range_parts[1].parse::<u16>()) {
                    for port in start..=end {
                        targets.push(SocketAddr::new(ip, port));
                    }
                }
            } else if let Ok(port) = port_str.parse::<u16>() {
                targets.push(SocketAddr::new(ip, port));
            }
        }

        if targets.is_empty() {
            return Err(color_eyre::eyre::eyre!("No valid ports found"));
        }

        Ok(targets)
    }

    /// Get scan statistics
    pub fn get_scan_stats(&self) -> ScanStats {
        let target_count = 1; // Simplificado - poderia ser CIDR expansion
        let port_count = self.count_ports();

        ScanStats {
            target_count,
            port_count,
            total_connections: target_count * port_count,
            estimated_time: self.estimate_scan_time(),
        }
    }

    /// Count total ports to scan
    fn count_ports(&self) -> usize {
        let mut count = 0;

        for port_str in self.ports.split(',') {
            let port_str = port_str.trim();

            if port_str.contains('-') {
                let range_parts: Vec<&str> = port_str.split('-').collect();
                if range_parts.len() == 2 {
                    if let (Ok(start), Ok(end)) = (range_parts[0].parse::<u16>(), range_parts[1].parse::<u16>()) {
                        count += (end - start + 1) as usize;
                    }
                }
            } else if let Ok(_) = port_str.parse::<u16>() {
                count += 1;
            }
        }

        count
    }

    /// Estimate scan time based on configuration
    fn estimate_scan_time(&self) -> Duration {
        let port_count = self.count_ports();
        let timeout_per_port = if self.fast_mode { self.timeout / 2 } else { self.timeout };
        let base_time = Duration::from_millis(timeout_per_port.as_millis() as u64 * port_count as u64);

        // Adjust for threads
        let thread_factor = 1.0 / self.threads as f64;

        // Adjust for stealth mode
        let stealth_factor = if self.stealth_mode { 2.0 } else { 1.0 };

        Duration::from_millis((base_time.as_millis() as f64 * thread_factor * stealth_factor) as u64)
    }
}

/// Scan statistics
#[derive(Debug, Clone)]
pub struct ScanStats {
    pub target_count: usize,
    pub port_count: usize,
    pub total_connections: usize,
    pub estimated_time: Duration,
}

/// Helper function to validate port range format efficiently
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hybrid_parser_validation() {
        let parser = HybridScanParser {
            target: "192.168.1.1".to_string(),
            ports: "80,443,8080".to_string(),
            scan_type: "quick".to_string(),
            timeout_ms: 5000,
            threads: 100,
            rate_limit: 1000,
            output: "json".to_string(),
            output_file: None,
            fast: false,
            stealth: false,
            verbose: false,
            quiet: false,
            no_dns: false,
            aggressive: false,
            version_detection: false,
            scripts: None,
        };

        assert!(parser.validate().is_ok());
        let unified = parser.parse().unwrap();
        assert_eq!(unified.scan_type, ScanType::Quick);
        assert_eq!(unified.output, OutputFormat::Json);
    }

    #[test]
    fn test_scan_targets_parsing() {
        let cmd = UnifiedScanCommand {
            target: "192.168.1.1".to_string(),
            ports: "80,443".to_string(),
            scan_type: ScanType::Quick,
            timeout: Duration::from_secs(5),
            threads: 100,
            rate_limit: 1000,
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

        let targets = cmd.get_scan_targets().unwrap();
        assert_eq!(targets.len(), 2);
        assert!(targets.iter().any(|addr| addr.port() == 80));
        assert!(targets.iter().any(|addr| addr.port() == 443));
    }
}