// --- FENRIR COMPREHENSIVE KALI LINUX TOOLS INTEGRATION ---
// 100+ Kali tools with async orchestration and detailed logging
// For authorized security testing only

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::process::Command;
use std::sync::Arc;
use tokio::sync::Mutex;

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
        let log_file = format!(
            "fenrir_logs/brain_{}_{}.json",
            target.replace(".", "_"),
            timestamp
        );

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
        report.push_str(&format!(
            "**Generated**: {}\n\n",
            Utc::now().format("%Y-%m-%d %H:%M:%S UTC")
        ));

        report.push_str("---\n\n");

        for (idx, decision) in decisions.iter().enumerate() {
            report.push_str(&format!(
                "## Decision {}: {:?}\n\n",
                idx + 1,
                decision.decision_type
            ));
            report.push_str(&format!(
                "**Time**: {}\n",
                decision.timestamp.format("%H:%M:%S")
            ));
            report.push_str(&format!("**Tool**: {}\n", decision.tool_selected));
            report.push_str(&format!("**Target**: {}\n", decision.target));
            report.push_str(&format!("**Success**: {}\n\n", decision.success));
            report.push_str(&format!("**Reasoning**:\n{}\n\n", decision.reasoning));
            report.push_str(&format!(
                "**Output**:\n```\n{}\n```\n\n",
                decision.output_summary
            ));
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
    Critical, // Immediate action required
    #[serde(rename = "high")]
    High, // Urgent attention needed
    #[serde(rename = "medium")]
    Medium, // Should be addressed
    #[serde(rename = "low")]
    Low, // Informational
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
        if tool_output.to_lowercase().contains("sql")
            && (tool_output.to_lowercase().contains("inject")
                || tool_output.to_lowercase().contains("syntax error")
                || tool_output.to_lowercase().contains("mysql"))
        {
            self.detected_breaches.push(Breach {
                breach_id: uuid::Uuid::new_v4().to_string(),
                severity: BreachSeverity::Critical,
                breach_type: BreachType::SQLInjection,
                description: format!(
                    "Potential SQL injection vulnerability detected by {}",
                    tool_name
                ),
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
        if tool_output.to_lowercase().contains("xss")
            || tool_output.to_lowercase().contains("cross-site")
            || tool_output.contains("<script>")
        {
            self.detected_breaches.push(Breach {
                breach_id: uuid::Uuid::new_v4().to_string(),
                severity: BreachSeverity::High,
                breach_type: BreachType::XSS,
                description: format!(
                    "Cross-site scripting vulnerability detected by {}",
                    tool_name
                ),
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
        if tool_output.to_lowercase().contains("admin")
            && (tool_output.to_lowercase().contains("bypass")
                || tool_output.to_lowercase().contains("unauthorized")
                || tool_output.to_lowercase().contains("authentication"))
        {
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
        let email_regex =
            regex::Regex::new(r"[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}").unwrap();
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
        if tool_output.contains("api_key")
            || tool_output.contains("apikey")
            || tool_output.contains("API_KEY")
            || tool_output.contains("secret")
        {
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
                if matches!(
                    ext.to_str(),
                    Some("jpg")
                        | Some("jpeg")
                        | Some("png")
                        | Some("gif")
                        | Some("bmp")
                        | Some("svg")
                        | Some("webp")
                ) {
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
                if matches!(
                    ext.to_str(),
                    Some("pdf")
                        | Some("doc")
                        | Some("docx")
                        | Some("txt")
                        | Some("xls")
                        | Some("xlsx")
                        | Some("ppt")
                ) {
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

        summary.push_str(&format!(
            "**Breaches Detected**: {}\n",
            self.detected_breaches.len()
        ));
        summary.push_str(&format!(
            "**Sensitive Data Items**: {}\n",
            self.sensitive_data.len()
        ));
        summary.push_str(&format!(
            "**Files Analyzed**: {}\n",
            self.files_analyzed.len()
        ));
        summary.push_str(&format!(
            "**Images Found**: {}\n\n",
            self.images_found.len()
        ));

        if !self.detected_breaches.is_empty() {
            summary.push_str("### 🚨 DETECTED BREACHES\n\n");
            for (idx, breach) in self.detected_breaches.iter().enumerate() {
                summary.push_str(&format!(
                    "#### {}. {:?} - {:?}\n\n",
                    idx + 1,
                    breach.breach_type,
                    breach.severity
                ));
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
            install_command: Some("brew install nmap".to_string()),
            execution_time_estimate: 60,
        },
        KaliTool {
            name: "masscan".to_string(),
            category: KaliToolCategory::NetworkScanning,
            description: "Mass IP port scanner".to_string(),
            command: "masscan".to_string(),
            typical_args: vec!["-p80,8000-8100".to_string()],
            requires_root: true,
            install_command: Some("brew install masscan".to_string()),
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
            install_command: Some("brew install dnsenum".to_string()),
            execution_time_estimate: 45,
        },
        KaliTool {
            name: "dnsrecon".to_string(),
            category: KaliToolCategory::DnsEnumeration,
            description: "DNS reconnaissance script".to_string(),
            command: "dnsrecon".to_string(),
            typical_args: vec!["-d".to_string()],
            requires_root: false,
            install_command: Some("brew install dnsrecon".to_string()),
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
            install_command: Some("brew install theharvester".to_string()),
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
            install_command: Some("brew install maltego".to_string()),
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
            install_command: Some("brew install tshark".to_string()),
            execution_time_estimate: 0,
        },
        KaliTool {
            name: "tcpdump".to_string(),
            category: KaliToolCategory::NetworkSniffing,
            description: "Packet analyzer".to_string(),
            command: "tcpdump".to_string(),
            typical_args: vec!["-i".to_string(), "eth0".to_string()],
            requires_root: true,
            install_command: Some("brew install tcpdump".to_string()),
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
            install_command: Some("brew install nikto".to_string()),
            execution_time_estimate: 120,
        },
        KaliTool {
            name: "sqlmap".to_string(),
            category: KaliToolCategory::DatabaseExploitation,
            description: "Automatic SQL injection tool".to_string(),
            command: "sqlmap".to_string(),
            typical_args: vec!["-u".to_string(), "--batch".to_string()],
            requires_root: false,
            install_command: Some("brew install sqlmap".to_string()),
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
            install_command: Some("brew install nessus".to_string()),
            execution_time_estimate: 600,
        },
        KaliTool {
            name: "openvas".to_string(),
            category: KaliToolCategory::VulnerabilityAnalysis,
            description: "Open source vulnerability scanner".to_string(),
            command: "openvas".to_string(),
            typical_args: vec![],
            requires_root: false,
            install_command: Some("brew install openvas".to_string()),
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
            install_command: Some("brew install metasploit-framework".to_string()),
            execution_time_estimate: 0,
        },
        KaliTool {
            name: "searchsploit".to_string(),
            category: KaliToolCategory::ExploitationTools,
            description: "Exploit database search".to_string(),
            command: "searchsploit".to_string(),
            typical_args: vec![],
            requires_root: false,
            install_command: Some("brew install exploitdb".to_string()),
            execution_time_estimate: 10,
        },
        KaliTool {
            name: "empire".to_string(),
            category: KaliToolCategory::PostExploitation,
            description: "Post-exploitation framework".to_string(),
            command: "empire".to_string(),
            typical_args: vec![],
            requires_root: false,
            install_command: Some("brew install empire".to_string()),
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
            install_command: Some("brew install john".to_string()),
            execution_time_estimate: 300,
        },
        KaliTool {
            name: "hashcat".to_string(),
            category: KaliToolCategory::PasswordAttacks,
            description: "GPU-based password recovery".to_string(),
            command: "hashcat".to_string(),
            typical_args: vec!["-m".to_string(), "0".to_string()],
            requires_root: false,
            install_command: Some("brew install hashcat".to_string()),
            execution_time_estimate: 300,
        },
        KaliTool {
            name: "hydra".to_string(),
            category: KaliToolCategory::PasswordAttacks,
            description: "Parallel login cracker".to_string(),
            command: "hydra".to_string(),
            typical_args: vec!["-l".to_string(), "user".to_string()],
            requires_root: false,
            install_command: Some("brew install hydra".to_string()),
            execution_time_estimate: 180,
        },
        KaliTool {
            name: "medusa".to_string(),
            category: KaliToolCategory::PasswordAttacks,
            description: "Parallel login brute-forcer".to_string(),
            command: "medusa".to_string(),
            typical_args: vec!["-h".to_string()],
            requires_root: false,
            install_command: Some("Linux only: sudo apt install medusa (Not available on macOS)".to_string()),
            execution_time_estimate: 180,
        },
        KaliTool {
            name: "cewl".to_string(),
            category: KaliToolCategory::PasswordAttacks,
            description: "Custom wordlist generator".to_string(),
            command: "cewl".to_string(),
            typical_args: vec!["-w".to_string(), "wordlist.txt".to_string()],
            requires_root: false,
            install_command: Some("brew install cewl".to_string()),
            execution_time_estimate: 60,
        },
        KaliTool {
            name: "crunch".to_string(),
            category: KaliToolCategory::PasswordAttacks,
            description: "Wordlist generator".to_string(),
            command: "crunch".to_string(),
            typical_args: vec!["8".to_string(), "8".to_string()],
            requires_root: false,
            install_command: Some("brew install crunch".to_string()),
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
            install_command: Some("brew install aircrack-ng".to_string()),
            execution_time_estimate: 300,
        },
        KaliTool {
            name: "wifite".to_string(),
            category: KaliToolCategory::WirelessAttacks,
            description: "Automated wireless attack tool".to_string(),
            command: "wifite".to_string(),
            typical_args: vec!["--all".to_string()],
            requires_root: true,
            install_command: Some("brew install wifite".to_string()),
            execution_time_estimate: 300,
        },
        KaliTool {
            name: "reaver".to_string(),
            category: KaliToolCategory::WirelessAttacks,
            description: "WPS PIN brute force attack".to_string(),
            command: "reaver".to_string(),
            typical_args: vec!["-b".to_string()],
            requires_root: true,
            install_command: Some("brew install reaver".to_string()),
            execution_time_estimate: 600,
        },
        KaliTool {
            name: "bully".to_string(),
            category: KaliToolCategory::WirelessAttacks,
            description: "WPS PIN brute force attack".to_string(),
            command: "bully".to_string(),
            typical_args: vec![],
            requires_root: true,
            install_command: Some("brew install bully".to_string()),
            execution_time_estimate: 600,
        },
        KaliTool {
            name: "kismet".to_string(),
            category: KaliToolCategory::WirelessAttacks,
            description: "Wireless network detector".to_string(),
            command: "kismet".to_string(),
            typical_args: vec![],
            requires_root: true,
            install_command: Some("brew install kismet".to_string()),
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
            install_command: Some("brew install burpsuite".to_string()),
            execution_time_estimate: 0,
        },
        KaliTool {
            name: "owasp-zap".to_string(),
            category: KaliToolCategory::WebApplicationAnalysis,
            description: "Web application security scanner".to_string(),
            command: "zap-cli".to_string(),
            typical_args: vec!["quick-scan".to_string()],
            requires_root: false,
            install_command: Some("brew install zaproxy".to_string()),
            execution_time_estimate: 300,
        },
        KaliTool {
            name: "dirb".to_string(),
            category: KaliToolCategory::WebApplicationAnalysis,
            description: "Web content scanner".to_string(),
            command: "dirb".to_string(),
            typical_args: vec![],
            requires_root: false,
            install_command: Some("brew install dirb".to_string()),
            execution_time_estimate: 120,
        },
        KaliTool {
            name: "gobuster".to_string(),
            category: KaliToolCategory::WebApplicationAnalysis,
            description: "Directory/file & DNS busting tool".to_string(),
            command: "gobuster".to_string(),
            typical_args: vec!["dir".to_string(), "-u".to_string()],
            requires_root: false,
            install_command: Some("brew install gobuster".to_string()),
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
            install_command: Some("brew install ettercap-text-only".to_string()),
            execution_time_estimate: 0,
        },
        KaliTool {
            name: "bettercap".to_string(),
            category: KaliToolCategory::ManInTheMiddle,
            description: "Swiss army knife for network attacks".to_string(),
            command: "bettercap".to_string(),
            typical_args: vec![],
            requires_root: true,
            install_command: Some("brew install bettercap".to_string()),
            execution_time_estimate: 0,
        },
        KaliTool {
            name: "mitmproxy".to_string(),
            category: KaliToolCategory::ManInTheMiddle,
            description: "Interactive HTTPS proxy".to_string(),
            command: "mitmproxy".to_string(),
            typical_args: vec![],
            requires_root: false,
            install_command: Some("brew install mitmproxy".to_string()),
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
            install_command: Some("brew install ghidra".to_string()),
            execution_time_estimate: 0,
        },
        KaliTool {
            name: "radare2".to_string(),
            category: KaliToolCategory::ReverseEngineering,
            description: "Reverse engineering framework".to_string(),
            command: "r2".to_string(),
            typical_args: vec!["-A".to_string()],
            requires_root: false,
            install_command: Some("brew install radare2".to_string()),
            execution_time_estimate: 0,
        },
        KaliTool {
            name: "objdump".to_string(),
            category: KaliToolCategory::ReverseEngineering,
            description: "Binary file analysis tool".to_string(),
            command: "objdump".to_string(),
            typical_args: vec!["-d".to_string()],
            requires_root: false,
            install_command: Some("brew install binutils".to_string()),
            execution_time_estimate: 10,
        },
        KaliTool {
            name: "strings".to_string(),
            category: KaliToolCategory::ReverseEngineering,
            description: "Extract printable strings from files".to_string(),
            command: "strings".to_string(),
            typical_args: vec![],
            requires_root: false,
            install_command: Some("brew install binutils".to_string()),
            execution_time_estimate: 5,
        },
        KaliTool {
            name: "gdb".to_string(),
            category: KaliToolCategory::ReverseEngineering,
            description: "GNU debugger".to_string(),
            command: "gdb".to_string(),
            typical_args: vec![],
            requires_root: false,
            install_command: Some("brew install gdb".to_string()),
            execution_time_estimate: 0,
        },
        KaliTool {
            name: "binary_ninja".to_string(),
            category: KaliToolCategory::ReverseEngineering,
            description: "Binary analysis platform".to_string(),
            command: "binaryninja".to_string(),
            typical_args: vec![],
            requires_root: false,
            install_command: Some("brew install binaryninja".to_string()),
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
            install_command: Some("brew install autopsy".to_string()),
            execution_time_estimate: 0,
        },
        KaliTool {
            name: "binwalk".to_string(),
            category: KaliToolCategory::FileForensics,
            description: "Firmware analysis tool".to_string(),
            command: "binwalk".to_string(),
            typical_args: vec!["-e".to_string()],
            requires_root: false,
            install_command: Some("brew install binwalk".to_string()),
            execution_time_estimate: 60,
        },
        KaliTool {
            name: "volatility".to_string(),
            category: KaliToolCategory::MemoryForensics,
            description: "Memory forensics framework".to_string(),
            command: "vol".to_string(),
            typical_args: vec!["-f".to_string()],
            requires_root: false,
            install_command: Some("brew install volatility".to_string()),
            execution_time_estimate: 120,
        },
        KaliTool {
            name: "sleuthkit".to_string(),
            category: KaliToolCategory::FileForensics,
            description: "File system forensics toolkit".to_string(),
            command: "fls".to_string(),
            typical_args: vec![],
            requires_root: false,
            install_command: Some("brew install sleuthkit".to_string()),
            execution_time_estimate: 60,
        },
        KaliTool {
            name: "foremost".to_string(),
            category: KaliToolCategory::FileForensics,
            description: "File recovery tool".to_string(),
            command: "foremost".to_string(),
            typical_args: vec![],
            requires_root: false,
            install_command: Some("brew install foremost".to_string()),
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
            install_command: Some("brew install mimikatz".to_string()),
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
            install_command: Some("brew install afl++".to_string()),
            execution_time_estimate: 0,
        },
        KaliTool {
            name: "zzuf".to_string(),
            category: KaliToolCategory::Fuzzing,
            description: "Transparent application fuzzer".to_string(),
            command: "zzuf".to_string(),
            typical_args: vec![],
            requires_root: false,
            install_command: Some("brew install zzuf".to_string()),
            execution_time_estimate: 0,
        },

        // ========== ADDITIONAL INFORMATION GATHERING ==========
        KaliTool {
            name: "amass".to_string(),
            category: KaliToolCategory::OSInt,
            description: "In-depth subdomain enumeration".to_string(),
            command: "amass".to_string(),
            typical_args: vec!["enum".to_string(), "-d".to_string()],
            requires_root: false,
            install_command: Some("go install -v github.com/OWASP/Amass/v3/...@master".to_string()),
            execution_time_estimate: 300,
        },
        KaliTool {
            name: "subfinder".to_string(),
            category: KaliToolCategory::DnsEnumeration,
            description: "Subdomain discovery tool".to_string(),
            command: "subfinder".to_string(),
            typical_args: vec!["-d".to_string()],
            requires_root: false,
            install_command: Some("go install -v github.com/projectdiscovery/subfinder/v2/cmd/subfinder@latest".to_string()),
            execution_time_estimate: 120,
        },
        KaliTool {
            name: "assetfinder".to_string(),
            category: KaliToolCategory::DnsEnumeration,
            description: "Find domains and subdomains".to_string(),
            command: "assetfinder".to_string(),
            typical_args: vec!["--subs-only".to_string()],
            requires_root: false,
            install_command: Some("go install github.com/tomnomnom/assetfinder@latest".to_string()),
            execution_time_estimate: 90,
        },
        KaliTool {
            name: "fierce".to_string(),
            category: KaliToolCategory::DnsEnumeration,
            description: "DNS reconnaissance tool".to_string(),
            command: "fierce".to_string(),
            typical_args: vec!["-dns".to_string()],
            requires_root: false,
            install_command: Some("brew install fierce".to_string()),
            execution_time_estimate: 180,
        },
        KaliTool {
            name: "dnsrecon".to_string(),
            category: KaliToolCategory::DnsEnumeration,
            description: "DNS enumeration script".to_string(),
            command: "dnsrecon".to_string(),
            typical_args: vec!["-d".to_string()],
            requires_root: false,
            install_command: Some("brew install dnsrecon".to_string()),
            execution_time_estimate: 120,
        },
        KaliTool {
            name: "recon-ng".to_string(),
            category: KaliToolCategory::OSInt,
            description: "Web reconnaissance framework".to_string(),
            command: "recon-ng".to_string(),
            typical_args: vec![],
            requires_root: false,
            install_command: Some("brew install recon-ng".to_string()),
            execution_time_estimate: 0,
        },
        KaliTool {
            name: "spiderfoot".to_string(),
            category: KaliToolCategory::OSInt,
            description: "OSINT automation tool".to_string(),
            command: "spiderfoot".to_string(),
            typical_args: vec![],
            requires_root: false,
            install_command: Some("brew install spiderfoot".to_string()),
            execution_time_estimate: 0,
        },

        // ========== ADDITIONAL VULNERABILITY ANALYSIS ==========
        KaliTool {
            name: "qualys".to_string(),
            category: KaliToolCategory::VulnerabilityAnalysis,
            description: "Vulnerability management platform".to_string(),
            command: "qualys".to_string(),
            typical_args: vec![],
            requires_root: false,
            install_command: Some("brew install qualys".to_string()),
            execution_time_estimate: 600,
        },
        KaliTool {
            name: "rapid7".to_string(),
            category: KaliToolCategory::VulnerabilityAnalysis,
            description: "Vulnerability scanner".to_string(),
            command: "rapid7".to_string(),
            typical_args: vec![],
            requires_root: false,
            install_command: Some("brew install rapid7".to_string()),
            execution_time_estimate: 600,
        },
        KaliTool {
            name: "acunetix".to_string(),
            category: KaliToolCategory::WebApplicationAnalysis,
            description: "Web vulnerability scanner".to_string(),
            command: "acunetix".to_string(),
            typical_args: vec![],
            requires_root: false,
            install_command: Some("brew install acunetix".to_string()),
            execution_time_estimate: 600,
        },

        // ========== ADDITIONAL WEB APPLICATION TOOLS ==========
        KaliTool {
            name: "dirbuster".to_string(),
            category: KaliToolCategory::WebApplicationAnalysis,
            description: "Web content scanner".to_string(),
            command: "dirbuster".to_string(),
            typical_args: vec![],
            requires_root: false,
            install_command: Some("brew install dirbuster".to_string()),
            execution_time_estimate: 300,
        },
        KaliTool {
            name: "dirsearch".to_string(),
            category: KaliToolCategory::WebApplicationAnalysis,
            description: "Web path scanner".to_string(),
            command: "dirsearch".to_string(),
            typical_args: vec!["-u".to_string()],
            requires_root: false,
            install_command: Some("pip3 install dirsearch".to_string()),
            execution_time_estimate: 180,
        },
        KaliTool {
            name: "wfuzz".to_string(),
            category: KaliToolCategory::WebApplicationAnalysis,
            description: "Web application brute forcer".to_string(),
            command: "wfuzz".to_string(),
            typical_args: vec![],
            requires_root: false,
            install_command: Some("brew install wfuzz".to_string()),
            execution_time_estimate: 120,
        },
        KaliTool {
            name: "sqlninja".to_string(),
            category: KaliToolCategory::DatabaseExploitation,
            description: "SQL injection tool".to_string(),
            command: "sqlninja".to_string(),
            typical_args: vec![],
            requires_root: false,
            install_command: Some("brew install sqlninja".to_string()),
            execution_time_estimate: 180,
        },
        KaliTool {
            name: "commix".to_string(),
            category: KaliToolCategory::ExploitationTools,
            description: "Command injection exploiter".to_string(),
            command: "commix".to_string(),
            typical_args: vec!["--url".to_string()],
            requires_root: false,
            install_command: Some("brew install commix".to_string()),
            execution_time_estimate: 120,
        },
        KaliTool {
            name: "joomlavs".to_string(),
            category: KaliToolCategory::WebApplicationAnalysis,
            description: "Joomla vulnerability scanner".to_string(),
            command: "joomlavs".to_string(),
            typical_args: vec![],
            requires_root: false,
            install_command: Some("brew install joomlavs".to_string()),
            execution_time_estimate: 120,
        },
        KaliTool {
            name: "wpscan".to_string(),
            category: KaliToolCategory::WebApplicationAnalysis,
            description: "WordPress vulnerability scanner".to_string(),
            command: "wpscan".to_string(),
            typical_args: vec!["--url".to_string()],
            requires_root: false,
            install_command: Some("brew install wpscan".to_string()),
            execution_time_estimate: 180,
        },
        KaliTool {
            name: "droopescan".to_string(),
            category: KaliToolCategory::WebApplicationAnalysis,
            description: "CMS vulnerability scanner".to_string(),
            command: "droopescan".to_string(),
            typical_args: vec!["scan".to_string(), "drupal".to_string()],
            requires_root: false,
            install_command: Some("brew install droopescan".to_string()),
            execution_time_estimate: 120,
        },
        KaliTool {
            name: "cmsmap".to_string(),
            category: KaliToolCategory::WebApplicationAnalysis,
            description: "CMS scanner".to_string(),
            command: "cmsmap".to_string(),
            typical_args: vec![],
            requires_root: false,
            install_command: Some("brew install cmsmap".to_string()),
            execution_time_estimate: 90,
        },

        // ========== ADDITIONAL WIRELESS TOOLS ==========
        KaliTool {
            name: "fluxion".to_string(),
            category: KaliToolCategory::WirelessAttacks,
            description: "WiFi attack framework".to_string(),
            command: "fluxion".to_string(),
            typical_args: vec![],
            requires_root: true,
            install_command: Some("git clone https://github.com/FluxionNetwork/fluxion.git".to_string()),
            execution_time_estimate: 600,
        },
        KaliTool {
            name: "pixiewps".to_string(),
            category: KaliToolCategory::WirelessAttacks,
            description: "WPS PIN attack tool".to_string(),
            command: "pixiewps".to_string(),
            typical_args: vec![],
            requires_root: false,
            install_command: Some("brew install pixiewps".to_string()),
            execution_time_estimate: 300,
        },
        KaliTool {
            name: "fern-wifi-cracker".to_string(),
            category: KaliToolCategory::WirelessAttacks,
            description: "Wireless security auditing tool".to_string(),
            command: "fern-wifi-cracker".to_string(),
            typical_args: vec![],
            requires_root: true,
            install_command: Some("brew install fern-wifi-cracker".to_string()),
            execution_time_estimate: 0,
        },
        KaliTool {
            name: "ghost-phisher".to_string(),
            category: KaliToolCategory::WirelessAttacks,
            description: "Wireless and Ethernet security auditing".to_string(),
            command: "ghost-phisher".to_string(),
            typical_args: vec![],
            requires_root: true,
            install_command: Some("brew install ghost-phisher".to_string()),
            execution_time_estimate: 0,
        },
        KaliTool {
            name: "airgeddon".to_string(),
            category: KaliToolCategory::WirelessAttacks,
            description: "Wireless attack script".to_string(),
            command: "airgeddon".to_string(),
            typical_args: vec![],
            requires_root: true,
            install_command: Some("git clone https://github.com/v1s1t0r1sh3r3/airgeddon.git".to_string()),
            execution_time_estimate: 0,
        },

        // ========== ADDITIONAL PASSWORD ATTACK TOOLS ==========
        KaliTool {
            name: "patator".to_string(),
            category: KaliToolCategory::PasswordAttacks,
            description: "Multi-purpose brute-forcer".to_string(),
            command: "patator".to_string(),
            typical_args: vec![],
            requires_root: false,
            install_command: Some("Linux only: sudo apt install patator (Not available on macOS)".to_string()),
            execution_time_estimate: 300,
        },
        KaliTool {
            name: "thc-hydra".to_string(),
            category: KaliToolCategory::PasswordAttacks,
            description: "Network login cracker".to_string(),
            command: "hydra".to_string(),
            typical_args: vec!["-l".to_string(), "user".to_string()],
            requires_root: false,
            install_command: Some("brew install hydra".to_string()),
            execution_time_estimate: 300,
        },
        KaliTool {
            name: "ncrack".to_string(),
            category: KaliToolCategory::PasswordAttacks,
            description: "Network authentication cracking tool".to_string(),
            command: "ncrack".to_string(),
            typical_args: vec![],
            requires_root: false,
            install_command: Some("Linux only: sudo apt install ncrack (Not available on macOS)".to_string()),
            execution_time_estimate: 300,
        },
        KaliTool {
            name: "crowbar".to_string(),
            category: KaliToolCategory::PasswordAttacks,
            description: "Brute force tool for various protocols".to_string(),
            command: "crowbar".to_string(),
            typical_args: vec![],
            requires_root: false,
            install_command: Some("brew install crowbar".to_string()),
            execution_time_estimate: 300,
        },
        KaliTool {
            name: "rsmangler".to_string(),
            category: KaliToolCategory::PasswordAttacks,
            description: "Wordlist mangling tool".to_string(),
            command: "rsmangler".to_string(),
            typical_args: vec![],
            requires_root: false,
            install_command: Some("brew install rsmangler".to_string()),
            execution_time_estimate: 60,
        },
        KaliTool {
            name: "cupp".to_string(),
            category: KaliToolCategory::PasswordAttacks,
            description: "Common User Passwords Profiler".to_string(),
            command: "cupp".to_string(),
            typical_args: vec!["-i".to_string()],
            requires_root: false,
            install_command: Some("brew install cupp".to_string()),
            execution_time_estimate: 30,
        },

        // ========== ADDITIONAL EXPLOITATION TOOLS ==========
        KaliTool {
            name: "armitage".to_string(),
            category: KaliToolCategory::ExploitationTools,
            description: "Metasploit GUI".to_string(),
            command: "armitage".to_string(),
            typical_args: vec![],
            requires_root: false,
            install_command: Some("brew install armitage".to_string()),
            execution_time_estimate: 0,
        },
        KaliTool {
            name: "cobalt-strike".to_string(),
            category: KaliToolCategory::ExploitationTools,
            description: "Advanced threat emulation".to_string(),
            command: "cobalt-strike".to_string(),
            typical_args: vec![],
            requires_root: false,
            install_command: Some("Commercial tool - requires license".to_string()),
            execution_time_estimate: 0,
        },
        KaliTool {
            name: "veil-evasion".to_string(),
            category: KaliToolCategory::ExploitDevelopment,
            description: "Payload generator".to_string(),
            command: "veil-evasion".to_string(),
            typical_args: vec![],
            requires_root: false,
            install_command: Some("brew install veil".to_string()),
            execution_time_estimate: 60,
        },
        KaliTool {
            name: "unicorn".to_string(),
            category: KaliToolCategory::ExploitDevelopment,
            description: "Shellcode generator".to_string(),
            command: "unicorn".to_string(),
            typical_args: vec![],
            requires_root: false,
            install_command: Some("pip3 install unicorn".to_string()),
            execution_time_estimate: 30,
        },
        KaliTool {
            name: "msfvenom".to_string(),
            category: KaliToolCategory::ExploitDevelopment,
            description: "Payload generator".to_string(),
            command: "msfvenom".to_string(),
            typical_args: vec!["-p".to_string(), "windows/meterpreter/reverse_tcp".to_string()],
            requires_root: false,
            install_command: Some("brew install metasploit-framework".to_string()),
            execution_time_estimate: 30,
        },
        KaliTool {
            name: "ysoserial".to_string(),
            category: KaliToolCategory::ExploitDevelopment,
            description: "Java deserialization exploit".to_string(),
            command: "ysoserial".to_string(),
            typical_args: vec![],
            requires_root: false,
            install_command: Some("brew install ysoserial".to_string()),
            execution_time_estimate: 30,
        },

        // ========== ADDITIONAL FORENSICS TOOLS ==========
        KaliTool {
            name: "guymager".to_string(),
            category: KaliToolCategory::Forensics,
            description: "Forensic imager".to_string(),
            command: "guymager".to_string(),
            typical_args: vec![],
            requires_root: true,
            install_command: Some("brew install guymager".to_string()),
            execution_time_estimate: 1800,
        },
        KaliTool {
            name: "dc3dd".to_string(),
            category: KaliToolCategory::Forensics,
            description: "Enhanced dd for forensics".to_string(),
            command: "dc3dd".to_string(),
            typical_args: vec![],
            requires_root: true,
            install_command: Some("brew install dc3dd".to_string()),
            execution_time_estimate: 1800,
        },
        KaliTool {
            name: "extundelete".to_string(),
            category: KaliToolCategory::FileForensics,
            description: "Ext filesystem undelete tool".to_string(),
            command: "extundelete".to_string(),
            typical_args: vec![],
            requires_root: true,
            install_command: Some("brew install extundelete".to_string()),
            execution_time_estimate: 300,
        },
        KaliTool {
            name: "scalpel".to_string(),
            category: KaliToolCategory::FileForensics,
            description: "File carver".to_string(),
            command: "scalpel".to_string(),
            typical_args: vec![],
            requires_root: false,
            install_command: Some("brew install scalpel".to_string()),
            execution_time_estimate: 600,
        },
        KaliTool {
            name: "testdisk".to_string(),
            category: KaliToolCategory::FileForensics,
            description: "Data recovery tool".to_string(),
            command: "testdisk".to_string(),
            typical_args: vec![],
            requires_root: true,
            install_command: Some("brew install testdisk".to_string()),
            execution_time_estimate: 0,
        },
        KaliTool {
            name: "photorec".to_string(),
            category: KaliToolCategory::FileForensics,
            description: "File recovery tool".to_string(),
            command: "photorec".to_string(),
            typical_args: vec![],
            requires_root: false,
            install_command: Some("brew install testdisk".to_string()),
            execution_time_estimate: 600,
        },
        KaliTool {
            name: "bulk_extractor".to_string(),
            category: KaliToolCategory::FileForensics,
            description: "Feature extraction tool".to_string(),
            command: "bulk_extractor".to_string(),
            typical_args: vec![],
            requires_root: false,
            install_command: Some("brew install bulk-extractor".to_string()),
            execution_time_estimate: 600,
        },

        // ========== ADDITIONAL SOCIAL ENGINEERING TOOLS ==========
        KaliTool {
            name: "setoolkit".to_string(),
            category: KaliToolCategory::SocialEngineering,
            description: "Social engineering toolkit".to_string(),
            command: "setoolkit".to_string(),
            typical_args: vec![],
            requires_root: false,
            install_command: Some("brew install set".to_string()),
            execution_time_estimate: 0,
        },
        KaliTool {
            name: "king-phisher".to_string(),
            category: KaliToolCategory::SocialEngineering,
            description: "Phishing campaign toolkit".to_string(),
            command: "king-phisher".to_string(),
            typical_args: vec![],
            requires_root: false,
            install_command: Some("brew install king-phisher".to_string()),
            execution_time_estimate: 0,
        },
        KaliTool {
            name: "gophish".to_string(),
            category: KaliToolCategory::SocialEngineering,
            description: "Open-source phishing toolkit".to_string(),
            command: "gophish".to_string(),
            typical_args: vec![],
            requires_root: false,
            install_command: Some("Download from https://getgophish.com/".to_string()),
            execution_time_estimate: 0,
        },
        KaliTool {
            name: "evilginx2".to_string(),
            category: KaliToolCategory::SocialEngineering,
            description: "Man-in-the-middle attack framework".to_string(),
            command: "evilginx2".to_string(),
            typical_args: vec![],
            requires_root: false,
            install_command: Some("git clone https://github.com/kgretzky/evilginx2.git".to_string()),
            execution_time_estimate: 0,
        },
        KaliTool {
            name: "blackeye".to_string(),
            category: KaliToolCategory::SocialEngineering,
            description: "Phishing pages generator".to_string(),
            command: "blackeye".to_string(),
            typical_args: vec![],
            requires_root: false,
            install_command: Some("git clone https://github.com/An0nUD4Y/blackeye.git".to_string()),
            execution_time_estimate: 0,
        },

        // ========== ADDITIONAL FUZZING TOOLS ==========
        KaliTool {
            name: "honggfuzz".to_string(),
            category: KaliToolCategory::Fuzzing,
            description: "Security-oriented fuzzer".to_string(),
            command: "honggfuzz".to_string(),
            typical_args: vec![],
            requires_root: false,
            install_command: Some("brew install honggfuzz".to_string()),
            execution_time_estimate: 0,
        },
        KaliTool {
            name: "libfuzzer".to_string(),
            category: KaliToolCategory::Fuzzing,
            description: "Library for coverage-guided fuzzing".to_string(),
            command: "libfuzzer".to_string(),
            typical_args: vec![],
            requires_root: false,
            install_command: Some("Included with clang".to_string()),
            execution_time_estimate: 0,
        },
        KaliTool {
            name: "radamsa".to_string(),
            category: KaliToolCategory::Fuzzing,
            description: "General purpose fuzzer".to_string(),
            command: "radamsa".to_string(),
            typical_args: vec![],
            requires_root: false,
            install_command: Some("brew install radamsa".to_string()),
            execution_time_estimate: 0,
        },

        // ========== ADDITIONAL REPORTING TOOLS ==========
        KaliTool {
            name: "dradis".to_string(),
            category: KaliToolCategory::Reporting,
            description: "Collaboration framework for security teams".to_string(),
            command: "dradis".to_string(),
            typical_args: vec![],
            requires_root: false,
            install_command: Some("brew install dradis".to_string()),
            execution_time_estimate: 0,
        },
        KaliTool {
            name: "faraday".to_string(),
            category: KaliToolCategory::Reporting,
            description: "Multiuser penetration test IDE".to_string(),
            command: "faraday".to_string(),
            typical_args: vec![],
            requires_root: false,
            install_command: Some("brew install faraday".to_string()),
            execution_time_estimate: 0,
        },
        KaliTool {
            name: "pipal".to_string(),
            category: KaliToolCategory::Reporting,
            description: "Password analysis tool".to_string(),
            command: "pipal".to_string(),
            typical_args: vec![],
            requires_root: false,
            install_command: Some("brew install pipal".to_string()),
            execution_time_estimate: 60,
        },

        // ========== ADDITIONAL NETWORK FORENSICS ==========
        KaliTool {
            name: "chaosreader".to_string(),
            category: KaliToolCategory::NetworkForensics,
            description: "Network session extractor".to_string(),
            command: "chaosreader".to_string(),
            typical_args: vec![],
            requires_root: false,
            install_command: Some("brew install chaosreader".to_string()),
            execution_time_estimate: 120,
        },
        KaliTool {
            name: "tcpflow".to_string(),
            category: KaliToolCategory::NetworkForensics,
            description: "TCP flow recorder".to_string(),
            command: "tcpflow".to_string(),
            typical_args: vec![],
            requires_root: true,
            install_command: Some("brew install tcpflow".to_string()),
            execution_time_estimate: 0,
        },
        KaliTool {
            name: "bro".to_string(),
            category: KaliToolCategory::NetworkForensics,
            description: "Network analysis framework".to_string(),
            command: "bro".to_string(),
            typical_args: vec![],
            requires_root: true,
            install_command: Some("brew install bro".to_string()),
            execution_time_estimate: 0,
        },

        // ========== ADDITIONAL PRIVILEGE ESCALATION ==========
        KaliTool {
            name: "beroot".to_string(),
            category: KaliToolCategory::PrivilegeEscalation,
            description: "Privilege escalation detection".to_string(),
            command: "beroot".to_string(),
            typical_args: vec![],
            requires_root: false,
            install_command: Some("pip3 install beRoot".to_string()),
            execution_time_estimate: 60,
        },
        KaliTool {
            name: "peas".to_string(),
            category: KaliToolCategory::PrivilegeEscalation,
            description: "Privilege escalation awesome scripts".to_string(),
            command: "peas".to_string(),
            typical_args: vec![],
            requires_root: false,
            install_command: Some("git clone https://github.com/carlospolop/PEASS-ng.git".to_string()),
            execution_time_estimate: 180,
        },
        KaliTool {
            name: "gtfobins".to_string(),
            category: KaliToolCategory::PrivilegeEscalation,
            description: "GTFOBins - Living off the land".to_string(),
            command: "gtfobins".to_string(),
            typical_args: vec![],
            requires_root: false,
            install_command: Some("curl -L https://gtfobins.github.io/".to_string()),
            execution_time_estimate: 0,
        },

        // ========== ADDITIONAL EXPLOIT DEVELOPMENT ==========
        KaliTool {
            name: "ropgadget".to_string(),
            category: KaliToolCategory::ExploitDevelopment,
            description: "ROP gadget finder".to_string(),
            command: "ropgadget".to_string(),
            typical_args: vec![],
            requires_root: false,
            install_command: Some("brew install ropgadget".to_string()),
            execution_time_estimate: 30,
        },
        KaliTool {
            name: "pwntools".to_string(),
            category: KaliToolCategory::ExploitDevelopment,
            description: "CTF framework and exploit development library".to_string(),
            command: "python3".to_string(),
            typical_args: vec!["-c".to_string(), "import pwn".to_string()],
            requires_root: false,
            install_command: Some("pip3 install pwntools".to_string()),
            execution_time_estimate: 0,
        },
        KaliTool {
            name: "angr".to_string(),
            category: KaliToolCategory::ReverseEngineering,
            description: "Binary analysis framework".to_string(),
            command: "python3".to_string(),
            typical_args: vec!["-c".to_string(), "import angr".to_string()],
            requires_root: false,
            install_command: Some("pip3 install angr".to_string()),
            execution_time_estimate: 0,
        },

        // ========== ADDITIONAL DOS TOOLS ==========
        KaliTool {
            name: "hping3".to_string(),
            category: KaliToolCategory::DenialOfService,
            description: "TCP/IP packet assembler/analyzer".to_string(),
            command: "hping3".to_string(),
            typical_args: vec!["--flood".to_string()],
            requires_root: true,
            install_command: Some("brew install hping3".to_string()),
            execution_time_estimate: 0,
        },
        KaliTool {
            name: "slowloris".to_string(),
            category: KaliToolCategory::DenialOfService,
            description: "Low bandwidth DoS tool".to_string(),
            command: "slowloris".to_string(),
            typical_args: vec![],
            requires_root: false,
            install_command: Some("brew install slowloris".to_string()),
            execution_time_estimate: 0,
        },
        KaliTool {
            name: "t50".to_string(),
            category: KaliToolCategory::DenialOfService,
            description: "Mixed packet injector tool".to_string(),
            command: "t50".to_string(),
            typical_args: vec![],
            requires_root: true,
            install_command: Some("brew install t50".to_string()),
            execution_time_estimate: 0,
        },

        // ========== ADDITIONAL MITM TOOLS ==========
        KaliTool {
            name: "sslstrip".to_string(),
            category: KaliToolCategory::ManInTheMiddle,
            description: "SSL stripping attack".to_string(),
            command: "sslstrip".to_string(),
            typical_args: vec![],
            requires_root: false,
            install_command: Some("brew install sslstrip".to_string()),
            execution_time_estimate: 0,
        },
        KaliTool {
            name: "sslsplit".to_string(),
            category: KaliToolCategory::ManInTheMiddle,
            description: "SSL/TLS interception tool".to_string(),
            command: "sslsplit".to_string(),
            typical_args: vec![],
            requires_root: true,
            install_command: Some("brew install sslsplit".to_string()),
            execution_time_estimate: 0,
        },
        KaliTool {
            name: "responder".to_string(),
            category: KaliToolCategory::ManInTheMiddle,
            description: "LLMNR, NBT-NS and MDNS poisoner".to_string(),
            command: "responder".to_string(),
            typical_args: vec!["-I".to_string(), "eth0".to_string()],
            requires_root: true,
            install_command: Some("brew install responder".to_string()),
            execution_time_estimate: 0,
        },

        // ========== ADDITIONAL SNIFFING/SPOOFING ==========
        KaliTool {
            name: "macchanger".to_string(),
            category: KaliToolCategory::SniffingSpoofing,
            description: "MAC address changer".to_string(),
            command: "macchanger".to_string(),
            typical_args: vec!["-r".to_string(), "eth0".to_string()],
            requires_root: true,
            install_command: Some("brew install macchanger".to_string()),
            execution_time_estimate: 5,
        },
        KaliTool {
            name: "arpspoof".to_string(),
            category: KaliToolCategory::SniffingSpoofing,
            description: "ARP spoofer".to_string(),
            command: "arpspoof".to_string(),
            typical_args: vec![],
            requires_root: true,
            install_command: Some("brew install dsniff".to_string()),
            execution_time_estimate: 0,
        },
        KaliTool {
            name: "dnsspoof".to_string(),
            category: KaliToolCategory::SniffingSpoofing,
            description: "DNS spoofer".to_string(),
            command: "dnsspoof".to_string(),
            typical_args: vec![],
            requires_root: true,
            install_command: Some("brew install dsniff".to_string()),
            execution_time_estimate: 0,
        },

        // ========== ADDITIONAL POST-EXPLOITATION ==========
        KaliTool {
            name: "empire".to_string(),
            category: KaliToolCategory::PostExploitation,
            description: "Post-exploitation framework".to_string(),
            command: "empire".to_string(),
            typical_args: vec![],
            requires_root: false,
            install_command: Some("brew install powershell-empire".to_string()),
            execution_time_estimate: 0,
        },
        KaliTool {
            name: "covenant".to_string(),
            category: KaliToolCategory::PostExploitation,
            description: ".NET command and control framework".to_string(),
            command: "covenant".to_string(),
            typical_args: vec![],
            requires_root: false,
            install_command: Some("git clone https://github.com/cobbr/Covenant.git".to_string()),
            execution_time_estimate: 0,
        },
        KaliTool {
            name: "silenttrinity".to_string(),
            category: KaliToolCategory::PostExploitation,
            description: "Post-exploitation agent".to_string(),
            command: "silenttrinity".to_string(),
            typical_args: vec![],
            requires_root: false,
            install_command: Some("git clone https://github.com/byt3bl33d3r/SILENTTRINITY.git".to_string()),
            execution_time_estimate: 0,
        },

        // ========== ADDITIONAL PERSISTENCE TOOLS ==========
        KaliTool {
            name: "metasploit-persistence".to_string(),
            category: KaliToolCategory::PersistenceMechanisms,
            description: "Metasploit persistence modules".to_string(),
            command: "msfconsole".to_string(),
            typical_args: vec!["-x".to_string(), "use exploit/windows/local/persistence".to_string()],
            requires_root: false,
            install_command: Some("brew install metasploit-framework".to_string()),
            execution_time_estimate: 60,
        },
        KaliTool {
            name: "turla-persistence".to_string(),
            category: KaliToolCategory::PersistenceMechanisms,
            description: "Advanced persistence techniques".to_string(),
            command: "turla".to_string(),
            typical_args: vec![],
            requires_root: false,
            install_command: Some("Research Turla persistence methods".to_string()),
            execution_time_estimate: 0,
        },

        // ========== ADDITIONAL ANTI-FORENSICS ==========
        KaliTool {
            name: "shred".to_string(),
            category: KaliToolCategory::AntiForensics,
            description: "Secure file deletion".to_string(),
            command: "shred".to_string(),
            typical_args: vec!["-u".to_string(), "-z".to_string()],
            requires_root: false,
            install_command: Some("brew install coreutils".to_string()),
            execution_time_estimate: 30,
        },
        KaliTool {
            name: "wipe".to_string(),
            category: KaliToolCategory::AntiForensics,
            description: "Secure deletion tool".to_string(),
            command: "wipe".to_string(),
            typical_args: vec![],
            requires_root: false,
            install_command: Some("brew install wipe".to_string()),
            execution_time_estimate: 60,
        },
        KaliTool {
            name: "srm".to_string(),
            category: KaliToolCategory::AntiForensics,
            description: "Secure remove".to_string(),
            command: "srm".to_string(),
            typical_args: vec![],
            requires_root: false,
            install_command: Some("brew install secure-delete".to_string()),
            execution_time_estimate: 30,
        },

        // ========== COMPREHENSIVE DATABASE EXPLOITATION TOOLS ==========
        KaliTool {
            name: "sqlninja".to_string(),
            category: KaliToolCategory::DatabaseExploitation,
            description: "SQL injection tool".to_string(),
            command: "sqlninja".to_string(),
            typical_args: vec![],
            requires_root: false,
            install_command: Some("brew install sqlninja".to_string()),
            execution_time_estimate: 180,
        },
        KaliTool {
            name: "tnscmd10g".to_string(),
            category: KaliToolCategory::DatabaseExploitation,
            description: "Oracle TNS listener command tool".to_string(),
            command: "tnscmd10g".to_string(),
            typical_args: vec!["ping".to_string()],
            requires_root: false,
            install_command: Some("brew install tnscmd10g".to_string()),
            execution_time_estimate: 30,
        },
        KaliTool {
            name: "odat".to_string(),
            category: KaliToolCategory::DatabaseExploitation,
            description: "Oracle Database Attacking Tool".to_string(),
            command: "odat".to_string(),
            typical_args: vec![],
            requires_root: false,
            install_command: Some("pip3 install odat".to_string()),
            execution_time_estimate: 120,
        },
        KaliTool {
            name: "mssqlclient".to_string(),
            category: KaliToolCategory::DatabaseExploitation,
            description: "MSSQL client".to_string(),
            command: "mssqlclient".to_string(),
            typical_args: vec![],
            requires_root: false,
            install_command: Some("brew install impacket-scripts".to_string()),
            execution_time_estimate: 60,
        },
        KaliTool {
            name: "cassandra".to_string(),
            category: KaliToolCategory::DatabaseExploitation,
            description: "Cassandra database tools".to_string(),
            command: "cassandra".to_string(),
            typical_args: vec![],
            requires_root: false,
            install_command: Some("brew install cassandra".to_string()),
            execution_time_estimate: 0,
        },

        // ========== COMPREHENSIVE WEB APPLICATION TOOLS ==========
        KaliTool {
            name: "skipfish".to_string(),
            category: KaliToolCategory::WebApplicationAnalysis,
            description: "Web application security scanner".to_string(),
            command: "skipfish".to_string(),
            typical_args: vec!["-o".to_string(), "output".to_string()],
            requires_root: false,
            install_command: Some("brew install skipfish".to_string()),
            execution_time_estimate: 300,
        },
        KaliTool {
            name: "w3af".to_string(),
            category: KaliToolCategory::WebApplicationAnalysis,
            description: "Web application attack and audit framework".to_string(),
            command: "w3af".to_string(),
            typical_args: vec![],
            requires_root: false,
            install_command: Some("brew install w3af".to_string()),
            execution_time_estimate: 0,
        },
        KaliTool {
            name: "webscarab".to_string(),
            category: KaliToolCategory::WebApplicationAnalysis,
            description: "Web application security testing framework".to_string(),
            command: "webscarab".to_string(),
            typical_args: vec![],
            requires_root: false,
            install_command: Some("brew install webscarab".to_string()),
            execution_time_estimate: 0,
        },
        KaliTool {
            name: "paros".to_string(),
            category: KaliToolCategory::WebApplicationAnalysis,
            description: "Web application security assessment tool".to_string(),
            command: "paros".to_string(),
            typical_args: vec![],
            requires_root: false,
            install_command: Some("brew install paros".to_string()),
            execution_time_estimate: 0,
        },
        KaliTool {
            name: "webshag".to_string(),
            category: KaliToolCategory::WebApplicationAnalysis,
            description: "Web server auditing tool".to_string(),
            command: "webshag".to_string(),
            typical_args: vec![],
            requires_root: false,
            install_command: Some("brew install webshag".to_string()),
            execution_time_estimate: 120,
        },
        KaliTool {
            name: "whatweb".to_string(),
            category: KaliToolCategory::WebApplicationAnalysis,
            description: "Web scanner".to_string(),
            command: "whatweb".to_string(),
            typical_args: vec![],
            requires_root: false,
            install_command: Some("brew install whatweb".to_string()),
            execution_time_estimate: 60,
        },
        KaliTool {
            name: "httprint".to_string(),
            category: KaliToolCategory::WebApplicationAnalysis,
            description: "Web server fingerprinting tool".to_string(),
            command: "httprint".to_string(),
            typical_args: vec![],
            requires_root: false,
            install_command: Some("brew install httprint".to_string()),
            execution_time_estimate: 60,
        },
        KaliTool {
            name: "httrack".to_string(),
            category: KaliToolCategory::WebApplicationAnalysis,
            description: "Website copier".to_string(),
            command: "httrack".to_string(),
            typical_args: vec![],
            requires_root: false,
            install_command: Some("brew install httrack".to_string()),
            execution_time_estimate: 300,
        },

        // ========== COMPREHENSIVE WIRELESS TOOLS ==========
        KaliTool {
            name: "mdk3".to_string(),
            category: KaliToolCategory::WirelessAttacks,
            description: "WiFi testing tool".to_string(),
            command: "mdk3".to_string(),
            typical_args: vec![],
            requires_root: true,
            install_command: Some("brew install mdk3".to_string()),
            execution_time_estimate: 0,
        },
        KaliTool {
            name: "mdk4".to_string(),
            category: KaliToolCategory::WirelessAttacks,
            description: "WiFi testing tool".to_string(),
            command: "mdk4".to_string(),
            typical_args: vec![],
            requires_root: true,
            install_command: Some("brew install mdk4".to_string()),
            execution_time_estimate: 0,
        },
        KaliTool {
            name: "hostapd".to_string(),
            category: KaliToolCategory::WirelessAttacks,
            description: "User space IEEE 802.11 AP and authentication server".to_string(),
            command: "hostapd".to_string(),
            typical_args: vec![],
            requires_root: true,
            install_command: Some("brew install hostapd".to_string()),
            execution_time_estimate: 0,
        },
        KaliTool {
            name: "hostapd-wpe".to_string(),
            category: KaliToolCategory::WirelessAttacks,
            description: "Modified hostapd for wireless pentesting".to_string(),
            command: "hostapd-wpe".to_string(),
            typical_args: vec![],
            requires_root: true,
            install_command: Some("brew install hostapd-wpe".to_string()),
            execution_time_estimate: 0,
        },
        KaliTool {
            name: "free-radius".to_string(),
            category: KaliToolCategory::WirelessAttacks,
            description: "RADIUS server".to_string(),
            command: "freeradius".to_string(),
            typical_args: vec![],
            requires_root: true,
            install_command: Some("brew install freeradius".to_string()),
            execution_time_estimate: 0,
        },
        KaliTool {
            name: "asleap".to_string(),
            category: KaliToolCategory::WirelessAttacks,
            description: "LEAP dictionary cracker".to_string(),
            command: "asleap".to_string(),
            typical_args: vec![],
            requires_root: false,
            install_command: Some("brew install asleap".to_string()),
            execution_time_estimate: 300,
        },

        // ========== COMPREHENSIVE SOCIAL ENGINEERING TOOLS ==========
        KaliTool {
            name: "msfvenom".to_string(),
            category: KaliToolCategory::SocialEngineering,
            description: "Payload generator".to_string(),
            command: "msfvenom".to_string(),
            typical_args: vec!["-p".to_string(), "windows/meterpreter/reverse_tcp".to_string()],
            requires_root: false,
            install_command: Some("brew install metasploit-framework".to_string()),
            execution_time_estimate: 30,
        },

        // ========== ADVANCED OAUTH2 & SOCIAL NETWORK TOOLS ==========
        KaliTool {
            name: "oauth2-tool".to_string(),
            category: KaliToolCategory::WebApplicationAnalysis,
            description: "Advanced OAuth2 security testing tool".to_string(),
            command: "python3".to_string(),
            typical_args: vec!["-c".to_string(), "import oauthlib".to_string()],
            requires_root: false,
            install_command: Some("pip3 install oauthlib requests".to_string()),
            execution_time_estimate: 60,
        },
        KaliTool {
            name: "social-engineer-toolkit".to_string(),
            category: KaliToolCategory::SocialEngineering,
            description: "Social engineering toolkit with OAuth bypass".to_string(),
            command: "setoolkit".to_string(),
            typical_args: vec![],
            requires_root: false,
            install_command: Some("brew install set".to_string()),
            execution_time_estimate: 0,
        },
        KaliTool {
            name: "maltego".to_string(),
            category: KaliToolCategory::OSInt,
            description: "Advanced OSINT and social network analysis".to_string(),
            command: "maltego".to_string(),
            typical_args: vec![],
            requires_root: false,
            install_command: Some("brew install maltego".to_string()),
            execution_time_estimate: 0,
        },
        KaliTool {
            name: "recon-ng".to_string(),
            category: KaliToolCategory::OSInt,
            description: "Web reconnaissance framework with social modules".to_string(),
            command: "recon-ng".to_string(),
            typical_args: vec![],
            requires_root: false,
            install_command: Some("brew install recon-ng".to_string()),
            execution_time_estimate: 0,
        },
        KaliTool {
            name: "spiderfoot".to_string(),
            category: KaliToolCategory::OSInt,
            description: "OSINT automation tool for social networks".to_string(),
            command: "spiderfoot".to_string(),
            typical_args: vec![],
            requires_root: false,
            install_command: Some("brew install spiderfoot".to_string()),
            execution_time_estimate: 0,
        },
        KaliTool {
            name: "theharvester".to_string(),
            category: KaliToolCategory::OSInt,
            description: "Email and social network harvesting".to_string(),
            command: "theharvester".to_string(),
            typical_args: vec!["-d".to_string(), "-b".to_string(), "all".to_string()],
            requires_root: false,
            install_command: Some("brew install theharvester".to_string()),
            execution_time_estimate: 120,
        },
        KaliTool {
            name: "sherlock".to_string(),
            category: KaliToolCategory::OSInt,
            description: "Username enumeration across social networks".to_string(),
            command: "sherlock".to_string(),
            typical_args: vec!["--print-found".to_string()],
            requires_root: false,
            install_command: Some("pip3 install sherlock".to_string()),
            execution_time_estimate: 180,
        },
        KaliTool {
            name: "social-analyzer".to_string(),
            category: KaliToolCategory::OSInt,
            description: "Find social media profiles".to_string(),
            command: "social-analyzer".to_string(),
            typical_args: vec!["--username".to_string()],
            requires_root: false,
            install_command: Some("npm install -g social-analyzer".to_string()),
            execution_time_estimate: 60,
        },
        KaliTool {
            name: "email2phonenumber".to_string(),
            category: KaliToolCategory::OSInt,
            description: "OSINT tool to obtain phone numbers from emails".to_string(),
            command: "email2phonenumber".to_string(),
            typical_args: vec![],
            requires_root: false,
            install_command: Some("pip3 install email2phonenumber".to_string()),
            execution_time_estimate: 30,
        },
        KaliTool {
            name: "holehe".to_string(),
            category: KaliToolCategory::OSInt,
            description: "Check if email is used on various sites".to_string(),
            command: "holehe".to_string(),
            typical_args: vec![],
            requires_root: false,
            install_command: Some("pip3 install holehe".to_string()),
            execution_time_estimate: 60,
        },
        KaliTool {
            name: "ghunt".to_string(),
            category: KaliToolCategory::OSInt,
            description: "Google account OSINT tool".to_string(),
            command: "ghunt".to_string(),
            typical_args: vec![],
            requires_root: false,
            install_command: Some("git clone https://github.com/mxrch/GHunt.git".to_string()),
            execution_time_estimate: 90,
        },
        KaliTool {
            name: "linkedin2username".to_string(),
            category: KaliToolCategory::OSInt,
            description: "Generate username lists from LinkedIn".to_string(),
            command: "linkedin2username".to_string(),
            typical_args: vec![],
            requires_root: false,
            install_command: Some("pip3 install linkedin2username".to_string()),
            execution_time_estimate: 60,
        },
        KaliTool {
            name: "instagram-scraper".to_string(),
            category: KaliToolCategory::OSInt,
            description: "Instagram profile scraper".to_string(),
            command: "instagram-scraper".to_string(),
            typical_args: vec![],
            requires_root: false,
            install_command: Some("pip3 install instagram-scraper".to_string()),
            execution_time_estimate: 30,
        },
        KaliTool {
            name: "tiktok-scraper".to_string(),
            category: KaliToolCategory::OSInt,
            description: "TikTok profile and video scraper".to_string(),
            command: "tiktok-scraper".to_string(),
            typical_args: vec![],
            requires_root: false,
            install_command: Some("pip3 install tiktok-scraper".to_string()),
            execution_time_estimate: 45,
        },
        KaliTool {
            name: "twitter-scraper".to_string(),
            category: KaliToolCategory::OSInt,
            description: "Twitter/X profile scraper".to_string(),
            command: "twitter-scraper".to_string(),
            typical_args: vec![],
            requires_root: false,
            install_command: Some("pip3 install twitter-scraper".to_string()),
            execution_time_estimate: 45,
        },
        KaliTool {
            name: "facebook-scraper".to_string(),
            category: KaliToolCategory::OSInt,
            description: "Facebook profile scraper".to_string(),
            command: "facebook-scraper".to_string(),
            typical_args: vec![],
            requires_root: false,
            install_command: Some("pip3 install facebook-scraper".to_string()),
            execution_time_estimate: 45,
        },
        KaliTool {
            name: "discord-scraper".to_string(),
            category: KaliToolCategory::OSInt,
            description: "Discord server and user scraper".to_string(),
            command: "discord-scraper".to_string(),
            typical_args: vec![],
            requires_root: false,
            install_command: Some("pip3 install discord-scraper".to_string()),
            execution_time_estimate: 45,
        },
        KaliTool {
            name: "snapchat-scraper".to_string(),
            category: KaliToolCategory::OSInt,
            description: "Snapchat profile scraper".to_string(),
            command: "snapchat-scraper".to_string(),
            typical_args: vec![],
            requires_root: false,
            install_command: Some("pip3 install snapchat-scraper".to_string()),
            execution_time_estimate: 45,
        },

        // ========== ADVANCED PASSWORD CRACKING TOOLS ==========
        KaliTool {
            name: "john-the-ripper".to_string(),
            category: KaliToolCategory::PasswordAttacks,
            description: "Advanced password cracker with social engineering rules".to_string(),
            command: "john".to_string(),
            typical_args: vec!["--wordlist=/usr/share/wordlists/rockyou.txt".to_string()],
            requires_root: false,
            install_command: Some("brew install john".to_string()),
            execution_time_estimate: 600,
        },
        KaliTool {
            name: "hashcat".to_string(),
            category: KaliToolCategory::PasswordAttacks,
            description: "GPU-accelerated password cracker".to_string(),
            command: "hashcat".to_string(),
            typical_args: vec!["-m".to_string(), "0".to_string(), "-a".to_string(), "0".to_string()],
            requires_root: false,
            install_command: Some("brew install hashcat".to_string()),
            execution_time_estimate: 600,
        },
        KaliTool {
            name: "hydra".to_string(),
            category: KaliToolCategory::PasswordAttacks,
            description: "Online password cracking for social platforms".to_string(),
            command: "hydra".to_string(),
            typical_args: vec!["-l".to_string(), "username".to_string(), "-P".to_string(), "wordlist.txt".to_string()],
            requires_root: false,
            install_command: Some("brew install hydra".to_string()),
            execution_time_estimate: 300,
        },
        KaliTool {
            name: "patator".to_string(),
            category: KaliToolCategory::PasswordAttacks,
            description: "Multi-purpose brute-forcer for social logins".to_string(),
            command: "patator".to_string(),
            typical_args: vec![],
            requires_root: false,
            install_command: Some("brew install patator".to_string()),
            execution_time_estimate: 300,
        },
        KaliTool {
            name: "medusa".to_string(),
            category: KaliToolCategory::PasswordAttacks,
            description: "Parallel login brute-forcer".to_string(),
            command: "medusa".to_string(),
            typical_args: vec!["-h".to_string(), "target".to_string()],
            requires_root: false,
            install_command: Some("brew install medusa".to_string()),
            execution_time_estimate: 300,
        },
        KaliTool {
            name: "ncrack".to_string(),
            category: KaliToolCategory::PasswordAttacks,
            description: "Network authentication cracking tool".to_string(),
            command: "ncrack".to_string(),
            typical_args: vec![],
            requires_root: false,
            install_command: Some("brew install ncrack".to_string()),
            execution_time_estimate: 300,
        },
        KaliTool {
            name: "cewl".to_string(),
            category: KaliToolCategory::PasswordAttacks,
            description: "Custom wordlist generator from social profiles".to_string(),
            command: "cewl".to_string(),
            typical_args: vec!["-d".to_string(), "2".to_string(), "-m".to_string(), "5".to_string()],
            requires_root: false,
            install_command: Some("brew install cewl".to_string()),
            execution_time_estimate: 120,
        },
        KaliTool {
            name: "crunch".to_string(),
            category: KaliToolCategory::PasswordAttacks,
            description: "Wordlist generator for social engineering attacks".to_string(),
            command: "crunch".to_string(),
            typical_args: vec!["8".to_string(), "8".to_string(), "-t".to_string(), "@@@@%%%%".to_string()],
            requires_root: false,
            install_command: Some("brew install crunch".to_string()),
            execution_time_estimate: 60,
        },
        KaliTool {
            name: "cupp".to_string(),
            category: KaliToolCategory::PasswordAttacks,
            description: "Common User Passwords Profiler".to_string(),
            command: "cupp".to_string(),
            typical_args: vec!["-i".to_string()],
            requires_root: false,
            install_command: Some("brew install cupp".to_string()),
            execution_time_estimate: 30,
        },
        KaliTool {
            name: "rsmangler".to_string(),
            category: KaliToolCategory::PasswordAttacks,
            description: "Wordlist mangling tool for social passwords".to_string(),
            command: "rsmangler".to_string(),
            typical_args: vec![],
            requires_root: false,
            install_command: Some("brew install rsmangler".to_string()),
            execution_time_estimate: 60,
        },

        // ========== ADVANCED PHISHING TOOLS ==========
        KaliTool {
            name: "gophish".to_string(),
            category: KaliToolCategory::SocialEngineering,
            description: "Open-source phishing toolkit".to_string(),
            command: "gophish".to_string(),
            typical_args: vec![],
            requires_root: false,
            install_command: Some("Download from https://getgophish.com/".to_string()),
            execution_time_estimate: 0,
        },
        KaliTool {
            name: "king-phisher".to_string(),
            category: KaliToolCategory::SocialEngineering,
            description: "Phishing campaign toolkit".to_string(),
            command: "king-phisher".to_string(),
            typical_args: vec![],
            requires_root: false,
            install_command: Some("brew install king-phisher".to_string()),
            execution_time_estimate: 0,
        },
        KaliTool {
            name: "evilginx2".to_string(),
            category: KaliToolCategory::SocialEngineering,
            description: "Man-in-the-middle attack framework for OAuth".to_string(),
            command: "evilginx2".to_string(),
            typical_args: vec![],
            requires_root: false,
            install_command: Some("git clone https://github.com/kgretzky/evilginx2.git".to_string()),
            execution_time_estimate: 0,
        },
        KaliTool {
            name: "modlishka".to_string(),
            category: KaliToolCategory::SocialEngineering,
            description: "Reverse proxy for phishing".to_string(),
            command: "modlishka".to_string(),
            typical_args: vec![],
            requires_root: false,
            install_command: Some("git clone https://github.com/drk1wi/Modlishka.git".to_string()),
            execution_time_estimate: 0,
        },
        KaliTool {
            name: "blackeye".to_string(),
            category: KaliToolCategory::SocialEngineering,
            description: "Phishing pages generator".to_string(),
            command: "blackeye".to_string(),
            typical_args: vec![],
            requires_root: false,
            install_command: Some("git clone https://github.com/An0nUD4Y/blackeye.git".to_string()),
            execution_time_estimate: 0,
        },
        KaliTool {
            name: "hiddeneye".to_string(),
            category: KaliToolCategory::SocialEngineering,
            description: "Modern phishing tool with OAuth support".to_string(),
            command: "hiddeneye".to_string(),
            typical_args: vec![],
            requires_root: false,
            install_command: Some("git clone https://github.com/DarkSecDevelopers/HiddenEye.git".to_string()),
            execution_time_estimate: 0,
        },
        KaliTool {
            name: "socialfish".to_string(),
            category: KaliToolCategory::SocialEngineering,
            description: "Automated phishing tool".to_string(),
            command: "socialfish".to_string(),
            typical_args: vec![],
            requires_root: false,
            install_command: Some("git clone https://github.com/UndeadSec/SocialFish.git".to_string()),
            execution_time_estimate: 0,
        },
        KaliTool {
            name: "zphisher".to_string(),
            category: KaliToolCategory::SocialEngineering,
            description: "Automated phishing tool for social networks".to_string(),
            command: "zphisher".to_string(),
            typical_args: vec![],
            requires_root: false,
            install_command: Some("git clone https://github.com/htr-tech/zphisher.git".to_string()),
            execution_time_estimate: 0,
        },

        // ========== ADVANCED 2FA BYPASS TOOLS ==========
        KaliTool {
            name: "sms-bomber".to_string(),
            category: KaliToolCategory::SocialEngineering,
            description: "SMS bombing tool for 2FA bypass".to_string(),
            command: "sms-bomber".to_string(),
            typical_args: vec![],
            requires_root: false,
            install_command: Some("pip3 install sms-bomber".to_string()),
            execution_time_estimate: 60,
        },
        KaliTool {
            name: "totp-bypass".to_string(),
            category: KaliToolCategory::SocialEngineering,
            description: "TOTP token manipulation tool".to_string(),
            command: "python3".to_string(),
            typical_args: vec!["-c".to_string(), "import pyotp".to_string()],
            requires_root: false,
            install_command: Some("pip3 install pyotp".to_string()),
            execution_time_estimate: 30,
        },
        KaliTool {
            name: "sim-swapper".to_string(),
            category: KaliToolCategory::SocialEngineering,
            description: "SIM swapping attack simulation".to_string(),
            command: "sim-swapper".to_string(),
            typical_args: vec![],
            requires_root: false,
            install_command: Some("Research SIM swapping techniques".to_string()),
            execution_time_estimate: 0,
        },

        // ========== ADVANCED EMAIL CRACKING TOOLS ==========
        KaliTool {
            name: "swaks".to_string(),
            category: KaliToolCategory::PasswordAttacks,
            description: "SMTP testing tool for email cracking".to_string(),
            command: "swaks".to_string(),
            typical_args: vec!["--to".to_string(), "target@example.com".to_string()],
            requires_root: false,
            install_command: Some("brew install swaks".to_string()),
            execution_time_estimate: 30,
        },
        KaliTool {
            name: "smtp-user-enum".to_string(),
            category: KaliToolCategory::PasswordAttacks,
            description: "SMTP user enumeration tool".to_string(),
            command: "smtp-user-enum".to_string(),
            typical_args: vec!["-M".to_string(), "VRFY".to_string(), "-u".to_string()],
            requires_root: false,
            install_command: Some("brew install smtp-user-enum".to_string()),
            execution_time_estimate: 60,
        },
        KaliTool {
            name: "email-harvester".to_string(),
            category: KaliToolCategory::OSInt,
            description: "Advanced email harvesting tool".to_string(),
            command: "email-harvester".to_string(),
            typical_args: vec![],
            requires_root: false,
            install_command: Some("pip3 install email-harvester".to_string()),
            execution_time_estimate: 90,
        },
        KaliTool {
            name: "infoga".to_string(),
            category: KaliToolCategory::OSInt,
            description: "Email OSINT tool".to_string(),
            command: "infoga".to_string(),
            typical_args: vec!["-t".to_string()],
            requires_root: false,
            install_command: Some("git clone https://github.com/m4ll0k/Infoga.git".to_string()),
            execution_time_estimate: 120,
        },
        KaliTool {
            name: "veil-framework".to_string(),
            category: KaliToolCategory::SocialEngineering,
            description: "Payload generator".to_string(),
            command: "veil".to_string(),
            typical_args: vec![],
            requires_root: false,
            install_command: Some("brew install veil".to_string()),
            execution_time_estimate: 0,
        },
        KaliTool {
            name: "the-backdoor-factory".to_string(),
            category: KaliToolCategory::SocialEngineering,
            description: "Patch PE, ELF, Mach-O binaries with shellcode".to_string(),
            command: "backdoor-factory".to_string(),
            typical_args: vec![],
            requires_root: false,
            install_command: Some("brew install the-backdoor-factory".to_string()),
            execution_time_estimate: 60,
        },
        KaliTool {
            name: "shellter".to_string(),
            category: KaliToolCategory::SocialEngineering,
            description: "Dynamic shellcode injection tool".to_string(),
            command: "shellter".to_string(),
            typical_args: vec![],
            requires_root: false,
            install_command: Some("brew install shellter".to_string()),
            execution_time_estimate: 60,
        },
        KaliTool {
            name: "hyperion".to_string(),
            category: KaliToolCategory::SocialEngineering,
            description: "Runtime encryptor for 32-bit PE files".to_string(),
            command: "hyperion".to_string(),
            typical_args: vec![],
            requires_root: false,
            install_command: Some("brew install hyperion".to_string()),
            execution_time_estimate: 30,
        },
        KaliTool {
            name: "upx".to_string(),
            category: KaliToolCategory::SocialEngineering,
            description: "Ultimate packer for executables".to_string(),
            command: "upx".to_string(),
            typical_args: vec![],
            requires_root: false,
            install_command: Some("brew install upx-ucl".to_string()),
            execution_time_estimate: 30,
        },

        // ========== COMPREHENSIVE FORENSICS TOOLS ==========
        KaliTool {
            name: "chkrootkit".to_string(),
            category: KaliToolCategory::Forensics,
            description: "Rootkit detector".to_string(),
            command: "chkrootkit".to_string(),
            typical_args: vec![],
            requires_root: true,
            install_command: Some("brew install chkrootkit".to_string()),
            execution_time_estimate: 120,
        },
        KaliTool {
            name: "rkhunter".to_string(),
            category: KaliToolCategory::Forensics,
            description: "Rootkit hunter".to_string(),
            command: "rkhunter".to_string(),
            typical_args: vec!["--check".to_string()],
            requires_root: true,
            install_command: Some("brew install rkhunter".to_string()),
            execution_time_estimate: 180,
        },
        KaliTool {
            name: "clamav".to_string(),
            category: KaliToolCategory::Forensics,
            description: "Antivirus engine".to_string(),
            command: "clamscan".to_string(),
            typical_args: vec![],
            requires_root: false,
            install_command: Some("brew install clamav".to_string()),
            execution_time_estimate: 300,
        },
        KaliTool {
            name: "yara".to_string(),
            category: KaliToolCategory::Forensics,
            description: "Pattern matching tool for malware researchers".to_string(),
            command: "yara".to_string(),
            typical_args: vec![],
            requires_root: false,
            install_command: Some("brew install yara".to_string()),
            execution_time_estimate: 60,
        },
        KaliTool {
            name: "ssdeep".to_string(),
            category: KaliToolCategory::Forensics,
            description: "Fuzzy hashing tool".to_string(),
            command: "ssdeep".to_string(),
            typical_args: vec![],
            requires_root: false,
            install_command: Some("brew install ssdeep".to_string()),
            execution_time_estimate: 30,
        },
        KaliTool {
            name: "exiftool".to_string(),
            category: KaliToolCategory::FileForensics,
            description: "Metadata reader".to_string(),
            command: "exiftool".to_string(),
            typical_args: vec![],
            requires_root: false,
            install_command: Some("brew install libimage-exiftool-perl".to_string()),
            execution_time_estimate: 30,
        },
        KaliTool {
            name: "pdf-parser".to_string(),
            category: KaliToolCategory::FileForensics,
            description: "PDF analysis tool".to_string(),
            command: "pdf-parser".to_string(),
            typical_args: vec![],
            requires_root: false,
            install_command: Some("brew install pdf-parser".to_string()),
            execution_time_estimate: 30,
        },
        KaliTool {
            name: "pdfid".to_string(),
            category: KaliToolCategory::FileForensics,
            description: "PDF analysis tool".to_string(),
            command: "pdfid".to_string(),
            typical_args: vec![],
            requires_root: false,
            install_command: Some("brew install pdfid".to_string()),
            execution_time_estimate: 30,
        },
        KaliTool {
            name: "peepdf".to_string(),
            category: KaliToolCategory::FileForensics,
            description: "PDF analysis tool".to_string(),
            command: "peepdf".to_string(),
            typical_args: vec![],
            requires_root: false,
            install_command: Some("brew install peepdf".to_string()),
            execution_time_estimate: 60,
        },
        KaliTool {
            name: "oledump".to_string(),
            category: KaliToolCategory::FileForensics,
            description: "OLE file analysis".to_string(),
            command: "oledump".to_string(),
            typical_args: vec![],
            requires_root: false,
            install_command: Some("brew install oledump".to_string()),
            execution_time_estimate: 30,
        },
        KaliTool {
            name: "rtfobj".to_string(),
            category: KaliToolCategory::FileForensics,
            description: "RTF file analysis".to_string(),
            command: "rtfobj".to_string(),
            typical_args: vec![],
            requires_root: false,
            install_command: Some("pip3 install oletools".to_string()),
            execution_time_estimate: 30,
        },

        // ========== COMPREHENSIVE REVERSE ENGINEERING TOOLS ==========
        KaliTool {
            name: "edb-debugger".to_string(),
            category: KaliToolCategory::ReverseEngineering,
            description: "Multi-architecture debugger".to_string(),
            command: "edb".to_string(),
            typical_args: vec![],
            requires_root: false,
            install_command: Some("brew install edb-debugger".to_string()),
            execution_time_estimate: 0,
        },
        KaliTool {
            name: "nemesis".to_string(),
            category: KaliToolCategory::ReverseEngineering,
            description: "Multi-architecture binary analysis framework".to_string(),
            command: "nemesis".to_string(),
            typical_args: vec![],
            requires_root: false,
            install_command: Some("brew install nemesis".to_string()),
            execution_time_estimate: 0,
        },
        KaliTool {
            name: "plasma".to_string(),
            category: KaliToolCategory::ReverseEngineering,
            description: "Interactive disassembler".to_string(),
            command: "plasma".to_string(),
            typical_args: vec![],
            requires_root: false,
            install_command: Some("pip3 install plasma".to_string()),
            execution_time_estimate: 0,
        },
        KaliTool {
            name: "barf".to_string(),
            category: KaliToolCategory::ReverseEngineering,
            description: "Binary Analysis and Reverse engineering Framework".to_string(),
            command: "barf".to_string(),
            typical_args: vec![],
            requires_root: false,
            install_command: Some("pip3 install barf".to_string()),
            execution_time_estimate: 0,
        },
        KaliTool {
            name: "miasm".to_string(),
            category: KaliToolCategory::ReverseEngineering,
            description: "Reverse engineering framework".to_string(),
            command: "python3".to_string(),
            typical_args: vec!["-c".to_string(), "import miasm".to_string()],
            requires_root: false,
            install_command: Some("pip3 install miasm".to_string()),
            execution_time_estimate: 0,
        },
        KaliTool {
            name: "capstone".to_string(),
            category: KaliToolCategory::ReverseEngineering,
            description: "Multi-architecture disassembly framework".to_string(),
            command: "python3".to_string(),
            typical_args: vec!["-c".to_string(), "import capstone".to_string()],
            requires_root: false,
            install_command: Some("pip3 install capstone".to_string()),
            execution_time_estimate: 0,
        },
        KaliTool {
            name: "keystone".to_string(),
            category: KaliToolCategory::ReverseEngineering,
            description: "Assembler framework".to_string(),
            command: "python3".to_string(),
            typical_args: vec!["-c".to_string(), "import keystone".to_string()],
            requires_root: false,
            install_command: Some("pip3 install keystone-engine".to_string()),
            execution_time_estimate: 0,
        },
        KaliTool {
            name: "unicorn".to_string(),
            category: KaliToolCategory::ReverseEngineering,
            description: "CPU emulator framework".to_string(),
            command: "python3".to_string(),
            typical_args: vec!["-c".to_string(), "import unicorn".to_string()],
            requires_root: false,
            install_command: Some("pip3 install unicorn".to_string()),
            execution_time_estimate: 0,
        },

        // ========== COMPREHENSIVE EXPLOITATION TOOLS ==========
        KaliTool {
            name: "beef".to_string(),
            category: KaliToolCategory::ExploitationTools,
            description: "Browser Exploitation Framework".to_string(),
            command: "beef".to_string(),
            typical_args: vec![],
            requires_root: false,
            install_command: Some("brew install beef-xss".to_string()),
            execution_time_estimate: 0,
        },
        KaliTool {
            name: "termineter".to_string(),
            category: KaliToolCategory::ExploitationTools,
            description: "Smart meter testing framework".to_string(),
            command: "termineter".to_string(),
            typical_args: vec![],
            requires_root: false,
            install_command: Some("brew install termineter".to_string()),
            execution_time_estimate: 0,
        },
        KaliTool {
            name: "king-phisher".to_string(),
            category: KaliToolCategory::ExploitationTools,
            description: "Phishing campaign toolkit".to_string(),
            command: "king-phisher".to_string(),
            typical_args: vec![],
            requires_root: false,
            install_command: Some("brew install king-phisher".to_string()),
            execution_time_estimate: 0,
        },
        KaliTool {
            name: "wifiphisher".to_string(),
            category: KaliToolCategory::ExploitationTools,
            description: "Automated victim-customized phishing attacks".to_string(),
            command: "wifiphisher".to_string(),
            typical_args: vec![],
            requires_root: true,
            install_command: Some("brew install wifiphisher".to_string()),
            execution_time_estimate: 0,
        },
        KaliTool {
            name: "sslstrip".to_string(),
            category: KaliToolCategory::ExploitationTools,
            description: "SSL stripping attack".to_string(),
            command: "sslstrip".to_string(),
            typical_args: vec![],
            requires_root: false,
            install_command: Some("brew install sslstrip".to_string()),
            execution_time_estimate: 0,
        },
        KaliTool {
            name: "sslsplit".to_string(),
            category: KaliToolCategory::ExploitationTools,
            description: "SSL/TLS interception tool".to_string(),
            command: "sslsplit".to_string(),
            typical_args: vec![],
            requires_root: true,
            install_command: Some("brew install sslsplit".to_string()),
            execution_time_estimate: 0,
        },

        // ========== COMPREHENSIVE PASSWORD ATTACK TOOLS ==========
        KaliTool {
            name: "rainbowcrack".to_string(),
            category: KaliToolCategory::PasswordAttacks,
            description: "Rainbow table attack tool".to_string(),
            command: "rcrack".to_string(),
            typical_args: vec![],
            requires_root: false,
            install_command: Some("brew install rainbowcrack".to_string()),
            execution_time_estimate: 300,
        },
        KaliTool {
            name: "ophcrack".to_string(),
            category: KaliToolCategory::PasswordAttacks,
            description: "Windows password cracker".to_string(),
            command: "ophcrack".to_string(),
            typical_args: vec![],
            requires_root: false,
            install_command: Some("brew install ophcrack".to_string()),
            execution_time_estimate: 300,
        },
        KaliTool {
            name: "l0phtcrack".to_string(),
            category: KaliToolCategory::PasswordAttacks,
            description: "Password auditing and recovery".to_string(),
            command: "lc7".to_string(),
            typical_args: vec![],
            requires_root: false,
            install_command: Some("brew install l0phtcrack".to_string()),
            execution_time_estimate: 0,
        },
        KaliTool {
            name: "fcrackzip".to_string(),
            category: KaliToolCategory::PasswordAttacks,
            description: "ZIP password cracker".to_string(),
            command: "fcrackzip".to_string(),
            typical_args: vec![],
            requires_root: false,
            install_command: Some("brew install fcrackzip".to_string()),
            execution_time_estimate: 300,
        },
        KaliTool {
            name: "rarcrack".to_string(),
            category: KaliToolCategory::PasswordAttacks,
            description: "RAR password cracker".to_string(),
            command: "rarcrack".to_string(),
            typical_args: vec![],
            requires_root: false,
            install_command: Some("brew install rarcrack".to_string()),
            execution_time_estimate: 300,
        },
        KaliTool {
            name: "pdfcrack".to_string(),
            category: KaliToolCategory::PasswordAttacks,
            description: "PDF password cracker".to_string(),
            command: "pdfcrack".to_string(),
            typical_args: vec![],
            requires_root: false,
            install_command: Some("brew install pdfcrack".to_string()),
            execution_time_estimate: 300,
        },
        KaliTool {
            name: "vnc-crack".to_string(),
            category: KaliToolCategory::PasswordAttacks,
            description: "VNC password cracker".to_string(),
            command: "vnc-crack".to_string(),
            typical_args: vec![],
            requires_root: false,
            install_command: Some("brew install vnc-crack".to_string()),
            execution_time_estimate: 300,
        },

        // ========== COMPREHENSIVE FUZZING TOOLS ==========
        KaliTool {
            name: "spike".to_string(),
            category: KaliToolCategory::Fuzzing,
            description: "Network protocol fuzzer".to_string(),
            command: "spike".to_string(),
            typical_args: vec![],
            requires_root: false,
            install_command: Some("brew install spike".to_string()),
            execution_time_estimate: 0,
        },
        KaliTool {
            name: "peach".to_string(),
            category: KaliToolCategory::Fuzzing,
            description: "Smart fuzzer".to_string(),
            command: "peach".to_string(),
            typical_args: vec![],
            requires_root: false,
            install_command: Some("brew install peach".to_string()),
            execution_time_estimate: 0,
        },
        KaliTool {
            name: "sulley".to_string(),
            category: KaliToolCategory::Fuzzing,
            description: "Fuzzer development and fuzz testing framework".to_string(),
            command: "sulley".to_string(),
            typical_args: vec![],
            requires_root: false,
            install_command: Some("pip3 install sulley".to_string()),
            execution_time_estimate: 0,
        },
        KaliTool {
            name: "boofuzz".to_string(),
            category: KaliToolCategory::Fuzzing,
            description: "Network protocol fuzzing for humans".to_string(),
            command: "boofuzz".to_string(),
            typical_args: vec![],
            requires_root: false,
            install_command: Some("pip3 install boofuzz".to_string()),
            execution_time_estimate: 0,
        },
        KaliTool {
            name: "kitty".to_string(),
            category: KaliToolCategory::Fuzzing,
            description: "Fuzzer framework".to_string(),
            command: "kitty".to_string(),
            typical_args: vec![],
            requires_root: false,
            install_command: Some("pip3 install kitty".to_string()),
            execution_time_estimate: 0,
        },

        // ========== COMPREHENSIVE REPORTING TOOLS ==========
        KaliTool {
            name: "casefile".to_string(),
            category: KaliToolCategory::Reporting,
            description: "Mind mapping tool".to_string(),
            command: "casefile".to_string(),
            typical_args: vec![],
            requires_root: false,
            install_command: Some("brew install casefile".to_string()),
            execution_time_estimate: 0,
        },
        KaliTool {
            name: "maltego".to_string(),
            category: KaliToolCategory::Reporting,
            description: "Open source intelligence and forensics".to_string(),
            command: "maltego".to_string(),
            typical_args: vec![],
            requires_root: false,
            install_command: Some("brew install maltego".to_string()),
            execution_time_estimate: 0,
        },
        KaliTool {
            name: "recon-ng".to_string(),
            category: KaliToolCategory::Reporting,
            description: "Web reconnaissance framework".to_string(),
            command: "recon-ng".to_string(),
            typical_args: vec![],
            requires_root: false,
            install_command: Some("brew install recon-ng".to_string()),
            execution_time_estimate: 0,
        },
        KaliTool {
            name: "keepnote".to_string(),
            category: KaliToolCategory::Reporting,
            description: "Note taking and organization".to_string(),
            command: "keepnote".to_string(),
            typical_args: vec![],
            requires_root: false,
            install_command: Some("brew install keepnote".to_string()),
            execution_time_estimate: 0,
        },
        KaliTool {
            name: "cherrytree".to_string(),
            category: KaliToolCategory::Reporting,
            description: "Hierarchical note taking application".to_string(),
            command: "cherrytree".to_string(),
            typical_args: vec![],
            requires_root: false,
            install_command: Some("brew install cherrytree".to_string()),
            execution_time_estimate: 0,
        },

        // ========== COMPREHENSIVE SYSTEM TOOLS ==========
        KaliTool {
            name: "chkconfig".to_string(),
            category: KaliToolCategory::Forensics,
            description: "System service manager".to_string(),
            command: "chkconfig".to_string(),
            typical_args: vec![],
            requires_root: true,
            install_command: Some("brew install chkconfig".to_string()),
            execution_time_estimate: 30,
        },
        KaliTool {
            name: "sysctl".to_string(),
            category: KaliToolCategory::Forensics,
            description: "Configure kernel parameters".to_string(),
            command: "sysctl".to_string(),
            typical_args: vec!["-a".to_string()],
            requires_root: true,
            install_command: Some("brew install procps".to_string()),
            execution_time_estimate: 10,
        },
        KaliTool {
            name: "lsof".to_string(),
            category: KaliToolCategory::Forensics,
            description: "List open files".to_string(),
            command: "lsof".to_string(),
            typical_args: vec![],
            requires_root: true,
            install_command: Some("brew install lsof".to_string()),
            execution_time_estimate: 30,
        },
        KaliTool {
            name: "ps".to_string(),
            category: KaliToolCategory::Forensics,
            description: "Process status".to_string(),
            command: "ps".to_string(),
            typical_args: vec!["aux".to_string()],
            requires_root: false,
            install_command: Some("brew install procps".to_string()),
            execution_time_estimate: 5,
        },
        KaliTool {
            name: "netstat".to_string(),
            category: KaliToolCategory::NetworkForensics,
            description: "Network statistics".to_string(),
            command: "netstat".to_string(),
            typical_args: vec!["-tuln".to_string()],
            requires_root: false,
            install_command: Some("brew install net-tools".to_string()),
            execution_time_estimate: 10,
        },
        KaliTool {
            name: "ss".to_string(),
            category: KaliToolCategory::NetworkForensics,
            description: "Socket statistics".to_string(),
            command: "ss".to_string(),
            typical_args: vec!["-tuln".to_string()],
            requires_root: false,
            install_command: Some("brew install iproute2".to_string()),
            execution_time_estimate: 10,
        },
        KaliTool {
            name: "iptables".to_string(),
            category: KaliToolCategory::Forensics,
            description: "Firewall administration".to_string(),
            command: "iptables".to_string(),
            typical_args: vec!["-L".to_string()],
            requires_root: true,
            install_command: Some("brew install iptables".to_string()),
            execution_time_estimate: 10,
        },
        KaliTool {
            name: "ufw".to_string(),
            category: KaliToolCategory::Forensics,
            description: "Uncomplicated firewall".to_string(),
            command: "ufw".to_string(),
            typical_args: vec!["status".to_string()],
            requires_root: true,
            install_command: Some("brew install ufw".to_string()),
            execution_time_estimate: 5,
        },

        // ========== ADDITIONAL INFORMATION GATHERING TOOLS ==========
        KaliTool {
            name: "whois".to_string(),
            category: KaliToolCategory::OSInt,
            description: "Domain information lookup".to_string(),
            command: "whois".to_string(),
            typical_args: vec![],
            requires_root: false,
            install_command: Some("brew install whois".to_string()),
            execution_time_estimate: 10,
        },
        KaliTool {
            name: "dig".to_string(),
            category: KaliToolCategory::DnsEnumeration,
            description: "DNS lookup utility".to_string(),
            command: "dig".to_string(),
            typical_args: vec![],
            requires_root: false,
            install_command: Some("brew install dnsutils".to_string()),
            execution_time_estimate: 5,
        },
        KaliTool {
            name: "nslookup".to_string(),
            category: KaliToolCategory::DnsEnumeration,
            description: "DNS query tool".to_string(),
            command: "nslookup".to_string(),
            typical_args: vec![],
            requires_root: false,
            install_command: Some("brew install dnsutils".to_string()),
            execution_time_estimate: 5,
        },
        KaliTool {
            name: "host".to_string(),
            category: KaliToolCategory::DnsEnumeration,
            description: "DNS lookup utility".to_string(),
            command: "host".to_string(),
            typical_args: vec![],
            requires_root: false,
            install_command: Some("brew install bind9-host".to_string()),
            execution_time_estimate: 5,
        },
        KaliTool {
            name: "traceroute".to_string(),
            category: KaliToolCategory::NetworkScanning,
            description: "Trace packet route".to_string(),
            command: "traceroute".to_string(),
            typical_args: vec![],
            requires_root: false,
            install_command: Some("brew install traceroute".to_string()),
            execution_time_estimate: 30,
        },
        KaliTool {
            name: "tracepath".to_string(),
            category: KaliToolCategory::NetworkScanning,
            description: "Trace packet path".to_string(),
            command: "tracepath".to_string(),
            typical_args: vec![],
            requires_root: false,
            install_command: Some("brew install iputils-tracepath".to_string()),
            execution_time_estimate: 30,
        },
        KaliTool {
            name: "mtr".to_string(),
            category: KaliToolCategory::NetworkScanning,
            description: "Network diagnostic tool".to_string(),
            command: "mtr".to_string(),
            typical_args: vec![],
            requires_root: false,
            install_command: Some("brew install mtr".to_string()),
            execution_time_estimate: 60,
        },
        KaliTool {
            name: "hping3".to_string(),
            category: KaliToolCategory::NetworkScanning,
            description: "TCP/IP packet assembler/analyzer".to_string(),
            command: "hping3".to_string(),
            typical_args: vec!["--traceroute".to_string()],
            requires_root: true,
            install_command: Some("brew install hping3".to_string()),
            execution_time_estimate: 60,
        },
        KaliTool {
            name: "fping".to_string(),
            category: KaliToolCategory::NetworkScanning,
            description: "Send ICMP echo probes".to_string(),
            command: "fping".to_string(),
            typical_args: vec![],
            requires_root: false,
            install_command: Some("brew install fping".to_string()),
            execution_time_estimate: 30,
        },
        KaliTool {
            name: "arping".to_string(),
            category: KaliToolCategory::NetworkScanning,
            description: "ARP ping utility".to_string(),
            command: "arping".to_string(),
            typical_args: vec![],
            requires_root: true,
            install_command: Some("brew install arping".to_string()),
            execution_time_estimate: 10,
        },
        KaliTool {
            name: "arp-scan".to_string(),
            category: KaliToolCategory::NetworkScanning,
            description: "ARP scanning tool".to_string(),
            command: "arp-scan".to_string(),
            typical_args: vec!["--localnet".to_string()],
            requires_root: true,
            install_command: Some("brew install arp-scan".to_string()),
            execution_time_estimate: 15,
        },
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

    pub async fn execute_tool(
        &mut self,
        tool: &KaliTool,
        args: &[String],
    ) -> Result<String, String> {
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
                output_summary: format!(
                    "Tool not found. Install: {}",
                    tool.install_command
                        .as_ref()
                        .unwrap_or(&"unknown".to_string())
                ),
                execution_time_ms: 0,
                next_steps: vec![format!(
                    "Install tool: {}",
                    tool.install_command.as_ref().unwrap_or(&"".to_string())
                )],
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

        let output = Command::new(&tool.command).args(args).output();

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
                    decision_type: if success {
                        DecisionType::ToolSelection
                    } else {
                        DecisionType::StrategyChange
                    },
                    reasoning: format!(
                        "Executed {} - Status: {}",
                        tool.name,
                        if success { "Success" } else { "Failed" }
                    ),
                    tool_selected: tool.name.clone(),
                    target: self.target.clone(),
                    success,
                    output_summary: combined_output.chars().take(500).collect(),
                    execution_time_ms: execution_time,
                    next_steps: if success {
                        vec![
                            "Analyze output for vulnerabilities".to_string(),
                            "Check for sensitive data".to_string(),
                        ]
                    } else {
                        vec![
                            "Review error logs".to_string(),
                            "Try alternative tool".to_string(),
                        ]
                    },
                };

                self.logger.log_decision(decision).await;

                // Analyze output for breaches
                self.breach_detector
                    .analyze_output(&combined_output, &tool.name);

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
                    next_steps: vec![
                        "Check tool installation".to_string(),
                        "Verify permissions".to_string(),
                    ],
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

        let recon_tools: Vec<KaliTool> = self
            .tools
            .iter()
            .filter(|t| {
                matches!(
                    t.category,
                    KaliToolCategory::NetworkScanning | KaliToolCategory::DnsEnumeration
                )
            })
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

        let vuln_tools: Vec<KaliTool> = self
            .tools
            .iter()
            .filter(|t| {
                matches!(
                    t.category,
                    KaliToolCategory::VulnerabilityAnalysis
                        | KaliToolCategory::WebApplicationAnalysis
                )
            })
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
        self.breach_detector
            .scan_files_for_sensitive_data(".")
            .await;

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
        Command::new("which")
            .arg(&self.command)
            .output()
            .map(|output| output.status.success())
            .unwrap_or(false)
    }
}
