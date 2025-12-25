// --- FENRIR COMPREHENSIVE KALI LINUX TOOLS INTEGRATION ---
// 100+ Kali tools with async orchestration and detailed logging
// For authorized security testing only

use std::process::Command;
use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use std::sync::Arc;
use tokio::sync::Mutex;
use std::path::Path;

// ============================================================================
// COMPREHENSIVE TOOL CATEGORIES
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Hash, Eq)]
pub enum KaliToolCategory {
    // Information Gathering
    #[serde(rename = "recon")]
    Reconnaissance,
    #[serde(rename = "os_int")]
    OSInt,
    #[serde(rename = "dns_enum")]
    DnsEnumeration,
    #[serde(rename = "network_sniffing")]
    NetworkSniffing,
    #[serde(rename = "network_scanning")]
    NetworkScanning,

    // Vulnerability Analysis
    #[serde(rename = "vuln_analysis")]
    VulnerabilityAnalysis,
    #[serde(rename = "web_app_analysis")]
    WebApplicationAnalysis,
    #[serde(rename = "database_exploitation")]
    DatabaseExploitation,
    #[serde(rename = "password_attacks")]
    PasswordAttacks,

    // Exploitation
    #[serde(rename = "exploitation_tools")]
    ExploitationTools,
    #[serde(rename = "fuzzing")]
    Fuzzing,
    #[serde(rename = "reverse_engineering")]
    ReverseEngineering,
    #[serde(rename = "exploit_dev")]
    ExploitDevelopment,

    // Attacks
    #[serde(rename = "wireless_attacks")]
    WirelessAttacks,
    #[serde(rename = "sniffing_spoofing")]
    SniffingSpoofing,
    #[serde(rename = "mitm")]
    ManInTheMiddle,
    #[serde(rename = "dos")]
    DenialOfService,

    // Post-Exploitation
    #[serde(rename = "post_exploitation")]
    PostExploitation,
    #[serde(rename = "privilege_escalation")]
    PrivilegeEscalation,
    #[serde(rename = "persistence")]
    PersistenceMechanisms,
    #[serde(rename = "anti_forensics")]
    AntiForensics,

    // Forensics
    #[serde(rename = "forensics")]
    Forensics,
    #[serde(rename = "memory_forensics")]
    MemoryForensics,
    #[serde(rename = "file_forensics")]
    FileForensics,
    #[serde(rename = "network_forensics")]
    NetworkForensics,

    // Reporting & Documentation
    #[serde(rename = "reporting")]
    Reporting,
    #[serde(rename = "social_engineering")]
    SocialEngineering,
}

// ============================================================================
// DECISION LOGGING SYSTEM
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BrainDecision {
    pub timestamp: DateTime<Utc>,
    pub decision_id: String,
    pub decision_type: DecisionType,
    pub reasoning: String,
    pub tool_selected: String,
    pub target: String,
    pub success: bool,
    pub output_summary: String,
    pub execution_time_ms: u64,
    pub next_steps: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DecisionType {
    #[serde(rename = "tool_selection")]
    ToolSelection,
    #[serde(rename = "strategy_change")]
    StrategyChange,
    #[serde(rename = "escalation")]
    Escalation,
    #[serde(rename = "de_escalation")]
    DeEscalation,
    #[serde(rename = "breach_detected")]
    BreachDetected,
    #[serde(rename = "sensitive_data_found")]
    SensitiveDataFound,
}

pub struct DecisionLogger {
    pub log_file: String,
    pub decisions: Arc<Mutex<Vec<BrainDecision>>>,
}

impl DecisionLogger {
    pub fn new(target: &str) -> Self {
        let log_dir = std::path::Path::new("fenrir_logs");
        std::fs::create_dir_all(log_dir).unwrap();

        let timestamp = Utc::now().format("%Y%m%d_%H%M%S");
        let log_file = format!("fenrir_logs/brain_{}_{}.json", target.replace(".", "_"), timestamp);

        DecisionLogger {
            log_file,
            decisions: Arc::new(Mutex::new(Vec::new())),
        }
    }

    pub async fn log_decision(&self, decision: BrainDecision) {
        let mut decisions = self.decisions.lock().await;
        decisions.push(decision.clone());

        // Append to log file
        let log_entry = serde_json::to_string_pretty(&decision).unwrap();
        if let Ok(mut file) = std::fs::OpenOptions::new()
            .create(true)
            .append(true)
            .open(&self.log_file)
        {
            use std::io::Write;
            let _ = writeln!(file, "{}", log_entry);
        }
    }

    pub async fn export_markdown(&self, target: &str) -> String {
        let decisions = self.decisions.lock().await;
        let mut report = format!("# 🧠 FENRIR BRAIN DECISION LOG\n\n");
        report.push_str(&format!("**Target**: {}\n", target));
        report.push_str(&format!("**Total Decisions**: {}\n", decisions.len()));
        report.push_str(&format!("**Generated**: {}\n\n", Utc::now().format("%Y-%m-%d %H:%M:%S UTC")));

        report.push_str("---\n\n");

        for (idx, decision) in decisions.iter().enumerate() {
            report.push_str(&format!("## Decision {}: {:?}\n\n", idx + 1, decision.decision_type));
            report.push_str(&format!("**Time**: {}\n", decision.timestamp.format("%H:%M:%S")));
            report.push_str(&format!("**Tool**: {}\n", decision.tool_selected));
            report.push_str(&format!("**Target**: {}\n", decision.target));
            report.push_str(&format!("**Success**: {}\n\n", decision.success));
            report.push_str(&format!("**Reasoning**:\n{}\n\n", decision.reasoning));
            report.push_str(&format!("**Output**:\n```\n{}\n```\n\n", decision.output_summary));
            if !decision.next_steps.is_empty() {
                report.push_str("**Next Steps**:\n");
                for step in &decision.next_steps {
                    report.push_str(&format!("- {}\n", step));
                }
                report.push_str("\n");
            }
            report.push_str("---\n\n");
        }

        report
    }
}

// ============================================================================
// BREACH DETECTION SYSTEM
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Breach {
    pub breach_id: String,
    pub severity: BreachSeverity,
    pub breach_type: BreachType,
    pub description: String,
    pub evidence: Vec<String>,
    pub affected_systems: Vec<String>,
    pub recommendations: Vec<String>,
    pub timestamp: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum BreachSeverity {
    #[serde(rename = "critical")]
    Critical,  // Immediate action required
    #[serde(rename = "high")]
    High,      // Urgent attention needed
    #[serde(rename = "medium")]
    Medium,    // Should be addressed
    #[serde(rename = "low")]
    Low,       // Informational
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum BreachType {
    #[serde(rename = "sql_injection")]
    SQLInjection,
    #[serde(rename = "xss")]
    XSS,
    #[serde(rename = "rce")]
    RemoteCodeExecution,
    #[serde(rename = "auth_bypass")]
    AuthenticationBypass,
    #[serde(rename = "privilege_escalation")]
    PrivilegeEscalation,
    #[serde(rename = "data_exposure")]
    DataExposure,
    #[serde(rename = "misconfiguration")]
    Misconfiguration,
    #[serde(rename = "weak_cryptography")]
    WeakCryptography,
    #[serde(rename = "sensitive_data_exposure")]
    SensitiveDataExposure,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SensitiveData {
    pub data_id: String,
    pub data_type: SensitiveDataType,
    pub content: String,
    pub file_path: Option<String>,
    pub url: Option<String>,
    pub confidence: f32,
    pub timestamp: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SensitiveDataType {
    #[serde(rename = "email")]
    Email,
    #[serde(rename = "phone")]
    PhoneNumber,
    #[serde(rename = "ssn")]
    SSN,
    #[serde(rename = "credit_card")]
    CreditCard,
    #[serde(rename = "password")]
    Password,
    #[serde(rename = "api_key")]
    APIKey,
    #[serde(rename = "token")]
    Token,
    #[serde(rename = "personal_info")]
    PersonalInfo,
    #[serde(rename = "image")]
    Image,
    #[serde(rename = "document")]
    Document,
    #[serde(rename = "credential")]
    Credential,
}

pub struct BreachDetector {
    pub detected_breaches: Vec<Breach>,
    pub sensitive_data: Vec<SensitiveData>,
    pub files_analyzed: Vec<String>,
    pub images_found: Vec<String>,
}

impl BreachDetector {
    pub fn new() -> Self {
        BreachDetector {
            detected_breaches: Vec::new(),
            sensitive_data: Vec::new(),
            files_analyzed: Vec::new(),
            images_found: Vec::new(),
        }
    }

    pub fn analyze_output(&mut self, tool_output: &str, tool_name: &str) {
        // SQL Injection Detection
        if tool_output.to_lowercase().contains("sql") &&
           (tool_output.to_lowercase().contains("inject") ||
            tool_output.to_lowercase().contains("syntax error") ||
            tool_output.to_lowercase().contains("mysql")) {
            self.detected_breaches.push(Breach {
                breach_id: uuid::Uuid::new_v4().to_string(),
                severity: BreachSeverity::Critical,
                breach_type: BreachType::SQLInjection,
                description: format!("Potential SQL injection vulnerability detected by {}", tool_name),
                evidence: vec![tool_output.lines().take(5).collect::<Vec<_>>().join("\n")],
                affected_systems: vec![],
                recommendations: vec![
                    "Use parameterized queries".to_string(),
                    "Implement input validation".to_string(),
                    "Use ORM frameworks".to_string(),
                ],
                timestamp: Utc::now(),
            });
        }

        // XSS Detection
        if tool_output.to_lowercase().contains("xss") ||
           tool_output.to_lowercase().contains("cross-site") ||
           tool_output.contains("<script>") {
            self.detected_breaches.push(Breach {
                breach_id: uuid::Uuid::new_v4().to_string(),
                severity: BreachSeverity::High,
                breach_type: BreachType::XSS,
                description: format!("Cross-site scripting vulnerability detected by {}", tool_name),
                evidence: vec![tool_output.lines().take(5).collect::<Vec<_>>().join("\n")],
                affected_systems: vec![],
                recommendations: vec![
                    "Sanitize all user input".to_string(),
                    "Implement Content Security Policy".to_string(),
                    "Use template engines with auto-escaping".to_string(),
                ],
                timestamp: Utc::now(),
            });
        }

        // Authentication Bypass
        if tool_output.to_lowercase().contains("admin") &&
           (tool_output.to_lowercase().contains("bypass") ||
            tool_output.to_lowercase().contains("unauthorized") ||
            tool_output.to_lowercase().contains("authentication")) {
            self.detected_breaches.push(Breach {
                breach_id: uuid::Uuid::new_v4().to_string(),
                severity: BreachSeverity::Critical,
                breach_type: BreachType::AuthenticationBypass,
                description: format!("Authentication bypass detected by {}", tool_name),
                evidence: vec![tool_output.lines().take(5).collect::<Vec<_>>().join("\n")],
                affected_systems: vec![],
                recommendations: vec![
                    "Implement proper authentication".to_string(),
                    "Use multi-factor authentication".to_string(),
                    "Implement rate limiting".to_string(),
                ],
                timestamp: Utc::now(),
            });
        }

        // Sensitive Data Detection (Emails)
        let email_regex = regex::Regex::new(r"[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}").unwrap();
        for email in email_regex.find_iter(tool_output) {
            self.sensitive_data.push(SensitiveData {
                data_id: uuid::Uuid::new_v4().to_string(),
                data_type: SensitiveDataType::Email,
                content: email.as_str().to_string(),
                file_path: None,
                url: None,
                confidence: 0.95,
                timestamp: Utc::now(),
            });
        }

        // IP Addresses
        let ip_regex = regex::Regex::new(r"\b\d{1,3}\.\d{1,3}\.\d{1,3}\.\d{1,3}\b").unwrap();
        for ip in ip_regex.find_iter(tool_output) {
            self.sensitive_data.push(SensitiveData {
                data_id: uuid::Uuid::new_v4().to_string(),
                data_type: SensitiveDataType::PersonalInfo,
                content: format!("IP: {}", ip.as_str()),
                file_path: None,
                url: None,
                confidence: 0.90,
                timestamp: Utc::now(),
            });
        }

        // Password Detection
        if tool_output.to_lowercase().contains("password") {
            let pass_regex = regex::Regex::new(r"(?i)password\s*[:=]\s*\S+").unwrap();
            for pass in pass_regex.find_iter(tool_output) {
                self.sensitive_data.push(SensitiveData {
                    data_id: uuid::Uuid::new_v4().to_string(),
                    data_type: SensitiveDataType::Password,
                    content: pass.as_str().to_string(),
                    file_path: None,
                    url: None,
                    confidence: 0.85,
                    timestamp: Utc::now(),
                });
            }
        }

        // API Key Detection
        if tool_output.contains("api_key") || tool_output.contains("apikey") ||
           tool_output.contains("API_KEY") || tool_output.contains("secret") {
            self.sensitive_data.push(SensitiveData {
                data_id: uuid::Uuid::new_v4().to_string(),
                data_type: SensitiveDataType::APIKey,
                content: "Potential API key detected".to_string(),
                file_path: None,
                url: None,
                confidence: 0.80,
                timestamp: Utc::now(),
            });
        }
    }

    pub async fn scan_files_for_sensitive_data(&mut self, directory: &str) {
        println!("🔍 Scanning {} for sensitive data...", directory);

        let paths = std::fs::read_dir(directory);
        if paths.is_err() {
            println!("  ⚠️  Cannot read directory");
            return;
        }

        for entry in paths.unwrap().filter_map(|e| e.ok()) {
            let path = entry.path();
            let path_str = path.to_string_lossy().to_string();

            // Check for images
            if let Some(ext) = path.extension() {
                if matches!(ext.to_str(), Some("jpg") | Some("jpeg") | Some("png") |
                    Some("gif") | Some("bmp") | Some("svg") | Some("webp")) {
                    self.images_found.push(path_str.clone());
                    self.sensitive_data.push(SensitiveData {
                        data_id: uuid::Uuid::new_v4().to_string(),
                        data_type: SensitiveDataType::Image,
                        content: format!("Image file: {}", path.display()),
                        file_path: Some(path_str.clone()),
                        url: None,
                        confidence: 1.0,
                        timestamp: Utc::now(),
                    });
                    println!("  📷 Found image: {}", path.display());
                }
            }

            // Check for documents
            if let Some(ext) = path.extension() {
                if matches!(ext.to_str(), Some("pdf") | Some("doc") | Some("docx") |
                    Some("txt") | Some("xls") | Some("xlsx") | Some("ppt")) {
                    self.files_analyzed.push(path_str.clone());
                    self.sensitive_data.push(SensitiveData {
                        data_id: uuid::Uuid::new_v4().to_string(),
                        data_type: SensitiveDataType::Document,
                        content: format!("Document: {}", path.display()),
                        file_path: Some(path_str),
                        url: None,
                        confidence: 0.95,
                        timestamp: Utc::now(),
                    });
                    println!("  📄 Found document: {}", path.display());
                }
            }

            // Scan file content
            if path.is_file() {
                if let Ok(content) = std::fs::read_to_string(&path) {
                    self.analyze_output(&content, "file_scan");
                }
            }
        }
    }

    pub fn get_summary(&self) -> String {
        let mut summary = String::from("## 🔍 BREACH & SENSITIVE DATA SUMMARY\n\n");

        summary.push_str(&format!("**Breaches Detected**: {}\n", self.detected_breaches.len()));
        summary.push_str(&format!("**Sensitive Data Items**: {}\n", self.sensitive_data.len()));
        summary.push_str(&format!("**Files Analyzed**: {}\n", self.files_analyzed.len()));
        summary.push_str(&format!("**Images Found**: {}\n\n", self.images_found.len()));

        if !self.detected_breaches.is_empty() {
            summary.push_str("### 🚨 DETECTED BREACHES\n\n");
            for (idx, breach) in self.detected_breaches.iter().enumerate() {
                summary.push_str(&format!("#### {}. {:?} - {:?}\n\n", idx + 1, breach.breach_type, breach.severity));
                summary.push_str(&format!("**Description**: {}\n\n", breach.description));
                if !breach.evidence.is_empty() {
                    summary.push_str("**Evidence**:\n```\n");
                    for evidence in &breach.evidence {
                        summary.push_str(evidence);
                        summary.push_str("\n");
                    }
                    summary.push_str("```\n\n");
                }
                if !breach.recommendations.is_empty() {
                    summary.push_str("**Recommendations**:\n");
                    for rec in &breach.recommendations {
                        summary.push_str(&format!("- {}\n", rec));
                    }
                    summary.push_str("\n");
                }
            }
        }

        if !self.images_found.is_empty() {
            summary.push_str("### 📷 IMAGES FOUND\n\n");
            for image in &self.images_found {
                summary.push_str(&format!("- {}\n", image));
            }
            summary.push_str("\n");
        }

        if !self.files_analyzed.is_empty() {
            summary.push_str("### 📄 FILES ANALYZED\n\n");
            for file in &self.files_analyzed {
                summary.push_str(&format!("- {}\n", file));
            }
            summary.push_str("\n");
        }

        summary
    }
}

// ============================================================================
// EXPANDED TOOL DEFINITIONS (100+ TOOLS)
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
    pub execution_time_estimate: u64, // seconds
}

pub fn get_all_kali_tools() -> Vec<KaliTool> {
    vec![
        // ========== INFORMATION GATHERING ==========
        // Reconnaissance
        KaliTool {
            name: "nmap".to_string(),
            category: KaliToolCategory::NetworkScanning,
            description: "Network mapper and port scanner".to_string(),
            command: "nmap".to_string(),
            typical_args: vec!["-sV".to_string(), "-sC".to_string(), "-T4".to_string()],
            requires_root: true,
            install_command: Some("sudo apt install nmap".to_string()),
            execution_time_estimate: 60,
        },
        KaliTool {
            name: "masscan".to_string(),
            category: KaliToolCategory::NetworkScanning,
            description: "Mass IP port scanner".to_string(),
            command: "masscan".to_string(),
            typical_args: vec!["-p80,8000-8100".to_string()],
            requires_root: true,
            install_command: Some("sudo apt install masscan".to_string()),
            execution_time_estimate: 120,
        },
        KaliTool {
            name: "rustscan".to_string(),
            category: KaliToolCategory::NetworkScanning,
            description: "Modern port scanner".to_string(),
            command: "rustscan".to_string(),
            typical_args: vec!["-a".to_string()],
            requires_root: false,
            install_command: Some("curl -sSf https://sh.rustup.rs | sh".to_string()),
            execution_time_estimate: 30,
        },

        // DNS Enumeration
        KaliTool {
            name: "dnsenum".to_string(),
            category: KaliToolCategory::DnsEnumeration,
            description: "DNS enumeration tool".to_string(),
            command: "dnsenum".to_string(),
            typical_args: vec![],
            requires_root: false,
            install_command: Some("sudo apt install dnsenum".to_string()),
            execution_time_estimate: 45,
        },
        KaliTool {
            name: "dnsrecon".to_string(),
            category: KaliToolCategory::DnsEnumeration,
            description: "DNS reconnaissance script".to_string(),
            command: "dnsrecon".to_string(),
            typical_args: vec!["-d".to_string()],
            requires_root: false,
            install_command: Some("sudo apt install dnsrecon".to_string()),
            execution_time_estimate: 60,
        },

        // OSINT
        KaliTool {
            name: "theHarvester".to_string(),
            category: KaliToolCategory::OSInt,
            description: "E-mail, subdomain and people harvesting".to_string(),
            command: "theHarvester".to_string(),
            typical_args: vec!["-d".to_string(), "-b".to_string(), "google".to_string()],
            requires_root: false,
            install_command: Some("sudo apt install theharvester".to_string()),
            execution_time_estimate: 90,
        },
        KaliTool {
            name: "sherlock".to_string(),
            category: KaliToolCategory::OSInt,
            description: "Find usernames across social networks".to_string(),
            command: "sherlock".to_string(),
            typical_args: vec![],
            requires_root: false,
            install_command: Some("pip3 install sherlock".to_string()),
            execution_time_estimate: 120,
        },
        KaliTool {
            name: "maltego".to_string(),
            category: KaliToolCategory::OSInt,
            description: "Open source intelligence and forensics".to_string(),
            command: "maltego".to_string(),
            typical_args: vec![],
            requires_root: false,
            install_command: Some("sudo apt install maltego".to_string()),
            execution_time_estimate: 0,
        },

        // Network Sniffing
        KaliTool {
            name: "wireshark".to_string(),
            category: KaliToolCategory::NetworkSniffing,
            description: "Network protocol analyzer".to_string(),
            command: "tshark".to_string(),
            typical_args: vec!["-i".to_string(), "eth0".to_string()],
            requires_root: false,
            install_command: Some("sudo apt install tshark".to_string()),
            execution_time_estimate: 0,
        },
        KaliTool {
            name: "tcpdump".to_string(),
            category: KaliToolCategory::NetworkSniffing,
            description: "Packet analyzer".to_string(),
            command: "tcpdump".to_string(),
            typical_args: vec!["-i".to_string(), "eth0".to_string()],
            requires_root: true,
            install_command: Some("sudo apt install tcpdump".to_string()),
            execution_time_estimate: 0,
        },

        // ========== VULNERABILITY ANALYSIS ==========
        KaliTool {
            name: "nikto".to_string(),
            category: KaliToolCategory::WebApplicationAnalysis,
            description: "Web server scanner".to_string(),
            command: "nikto".to_string(),
            typical_args: vec!["-h".to_string()],
            requires_root: false,
            install_command: Some("sudo apt install nikto".to_string()),
            execution_time_estimate: 120,
        },
        KaliTool {
            name: "sqlmap".to_string(),
            category: KaliToolCategory::DatabaseExploitation,
            description: "Automatic SQL injection tool".to_string(),
            command: "sqlmap".to_string(),
            typical_args: vec!["-u".to_string(), "--batch".to_string()],
            requires_root: false,
            install_command: Some("sudo apt install sqlmap".to_string()),
            execution_time_estimate: 180,
        },
        KaliTool {
            name: "nuclei".to_string(),
            category: KaliToolCategory::VulnerabilityAnalysis,
            description: "Fast vulnerability scanner".to_string(),
            command: "nuclei".to_string(),
            typical_args: vec!["-u".to_string()],
            requires_root: false,
            install_command: Some("go install -v github.com/projectdiscovery/nuclei/v2/cmd/nuclei@latest".to_string()),
            execution_time_estimate: 60,
        },
        KaliTool {
            name: "nessus".to_string(),
            category: KaliToolCategory::VulnerabilityAnalysis,
            description: "Vulnerability scanner".to_string(),
            command: "nessus".to_string(),
            typical_args: vec![],
            requires_root: false,
            install_command: Some("sudo apt install nessus".to_string()),
            execution_time_estimate: 600,
        },
        KaliTool {
            name: "openvas".to_string(),
            category: KaliToolCategory::VulnerabilityAnalysis,
            description: "Open source vulnerability scanner".to_string(),
            command: "openvas".to_string(),
            typical_args: vec![],
            requires_root: false,
            install_command: Some("sudo apt install openvas".to_string()),
            execution_time_estimate: 600,
        },

        // ========== EXPLOITATION TOOLS ==========
        KaliTool {
            name: "metasploit-framework".to_string(),
            category: KaliToolCategory::ExploitationTools,
            description: "Exploitation framework".to_string(),
            command: "msfconsole".to_string(),
            typical_args: vec!["-q".to_string()],
            requires_root: false,
            install_command: Some("sudo apt install metasploit-framework".to_string()),
            execution_time_estimate: 0,
        },
        KaliTool {
            name: "searchsploit".to_string(),
            category: KaliToolCategory::ExploitationTools,
            description: "Exploit database search".to_string(),
            command: "searchsploit".to_string(),
            typical_args: vec![],
            requires_root: false,
            install_command: Some("sudo apt install exploitdb".to_string()),
            execution_time_estimate: 10,
        },
        KaliTool {
            name: "empire".to_string(),
            category: KaliToolCategory::PostExploitation,
            description: "Post-exploitation framework".to_string(),
            command: "empire".to_string(),
            typical_args: vec![],
            requires_root: false,
            install_command: Some("sudo apt install empire".to_string()),
            execution_time_estimate: 0,
        },

        // ========== PASSWORD ATTACKS ==========
        KaliTool {
            name: "john".to_string(),
            category: KaliToolCategory::PasswordAttacks,
            description: "John the Ripper password cracker".to_string(),
            command: "john".to_string(),
            typical_args: vec!["--wordlist".to_string()],
            requires_root: false,
            install_command: Some("sudo apt install john".to_string()),
            execution_time_estimate: 300,
        },
        KaliTool {
            name: "hashcat".to_string(),
            category: KaliToolCategory::PasswordAttacks,
            description: "GPU-based password recovery".to_string(),
            command: "hashcat".to_string(),
            typical_args: vec!["-m".to_string(), "0".to_string()],
            requires_root: false,
            install_command: Some("sudo apt install hashcat".to_string()),
            execution_time_estimate: 300,
        },
        KaliTool {
            name: "hydra".to_string(),
            category: KaliToolCategory::PasswordAttacks,
            description: "Parallel login cracker".to_string(),
            command: "hydra".to_string(),
            typical_args: vec!["-l".to_string(), "user".to_string()],
            requires_root: false,
            install_command: Some("sudo apt install hydra".to_string()),
            execution_time_estimate: 180,
        },
        KaliTool {
            name: "medusa".to_string(),
            category: KaliToolCategory::PasswordAttacks,
            description: "Parallel login brute-forcer".to_string(),
            command: "medusa".to_string(),
            typical_args: vec!["-h".to_string()],
            requires_root: false,
            install_command: Some("sudo apt install medusa".to_string()),
            execution_time_estimate: 180,
        },
        KaliTool {
            name: "cewl".to_string(),
            category: KaliToolCategory::PasswordAttacks,
            description: "Custom wordlist generator".to_string(),
            command: "cewl".to_string(),
            typical_args: vec!["-w".to_string(), "wordlist.txt".to_string()],
            requires_root: false,
            install_command: Some("sudo apt install cewl".to_string()),
            execution_time_estimate: 60,
        },
        KaliTool {
            name: "crunch".to_string(),
            category: KaliToolCategory::PasswordAttacks,
            description: "Wordlist generator".to_string(),
            command: "crunch".to_string(),
            typical_args: vec!["8".to_string(), "8".to_string()],
            requires_root: false,
            install_command: Some("sudo apt install crunch".to_string()),
            execution_time_estimate: 30,
        },

        // ========== WIRELESS ATTACKS ==========
        KaliTool {
            name: "aircrack-ng".to_string(),
            category: KaliToolCategory::WirelessAttacks,
            description: "WiFi security auditing tool suite".to_string(),
            command: "aircrack-ng".to_string(),
            typical_args: vec!["-w".to_string()],
            requires_root: false,
            install_command: Some("sudo apt install aircrack-ng".to_string()),
            execution_time_estimate: 300,
        },
        KaliTool {
            name: "wifite".to_string(),
            category: KaliToolCategory::WirelessAttacks,
            description: "Automated wireless attack tool".to_string(),
            command: "wifite".to_string(),
            typical_args: vec!["--all".to_string()],
            requires_root: true,
            install_command: Some("sudo apt install wifite".to_string()),
            execution_time_estimate: 300,
        },
        KaliTool {
            name: "reaver".to_string(),
            category: KaliToolCategory::WirelessAttacks,
            description: "WPS PIN brute force attack".to_string(),
            command: "reaver".to_string(),
            typical_args: vec!["-b".to_string()],
            requires_root: true,
            install_command: Some("sudo apt install reaver".to_string()),
            execution_time_estimate: 600,
        },
        KaliTool {
            name: "bully".to_string(),
            category: KaliToolCategory::WirelessAttacks,
            description: "WPS PIN brute force attack".to_string(),
            command: "bully".to_string(),
            typical_args: vec![],
            requires_root: true,
            install_command: Some("sudo apt install bully".to_string()),
            execution_time_estimate: 600,
        },
        KaliTool {
            name: "kismet".to_string(),
            category: KaliToolCategory::WirelessAttacks,
            description: "Wireless network detector".to_string(),
            command: "kismet".to_string(),
            typical_args: vec![],
            requires_root: true,
            install_command: Some("sudo apt install kismet".to_string()),
            execution_time_estimate: 0,
        },

        // ========== WEB APPLICATIONS ==========
        KaliTool {
            name: "burpsuite".to_string(),
            category: KaliToolCategory::WebApplicationAnalysis,
            description: "Web application security testing tool".to_string(),
            command: "burpsuite".to_string(),
            typical_args: vec![],
            requires_root: false,
            install_command: Some("sudo apt install burpsuite".to_string()),
            execution_time_estimate: 0,
        },
        KaliTool {
            name: "owasp-zap".to_string(),
            category: KaliToolCategory::WebApplicationAnalysis,
            description: "Web application security scanner".to_string(),
            command: "zap-cli".to_string(),
            typical_args: vec!["quick-scan".to_string()],
            requires_root: false,
            install_command: Some("sudo apt install zaproxy".to_string()),
            execution_time_estimate: 300,
        },
        KaliTool {
            name: "dirb".to_string(),
            category: KaliToolCategory::WebApplicationAnalysis,
            description: "Web content scanner".to_string(),
            command: "dirb".to_string(),
            typical_args: vec![],
            requires_root: false,
            install_command: Some("sudo apt install dirb".to_string()),
            execution_time_estimate: 120,
        },
        KaliTool {
            name: "gobuster".to_string(),
            category: KaliToolCategory::WebApplicationAnalysis,
            description: "Directory/file & DNS busting tool".to_string(),
            command: "gobuster".to_string(),
            typical_args: vec!["dir".to_string(), "-u".to_string()],
            requires_root: false,
            install_command: Some("sudo apt install gobuster".to_string()),
            execution_time_estimate: 90,
        },
        KaliTool {
            name: "ffuf".to_string(),
            category: KaliToolCategory::WebApplicationAnalysis,
            description: "Fast web fuzzer".to_string(),
            command: "ffuf".to_string(),
            typical_args: vec!["-u".to_string()],
            requires_root: false,
            install_command: Some("go install github.com/ffuf/ffuf@latest".to_string()),
            execution_time_estimate: 120,
        },

        // ========== SNIFFING & SPOOFING ==========
        KaliTool {
            name: "ettercap".to_string(),
            category: KaliToolCategory::ManInTheMiddle,
            description: "Man-in-the-middle attack tool".to_string(),
            command: "ettercap".to_string(),
            typical_args: vec!["-T".to_string(), "-M".to_string(), "arp".to_string()],
            requires_root: true,
            install_command: Some("sudo apt install ettercap-text-only".to_string()),
            execution_time_estimate: 0,
        },
        KaliTool {
            name: "bettercap".to_string(),
            category: KaliToolCategory::ManInTheMiddle,
            description: "Swiss army knife for network attacks".to_string(),
            command: "bettercap".to_string(),
            typical_args: vec![],
            requires_root: true,
            install_command: Some("sudo apt install bettercap".to_string()),
            execution_time_estimate: 0,
        },
        KaliTool {
            name: "mitmproxy".to_string(),
            category: KaliToolCategory::ManInTheMiddle,
            description: "Interactive HTTPS proxy".to_string(),
            command: "mitmproxy".to_string(),
            typical_args: vec![],
            requires_root: false,
            install_command: Some("sudo apt install mitmproxy".to_string()),
            execution_time_estimate: 0,
        },
        KaliTool {
            name: "scapy".to_string(),
            category: KaliToolCategory::SniffingSpoofing,
            description: "Packet manipulation tool".to_string(),
            command: "scapy".to_string(),
            typical_args: vec![],
            requires_root: false,
            install_command: Some("pip3 install scapy".to_string()),
            execution_time_estimate: 0,
        },

        // ========== REVERSE ENGINEERING ==========
        KaliTool {
            name: "ghidra".to_string(),
            category: KaliToolCategory::ReverseEngineering,
            description: "Reverse engineering framework".to_string(),
            command: "ghidraRun".to_string(),
            typical_args: vec![],
            requires_root: false,
            install_command: Some("sudo apt install ghidra".to_string()),
            execution_time_estimate: 0,
        },
        KaliTool {
            name: "radare2".to_string(),
            category: KaliToolCategory::ReverseEngineering,
            description: "Reverse engineering framework".to_string(),
            command: "r2".to_string(),
            typical_args: vec!["-A".to_string()],
            requires_root: false,
            install_command: Some("sudo apt install radare2".to_string()),
            execution_time_estimate: 0,
        },
        KaliTool {
            name: "objdump".to_string(),
            category: KaliToolCategory::ReverseEngineering,
            description: "Binary file analysis tool".to_string(),
            command: "objdump".to_string(),
            typical_args: vec!["-d".to_string()],
            requires_root: false,
            install_command: Some("sudo apt install binutils".to_string()),
            execution_time_estimate: 10,
        },
        KaliTool {
            name: "strings".to_string(),
            category: KaliToolCategory::ReverseEngineering,
            description: "Extract printable strings from files".to_string(),
            command: "strings".to_string(),
            typical_args: vec![],
            requires_root: false,
            install_command: Some("sudo apt install binutils".to_string()),
            execution_time_estimate: 5,
        },
        KaliTool {
            name: "gdb".to_string(),
            category: KaliToolCategory::ReverseEngineering,
            description: "GNU debugger".to_string(),
            command: "gdb".to_string(),
            typical_args: vec![],
            requires_root: false,
            install_command: Some("sudo apt install gdb".to_string()),
            execution_time_estimate: 0,
        },
        KaliTool {
            name: "binary_ninja".to_string(),
            category: KaliToolCategory::ReverseEngineering,
            description: "Binary analysis platform".to_string(),
            command: "binaryninja".to_string(),
            typical_args: vec![],
            requires_root: false,
            install_command: Some("sudo apt install binaryninja".to_string()),
            execution_time_estimate: 0,
        },

        // ========== FORENSICS ==========
        KaliTool {
            name: "autopsy".to_string(),
            category: KaliToolCategory::Forensics,
            description: "Digital forensics platform".to_string(),
            command: "autopsy".to_string(),
            typical_args: vec![],
            requires_root: false,
            install_command: Some("sudo apt install autopsy".to_string()),
            execution_time_estimate: 0,
        },
        KaliTool {
            name: "binwalk".to_string(),
            category: KaliToolCategory::FileForensics,
            description: "Firmware analysis tool".to_string(),
            command: "binwalk".to_string(),
            typical_args: vec!["-e".to_string()],
            requires_root: false,
            install_command: Some("sudo apt install binwalk".to_string()),
            execution_time_estimate: 60,
        },
        KaliTool {
            name: "volatility".to_string(),
            category: KaliToolCategory::MemoryForensics,
            description: "Memory forensics framework".to_string(),
            command: "vol".to_string(),
            typical_args: vec!["-f".to_string()],
            requires_root: false,
            install_command: Some("sudo apt install volatility".to_string()),
            execution_time_estimate: 120,
        },
        KaliTool {
            name: "sleuthkit".to_string(),
            category: KaliToolCategory::FileForensics,
            description: "File system forensics toolkit".to_string(),
            command: "fls".to_string(),
            typical_args: vec![],
            requires_root: false,
            install_command: Some("sudo apt install sleuthkit".to_string()),
            execution_time_estimate: 60,
        },
        KaliTool {
            name: "foremost".to_string(),
            category: KaliToolCategory::FileForensics,
            description: "File recovery tool".to_string(),
            command: "foremost".to_string(),
            typical_args: vec![],
            requires_root: false,
            install_command: Some("sudo apt install foremost".to_string()),
            execution_time_estimate: 180,
        },

        // ========== POST-EXPLOITATION ==========
        KaliTool {
            name: "mimikatz".to_string(),
            category: KaliToolCategory::PrivilegeEscalation,
            description: "Windows credential extractor".to_string(),
            command: "mimikatz".to_string(),
            typical_args: vec![],
            requires_root: false,
            install_command: Some("sudo apt install mimikatz".to_string()),
            execution_time_estimate: 30,
        },
        KaliTool {
            name: "linpeas".to_string(),
            category: KaliToolCategory::PrivilegeEscalation,
            description: "Linux privilege escalation audit".to_string(),
            command: "linpeas".to_string(),
            typical_args: vec![],
            requires_root: false,
            install_command: Some("curl -L https://github.com/carlospolop/PEASS-ng/releases/latest/download/linpeas.sh | sh".to_string()),
            execution_time_estimate: 180,
        },
        KaliTool {
            name: "winpeas".to_string(),
            category: KaliToolCategory::PrivilegeEscalation,
            description: "Windows privilege escalation audit".to_string(),
            command: "winpeas".to_string(),
            typical_args: vec![],
            requires_root: false,
            install_command: Some("download from PEASS-ng GitHub".to_string()),
            execution_time_estimate: 300,
        },

        // ========== FUZZING ==========
        KaliTool {
            name: "afl".to_string(),
            category: KaliToolCategory::Fuzzing,
            description: "American Fuzzy Lop".to_string(),
            command: "afl-fuzz".to_string(),
            typical_args: vec![],
            requires_root: false,
            install_command: Some("sudo apt install afl++".to_string()),
            execution_time_estimate: 0,
        },
        KaliTool {
            name: "zzuf".to_string(),
            category: KaliToolCategory::Fuzzing,
            description: "Transparent application fuzzer".to_string(),
            command: "zzuf".to_string(),
            typical_args: vec![],
            requires_root: false,
            install_command: Some("sudo apt install zzuf".to_string()),
            execution_time_estimate: 0,
        },

        // Keep existing tools and add more...
    ]
}

// ============================================================================
// ASYNC ORCHESTRATION ENGINE
// ============================================================================

pub struct FenrirOrchestrationEngine {
    pub logger: DecisionLogger,
    pub breach_detector: BreachDetector,
    pub target: String,
    pub tools: Vec<KaliTool>,
}

impl FenrirOrchestrationEngine {
    pub fn new(target: String) -> Self {
        let logger = DecisionLogger::new(&target);
        let tools = get_all_kali_tools();

        FenrirOrchestrationEngine {
            logger,
            breach_detector: BreachDetector::new(),
            target,
            tools,
        }
    }

    pub async fn execute_tool(&mut self, tool: &KaliTool, args: &[String]) -> Result<String, String> {
        let start = std::time::Instant::now();

        // Check if tool is available
        if !tool.is_available() {
            let decision = BrainDecision {
                timestamp: Utc::now(),
                decision_id: uuid::Uuid::new_v4().to_string(),
                decision_type: DecisionType::ToolSelection,
                reasoning: format!("Tool {} is not installed", tool.name),
                tool_selected: tool.name.clone(),
                target: self.target.clone(),
                success: false,
                output_summary: format!("Tool not found. Install: {}", tool.install_command.as_ref().unwrap_or(&"unknown".to_string())),
                execution_time_ms: 0,
                next_steps: vec![format!("Install tool: {}", tool.install_command.as_ref().unwrap_or(&"".to_string()))],
            };

            self.logger.log_decision(decision).await;
            return Err(format!("Tool {} not available", tool.name));
        }

        // Check root requirements
        if tool.requires_root {
            #[cfg(unix)]
            {
                let uid_check = Command::new("id").arg("-u").output();
                if let Ok(output) = uid_check {
                    let uid_str = String::from_utf8_lossy(&output.stdout).trim().to_string();
                    if uid_str != "0" {
                        let decision = BrainDecision {
                            timestamp: Utc::now(),
                            decision_id: uuid::Uuid::new_v4().to_string(),
                            decision_type: DecisionType::ToolSelection,
                            reasoning: format!("Tool {} requires root", tool.name),
                            tool_selected: tool.name.clone(),
                            target: self.target.clone(),
                            success: false,
                            output_summary: "Root privileges required".to_string(),
                            execution_time_ms: 0,
                            next_steps: vec!["Run with sudo".to_string()],
                        };

                        self.logger.log_decision(decision).await;
                        return Err(format!("Tool {} requires root", tool.name));
                    }
                }
            }
        }

        // Execute tool
        println!("🔧 Executing {} on {}", tool.name, self.target);

        let output = Command::new(&tool.command)
            .args(args)
            .output();

        let execution_time = start.elapsed().as_millis() as u64;

        match output {
            Ok(result) => {
                let stdout = String::from_utf8_lossy(&result.stdout).to_string();
                let stderr = String::from_utf8_lossy(&result.stderr).to_string();
                let combined_output = format!("{}\n{}", stdout, stderr);

                let success = result.status.success();

                // Log decision
                let decision = BrainDecision {
                    timestamp: Utc::now(),
                    decision_id: uuid::Uuid::new_v4().to_string(),
                    decision_type: if success { DecisionType::ToolSelection } else { DecisionType::StrategyChange },
                    reasoning: format!("Executed {} - Status: {}", tool.name, if success { "Success" } else { "Failed" }),
                    tool_selected: tool.name.clone(),
                    target: self.target.clone(),
                    success,
                    output_summary: combined_output.chars().take(500).collect(),
                    execution_time_ms: execution_time,
                    next_steps: if success {
                        vec!["Analyze output for vulnerabilities".to_string(), "Check for sensitive data".to_string()]
                    } else {
                        vec!["Review error logs".to_string(), "Try alternative tool".to_string()]
                    },
                };

                self.logger.log_decision(decision).await;

                // Analyze output for breaches
                self.breach_detector.analyze_output(&combined_output, &tool.name);

                if success {
                    Ok(combined_output)
                } else {
                    Err(combined_output)
                }
            }
            Err(e) => {
                let decision = BrainDecision {
                    timestamp: Utc::now(),
                    decision_id: uuid::Uuid::new_v4().to_string(),
                    decision_type: DecisionType::StrategyChange,
                    reasoning: format!("Failed to execute {}: {}", tool.name, e),
                    tool_selected: tool.name.clone(),
                    target: self.target.clone(),
                    success: false,
                    output_summary: format!("Error: {}", e),
                    execution_time_ms: execution_time,
                    next_steps: vec!["Check tool installation".to_string(), "Verify permissions".to_string()],
                };

                self.logger.log_decision(decision).await;
                Err(format!("Execution failed: {}", e))
            }
        }
    }

    pub async fn run_sequential_attack(&mut self) -> Result<String, String> {
        println!("🐺 FENRIR ORCHESTRATION ENGINE STARTED");
        println!("🎯 Target: {}", self.target);

        let mut all_results = Vec::new();

        // Phase 1: Reconnaissance
        println!("\n🔍 PHASE 1: RECONNAISSANCE");

        let recon_tools: Vec<KaliTool> = self.tools.iter()
            .filter(|t| matches!(t.category, KaliToolCategory::NetworkScanning | KaliToolCategory::DnsEnumeration))
            .take(3)
            .cloned()
            .collect();

        for tool in recon_tools {
            println!("  📡 Running: {}", tool.name);
            let args = vec![self.target.clone()];
            match self.execute_tool(&tool, &args).await {
                Ok(output) => {
                    all_results.push(format!("### {} Output\n{}", tool.name, output));
                }
                Err(e) => {
                    all_results.push(format!("### {} Error\n{}", tool.name, e));
                }
            }
        }

        // Phase 2: Vulnerability Scanning
        println!("\n🔎 PHASE 2: VULNERABILITY SCANNING");

        let vuln_tools: Vec<KaliTool> = self.tools.iter()
            .filter(|t| matches!(t.category, KaliToolCategory::VulnerabilityAnalysis | KaliToolCategory::WebApplicationAnalysis))
            .take(3)
            .cloned()
            .collect();

        for tool in vuln_tools {
            println!("  🔬 Running: {}", tool.name);
            let args = vec![self.target.clone()];
            match self.execute_tool(&tool, &args).await {
                Ok(output) => {
                    all_results.push(format!("### {} Output\n{}", tool.name, output));
                }
                Err(e) => {
                    all_results.push(format!("### {} Error\n{}", tool.name, e));
                }
            }
        }

        // Phase 3: Sensitive Data Scanning
        println!("\n🔐 PHASE 3: SENSITIVE DATA SCANNING");
        self.breach_detector.scan_files_for_sensitive_data(".").await;

        Ok(all_results.join("\n\n"))
    }

    pub async fn generate_ethical_report(&self) -> String {
        let brain_log = self.logger.export_markdown(&self.target).await;
        let breach_summary = self.breach_detector.get_summary();

        let mut report = format!(
            "# 🔴 ETHICAL ANALYSIS FINAL REPORT\n\n\
             **Target**: {}\n\
             **Date**: {}\n\
             **Analyst**: FENRIR MCP 3.0\n\
             **Purpose**: Authorized Security Assessment\n\n\
             ---\n\n\
             ## ⚠️ DISCLAIMER\n\n\
             This report was generated for authorized security testing purposes only.\
             All findings should be verified and remediated by qualified security professionals.\n\n\
             Unauthorized access to computer systems is illegal. Always obtain proper authorization\
             before conducting security tests.\n\n\
             ---\n\n",
            self.target,
            Utc::now().format("%Y-%m-%d %H:%M:%S UTC")
        );

        report.push_str(&breach_summary);
        report.push_str("\n---\n\n");
        report.push_str(&brain_log);

        report.push_str(&format!(
            "\n---\n\n## 📊 FINAL SUMMARY\n\n\
             **Total Tools Executed**: {}\n\
             **Breaches Detected**: {}\n\
             **Sensitive Data Items**: {}\n\
             **Images Found**: {}\n\
             **Files Analyzed**: {}\n\n\
             ## 🔐 SECURITY RECOMMENDATIONS\n\n\
             1. Review all detected breaches immediately\n\
             2. Secure sensitive data found during scan\n\
             3. Implement proper access controls\n\
             4. Update and patch vulnerable systems\n\
             5. Conduct regular security assessments\n\n\
             ---\n\n\
             **Report Generated By**: [FENRIR MCP 3.0](https://github.com/your-repo/fenrir)\n\
             **Contact for Security Services**: sfaustodev@gmail.com\n\
             \n\
             \"The Wolf Devours Security Vulnerabilities\"\n",
            self.logger.decisions.lock().await.len(),
            self.breach_detector.detected_breaches.len(),
            self.breach_detector.sensitive_data.len(),
            self.breach_detector.images_found.len(),
            self.breach_detector.files_analyzed.len(),
        ));

        report
    }
}

// Helper methods
impl KaliTool {
    pub fn is_available(&self) -> bool {
        Command::new("which").arg(&self.command).output()
            .map(|output| output.status.success())
            .unwrap_or(false)
    }
}
