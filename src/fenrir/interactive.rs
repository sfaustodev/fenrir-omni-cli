// --- FENRIR INTERACTIVE MODE v2.0 ---
// Complete rebuild with smart attack sequences, NLP integration, and async execution
// Red Team Tooling with stealth-first approach

use crate::fenrir_ai_layer;
use crate::nlp;
use futures::future::join_all;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::io::{self, Write};
use std::process::{Command, Stdio};
use std::sync::atomic::AtomicU64;
use std::sync::Arc;
use tokio::sync::Semaphore;
use tokio::task;

// ============================================================================
// CONSTANTS - MEMORY LIMITS
// ============================================================================

const ASYNC_THREAD_MEMORY_LIMIT_MB: u64 = 666;
const SEQUENTIAL_THREAD_MEMORY_LIMIT_MB: u64 = 2048;
const MAX_CONCURRENT_ASYNC_TASKS: usize = 10;

// ============================================================================
// SMART ATTACK SEQUENCE DEFINITIONS
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SmartAttackSequence {
    pub keyword: String,
    pub description: String,
    pub tools: Vec<AttackTool>,
    pub stealth_first: bool,
    pub async_execution: bool,
    pub memory_limit_mb: u64,
    pub category: AttackCategory,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AttackTool {
    pub name: String,
    pub command: String,
    pub stealth_args: Vec<String>,
    pub aggressive_args: Vec<String>,
    pub description: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum AttackCategory {
    Reconnaissance,
    Scanning,
    Exploitation,
    PasswordAttacks,
    WebApplications,
    WirelessAttacks,
    SocialEngineering,
    Forensics,
    PostExploitation,
    PrivilegeEscalation,
}

// ============================================================================
// COMPREHENSIVE KEYWORD MAPPINGS
// ============================================================================

pub fn get_all_smart_sequences() -> HashMap<String, SmartAttackSequence> {
    let mut sequences = HashMap::new();

    // PASSWORD - All password-related attacks
    sequences.insert("password".to_string(), SmartAttackSequence {
        keyword: "password".to_string(),
        description: "Complete password cracking and credential harvesting sequence".to_string(),
        tools: vec![
            AttackTool {
                name: "cewl".to_string(),
                command: "cewl".to_string(),
                stealth_args: vec!["-d", "2", "-m", "5", "-w", "wordlist.txt"].iter().map(|s| s.to_string()).collect(),
                aggressive_args: vec!["-d", "5", "-m", "3", "-w", "wordlist.txt", "-e", "-a"].iter().map(|s| s.to_string()).collect(),
                description: "Generate custom wordlist from target website".to_string(),
            },
            AttackTool {
                name: "crunch".to_string(),
                command: "crunch".to_string(),
                stealth_args: vec!["6", "8", "abcdefghijklmnopqrstuvwxyz0123456789"].iter().map(|s| s.to_string()).collect(),
                aggressive_args: vec!["4", "12", "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789!@#$%"].iter().map(|s| s.to_string()).collect(),
                description: "Generate custom wordlists with patterns".to_string(),
            },
            AttackTool {
                name: "hydra".to_string(),
                command: "hydra".to_string(),
                stealth_args: vec!["-t", "1", "-W", "5", "-f"].iter().map(|s| s.to_string()).collect(),
                aggressive_args: vec!["-t", "16", "-W", "1", "-f", "-V"].iter().map(|s| s.to_string()).collect(),
                description: "Online password brute-forcing".to_string(),
            },
            AttackTool {
                name: "hashcat".to_string(),
                command: "hashcat".to_string(),
                stealth_args: vec!["-a", "0", "-m", "0", "--quiet"].iter().map(|s| s.to_string()).collect(),
                aggressive_args: vec!["-a", "3", "-m", "0", "-w", "4", "--force"].iter().map(|s| s.to_string()).collect(),
                description: "GPU-accelerated password cracking".to_string(),
            },
            AttackTool {
                name: "john".to_string(),
                command: "john".to_string(),
                stealth_args: vec!["--wordlist=/usr/share/wordlists/rockyou.txt"].iter().map(|s| s.to_string()).collect(),
                aggressive_args: vec!["--incremental", "--fork=4"].iter().map(|s| s.to_string()).collect(),
                description: "John the Ripper password cracker".to_string(),
            },
            AttackTool {
                name: "patator".to_string(),
                command: "patator".to_string(),
                stealth_args: vec!["--rate-limit=1"].iter().map(|s| s.to_string()).collect(),
                aggressive_args: vec!["--threads=50"].iter().map(|s| s.to_string()).collect(),
                description: "Multi-purpose brute-forcer".to_string(),
            },
            AttackTool {
                name: "medusa".to_string(),
                command: "medusa".to_string(),
                stealth_args: vec!["-t", "1", "-T", "1"].iter().map(|s| s.to_string()).collect(),
                aggressive_args: vec!["-t", "10", "-T", "10"].iter().map(|s| s.to_string()).collect(),
                description: "Parallel password cracker".to_string(),
            },
            AttackTool {
                name: "ncrack".to_string(),
                command: "ncrack".to_string(),
                stealth_args: vec!["--connection-limit", "1"].iter().map(|s| s.to_string()).collect(),
                aggressive_args: vec!["--connection-limit", "10", "-T5"].iter().map(|s| s.to_string()).collect(),
                description: "Network authentication cracker".to_string(),
            },
        ],
        stealth_first: true,
        async_execution: true,
        memory_limit_mb: ASYNC_THREAD_MEMORY_LIMIT_MB,
        category: AttackCategory::PasswordAttacks,
    });

    // SCAN - Network scanning sequence
    sequences.insert("scan".to_string(), SmartAttackSequence {
        keyword: "scan".to_string(),
        description: "Comprehensive network scanning and enumeration".to_string(),
        tools: vec![
            AttackTool {
                name: "nmap".to_string(),
                command: "nmap".to_string(),
                stealth_args: vec!["-sS", "-T2", "-Pn", "--max-retries", "1", "-f"].iter().map(|s| s.to_string()).collect(),
                aggressive_args: vec!["-sV", "-sC", "-A", "-T4", "-p-", "--script", "vuln"].iter().map(|s| s.to_string()).collect(),
                description: "Network mapper and port scanner".to_string(),
            },
            AttackTool {
                name: "masscan".to_string(),
                command: "masscan".to_string(),
                stealth_args: vec!["--rate", "100", "-p", "80,443,22,21"].iter().map(|s| s.to_string()).collect(),
                aggressive_args: vec!["--rate", "10000", "-p", "0-65535"].iter().map(|s| s.to_string()).collect(),
                description: "Mass IP port scanner".to_string(),
            },
            AttackTool {
                name: "rustscan".to_string(),
                command: "rustscan".to_string(),
                stealth_args: vec!["--ulimit", "500", "-b", "100"].iter().map(|s| s.to_string()).collect(),
                aggressive_args: vec!["--ulimit", "5000", "-b", "1000", "--"].iter().map(|s| s.to_string()).collect(),
                description: "Modern fast port scanner".to_string(),
            },
            AttackTool {
                name: "nikto".to_string(),
                command: "nikto".to_string(),
                stealth_args: vec!["-Tuning", "1", "-evasion", "1"].iter().map(|s| s.to_string()).collect(),
                aggressive_args: vec!["-Tuning", "x", "-C", "all"].iter().map(|s| s.to_string()).collect(),
                description: "Web server scanner".to_string(),
            },
            AttackTool {
                name: "nuclei".to_string(),
                command: "nuclei".to_string(),
                stealth_args: vec!["-rl", "10", "-c", "5", "-silent"].iter().map(|s| s.to_string()).collect(),
                aggressive_args: vec!["-rl", "150", "-c", "50", "-severity", "critical,high,medium"].iter().map(|s| s.to_string()).collect(),
                description: "Vulnerability scanner with templates".to_string(),
            },
        ],
        stealth_first: true,
        async_execution: true,
        memory_limit_mb: ASYNC_THREAD_MEMORY_LIMIT_MB,
        category: AttackCategory::Scanning,
    });

    // RECON - Reconnaissance sequence
    sequences.insert("recon".to_string(), SmartAttackSequence {
        keyword: "recon".to_string(),
        description: "Information gathering and reconnaissance".to_string(),
        tools: vec![
            AttackTool {
                name: "theHarvester".to_string(),
                command: "theHarvester".to_string(),
                stealth_args: vec!["-b", "google", "-l", "100"].iter().map(|s| s.to_string()).collect(),
                aggressive_args: vec!["-b", "all", "-l", "500", "-g"].iter().map(|s| s.to_string()).collect(),
                description: "Email and subdomain harvester".to_string(),
            },
            AttackTool {
                name: "amass".to_string(),
                command: "amass".to_string(),
                stealth_args: vec!["enum", "-passive"].iter().map(|s| s.to_string()).collect(),
                aggressive_args: vec!["enum", "-active", "-brute", "-w", "/usr/share/wordlists/subdomains.txt"].iter().map(|s| s.to_string()).collect(),
                description: "Subdomain enumeration".to_string(),
            },
            AttackTool {
                name: "subfinder".to_string(),
                command: "subfinder".to_string(),
                stealth_args: vec!["-silent", "-t", "10"].iter().map(|s| s.to_string()).collect(),
                aggressive_args: vec!["-all", "-t", "100"].iter().map(|s| s.to_string()).collect(),
                description: "Subdomain discovery tool".to_string(),
            },
            AttackTool {
                name: "dnsrecon".to_string(),
                command: "dnsrecon".to_string(),
                stealth_args: vec!["-t", "std"].iter().map(|s| s.to_string()).collect(),
                aggressive_args: vec!["-t", "brt", "-D", "/usr/share/wordlists/dns.txt"].iter().map(|s| s.to_string()).collect(),
                description: "DNS enumeration".to_string(),
            },
            AttackTool {
                name: "whois".to_string(),
                command: "whois".to_string(),
                stealth_args: vec![].iter().map(|s: &&str| s.to_string()).collect(),
                aggressive_args: vec![].iter().map(|s: &&str| s.to_string()).collect(),
                description: "WHOIS lookup".to_string(),
            },
        ],
        stealth_first: true,
        async_execution: true,
        memory_limit_mb: ASYNC_THREAD_MEMORY_LIMIT_MB,
        category: AttackCategory::Reconnaissance,
    });

    // SOCIAL - Social engineering and OSINT
    sequences.insert("social".to_string(), SmartAttackSequence {
        keyword: "social".to_string(),
        description: "Social engineering reconnaissance and OSINT".to_string(),
        tools: vec![
            AttackTool {
                name: "sherlock".to_string(),
                command: "sherlock".to_string(),
                stealth_args: vec!["--timeout", "30"].iter().map(|s| s.to_string()).collect(),
                aggressive_args: vec!["--timeout", "10", "--print-all"].iter().map(|s| s.to_string()).collect(),
                description: "Username enumeration across platforms".to_string(),
            },
            AttackTool {
                name: "maltego".to_string(),
                command: "maltego".to_string(),
                stealth_args: vec![].iter().map(|s: &&str| s.to_string()).collect(),
                aggressive_args: vec![].iter().map(|s: &&str| s.to_string()).collect(),
                description: "OSINT and graphical link analysis".to_string(),
            },
            AttackTool {
                name: "recon-ng".to_string(),
                command: "recon-ng".to_string(),
                stealth_args: vec![].iter().map(|s: &&str| s.to_string()).collect(),
                aggressive_args: vec![].iter().map(|s: &&str| s.to_string()).collect(),
                description: "Web reconnaissance framework".to_string(),
            },
            AttackTool {
                name: "spiderfoot".to_string(),
                command: "spiderfoot".to_string(),
                stealth_args: vec!["-s"].iter().map(|s| s.to_string()).collect(),
                aggressive_args: vec!["-m", "all"].iter().map(|s| s.to_string()).collect(),
                description: "OSINT automation tool".to_string(),
            },
            AttackTool {
                name: "holehe".to_string(),
                command: "holehe".to_string(),
                stealth_args: vec![].iter().map(|s: &&str| s.to_string()).collect(),
                aggressive_args: vec![].iter().map(|s: &&str| s.to_string()).collect(),
                description: "Email to registered accounts".to_string(),
            },
        ],
        stealth_first: true,
        async_execution: false,
        memory_limit_mb: SEQUENTIAL_THREAD_MEMORY_LIMIT_MB,
        category: AttackCategory::SocialEngineering,
    });

    // WEB - Web application testing
    sequences.insert("web".to_string(), SmartAttackSequence {
        keyword: "web".to_string(),
        description: "Web application vulnerability testing".to_string(),
        tools: vec![
            AttackTool {
                name: "nikto".to_string(),
                command: "nikto".to_string(),
                stealth_args: vec!["-Tuning", "1", "-evasion", "1"].iter().map(|s| s.to_string()).collect(),
                aggressive_args: vec!["-Tuning", "x", "-C", "all"].iter().map(|s| s.to_string()).collect(),
                description: "Web server scanner".to_string(),
            },
            AttackTool {
                name: "gobuster".to_string(),
                command: "gobuster".to_string(),
                stealth_args: vec!["dir", "-t", "5", "-q"].iter().map(|s| s.to_string()).collect(),
                aggressive_args: vec!["dir", "-t", "50", "-e", "-x", "php,html,js,txt"].iter().map(|s| s.to_string()).collect(),
                description: "Directory/file brute-forcing".to_string(),
            },
            AttackTool {
                name: "ffuf".to_string(),
                command: "ffuf".to_string(),
                stealth_args: vec!["-rate", "10", "-t", "5"].iter().map(|s| s.to_string()).collect(),
                aggressive_args: vec!["-rate", "0", "-t", "100", "-recursion"].iter().map(|s| s.to_string()).collect(),
                description: "Fast web fuzzer".to_string(),
            },
            AttackTool {
                name: "sqlmap".to_string(),
                command: "sqlmap".to_string(),
                stealth_args: vec!["--level", "1", "--risk", "1", "--random-agent", "--delay", "2"].iter().map(|s| s.to_string()).collect(),
                aggressive_args: vec!["--level", "5", "--risk", "3", "--batch", "--threads", "10"].iter().map(|s| s.to_string()).collect(),
                description: "SQL injection automation".to_string(),
            },
            AttackTool {
                name: "wpscan".to_string(),
                command: "wpscan".to_string(),
                stealth_args: vec!["--stealthy"].iter().map(|s| s.to_string()).collect(),
                aggressive_args: vec!["--enumerate", "vp,vt,u", "--plugins-detection", "aggressive"].iter().map(|s| s.to_string()).collect(),
                description: "WordPress vulnerability scanner".to_string(),
            },
            AttackTool {
                name: "xsstrike".to_string(),
                command: "xsstrike".to_string(),
                stealth_args: vec!["--crawl", "-l", "2"].iter().map(|s| s.to_string()).collect(),
                aggressive_args: vec!["--crawl", "-l", "5", "--blind"].iter().map(|s| s.to_string()).collect(),
                description: "XSS detection suite".to_string(),
            },
        ],
        stealth_first: true,
        async_execution: true,
        memory_limit_mb: ASYNC_THREAD_MEMORY_LIMIT_MB,
        category: AttackCategory::WebApplications,
    });

    // WIRELESS - Wireless attacks
    sequences.insert("wireless".to_string(), SmartAttackSequence {
        keyword: "wireless".to_string(),
        description: "Wireless network attacks and auditing".to_string(),
        tools: vec![
            AttackTool {
                name: "aircrack-ng".to_string(),
                command: "aircrack-ng".to_string(),
                stealth_args: vec![].iter().map(|s: &&str| s.to_string()).collect(),
                aggressive_args: vec!["-b"].iter().map(|s| s.to_string()).collect(),
                description: "WiFi security auditing suite".to_string(),
            },
            AttackTool {
                name: "wifite".to_string(),
                command: "wifite".to_string(),
                stealth_args: vec!["--kill", "--no-wps"].iter().map(|s| s.to_string()).collect(),
                aggressive_args: vec!["--kill", "--all", "--wps", "--pmkid"].iter().map(|s| s.to_string()).collect(),
                description: "Automated wireless auditor".to_string(),
            },
            AttackTool {
                name: "reaver".to_string(),
                command: "reaver".to_string(),
                stealth_args: vec!["-d", "5", "-l", "30"].iter().map(|s| s.to_string()).collect(),
                aggressive_args: vec!["-vv", "-N", "-d", "0"].iter().map(|s| s.to_string()).collect(),
                description: "WPS brute-force attack".to_string(),
            },
            AttackTool {
                name: "bully".to_string(),
                command: "bully".to_string(),
                stealth_args: vec!["-d"].iter().map(|s| s.to_string()).collect(),
                aggressive_args: vec!["-v", "3"].iter().map(|s| s.to_string()).collect(),
                description: "WPS brute-force attack".to_string(),
            },
            AttackTool {
                name: "kismet".to_string(),
                command: "kismet".to_string(),
                stealth_args: vec!["--no-logging"].iter().map(|s| s.to_string()).collect(),
                aggressive_args: vec![].iter().map(|s: &&str| s.to_string()).collect(),
                description: "Wireless network detector".to_string(),
            },
        ],
        stealth_first: true,
        async_execution: false,
        memory_limit_mb: SEQUENTIAL_THREAD_MEMORY_LIMIT_MB,
        category: AttackCategory::WirelessAttacks,
    });

    // OAUTH - OAuth2 security testing
    sequences.insert("oauth".to_string(), SmartAttackSequence {
        keyword: "oauth".to_string(),
        description: "OAuth2 and authentication security testing".to_string(),
        tools: vec![
            AttackTool {
                name: "burpsuite".to_string(),
                command: "burpsuite".to_string(),
                stealth_args: vec![].iter().map(|s: &&str| s.to_string()).collect(),
                aggressive_args: vec![].iter().map(|s: &&str| s.to_string()).collect(),
                description: "Web security testing platform".to_string(),
            },
            AttackTool {
                name: "evilginx2".to_string(),
                command: "evilginx2".to_string(),
                stealth_args: vec![].iter().map(|s: &&str| s.to_string()).collect(),
                aggressive_args: vec![].iter().map(|s: &&str| s.to_string()).collect(),
                description: "Advanced phishing framework".to_string(),
            },
            AttackTool {
                name: "modlishka".to_string(),
                command: "modlishka".to_string(),
                stealth_args: vec![].iter().map(|s: &&str| s.to_string()).collect(),
                aggressive_args: vec![].iter().map(|s: &&str| s.to_string()).collect(),
                description: "Reverse proxy for 2FA phishing".to_string(),
            },
            AttackTool {
                name: "mitmproxy".to_string(),
                command: "mitmproxy".to_string(),
                stealth_args: vec![].iter().map(|s: &&str| s.to_string()).collect(),
                aggressive_args: vec![].iter().map(|s: &&str| s.to_string()).collect(),
                description: "Interactive HTTPS proxy".to_string(),
            },
        ],
        stealth_first: true,
        async_execution: false,
        memory_limit_mb: SEQUENTIAL_THREAD_MEMORY_LIMIT_MB,
        category: AttackCategory::WebApplications,
    });

    // DATABASE - Database exploitation
    sequences.insert("database".to_string(), SmartAttackSequence {
        keyword: "database".to_string(),
        description: "Database exploitation and enumeration".to_string(),
        tools: vec![
            AttackTool {
                name: "sqlmap".to_string(),
                command: "sqlmap".to_string(),
                stealth_args: vec!["--level", "1", "--risk", "1", "--random-agent", "--delay", "2"].iter().map(|s| s.to_string()).collect(),
                aggressive_args: vec!["--level", "5", "--risk", "3", "--batch", "--threads", "10", "--dump-all"].iter().map(|s| s.to_string()).collect(),
                description: "SQL injection automation".to_string(),
            },
            AttackTool {
                name: "odat".to_string(),
                command: "odat".to_string(),
                stealth_args: vec!["all", "-s"].iter().map(|s| s.to_string()).collect(),
                aggressive_args: vec!["all", "-s", "--sysdba"].iter().map(|s| s.to_string()).collect(),
                description: "Oracle database attacking tool".to_string(),
            },
            AttackTool {
                name: "mssqlclient".to_string(),
                command: "mssqlclient.py".to_string(),
                stealth_args: vec![].iter().map(|s: &&str| s.to_string()).collect(),
                aggressive_args: vec!["-windows-auth"].iter().map(|s| s.to_string()).collect(),
                description: "MSSQL client for pentesting".to_string(),
            },
            AttackTool {
                name: "mongoaudit".to_string(),
                command: "mongoaudit".to_string(),
                stealth_args: vec![].iter().map(|s: &&str| s.to_string()).collect(),
                aggressive_args: vec![].iter().map(|s: &&str| s.to_string()).collect(),
                description: "MongoDB security auditing".to_string(),
            },
        ],
        stealth_first: true,
        async_execution: true,
        memory_limit_mb: ASYNC_THREAD_MEMORY_LIMIT_MB,
        category: AttackCategory::Exploitation,
    });

    // FORENSIC - Digital forensics
    sequences.insert("forensic".to_string(), SmartAttackSequence {
        keyword: "forensic".to_string(),
        description: "Digital forensics and analysis".to_string(),
        tools: vec![
            AttackTool {
                name: "autopsy".to_string(),
                command: "autopsy".to_string(),
                stealth_args: vec![].iter().map(|s: &&str| s.to_string()).collect(),
                aggressive_args: vec![].iter().map(|s: &&str| s.to_string()).collect(),
                description: "Digital forensics platform".to_string(),
            },
            AttackTool {
                name: "volatility".to_string(),
                command: "volatility".to_string(),
                stealth_args: vec![].iter().map(|s: &&str| s.to_string()).collect(),
                aggressive_args: vec![].iter().map(|s: &&str| s.to_string()).collect(),
                description: "Memory forensics framework".to_string(),
            },
            AttackTool {
                name: "binwalk".to_string(),
                command: "binwalk".to_string(),
                stealth_args: vec![].iter().map(|s: &&str| s.to_string()).collect(),
                aggressive_args: vec!["-e", "-M"].iter().map(|s| s.to_string()).collect(),
                description: "Firmware analysis tool".to_string(),
            },
            AttackTool {
                name: "foremost".to_string(),
                command: "foremost".to_string(),
                stealth_args: vec![].iter().map(|s: &&str| s.to_string()).collect(),
                aggressive_args: vec!["-a"].iter().map(|s| s.to_string()).collect(),
                description: "File carving tool".to_string(),
            },
            AttackTool {
                name: "bulk_extractor".to_string(),
                command: "bulk_extractor".to_string(),
                stealth_args: vec![].iter().map(|s: &&str| s.to_string()).collect(),
                aggressive_args: vec!["-x", "all"].iter().map(|s| s.to_string()).collect(),
                description: "Extract useful information".to_string(),
            },
        ],
        stealth_first: false,
        async_execution: false,
        memory_limit_mb: SEQUENTIAL_THREAD_MEMORY_LIMIT_MB,
        category: AttackCategory::Forensics,
    });

    // EXPLOIT - Exploitation tools
    sequences.insert("exploit".to_string(), SmartAttackSequence {
        keyword: "exploit".to_string(),
        description: "Exploitation and payload delivery".to_string(),
        tools: vec![
            AttackTool {
                name: "metasploit".to_string(),
                command: "msfconsole".to_string(),
                stealth_args: vec!["-q", "-x", "set VERBOSE false"].iter().map(|s| s.to_string()).collect(),
                aggressive_args: vec!["-q"].iter().map(|s| s.to_string()).collect(),
                description: "Exploitation framework".to_string(),
            },
            AttackTool {
                name: "searchsploit".to_string(),
                command: "searchsploit".to_string(),
                stealth_args: vec![].iter().map(|s: &&str| s.to_string()).collect(),
                aggressive_args: vec!["-x"].iter().map(|s| s.to_string()).collect(),
                description: "Exploit database search".to_string(),
            },
            AttackTool {
                name: "msfvenom".to_string(),
                command: "msfvenom".to_string(),
                stealth_args: vec!["-e", "x86/shikata_ga_nai", "-i", "5"].iter().map(|s| s.to_string()).collect(),
                aggressive_args: vec![].iter().map(|s: &&str| s.to_string()).collect(),
                description: "Payload generator".to_string(),
            },
        ],
        stealth_first: true,
        async_execution: false,
        memory_limit_mb: SEQUENTIAL_THREAD_MEMORY_LIMIT_MB,
        category: AttackCategory::Exploitation,
    });

    // PRIVESC - Privilege escalation
    sequences.insert("privesc".to_string(), SmartAttackSequence {
        keyword: "privesc".to_string(),
        description: "Privilege escalation techniques".to_string(),
        tools: vec![
            AttackTool {
                name: "linpeas".to_string(),
                command: "linpeas.sh".to_string(),
                stealth_args: vec!["-q"].iter().map(|s| s.to_string()).collect(),
                aggressive_args: vec!["-a"].iter().map(|s| s.to_string()).collect(),
                description: "Linux privilege escalation".to_string(),
            },
            AttackTool {
                name: "winpeas".to_string(),
                command: "winpeas.exe".to_string(),
                stealth_args: vec!["quiet"].iter().map(|s| s.to_string()).collect(),
                aggressive_args: vec!["all"].iter().map(|s| s.to_string()).collect(),
                description: "Windows privilege escalation".to_string(),
            },
            AttackTool {
                name: "pspy".to_string(),
                command: "pspy".to_string(),
                stealth_args: vec![].iter().map(|s: &&str| s.to_string()).collect(),
                aggressive_args: vec!["-f"].iter().map(|s| s.to_string()).collect(),
                description: "Process snooping".to_string(),
            },
            AttackTool {
                name: "linux-exploit-suggester".to_string(),
                command: "linux-exploit-suggester.sh".to_string(),
                stealth_args: vec![].iter().map(|s: &&str| s.to_string()).collect(),
                aggressive_args: vec![].iter().map(|s: &&str| s.to_string()).collect(),
                description: "Kernel exploit suggester".to_string(),
            },
        ],
        stealth_first: true,
        async_execution: false,
        memory_limit_mb: SEQUENTIAL_THREAD_MEMORY_LIMIT_MB,
        category: AttackCategory::PrivilegeEscalation,
    });

    // SHELL - Reverse shell and C2
    sequences.insert("shell".to_string(), SmartAttackSequence {
        keyword: "shell".to_string(),
        description: "Reverse shell and command & control".to_string(),
        tools: vec![
            AttackTool {
                name: "netcat".to_string(),
                command: "nc".to_string(),
                stealth_args: vec!["-lvnp"].iter().map(|s| s.to_string()).collect(),
                aggressive_args: vec!["-lvnp"].iter().map(|s| s.to_string()).collect(),
                description: "Network utility for shells".to_string(),
            },
            AttackTool {
                name: "socat".to_string(),
                command: "socat".to_string(),
                stealth_args: vec![].iter().map(|s: &&str| s.to_string()).collect(),
                aggressive_args: vec![].iter().map(|s: &&str| s.to_string()).collect(),
                description: "Advanced netcat alternative".to_string(),
            },
            AttackTool {
                name: "pwncat".to_string(),
                command: "pwncat".to_string(),
                stealth_args: vec![].iter().map(|s: &&str| s.to_string()).collect(),
                aggressive_args: vec![].iter().map(|s: &&str| s.to_string()).collect(),
                description: "Post-exploitation platform".to_string(),
            },
        ],
        stealth_first: true,
        async_execution: false,
        memory_limit_mb: SEQUENTIAL_THREAD_MEMORY_LIMIT_MB,
        category: AttackCategory::PostExploitation,
    });

    // SNIFF - Network sniffing and MITM
    sequences.insert("sniff".to_string(), SmartAttackSequence {
        keyword: "sniff".to_string(),
        description: "Network sniffing and man-in-the-middle".to_string(),
        tools: vec![
            AttackTool {
                name: "wireshark".to_string(),
                command: "wireshark".to_string(),
                stealth_args: vec!["-k"].iter().map(|s| s.to_string()).collect(),
                aggressive_args: vec!["-k"].iter().map(|s| s.to_string()).collect(),
                description: "Network protocol analyzer".to_string(),
            },
            AttackTool {
                name: "tcpdump".to_string(),
                command: "tcpdump".to_string(),
                stealth_args: vec!["-q", "-n"].iter().map(|s| s.to_string()).collect(),
                aggressive_args: vec!["-vvv", "-X"].iter().map(|s| s.to_string()).collect(),
                description: "Command-line packet analyzer".to_string(),
            },
            AttackTool {
                name: "ettercap".to_string(),
                command: "ettercap".to_string(),
                stealth_args: vec!["-T", "-q"].iter().map(|s| s.to_string()).collect(),
                aggressive_args: vec!["-T", "-M", "arp:remote"].iter().map(|s| s.to_string()).collect(),
                description: "MITM attack tool".to_string(),
            },
            AttackTool {
                name: "bettercap".to_string(),
                command: "bettercap".to_string(),
                stealth_args: vec!["--silent"].iter().map(|s| s.to_string()).collect(),
                aggressive_args: vec![].iter().map(|s: &&str| s.to_string()).collect(),
                description: "Network attack framework".to_string(),
            },
            AttackTool {
                name: "responder".to_string(),
                command: "responder".to_string(),
                stealth_args: vec!["-A"].iter().map(|s| s.to_string()).collect(),
                aggressive_args: vec!["-wrf"].iter().map(|s| s.to_string()).collect(),
                description: "LLMNR/NBT-NS/MDNS poisoner".to_string(),
            },
        ],
        stealth_first: true,
        async_execution: false,
        memory_limit_mb: SEQUENTIAL_THREAD_MEMORY_LIMIT_MB,
        category: AttackCategory::Reconnaissance,
    });

    sequences
}

// ============================================================================
// USER INPUT STRUCTURE
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserInput {
    pub subject: Option<String>,      // Target (IP, domain, email, etc.)
    pub keywords: Vec<String>,        // Attack keywords to trigger sequences
    pub context: Option<String>,      // Optional strategy/instructions
    pub original_text: String,        // Original user input
    pub translated_text: Option<String>, // Translated if not English
    pub confidence: f32,
}

impl UserInput {
    pub fn new() -> Self {
        UserInput {
            subject: None,
            keywords: Vec::new(),
            context: None,
            original_text: String::new(),
            translated_text: None,
            confidence: 0.0,
        }
    }

    pub fn is_complete(&self) -> bool {
        self.subject.is_some() && !self.keywords.is_empty()
    }

    pub fn missing_parts(&self) -> Vec<String> {
        let mut missing = Vec::new();
        if self.subject.is_none() {
            missing.push("Subject (target IP, domain, email, or username)".to_string());
        }
        if self.keywords.is_empty() {
            missing.push("Keywords (attack types like: password, scan, web, social, etc.)".to_string());
        }
        missing
    }
}

// ============================================================================
// ATTACK EXECUTOR WITH MEMORY LIMITS
// ============================================================================

pub struct SmartAttackExecutor {
    semaphore: Arc<Semaphore>,
    async_memory_used: Arc<AtomicU64>,
    sequential_memory_used: Arc<AtomicU64>,
    http_client: Client,
}

impl SmartAttackExecutor {
    pub fn new() -> Self {
        SmartAttackExecutor {
            semaphore: Arc::new(Semaphore::new(MAX_CONCURRENT_ASYNC_TASKS)),
            async_memory_used: Arc::new(AtomicU64::new(0)),
            sequential_memory_used: Arc::new(AtomicU64::new(0)),
            http_client: Client::new(),
        }
    }

    /// Execute stealth scan before any attack
    pub async fn execute_stealth_scan(&self, target: &str) -> Result<String, String> {
        println!("🐺 FENRIR STEALTH SCAN PHASE");
        println!("   Target: {}", target);
        println!("   Mode: Stealth (-sS -T2 -Pn --max-retries 1)");

        let target_owned = target.to_string();
        let result = task::spawn_blocking(move || {
            let output = Command::new("nmap")
                .args(&["-sS", "-T2", "-Pn", "--max-retries", "1", "-f", &target_owned])
                .stdout(Stdio::piped())
                .stderr(Stdio::piped())
                .output();

            match output {
                Ok(out) => {
                    if out.status.success() {
                        let stdout = String::from_utf8_lossy(&out.stdout).to_string();
                        Ok(format!("✅ Stealth scan completed\n{}", stdout))
                    } else {
                        Err(format!("Stealth scan failed: {}", String::from_utf8_lossy(&out.stderr)))
                    }
                }
                Err(e) => Err(format!("Stealth scan error: {}", e)),
            }
        }).await.map_err(|e| format!("Task error: {}", e))?;

        result
    }

    /// Execute aggressive scan as fallback
    pub async fn execute_aggressive_scan(&self, target: &str) -> Result<String, String> {
        println!("⚠️  FENRIR AGGRESSIVE SCAN PHASE");
        println!("   Target: {}", target);
        println!("   Mode: Aggressive (-sV -sC -A -T4 -p-)");

        let target_owned = target.to_string();
        let result = task::spawn_blocking(move || {
            let output = Command::new("nmap")
                .args(&["-sV", "-sC", "-A", "-T4", "-p-", &target_owned])
                .stdout(Stdio::piped())
                .stderr(Stdio::piped())
                .output();

            match output {
                Ok(out) => {
                    if out.status.success() {
                        let stdout = String::from_utf8_lossy(&out.stdout).to_string();
                        Ok(format!("✅ Aggressive scan completed\n{}", stdout))
                    } else {
                        Err(format!("Aggressive scan failed: {}", String::from_utf8_lossy(&out.stderr)))
                    }
                }
                Err(e) => Err(format!("Aggressive scan error: {}", e)),
            }
        }).await.map_err(|e| format!("Task error: {}", e))?;

        result
    }

    /// Execute a single tool with memory monitoring
    async fn execute_tool(&self, tool: &AttackTool, target: &str, stealth: bool) -> String {
        let args = if stealth { &tool.stealth_args } else { &tool.aggressive_args };
        let mut full_args: Vec<String> = args.clone();
        
        // Add target to args if not empty
        if !target.is_empty() {
            full_args.push(target.to_string());
        }

        let tool_name = tool.name.clone();
        let command = tool.command.clone();
        let description = tool.description.clone();

        // Check if tool is available
        let check = Command::new("which")
            .arg(&command)
            .output();

        match check {
            Ok(output) if output.status.success() => {
                format!("   ✅ {} - {} [Available]", tool_name, description)
            }
            _ => {
                format!("   ❌ {} - {} [Not installed]", tool_name, description)
            }
        }
    }

    /// Execute async attack sequence with memory limit
    pub async fn execute_async_sequence(&self, sequence: &SmartAttackSequence, target: &str) -> Vec<String> {
        let mut results = Vec::new();
        let permit = self.semaphore.acquire().await.unwrap();

        results.push(format!("🚀 ASYNC: {} Sequence ({}MB limit)", sequence.keyword.to_uppercase(), sequence.memory_limit_mb));
        results.push(format!("   Description: {}", sequence.description));

        // Execute tools in parallel
        let mut handles = Vec::new();
        for tool in &sequence.tools {
            let tool_clone = tool.clone();
            let target_clone = target.to_string();
            let stealth = sequence.stealth_first;

            let handle = task::spawn(async move {
                let args = if stealth { &tool_clone.stealth_args } else { &tool_clone.aggressive_args };
                let check = Command::new("which")
                    .arg(&tool_clone.command)
                    .output();

                match check {
                    Ok(output) if output.status.success() => {
                        format!("   ✅ {} - {} [Ready]", tool_clone.name, tool_clone.description)
                    }
                    _ => {
                        format!("   ❌ {} - {} [Not installed]", tool_clone.name, tool_clone.description)
                    }
                }
            });
            handles.push(handle);
        }

        let tool_results = join_all(handles).await;
        for result in tool_results {
            if let Ok(r) = result {
                results.push(r);
            }
        }

        drop(permit);
        results
    }

    /// Execute sequential attack sequence with higher memory limit
    pub async fn execute_sequential_sequence(&self, sequence: &SmartAttackSequence, target: &str) -> Vec<String> {
        let mut results = Vec::new();

        results.push(format!("🔄 SEQUENTIAL: {} Sequence ({}MB limit)", sequence.keyword.to_uppercase(), sequence.memory_limit_mb));
        results.push(format!("   Description: {}", sequence.description));

        for tool in &sequence.tools {
            let result = self.execute_tool(tool, target, sequence.stealth_first).await;
            results.push(result);
        }

        results
    }

    /// Main execution entry point
    pub async fn execute_smart_attack(&self, input: &UserInput) -> Vec<String> {
        let mut all_results = Vec::new();
        let sequences = get_all_smart_sequences();
        let target = input.subject.as_deref().unwrap_or("localhost");

        // Phase 1: Stealth Scan (always first, unless forensic)
        if !input.keywords.iter().any(|k| k == "forensic") {
            match self.execute_stealth_scan(target).await {
                Ok(result) => all_results.push(result),
                Err(e) => {
                    all_results.push(format!("⚠️  Stealth scan failed: {}", e));
                    all_results.push("   Switching to aggressive mode...".to_string());
                    match self.execute_aggressive_scan(target).await {
                        Ok(result) => all_results.push(result),
                        Err(e) => all_results.push(format!("❌ Aggressive scan also failed: {}", e)),
                    }
                }
            }
        }

        // Phase 2: Execute triggered sequences
        let mut async_sequences = Vec::new();
        let mut sequential_sequences = Vec::new();

        for keyword in &input.keywords {
            if let Some(sequence) = sequences.get(keyword) {
                if sequence.async_execution {
                    async_sequences.push(sequence.clone());
                } else {
                    sequential_sequences.push(sequence.clone());
                }
            } else {
                all_results.push(format!("⚠️  Unknown keyword: {} - No sequence found", keyword));
            }
        }

        // Execute async sequences in parallel
        if !async_sequences.is_empty() {
            all_results.push("\n═══════════════════════════════════════════════════════════".to_string());
            all_results.push("🚀 ASYNC ATTACK PHASE - Parallel Execution".to_string());
            all_results.push("═══════════════════════════════════════════════════════════".to_string());

            let mut handles = Vec::new();
            for sequence in async_sequences {
                let target_clone = target.to_string();
                let executor = SmartAttackExecutor::new();
                let handle = task::spawn(async move {
                    executor.execute_async_sequence(&sequence, &target_clone).await
                });
                handles.push(handle);
            }

            let results = join_all(handles).await;
            for result in results {
                if let Ok(r) = result {
                    all_results.extend(r);
                }
            }
        }

        // Execute sequential sequences one by one
        if !sequential_sequences.is_empty() {
            all_results.push("\n═══════════════════════════════════════════════════════════".to_string());
            all_results.push("🔄 SEQUENTIAL ATTACK PHASE - One by One".to_string());
            all_results.push("═══════════════════════════════════════════════════════════".to_string());

            for sequence in sequential_sequences {
                let results = self.execute_sequential_sequence(&sequence, target).await;
                all_results.extend(results);
            }
        }

        all_results
    }
}

// ============================================================================
// INTERACTIVE MODE - MAIN ENTRY POINT
// ============================================================================

pub async fn run_interactive_mode() -> Result<(), Box<dyn std::error::Error>> {
    // Load environment variables
    fenrir_ai_layer::load_env();

    let http_client = Client::new();
    let executor = SmartAttackExecutor::new();
    let stdin = io::stdin();

    // Clear screen and show banner
    print_banner();

    loop {
        // Get user input
        print!("\n🐺 fenrir> ");
        io::stdout().flush()?;

        let mut input_line = String::new();
        match stdin.read_line(&mut input_line) {
            Ok(0) => {
                println!("\n🐺 Goodbye!");
                break;
            }
            Ok(_) => {
                let user_text = input_line.trim();
                if user_text.is_empty() {
                    continue;
                }

                // Handle exit commands
                if matches!(user_text.to_lowercase().as_str(), "exit" | "quit" | "q" | "sair") {
                    println!("\n🐺 Goodbye!");
                    break;
                }

                // Handle help command
                if matches!(user_text.to_lowercase().as_str(), "help" | "?" | "h") {
                    print_help();
                    continue;
                }

                // Handle keywords command
                if matches!(user_text.to_lowercase().as_str(), "keywords" | "k") {
                    print_keywords();
                    continue;
                }

                // Process input through NLP
                println!("\n🔍 Processing input...");

                match nlp::parse_command(&http_client, user_text).await {
                    Ok(parsed) => {
                        // Display interpretation
                        println!("\n╔══════════════════════════════════════════════════════════╗");
                        println!("║              🤖 ZAI INTERPRETATION                       ║");
                        println!("╠══════════════════════════════════════════════════════════╣");
                        println!("║ 📍 Subject:    {:<42} ║", parsed.subject.as_deref().unwrap_or("None"));
                        println!("║ 🎯 Keywords:   {:<42} ║", parsed.keywords.join(", "));
                        println!("║ 📋 Context:    {:<42} ║", truncate_str(parsed.context.as_deref().unwrap_or("None"), 42));
                        println!("║ 📊 Confidence: {:<42} ║", format!("{:.1}%", parsed.confidence * 100.0));
                        println!("╚══════════════════════════════════════════════════════════╝");

                        // Check if clarification needed
                        if parsed.needs_clarification {
                            println!("\n❓ Clarification needed:");
                            for question in &parsed.clarification_questions {
                                println!("   • {}", question);
                            }
                            println!("\n💡 Please provide more details and try again.");
                            continue;
                        }

                        // Show triggered sequences
                        println!("\n╔══════════════════════════════════════════════════════════╗");
                        println!("║           🎯 TRIGGERED ATTACK SEQUENCES                  ║");
                        println!("╠══════════════════════════════════════════════════════════╣");

                        let sequences = get_all_smart_sequences();
                        for keyword in &parsed.keywords {
                            if let Some(seq) = sequences.get(keyword) {
                                println!("║ 🔑 {} → {:<40} ║", keyword.to_uppercase(), seq.description);
                                println!("║    Tools: {:<47} ║", 
                                    seq.tools.iter().map(|t| t.name.clone()).collect::<Vec<_>>().join(", "));
                                println!("║    Mode: {} | Memory: {}MB{:<20} ║",
                                    if seq.async_execution { "Async" } else { "Sequential" },
                                    seq.memory_limit_mb,
                                    if seq.stealth_first { " | Stealth First" } else { "" }
                                );
                                println!("╟──────────────────────────────────────────────────────────╢");
                            }
                        }
                        println!("╚══════════════════════════════════════════════════════════╝");

                        // Double-check with user
                        print!("\n❓ Is this interpretation correct? (yes/no/edit): ");
                        io::stdout().flush()?;

                        let mut confirmation = String::new();
                        stdin.read_line(&mut confirmation)?;
                        let confirm = confirmation.trim().to_lowercase();

                        if confirm == "edit" || confirm == "e" {
                            println!("\n📝 Edit mode - Enter corrected values:");
                            
                            // Edit subject
                            print!("   Subject [{}]: ", parsed.subject.as_deref().unwrap_or(""));
                            io::stdout().flush()?;
                            let mut new_subject = String::new();
                            stdin.read_line(&mut new_subject)?;
                            let new_subject = new_subject.trim();

                            // Edit keywords
                            print!("   Keywords [{}]: ", parsed.keywords.join(", "));
                            io::stdout().flush()?;
                            let mut new_keywords = String::new();
                            stdin.read_line(&mut new_keywords)?;
                            let new_keywords = new_keywords.trim();

                            // Create updated input
                            let updated_input = UserInput {
                                subject: if new_subject.is_empty() { parsed.subject.clone() } else { Some(new_subject.to_string()) },
                                keywords: if new_keywords.is_empty() { 
                                    parsed.keywords.clone() 
                                } else { 
                                    new_keywords.split(',').map(|s| s.trim().to_lowercase()).collect() 
                                },
                                context: parsed.context.clone(),
                                original_text: user_text.to_string(),
                                translated_text: None,
                                confidence: 1.0,
                            };

                            // Execute with updated input
                            println!("\n🚀 Executing smart attack sequence with updated parameters...\n");
                            let results = executor.execute_smart_attack(&updated_input).await;
                            for result in results {
                                println!("{}", result);
                            }
                        } else if confirm == "yes" || confirm == "y" {
                            // Create UserInput from parsed command
                            let user_input = UserInput {
                                subject: parsed.subject.clone(),
                                keywords: parsed.keywords.clone(),
                                context: parsed.context.clone(),
                                original_text: user_text.to_string(),
                                translated_text: None,
                                confidence: parsed.confidence,
                            };

                            // Execute smart attack
                            println!("\n🚀 Executing smart attack sequence...\n");
                            let results = executor.execute_smart_attack(&user_input).await;
                            for result in results {
                                println!("{}", result);
                            }
                        } else {
                            println!("❌ Interpretation rejected. Please rephrase your request.");
                        }
                    }
                    Err(e) => {
                        println!("\n❌ NLP processing failed: {}", e);
                        println!("\n💡 Try using direct format:");
                        println!("   <target> <keywords> [context]");
                        println!("   Example: example.com password,web aggressive mode");
                    }
                }
            }
            Err(e) => {
                eprintln!("❌ Input error: {}", e);
                break;
            }
        }
    }

    Ok(())
}

// ============================================================================
// HELPER FUNCTIONS
// ============================================================================

fn truncate_str(s: &str, max_len: usize) -> String {
    if s.len() > max_len {
        format!("{}...", &s[..max_len - 3])
    } else {
        s.to_string()
    }
}

fn print_banner() {
    println!("\n");
    println!("╔══════════════════════════════════════════════════════════════════════╗");
    println!("║                                                                      ║");
    println!("║   ███████╗███████╗███╗   ██╗██████╗ ██╗██████╗                       ║");
    println!("║   ██╔════╝██╔════╝████╗  ██║██╔══██╗██║██╔══██╗                      ║");
    println!("║   █████╗  █████╗  ██╔██╗ ██║██████╔╝██║██████╔╝                      ║");
    println!("║   ██╔══╝  ██╔══╝  ██║╚██╗██║██╔══██╗██║██╔══██╗                      ║");
    println!("║   ██║     ███████╗██║ ╚████║██║  ██║██║██║  ██║                      ║");
    println!("║   ╚═╝     ╚══════╝╚═╝  ╚═══╝╚═╝  ╚═╝╚═╝╚═╝  ╚═╝                      ║");
    println!("║                                                                      ║");
    println!("║   🐺 FENRIR v2.0 - Smart Attack Orchestration Platform               ║");
    println!("║   🎯 Red Team Tooling with Stealth-First Approach                    ║");
    println!("║                                                                      ║");
    println!("╠══════════════════════════════════════════════════════════════════════╣");
    println!("║                                                                      ║");
    println!("║   📝 INPUT FORMAT:                                                   ║");
    println!("║      Natural language OR: <subject> <keywords> [context]             ║");
    println!("║                                                                      ║");
    println!("║   📍 SUBJECT:  Target (IP, domain, email, username)                  ║");
    println!("║   🎯 KEYWORDS: Attack types (password, scan, web, social, etc.)      ║");
    println!("║   📋 CONTEXT:  Optional strategy/instructions                        ║");
    println!("║                                                                      ║");
    println!("║   💡 COMMANDS: help | keywords | exit                                ║");
    println!("║                                                                      ║");
    println!("╚══════════════════════════════════════════════════════════════════════╝");
}

fn print_help() {
    println!("\n╔══════════════════════════════════════════════════════════════════════╗");
    println!("║                        🐺 FENRIR HELP                                ║");
    println!("╠══════════════════════════════════════════════════════════════════════╣");
    println!("║                                                                      ║");
    println!("║   📝 NATURAL LANGUAGE EXAMPLES:                                      ║");
    println!("║      \"scan example.com for vulnerabilities\"                          ║");
    println!("║      \"crack passwords for john.doe@gmail.com\"                        ║");
    println!("║      \"find oauth2 issues in facebook login\"                          ║");
    println!("║      \"enumerate social media profiles for @username\"                 ║");
    println!("║                                                                      ║");
    println!("║   🎯 DIRECT FORMAT:                                                  ║");
    println!("║      <target> <keywords> [context]                                   ║");
    println!("║      example.com password,web aggressive                             ║");
    println!("║      192.168.1.1 scan,exploit stealth mode                           ║");
    println!("║                                                                      ║");
    println!("║   ⚡ ATTACK FLOW:                                                    ║");
    println!("║      1. Stealth scan (automatic before any attack)                   ║");
    println!("║      2. If stealth fails → Aggressive scan                           ║");
    println!("║      3. Async attacks (parallel, 666MB/thread)                       ║");
    println!("║      4. Sequential attacks (one-by-one, 2GB/thread)                  ║");
    println!("║                                                                      ║");
    println!("║   💡 COMMANDS:                                                       ║");
    println!("║      help      - Show this help                                      ║");
    println!("║      keywords  - List all available keywords                         ║");
    println!("║      exit      - Exit Fenrir                                         ║");
    println!("║                                                                      ║");
    println!("╚══════════════════════════════════════════════════════════════════════╝");
}

fn print_keywords() {
    let sequences = get_all_smart_sequences();

    println!("\n╔══════════════════════════════════════════════════════════════════════╗");
    println!("║                   🎯 AVAILABLE KEYWORDS                              ║");
    println!("╠══════════════════════════════════════════════════════════════════════╣");

    for (keyword, seq) in sequences.iter() {
        println!("║                                                                      ║");
        println!("║   🔑 {:<10} - {:<50} ║", keyword.to_uppercase(), seq.description);
        println!("║      Tools: {:<56} ║", 
            truncate_str(&seq.tools.iter().map(|t| t.name.clone()).collect::<Vec<_>>().join(", "), 56));
        println!("║      Mode: {} | Memory: {}MB | Stealth: {:<18} ║",
            if seq.async_execution { "Async     " } else { "Sequential" },
            seq.memory_limit_mb,
            if seq.stealth_first { "Yes" } else { "No " }
        );
    }

    println!("║                                                                      ║");
    println!("╚══════════════════════════════════════════════════════════════════════╝");
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_all_smart_sequences() {
        let sequences = get_all_smart_sequences();
        assert!(sequences.contains_key("password"));
        assert!(sequences.contains_key("scan"));
        assert!(sequences.contains_key("web"));
        assert!(sequences.contains_key("social"));
    }

    #[test]
    fn test_user_input_completeness() {
        let mut input = UserInput::new();
        assert!(!input.is_complete());

        input.subject = Some("example.com".to_string());
        assert!(!input.is_complete());

        input.keywords = vec!["password".to_string()];
        assert!(input.is_complete());
    }

    #[test]
    fn test_password_sequence_tools() {
        let sequences = get_all_smart_sequences();
        let password_seq = sequences.get("password").unwrap();
        
        assert!(password_seq.tools.iter().any(|t| t.name == "hydra"));
        assert!(password_seq.tools.iter().any(|t| t.name == "hashcat"));
        assert!(password_seq.tools.iter().any(|t| t.name == "john"));
        assert!(password_seq.stealth_first);
        assert!(password_seq.async_execution);
        assert_eq!(password_seq.memory_limit_mb, ASYNC_THREAD_MEMORY_LIMIT_MB);
    }
}
