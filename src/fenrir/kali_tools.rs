// --- FENRIR KALI LINUX TOOLS INTEGRATION ---
// Penetration testing and security assessment tools
// For authorized bug bounty and security auditing purposes

use serde::{Deserialize, Serialize};
use std::os::unix::process::CommandExt;
use std::path::Path;
use std::process::Command;

// ============================================================================
// KALI TOOL CATEGORIES
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum KaliToolCategory {
    #[serde(rename = "recon")]
    Reconnaissance,

    #[serde(rename = "scanning")]
    Scanning,

    #[serde(rename = "exploitation")]
    Exploitation,

    #[serde(rename = "password_attacks")]
    PasswordAttacks,

    #[serde(rename = "wireless_attacks")]
    WirelessAttacks,

    #[serde(rename = "web_applications")]
    WebApplications,

    #[serde(rename = "sniffing_spoofing")]
    SniffingSpoofing,

    #[serde(rename = "post_exploitation")]
    PostExploitation,

    #[serde(rename = "forensics")]
    Forensics,

    #[serde(rename = "reverse_engineering")]
    ReverseEngineering,
}

// ============================================================================
// KALI TOOL DEFINITIONS
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KaliTool {
    pub name: String,
    pub category: KaliToolCategory,
    pub description: String,
    pub command: String,
    pub typical_args: Vec<String>,
    pub requires_root: bool,
    pub install_command: Option<String>,
}

impl KaliTool {
    pub fn is_available(&self) -> bool {
        Command::new("which")
            .arg(&self.command)
            .output()
            .map(|output| output.status.success())
            .unwrap_or(false)
    }

    pub fn execute(&self, args: &[String]) -> Result<String, String> {
        if !self.is_available() {
            return Err(format!(
                "Tool '{}' is not installed. Install with: {}",
                self.name,
                self.install_command
                    .as_ref()
                    .unwrap_or(&format!("sudo apt install {}", self.name))
            ));
        }

        // Check if running as root on Unix-like systems
        if self.requires_root {
            #[cfg(unix)]
            {
                use std::process::Command;
                let uid_check = Command::new("id").arg("-u").output();

                if let Ok(output) = uid_check {
                    let uid_str = String::from_utf8_lossy(&output.stdout).trim().to_string();
                    if uid_str != "0" {
                        return Err(format!(
                            "Tool '{}' requires root privileges. Run with sudo.",
                            self.name
                        ));
                    }
                }
            }
        }

        let output = Command::new(&self.command).args(args).output();

        match output {
            Ok(result) => {
                if result.status.success() {
                    Ok(String::from_utf8_lossy(&result.stdout).to_string())
                } else {
                    let stderr = String::from_utf8_lossy(&result.stderr).to_string();
                    Err(format!("Command failed: {}", stderr))
                }
            }
            Err(e) => Err(format!("Failed to execute {}: {}", self.name, e)),
        }
    }
}

// ============================================================================
// KALI TOOLS DATABASE
// ============================================================================

pub fn get_kali_tools() -> Vec<KaliTool> {
    vec![
        // RECONNAISSANCE
        KaliTool {
            name: "nmap".to_string(),
            category: KaliToolCategory::Reconnaissance,
            description: "Network mapper and port scanner".to_string(),
            command: "nmap".to_string(),
            typical_args: vec!["-sV".to_string(), "-sC".to_string()],
            requires_root: true,
            install_command: Some("sudo apt install nmap".to_string()),
        },
        KaliTool {
            name: "netdiscover".to_string(),
            category: KaliToolCategory::Reconnaissance,
            description: "Active/passive address reconnaissance tool".to_string(),
            command: "netdiscover".to_string(),
            typical_args: vec!["-r".to_string()],
            requires_root: true,
            install_command: Some("sudo apt install netdiscover".to_string()),
        },
        KaliTool {
            name: "theHarvester".to_string(),
            category: KaliToolCategory::Reconnaissance,
            description: "E-mail, subdomain and people harvesting".to_string(),
            command: "theHarvester".to_string(),
            typical_args: vec!["-d".to_string(), "-b".to_string(), "google".to_string()],
            requires_root: false,
            install_command: Some("sudo apt install theharvester".to_string()),
        },
        // SCANNING
        KaliTool {
            name: "nikto".to_string(),
            category: KaliToolCategory::Scanning,
            description: "Web server scanner".to_string(),
            command: "nikto".to_string(),
            typical_args: vec!["-h".to_string()],
            requires_root: false,
            install_command: Some("sudo apt install nikto".to_string()),
        },
        KaliTool {
            name: "masscan".to_string(),
            category: KaliToolCategory::Scanning,
            description: "Mass IP port scanner".to_string(),
            command: "masscan".to_string(),
            typical_args: vec!["-p80,8000-8100".to_string()],
            requires_root: true,
            install_command: Some("sudo apt install masscan".to_string()),
        },
        // EXPLOITATION
        KaliTool {
            name: "metasploit-framework".to_string(),
            category: KaliToolCategory::Exploitation,
            description: "Exploitation framework".to_string(),
            command: "msfconsole".to_string(),
            typical_args: vec!["-q".to_string()],
            requires_root: false,
            install_command: Some("sudo apt install metasploit-framework".to_string()),
        },
        KaliTool {
            name: "sqlmap".to_string(),
            category: KaliToolCategory::Exploitation,
            description: "Automatic SQL injection tool".to_string(),
            command: "sqlmap".to_string(),
            typical_args: vec!["-u".to_string()],
            requires_root: false,
            install_command: Some("sudo apt install sqlmap".to_string()),
        },
        KaliTool {
            name: "exploitdb".to_string(),
            category: KaliToolCategory::Exploitation,
            description: "Exploit database search".to_string(),
            command: "searchsploit".to_string(),
            typical_args: vec![],
            requires_root: false,
            install_command: Some("sudo apt install exploitdb".to_string()),
        },
        // PASSWORD ATTACKS
        KaliTool {
            name: "john".to_string(),
            category: KaliToolCategory::PasswordAttacks,
            description: "John the Ripper password cracker".to_string(),
            command: "john".to_string(),
            typical_args: vec!["--wordlist".to_string()],
            requires_root: false,
            install_command: Some("sudo apt install john".to_string()),
        },
        KaliTool {
            name: "hashcat".to_string(),
            category: KaliToolCategory::PasswordAttacks,
            description: "GPU-based password recovery".to_string(),
            command: "hashcat".to_string(),
            typical_args: vec!["-m".to_string(), "0".to_string()],
            requires_root: false,
            install_command: Some("sudo apt install hashcat".to_string()),
        },
        KaliTool {
            name: "hydra".to_string(),
            category: KaliToolCategory::PasswordAttacks,
            description: "Parallel login cracker".to_string(),
            command: "hydra".to_string(),
            typical_args: vec!["-l".to_string(), "user".to_string()],
            requires_root: false,
            install_command: Some("sudo apt install hydra".to_string()),
        },
        // WEB APPLICATIONS
        KaliTool {
            name: "burpsuite".to_string(),
            category: KaliToolCategory::WebApplications,
            description: "Web application security testing tool".to_string(),
            command: "burpsuite".to_string(),
            typical_args: vec![],
            requires_root: false,
            install_command: Some("sudo apt install burpsuite".to_string()),
        },
        KaliTool {
            name: "owasp-zap".to_string(),
            category: KaliToolCategory::WebApplications,
            description: "Web application security scanner".to_string(),
            command: "zap-cli".to_string(),
            typical_args: vec!["quick-scan".to_string()],
            requires_root: false,
            install_command: Some("sudo apt install zaproxy".to_string()),
        },
        // WIRELESS ATTACKS
        KaliTool {
            name: "aircrack-ng".to_string(),
            category: KaliToolCategory::WirelessAttacks,
            description: "WiFi security auditing tool suite".to_string(),
            command: "aircrack-ng".to_string(),
            typical_args: vec!["-w".to_string()],
            requires_root: false,
            install_command: Some("sudo apt install aircrack-ng".to_string()),
        },
        KaliTool {
            name: "wifite".to_string(),
            category: KaliToolCategory::WirelessAttacks,
            description: "Automated wireless attack tool".to_string(),
            command: "wifite".to_string(),
            typical_args: vec!["--all".to_string()],
            requires_root: true,
            install_command: Some("sudo apt install wifite".to_string()),
        },
        // SNIFFING/SPOOFING
        KaliTool {
            name: "wireshark".to_string(),
            category: KaliToolCategory::SniffingSpoofing,
            description: "Network protocol analyzer".to_string(),
            command: "wireshark".to_string(),
            typical_args: vec!["-i".to_string(), "eth0".to_string()],
            requires_root: false,
            install_command: Some("sudo apt install wireshark".to_string()),
        },
        KaliTool {
            name: "ettercap".to_string(),
            category: KaliToolCategory::SniffingSpoofing,
            description: "Man-in-the-middle attack tool".to_string(),
            command: "ettercap".to_string(),
            typical_args: vec!["-T".to_string(), "-M".to_string(), "arp".to_string()],
            requires_root: true,
            install_command: Some("sudo apt install ettercap-text-only".to_string()),
        },
        // REVERSE ENGINEERING
        KaliTool {
            name: "ghidra".to_string(),
            category: KaliToolCategory::ReverseEngineering,
            description: "Reverse engineering framework".to_string(),
            command: "ghidraRun".to_string(),
            typical_args: vec![],
            requires_root: false,
            install_command: Some("sudo apt install ghidra".to_string()),
        },
        KaliTool {
            name: "radare2".to_string(),
            category: KaliToolCategory::ReverseEngineering,
            description: "Reverse engineering framework".to_string(),
            command: "r2".to_string(),
            typical_args: vec!["-A".to_string()],
            requires_root: false,
            install_command: Some("sudo apt install radare2".to_string()),
        },
        KaliTool {
            name: "objdump".to_string(),
            category: KaliToolCategory::ReverseEngineering,
            description: "Binary file analysis tool".to_string(),
            command: "objdump".to_string(),
            typical_args: vec!["-d".to_string()],
            requires_root: false,
            install_command: Some("sudo apt install binutils".to_string()),
        },
        KaliTool {
            name: "strings".to_string(),
            category: KaliToolCategory::ReverseEngineering,
            description: "Extract printable strings from files".to_string(),
            command: "strings".to_string(),
            typical_args: vec![],
            requires_root: false,
            install_command: Some("sudo apt install binutils".to_string()),
        },
        // FORENSICS
        KaliTool {
            name: "autopsy".to_string(),
            category: KaliToolCategory::Forensics,
            description: "Digital forensics platform".to_string(),
            command: "autopsy".to_string(),
            typical_args: vec![],
            requires_root: false,
            install_command: Some("sudo apt install autopsy".to_string()),
        },
        KaliTool {
            name: "binwalk".to_string(),
            category: KaliToolCategory::Forensics,
            description: "Firmware analysis tool".to_string(),
            command: "binwalk".to_string(),
            typical_args: vec!["-e".to_string()],
            requires_root: false,
            install_command: Some("sudo apt install binwalk".to_string()),
        },
        KaliTool {
            name: "volatility".to_string(),
            category: KaliToolCategory::Forensics,
            description: "Memory forensics framework".to_string(),
            command: "vol".to_string(),
            typical_args: vec!["-f".to_string()],
            requires_root: false,
            install_command: Some("sudo apt install volatility".to_string()),
        },
    ]
}

// ============================================================================
// TOOL FINDER
// ============================================================================

pub fn find_tool(name: &str) -> Option<KaliTool> {
    get_kali_tools()
        .into_iter()
        .find(|tool| tool.name == name || tool.command == name)
}

pub fn get_tools_by_category(category: KaliToolCategory) -> Vec<KaliTool> {
    get_kali_tools()
        .into_iter()
        .filter(|tool| tool.category == category)
        .collect()
}

pub fn get_available_tools() -> Vec<KaliTool> {
    get_kali_tools()
        .into_iter()
        .filter(|tool| tool.is_available())
        .collect()
}

// ============================================================================
// BITE (MORDER) FUNCTION - ADVANCED PENETRATION TESTING
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BiteConfig {
    pub target: String,
    pub tools: Vec<String>,
    pub intensity: BiteIntensity,
    pub categories: Vec<KaliToolCategory>,
    pub auto_exploit: bool,
    pub report_path: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum BiteIntensity {
    #[serde(rename = "passive")]
    Passive, // Recon only, no active scanning
    #[serde(rename = "cautious")]
    Cautious, // Light scanning, stealthy
    #[serde(rename = "aggressive")]
    Aggressive, // Full penetration test
    #[serde(rename = "godmode")]
    GodMode, // All tools, maximum aggression (with authorization)
}

pub struct BiteResult {
    pub success: bool,
    pub findings: Vec<String>,
    pub vulnerabilities: Vec<String>,
    pub exploited: Vec<String>,
    pub report: String,
    pub execution_time: std::time::Duration,
}

/// BITE (MORDER) - Execute comprehensive penetration test
/// This function orchestrates multiple Kali tools for authorized security testing
pub async fn bite(target: &str, config: BiteConfig) -> Result<BiteResult, String> {
    println!("🐺 FENRIR BITE (MORDER) - Advanced Penetration Testing");
    println!("🎯 Target: {}", target);
    println!("🔥 Intensity: {:?}", config.intensity);
    println!("📋 Categories: {:?}", config.categories);

    let start = std::time::Instant::now();
    let mut findings = Vec::new();
    let mut vulnerabilities = Vec::new();
    let mut exploited = Vec::new();

    // Phase 1: Reconnaissance
    if matches!(
        config.intensity,
        BiteIntensity::Passive
            | BiteIntensity::Cautious
            | BiteIntensity::Aggressive
            | BiteIntensity::GodMode
    ) {
        println!("\n🔍 Phase 1: Reconnaissance");

        if let Some(nmap) = find_tool("nmap") {
            if nmap.is_available() {
                println!("  📡 Running nmap reconnaissance...");
                match nmap.execute(&[
                    "-sV".to_string(),
                    "-sC".to_string(),
                    "-T4".to_string(),
                    target.to_string(),
                ]) {
                    Ok(output) => {
                        findings.push(format!("NMAP RECON:\n{}", output));
                        println!("  ✅ Nmap complete");
                    }
                    Err(e) => println!("  ⚠️  Nmap failed: {}", e),
                }
            }
        }
    }

    // Phase 2: Vulnerability Scanning
    if matches!(
        config.intensity,
        BiteIntensity::Cautious | BiteIntensity::Aggressive | BiteIntensity::GodMode
    ) {
        println!("\n🔎 Phase 2: Vulnerability Scanning");

        for category in &config.categories {
            match category {
                KaliToolCategory::WebApplications => {
                    if let Some(nikto) = find_tool("nikto") {
                        if nikto.is_available() {
                            println!("  🌐 Running nikto web scan...");
                            match nikto.execute(&["-h".to_string(), target.to_string()]) {
                                Ok(output) => {
                                    vulnerabilities.push(format!("NIKTO WEB SCAN:\n{}", output));
                                    println!("  ✅ Nikto complete");
                                }
                                Err(e) => println!("  ⚠️  Nikto failed: {}", e),
                            }
                        }
                    }
                }
                _ => {}
            }
        }
    }

    // Phase 3: Exploitation (ONLY with explicit authorization for bug bounty/legal testing)
    if matches!(
        config.intensity,
        BiteIntensity::Aggressive | BiteIntensity::GodMode
    ) && config.auto_exploit
    {
        println!("\n💀 Phase 3: Exploitation (AUTHORIZED SECURITY TESTING ONLY)");

        if let Some(sqlmap) = find_tool("sqlmap") {
            if sqlmap.is_available() {
                println!("  ⚠️  Running SQL injection test...");
                match sqlmap.execute(&["-u".to_string(), target.to_string(), "--batch".to_string()])
                {
                    Ok(output) => {
                        if output.contains("injectable") {
                            exploited.push(format!("SQL INJECTION FOUND:\n{}", output));
                            println!("  🔥 SQL injection vulnerability found!");
                        }
                    }
                    Err(e) => println!("  ℹ️  SQLMap result: {}", e),
                }
            }
        }
    }

    // Phase 4: Post-Exploitation & Reverse Engineering (God Mode only)
    if matches!(config.intensity, BiteIntensity::GodMode) {
        println!("\n💎 Phase 4: Advanced Analysis");

        // Reverse engineering tools
        if let Some(radare2) = find_tool("radare2") {
            if radare2.is_available() {
                println!("  🔬 Radare2 available for binary analysis");
            }
        }

        if let Some(ghidra) = find_tool("ghidra") {
            if ghidra.is_available() {
                println!("  🔬 Ghidra available for deep reverse engineering");
            }
        }
    }

    // Generate report
    let report = generate_bite_report(&findings, &vulnerabilities, &exploited, target);

    let result = BiteResult {
        success: true,
        findings,
        vulnerabilities,
        exploited,
        report,
        execution_time: start.elapsed(),
    };

    println!(
        "\n✅ BITE COMPLETE - Time: {:.2}s",
        result.execution_time.as_secs_f64()
    );

    // Save report if path provided
    if let Some(path) = &config.report_path {
        std::fs::write(path, &result.report)
            .map_err(|e| format!("Failed to save report: {}", e))?;
        println!("📄 Report saved to: {}", path);
    }

    Ok(result)
}

fn generate_bite_report(
    findings: &[String],
    vulnerabilities: &[String],
    exploited: &[String],
    target: &str,
) -> String {
    let mut report = format!(
        "🐺 FENRIR BITE REPORT - {}\n\
         ════════════════════════════\n\n\
         Target: {}\n\
         Timestamp: {}\n\n",
        target,
        target,
        chrono::Utc::now().format("%Y-%m-%d %H:%M:%S UTC")
    );

    if !findings.is_empty() {
        report.push_str("🔍 RECONNAISSANCE FINDINGS:\n");
        for (i, finding) in findings.iter().enumerate() {
            report.push_str(&format!("\n[{}]\n{}\n", i + 1, finding));
        }
    }

    if !vulnerabilities.is_empty() {
        report.push_str("\n🔎 VULNERABILITIES DISCOVERED:\n");
        for (i, vuln) in vulnerabilities.iter().enumerate() {
            report.push_str(&format!("\n[{}]\n{}\n", i + 1, vuln));
        }
    }

    if !exploited.is_empty() {
        report.push_str("\n💀 SUCCESSFULLY EXPLOITED:\n");
        for (i, exp) in exploited.iter().enumerate() {
            report.push_str(&format!("\n[{}]\n{}\n", i + 1, exp));
        }
    }

    report.push_str(&format!(
        "\n═══════════════════════════\n\
         Generated by FENRIR MCP 2.0\n\
         For authorized bug bounty and security testing only\n"
    ));

    report
}

// ============================================================================
// SCAN FUNCTION - SECURITY ASSESSMENT PLANNING
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScanConfig {
    pub target: String,
    pub scan_type: ScanType,
    pub depth: ScanDepth,
    pub output_format: ScanOutput,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ScanType {
    #[serde(rename = "quick")]
    Quick, // Fast scan for open ports and services
    #[serde(rename = "comprehensive")]
    Comprehensive, // Full security assessment
    #[serde(rename = "stealth")]
    Stealth, // Quiet scan to avoid detection
    #[serde(rename = "compliance")]
    Compliance, // Compliance-focused scan (PCI-DSS, HIPAA, etc.)
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ScanDepth {
    #[serde(rename = "surface")]
    Surface, // Top-level only
    #[serde(rename = "deep")]
    Deep, // Thorough analysis
    #[serde(rename = "exhaustive")]
    Exhaustive, // Complete assessment
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ScanOutput {
    #[serde(rename = "markdown")]
    Markdown,
    #[serde(rename = "json")]
    Json,
    #[serde(rename = "console")]
    Console,
}

pub struct ScanResult {
    pub target: String,
    pub scan_type: ScanType,
    pub open_ports: Vec<PortInfo>,
    pub services: Vec<ServiceInfo>,
    pub recommendations: Vec<String>,
    pub risk_score: u8,
    pub security_plan: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PortInfo {
    pub port: u16,
    pub protocol: String,
    pub state: String,
    pub service: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceInfo {
    pub name: String,
    pub version: Option<String>,
    pub vulnerabilities: Vec<String>,
}

/// SCAN - Plan comprehensive security assessment
/// This function creates a detailed security audit plan without exploiting
pub async fn scan(target: &str, config: ScanConfig) -> Result<ScanResult, String> {
    println!("🔍 FENRIR SCAN - Security Assessment Planning");
    println!("🎯 Target: {}", target);
    println!("📊 Type: {:?}", config.scan_type);
    println!("📏 Depth: {:?}", config.depth);

    let mut open_ports = Vec::new();
    let mut services = Vec::new();
    let mut recommendations = Vec::new();

    // Phase 1: Port Scanning
    println!("\n📡 Phase 1: Port Discovery");

    if let Some(nmap) = find_tool("nmap") {
        if nmap.is_available() {
            println!("  🔍 Scanning ports...");

            let scan_args = match config.scan_type {
                ScanType::Quick => vec!["-T4".to_string(), "-F".to_string(), target.to_string()],
                ScanType::Stealth => vec!["-sS".to_string(), "-T1".to_string(), target.to_string()],
                ScanType::Comprehensive | ScanType::Compliance => {
                    vec![
                        "-p-".to_string(),
                        "-sV".to_string(),
                        "-sC".to_string(),
                        target.to_string(),
                    ]
                }
            };

            match nmap.execute(&scan_args) {
                Ok(output) => {
                    // Parse nmap output for ports and services
                    open_ports = parse_nmap_ports(&output);
                    services = parse_nmap_services(&output);
                    println!("  ✅ Found {} open ports", open_ports.len());
                }
                Err(e) => println!("  ⚠️  Nmap scan failed: {}", e),
            }
        }
    }

    // Phase 2: Service Analysis
    println!("\n🔬 Phase 2: Service Analysis");

    for service in &services {
        println!("  📦 Analyzing: {}", service.name);

        // Check for known vulnerabilities
        if let Some(searchsploit) = find_tool("searchsploit") {
            if searchsploit.is_available() {
                match searchsploit.execute(&[service.name.clone()]) {
                    Ok(exploits) => {
                        if !exploits.trim().is_empty() {
                            let vulns: Vec<String> = exploits
                                .lines()
                                .take(5)
                                .map(|line| format!("  - {}", line))
                                .collect();
                            recommendations.push(format!(
                                "Service '{}' has {} known exploits:\n{}",
                                service.name,
                                vulns.len(),
                                vulns.join("\n")
                            ));
                        }
                    }
                    Err(_) => {}
                }
            }
        }
    }

    // Phase 3: Risk Assessment
    println!("\n⚠️  Phase 3: Risk Assessment");

    let risk_score = calculate_risk_score(&open_ports, &services);
    println!("  📊 Risk Score: {}/100", risk_score);

    // Phase 4: Generate Security Plan
    println!("\n📋 Phase 4: Security Plan Generation");

    let security_plan = generate_security_plan(target, &open_ports, &services, risk_score, &config);

    // Add recommendations
    if risk_score > 70 {
        recommendations.push("🔴 CRITICAL: Immediate security assessment required".to_string());
    } else if risk_score > 40 {
        recommendations.push("🟡 WARNING: Security review recommended".to_string());
    } else {
        recommendations.push("🟢 GOOD: Basic security posture acceptable".to_string());
    }

    for port in &open_ports {
        if port.port == 22 || port.port == 3389 {
            recommendations.push(format!(
                "💡 Consider restricting access to {} ({}{})",
                port.port,
                port.service.as_ref().unwrap_or(&"unknown".to_string()),
                if port.port == 22 { " - SSH" } else { " - RDP" }
            ));
        }
    }

    println!("\n✅ SCAN COMPLETE");

    Ok(ScanResult {
        target: target.to_string(),
        scan_type: config.scan_type,
        open_ports,
        services,
        recommendations,
        risk_score,
        security_plan,
    })
}

fn parse_nmap_ports(output: &str) -> Vec<PortInfo> {
    let mut ports = Vec::new();

    for line in output.lines() {
        if line.contains("/tcp") || line.contains("/udp") {
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() >= 3 {
                let port_protocol: Vec<&str> = parts[0].split('/').collect();
                if port_protocol.len() == 2 {
                    if let Ok(port_num) = port_protocol[0].parse::<u16>() {
                        ports.push(PortInfo {
                            port: port_num,
                            protocol: port_protocol[1].to_string(),
                            state: parts[1].to_string(),
                            service: if parts.len() > 2 {
                                Some(parts[2].to_string())
                            } else {
                                None
                            },
                        });
                    }
                }
            }
        }
    }

    ports
}

fn parse_nmap_services(output: &str) -> Vec<ServiceInfo> {
    let mut services = Vec::new();
    let mut lines_iter = output.lines();

    while let Some(line) = lines_iter.next() {
        if line.contains("SERVICE") {
            // Parse service version info
            if let Some(service_line) = lines_iter.next() {
                let parts: Vec<&str> = service_line.split_whitespace().collect();
                if parts.len() >= 3 {
                    services.push(ServiceInfo {
                        name: parts[2].to_string(),
                        version: if parts.len() > 3 {
                            Some(parts[3..].join(" "))
                        } else {
                            None
                        },
                        vulnerabilities: Vec::new(),
                    });
                }
            }
        }
    }

    services
}

fn calculate_risk_score(ports: &[PortInfo], _services: &[ServiceInfo]) -> u8 {
    let mut score = 0u8;

    // Base score from number of open ports
    score += (ports.len() as u8) * 5;

    // Extra risk for high-risk ports
    for port in ports {
        match port.port {
            21 | 23 | 135 | 139 | 445 => score += 10, // FTP, Telnet, SMB
            22 | 3389 => score += 5,                  // SSH, RDP
            80 | 443 => score += 3,                   // HTTP, HTTPS
            3306 | 5432 | 1433 => score += 7,         // Databases
            _ => {}
        }
    }

    // Cap at 100
    score.min(100)
}

fn generate_security_plan(
    target: &str,
    ports: &[PortInfo],
    services: &[ServiceInfo],
    risk_score: u8,
    config: &ScanConfig,
) -> String {
    let mut plan = format!(
        "🔍 FENRIR SECURITY ASSESSMENT PLAN\n\
         ═════════════════════════════════\n\n\
         Target: {}\n\
         Risk Score: {}/100\n\
         Scan Type: {:?}\n\
         Depth: {:?}\n\n",
        target, risk_score, config.scan_type, config.depth
    );

    plan.push_str("📊 DISCOVERY SUMMARY:\n");
    plan.push_str(&format!("  • Open Ports: {}\n", ports.len()));
    plan.push_str(&format!("  • Services Detected: {}\n", services.len()));

    plan.push_str("\n🎯 ASSESSMENT PHASES:\n");

    match config.depth {
        ScanDepth::Surface => {
            plan.push_str("  1. ✅ Port Scanning (completed)\n");
            plan.push_str("  2. 🔍 Service Enumeration\n");
            plan.push_str("  3. 📊 Risk Assessment\n");
        }
        ScanDepth::Deep => {
            plan.push_str("  1. ✅ Port Scanning (completed)\n");
            plan.push_str("  2. 🔍 Service Enumeration\n");
            plan.push_str("  3. 🔎 Vulnerability Scanning\n");
            plan.push_str("  4. 📊 Risk Assessment\n");
            plan.push_str("  5. 📋 Remediation Planning\n");
        }
        ScanDepth::Exhaustive => {
            plan.push_str("  1. ✅ Port Scanning (completed)\n");
            plan.push_str("  2. 🔍 Service Enumeration\n");
            plan.push_str("  3. 🔎 Vulnerability Scanning\n");
            plan.push_str("  4. 💥 Penetration Testing (with authorization)\n");
            plan.push_str("  5. 🔬 Reverse Engineering (if applicable)\n");
            plan.push_str("  6. 📊 Risk Assessment\n");
            plan.push_str("  7. 📋 Remediation Planning\n");
            plan.push_str("  8. 📄 Compliance Reporting\n");
        }
    }

    plan.push_str(&format!(
        "\n════════════════════════════════\n\
         Generated by FENRIR MCP 2.0\n\
         Next: Use 'bite' for active penetration testing (with authorization)\n"
    ));

    plan
}
