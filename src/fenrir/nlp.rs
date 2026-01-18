// ============================================================================
// FENRIR NLP v2.0 - FUZZY LOCAL INTERPRETATION
// ============================================================================
// Simple, robust, flawless interpretation without external API dependencies
// Uses fuzzy matching and pattern recognition for target/keyword detection

use chrono::Utc;
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::{self, OpenOptions};
use std::io::Write;
use std::path::PathBuf;

// ============================================================================
// PARSED COMMAND STRUCTURE
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParsedCommand {
    pub command: String,
    pub explanation: String,
    pub source: String,
    pub subject: Option<String>,
    pub context: Option<String>,
    pub keywords: Vec<String>,
    pub confidence: f32,
    pub needs_clarification: bool,
    pub clarification_questions: Vec<String>,
    pub suggestions: Vec<String>,
}

impl ParsedCommand {
    pub fn new() -> Self {
        ParsedCommand {
            command: String::new(),
            explanation: String::new(),
            source: "fuzzy_local".to_string(),
            subject: None,
            context: None,
            keywords: Vec::new(),
            confidence: 0.0,
            needs_clarification: false,
            clarification_questions: Vec::new(),
            suggestions: Vec::new(),
        }
    }
}

// ============================================================================
// KEYWORD DEFINITIONS WITH ALIASES AND TRIGGERS
// ============================================================================

#[derive(Debug, Clone)]
pub struct KeywordDefinition {
    pub keyword: String,
    pub aliases: Vec<String>,
    pub description: String,
    pub tools: Vec<String>,
    pub async_execution: bool,
    pub memory_limit_mb: u64,
    pub suggested_contexts: Vec<String>,
}

fn get_keyword_definitions() -> Vec<KeywordDefinition> {
    vec![
        KeywordDefinition {
            keyword: "password".to_string(),
            aliases: vec![
                "senha", "contraseña", "pass", "pwd", "crack", "brute", "bruteforce",
                "hash", "credential", "credencial", "login", "auth", "hydra", "hashcat",
                "john", "cracker", "dictionary", "wordlist"
            ].iter().map(|s| s.to_string()).collect(),
            description: "Password cracking and credential harvesting".to_string(),
            tools: vec!["cewl", "crunch", "hydra", "hashcat", "john", "patator", "medusa", "ncrack"]
                .iter().map(|s| s.to_string()).collect(),
            async_execution: true,
            memory_limit_mb: 666,
            suggested_contexts: vec![
                "stealth mode".to_string(),
                "aggressive mode".to_string(),
                "use rockyou wordlist".to_string(),
                "ssh brute force".to_string(),
                "ftp brute force".to_string(),
            ],
        },
        KeywordDefinition {
            keyword: "scan".to_string(),
            aliases: vec![
                "escanear", "escaneo", "scanner", "port", "porta", "nmap", "masscan",
                "rustscan", "enumerate", "enumerar", "discovery", "recon", "reconnaissance",
                "fingerprint", "detect", "probe", "sweep"
            ].iter().map(|s| s.to_string()).collect(),
            description: "Network scanning and enumeration".to_string(),
            tools: vec!["nmap", "masscan", "rustscan", "nikto", "nuclei"]
                .iter().map(|s| s.to_string()).collect(),
            async_execution: true,
            memory_limit_mb: 666,
            suggested_contexts: vec![
                "stealth scan".to_string(),
                "full port scan".to_string(),
                "service detection".to_string(),
                "vulnerability scan".to_string(),
                "OS detection".to_string(),
            ],
        },
        KeywordDefinition {
            keyword: "web".to_string(),
            aliases: vec![
                "website", "sitio", "site", "http", "https", "webapp", "application",
                "sql", "sqli", "injection", "xss", "lfi", "rfi", "directory", "dirbuster",
                "gobuster", "ffuf", "nikto", "burp", "zap"
            ].iter().map(|s| s.to_string()).collect(),
            description: "Web application vulnerability testing".to_string(),
            tools: vec!["nikto", "gobuster", "ffuf", "sqlmap", "wpscan", "xsstrike", "nuclei"]
                .iter().map(|s| s.to_string()).collect(),
            async_execution: true,
            memory_limit_mb: 666,
            suggested_contexts: vec![
                "directory bruteforce".to_string(),
                "SQL injection test".to_string(),
                "XSS detection".to_string(),
                "WordPress scan".to_string(),
                "API fuzzing".to_string(),
            ],
        },
        KeywordDefinition {
            keyword: "social".to_string(),
            aliases: vec![
                "osint", "instagram", "facebook", "twitter", "linkedin", "tiktok",
                "sherlock", "username", "usuario", "profile", "perfil", "email",
                "harvester", "recon-ng", "maltego", "spiderfoot", "people", "person"
            ].iter().map(|s| s.to_string()).collect(),
            description: "Social engineering and OSINT reconnaissance".to_string(),
            tools: vec!["sherlock", "theHarvester", "maltego", "recon-ng", "spiderfoot", "holehe"]
                .iter().map(|s| s.to_string()).collect(),
            async_execution: false,
            memory_limit_mb: 2048,
            suggested_contexts: vec![
                "find all profiles".to_string(),
                "email enumeration".to_string(),
                "username search".to_string(),
                "company recon".to_string(),
            ],
        },
        KeywordDefinition {
            keyword: "wireless".to_string(),
            aliases: vec![
                "wifi", "wlan", "aircrack", "wifite", "wireless", "802.11", "wpa",
                "wpa2", "wep", "handshake", "deauth", "pmkid", "reaver", "bully"
            ].iter().map(|s| s.to_string()).collect(),
            description: "Wireless network attacks and auditing".to_string(),
            tools: vec!["aircrack-ng", "wifite", "reaver", "bully", "kismet"]
                .iter().map(|s| s.to_string()).collect(),
            async_execution: false,
            memory_limit_mb: 2048,
            suggested_contexts: vec![
                "capture handshake".to_string(),
                "WPS attack".to_string(),
                "deauth attack".to_string(),
                "PMKID capture".to_string(),
            ],
        },
        KeywordDefinition {
            keyword: "oauth".to_string(),
            aliases: vec![
                "oauth2", "authentication", "autenticacao", "token", "jwt", "bearer",
                "sso", "saml", "openid", "oidc", "authorization", "redirect", "callback"
            ].iter().map(|s| s.to_string()).collect(),
            description: "OAuth2 and authentication security testing".to_string(),
            tools: vec!["burpsuite", "evilginx2", "modlishka", "mitmproxy"]
                .iter().map(|s| s.to_string()).collect(),
            async_execution: false,
            memory_limit_mb: 2048,
            suggested_contexts: vec![
                "token analysis".to_string(),
                "redirect URI test".to_string(),
                "state parameter check".to_string(),
                "scope enumeration".to_string(),
            ],
        },
        KeywordDefinition {
            keyword: "database".to_string(),
            aliases: vec![
                "db", "mysql", "postgres", "postgresql", "mssql", "oracle", "mongodb",
                "redis", "sql", "nosql", "dump", "exfiltrate", "odat"
            ].iter().map(|s| s.to_string()).collect(),
            description: "Database exploitation and enumeration".to_string(),
            tools: vec!["sqlmap", "odat", "mssqlclient.py", "mongoaudit"]
                .iter().map(|s| s.to_string()).collect(),
            async_execution: true,
            memory_limit_mb: 666,
            suggested_contexts: vec![
                "dump all tables".to_string(),
                "enumerate databases".to_string(),
                "privilege escalation".to_string(),
                "data exfiltration".to_string(),
            ],
        },
        KeywordDefinition {
            keyword: "forensic".to_string(),
            aliases: vec![
                "forensics", "forense", "memory", "memoria", "disk", "disco", "image",
                "autopsy", "volatility", "binwalk", "carve", "recover", "analysis"
            ].iter().map(|s| s.to_string()).collect(),
            description: "Digital forensics and analysis".to_string(),
            tools: vec!["autopsy", "volatility", "binwalk", "foremost", "bulk_extractor"]
                .iter().map(|s| s.to_string()).collect(),
            async_execution: false,
            memory_limit_mb: 2048,
            suggested_contexts: vec![
                "memory analysis".to_string(),
                "file carving".to_string(),
                "timeline analysis".to_string(),
                "artifact extraction".to_string(),
            ],
        },
        KeywordDefinition {
            keyword: "exploit".to_string(),
            aliases: vec![
                "explotar", "metasploit", "msf", "msfconsole", "payload", "shellcode",
                "vulnerability", "vulnerabilidad", "cve", "poc", "rce", "lpe"
            ].iter().map(|s| s.to_string()).collect(),
            description: "Exploitation and payload delivery".to_string(),
            tools: vec!["msfconsole", "searchsploit", "msfvenom"]
                .iter().map(|s| s.to_string()).collect(),
            async_execution: false,
            memory_limit_mb: 2048,
            suggested_contexts: vec![
                "search exploits".to_string(),
                "generate payload".to_string(),
                "reverse shell".to_string(),
                "post exploitation".to_string(),
            ],
        },
        KeywordDefinition {
            keyword: "privesc".to_string(),
            aliases: vec![
                "privilege", "escalation", "escalacao", "root", "admin", "sudo",
                "linpeas", "winpeas", "pspy", "suid", "capabilities"
            ].iter().map(|s| s.to_string()).collect(),
            description: "Privilege escalation techniques".to_string(),
            tools: vec!["linpeas.sh", "winpeas.exe", "pspy", "linux-exploit-suggester.sh"]
                .iter().map(|s| s.to_string()).collect(),
            async_execution: false,
            memory_limit_mb: 2048,
            suggested_contexts: vec![
                "SUID binaries".to_string(),
                "kernel exploits".to_string(),
                "misconfigurations".to_string(),
                "cron jobs".to_string(),
            ],
        },
        KeywordDefinition {
            keyword: "shell".to_string(),
            aliases: vec![
                "reverse", "bind", "netcat", "nc", "socat", "listener", "c2",
                "command", "control", "backdoor", "pwncat"
            ].iter().map(|s| s.to_string()).collect(),
            description: "Reverse shell and C2 operations".to_string(),
            tools: vec!["nc", "socat", "pwncat"]
                .iter().map(|s| s.to_string()).collect(),
            async_execution: false,
            memory_limit_mb: 2048,
            suggested_contexts: vec![
                "start listener".to_string(),
                "reverse shell".to_string(),
                "bind shell".to_string(),
                "encrypted channel".to_string(),
            ],
        },
        KeywordDefinition {
            keyword: "sniff".to_string(),
            aliases: vec![
                "capture", "captura", "wireshark", "tcpdump", "packet", "pacote",
                "mitm", "arp", "spoof", "ettercap", "bettercap", "responder"
            ].iter().map(|s| s.to_string()).collect(),
            description: "Network sniffing and MITM attacks".to_string(),
            tools: vec!["wireshark", "tcpdump", "ettercap", "bettercap", "responder"]
                .iter().map(|s| s.to_string()).collect(),
            async_execution: false,
            memory_limit_mb: 2048,
            suggested_contexts: vec![
                "capture traffic".to_string(),
                "ARP spoofing".to_string(),
                "credential sniffing".to_string(),
                "LLMNR poisoning".to_string(),
            ],
        },
        KeywordDefinition {
            keyword: "recon".to_string(),
            aliases: vec![
                "reconnaissance", "reconhecimento", "information", "gathering",
                "subdomain", "subdominio", "dns", "whois", "amass", "subfinder"
            ].iter().map(|s| s.to_string()).collect(),
            description: "Information gathering and reconnaissance".to_string(),
            tools: vec!["theHarvester", "amass", "subfinder", "dnsrecon", "whois"]
                .iter().map(|s| s.to_string()).collect(),
            async_execution: true,
            memory_limit_mb: 666,
            suggested_contexts: vec![
                "subdomain enumeration".to_string(),
                "DNS records".to_string(),
                "WHOIS lookup".to_string(),
                "certificate transparency".to_string(),
            ],
        },
    ]
}

// ============================================================================
// TARGET DETECTION PATTERNS
// ============================================================================

#[derive(Debug, Clone)]
pub struct DetectedTarget {
    pub value: String,
    pub target_type: TargetType,
    pub confidence: f32,
}

#[derive(Debug, Clone, PartialEq)]
pub enum TargetType {
    IPv4,
    IPv6,
    Domain,
    URL,
    Email,
    Username,
    CIDR,
    PhoneNumber,
    Unknown,
}

impl TargetType {
    pub fn as_str(&self) -> &str {
        match self {
            TargetType::IPv4 => "IPv4 Address",
            TargetType::IPv6 => "IPv6 Address",
            TargetType::Domain => "Domain",
            TargetType::URL => "URL",
            TargetType::Email => "Email",
            TargetType::Username => "Username",
            TargetType::CIDR => "CIDR Range",
            TargetType::PhoneNumber => "Phone Number",
            TargetType::Unknown => "Unknown",
        }
    }
}

fn detect_targets(input: &str) -> Vec<DetectedTarget> {
    let mut targets = Vec::new();
    
    // IPv4 pattern
    let ipv4_regex = Regex::new(r"\b(\d{1,3}\.\d{1,3}\.\d{1,3}\.\d{1,3})\b").unwrap();
    for cap in ipv4_regex.captures_iter(input) {
        let ip = cap.get(1).unwrap().as_str();
        // Validate IP octets
        let valid = ip.split('.').all(|octet| {
            octet.parse::<u8>().is_ok()
        });
        if valid {
            targets.push(DetectedTarget {
                value: ip.to_string(),
                target_type: TargetType::IPv4,
                confidence: 1.0,
            });
        }
    }
    
    // CIDR pattern
    let cidr_regex = Regex::new(r"\b(\d{1,3}\.\d{1,3}\.\d{1,3}\.\d{1,3}/\d{1,2})\b").unwrap();
    for cap in cidr_regex.captures_iter(input) {
        targets.push(DetectedTarget {
            value: cap.get(1).unwrap().as_str().to_string(),
            target_type: TargetType::CIDR,
            confidence: 1.0,
        });
    }
    
    // URL pattern
    let url_regex = Regex::new(r"(https?://[^\s]+)").unwrap();
    for cap in url_regex.captures_iter(input) {
        targets.push(DetectedTarget {
            value: cap.get(1).unwrap().as_str().to_string(),
            target_type: TargetType::URL,
            confidence: 1.0,
        });
    }
    
    // Email pattern
    let email_regex = Regex::new(r"\b([a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,})\b").unwrap();
    for cap in email_regex.captures_iter(input) {
        targets.push(DetectedTarget {
            value: cap.get(1).unwrap().as_str().to_string(),
            target_type: TargetType::Email,
            confidence: 1.0,
        });
    }
    
    // Domain pattern (after URL and email to avoid duplicates)
    let domain_regex = Regex::new(r"\b([a-zA-Z0-9][-a-zA-Z0-9]*\.)+[a-zA-Z]{2,}\b").unwrap();
    for cap in domain_regex.captures_iter(input) {
        let domain = cap.get(0).unwrap().as_str();
        // Skip if already captured as URL or email
        if !targets.iter().any(|t| t.value.contains(domain)) {
            targets.push(DetectedTarget {
                value: domain.to_string(),
                target_type: TargetType::Domain,
                confidence: 0.9,
            });
        }
    }
    
    // Username pattern (@username)
    let username_regex = Regex::new(r"@([a-zA-Z0-9_]{3,})").unwrap();
    for cap in username_regex.captures_iter(input) {
        targets.push(DetectedTarget {
            value: cap.get(1).unwrap().as_str().to_string(),
            target_type: TargetType::Username,
            confidence: 0.9,
        });
    }
    
    // Phone number pattern
    let phone_regex = Regex::new(r"\+?\d{10,15}").unwrap();
    for cap in phone_regex.captures_iter(input) {
        targets.push(DetectedTarget {
            value: cap.get(0).unwrap().as_str().to_string(),
            target_type: TargetType::PhoneNumber,
            confidence: 0.7,
        });
    }
    
    targets
}

// ============================================================================
// FUZZY KEYWORD MATCHING
// ============================================================================

fn fuzzy_match(input: &str, pattern: &str) -> f32 {
    let input_lower = input.to_lowercase();
    let pattern_lower = pattern.to_lowercase();
    
    // Exact match
    if input_lower == pattern_lower {
        return 1.0;
    }
    
    // Contains match
    if input_lower.contains(&pattern_lower) {
        return 0.9;
    }
    
    // Levenshtein-like similarity for short strings
    if input_lower.len() <= 3 || pattern_lower.len() <= 3 {
        return 0.0;
    }
    
    // Check if pattern is a prefix
    if input_lower.starts_with(&pattern_lower) || pattern_lower.starts_with(&input_lower) {
        return 0.8;
    }
    
    // Simple character overlap ratio
    let input_chars: std::collections::HashSet<char> = input_lower.chars().collect();
    let pattern_chars: std::collections::HashSet<char> = pattern_lower.chars().collect();
    let intersection = input_chars.intersection(&pattern_chars).count();
    let union = input_chars.union(&pattern_chars).count();
    
    if union > 0 {
        let jaccard = intersection as f32 / union as f32;
        if jaccard > 0.6 {
            return jaccard * 0.7;
        }
    }
    
    0.0
}

fn detect_keywords(input: &str) -> Vec<(String, f32)> {
    let definitions = get_keyword_definitions();
    let mut detected: HashMap<String, f32> = HashMap::new();
    let input_lower = input.to_lowercase();
    let words: Vec<&str> = input_lower.split_whitespace().collect();
    
    for def in &definitions {
        let mut max_score: f32 = 0.0;
        
        // Check main keyword
        for word in &words {
            let score = fuzzy_match(word, &def.keyword);
            if score > max_score {
                max_score = score;
            }
        }
        
        // Check aliases
        for alias in &def.aliases {
            for word in &words {
                let score = fuzzy_match(word, alias);
                if score > max_score {
                    max_score = score;
                }
            }
            
            // Also check if alias is contained in input
            if input_lower.contains(&alias.to_lowercase()) {
                let score = 0.85;
                if score > max_score {
                    max_score = score;
                }
            }
        }
        
        // Check if any tool name is mentioned
        for tool in &def.tools {
            if input_lower.contains(&tool.to_lowercase()) {
                let score = 0.95;
                if score > max_score {
                    max_score = score;
                }
            }
        }
        
        if max_score >= 0.5 {
            detected.insert(def.keyword.clone(), max_score);
        }
    }
    
    let mut result: Vec<(String, f32)> = detected.into_iter().collect();
    result.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
    result
}

// ============================================================================
// CONTEXT DETECTION
// ============================================================================

fn detect_context(input: &str) -> Option<String> {
    let input_lower = input.to_lowercase();
    
    // Stealth indicators
    let stealth_words = ["stealth", "quiet", "slow", "careful", "silencioso", "furtivo", "lento"];
    for word in stealth_words {
        if input_lower.contains(word) {
            return Some("stealth mode".to_string());
        }
    }
    
    // Aggressive indicators
    let aggressive_words = ["aggressive", "fast", "quick", "full", "complete", "all", "rapido", "completo", "todo"];
    for word in aggressive_words {
        if input_lower.contains(word) {
            return Some("aggressive mode".to_string());
        }
    }
    
    // Specific technique mentions
    if input_lower.contains("wordlist") || input_lower.contains("dictionary") {
        return Some("dictionary attack".to_string());
    }
    
    if input_lower.contains("brute") {
        return Some("brute force".to_string());
    }
    
    None
}

// ============================================================================
// SUGGESTION GENERATOR
// ============================================================================

fn generate_suggestions(targets: &[DetectedTarget], keywords: &[(String, f32)]) -> Vec<String> {
    let mut suggestions = Vec::new();
    let definitions = get_keyword_definitions();
    
    // If no target detected, suggest providing one
    if targets.is_empty() {
        suggestions.push("💡 Provide a target: IP (192.168.1.1), domain (example.com), email (user@domain.com), or @username".to_string());
    }
    
    // If no keywords detected, suggest based on target type
    if keywords.is_empty() {
        if let Some(target) = targets.first() {
            match target.target_type {
                TargetType::IPv4 | TargetType::CIDR => {
                    suggestions.push("💡 Try: scan, exploit, sniff".to_string());
                }
                TargetType::Domain | TargetType::URL => {
                    suggestions.push("💡 Try: web, scan, recon, oauth".to_string());
                }
                TargetType::Email => {
                    suggestions.push("💡 Try: social, password, recon".to_string());
                }
                TargetType::Username => {
                    suggestions.push("💡 Try: social, password".to_string());
                }
                _ => {
                    suggestions.push("💡 Try keywords: scan, web, password, social, exploit".to_string());
                }
            }
        } else {
            suggestions.push("💡 Available keywords: password, scan, web, social, wireless, oauth, database, forensic, exploit, privesc, shell, sniff, recon".to_string());
        }
    } else {
        // Suggest contexts based on detected keywords
        for (keyword, _score) in keywords.iter().take(2) {
            if let Some(def) = definitions.iter().find(|d| d.keyword == *keyword) {
                if !def.suggested_contexts.is_empty() {
                    let ctx = &def.suggested_contexts[0];
                    suggestions.push(format!("💡 For {}: try adding '{}'", keyword, ctx));
                }
            }
        }
    }
    
    // Suggest combining keywords if only one detected
    if keywords.len() == 1 {
        let keyword = &keywords[0].0;
        match keyword.as_str() {
            "scan" => suggestions.push("💡 Combine with: web, exploit, or password".to_string()),
            "web" => suggestions.push("💡 Combine with: scan, database, or oauth".to_string()),
            "password" => suggestions.push("💡 Combine with: social, or scan".to_string()),
            "social" => suggestions.push("💡 Combine with: password, or recon".to_string()),
            _ => {}
        }
    }
    
    suggestions
}

// ============================================================================
// MAIN PARSE FUNCTION - SIMPLE AND ROBUST
// ============================================================================

pub async fn parse_command(_client: &reqwest::Client, user_input: &str) -> Result<ParsedCommand, String> {
    let mut parsed = ParsedCommand::new();
    parsed.source = "fuzzy_local".to_string();
    
    // Step 1: Detect targets
    let targets = detect_targets(user_input);
    if let Some(target) = targets.first() {
        parsed.subject = Some(target.value.clone());
    }
    
    // Step 2: Detect keywords using fuzzy matching
    let keywords = detect_keywords(user_input);
    parsed.keywords = keywords.iter().map(|(k, _)| k.clone()).collect();
    
    // Step 3: Detect context
    parsed.context = detect_context(user_input);
    
    // Step 4: Calculate confidence
    let target_confidence = if parsed.subject.is_some() { 0.5 } else { 0.0 };
    let keyword_confidence = if !parsed.keywords.is_empty() {
        keywords.iter().map(|(_, s)| s).sum::<f32>() / keywords.len() as f32 * 0.5
    } else {
        0.0
    };
    parsed.confidence = target_confidence + keyword_confidence;
    
    // Step 5: Generate suggestions
    parsed.suggestions = generate_suggestions(&targets, &keywords);
    
    // Step 6: Check if clarification needed
    if parsed.subject.is_none() {
        parsed.needs_clarification = true;
        parsed.clarification_questions.push("What is the target? (IP, domain, email, or @username)".to_string());
    }
    
    if parsed.keywords.is_empty() {
        parsed.needs_clarification = true;
        parsed.clarification_questions.push("What type of attack? (password, scan, web, social, exploit, etc.)".to_string());
    }
    
    // Step 7: Generate explanation
    if !parsed.keywords.is_empty() {
        let definitions = get_keyword_definitions();
        let mut tools_list = Vec::new();
        for keyword in &parsed.keywords {
            if let Some(def) = definitions.iter().find(|d| d.keyword == *keyword) {
                tools_list.extend(def.tools.iter().take(3).cloned());
            }
        }
        parsed.explanation = format!(
            "Attack sequence: {} → Tools: {}",
            parsed.keywords.join(" + "),
            tools_list.join(", ")
        );
    } else {
        parsed.explanation = "No attack sequence detected".to_string();
    }
    
    // Step 8: Generate command summary
    parsed.command = format!(
        "fenrir_attack --target {} --keywords {} {}",
        parsed.subject.as_deref().unwrap_or("?"),
        parsed.keywords.join(","),
        parsed.context.as_deref().unwrap_or("")
    ).trim().to_string();
    
    // Step 9: Log interaction
    log_interaction(user_input, &parsed);
    
    Ok(parsed)
}

// ============================================================================
// HELPER FUNCTIONS
// ============================================================================

pub fn get_keyword_info(keyword: &str) -> Option<(String, Vec<String>, bool, u64)> {
    let definitions = get_keyword_definitions();
    definitions.iter()
        .find(|d| d.keyword == keyword)
        .map(|d| (d.description.clone(), d.tools.clone(), d.async_execution, d.memory_limit_mb))
}

pub fn get_all_keywords() -> Vec<String> {
    get_keyword_definitions().iter().map(|d| d.keyword.clone()).collect()
}

pub fn get_keyword_tools(keyword: &str) -> Vec<String> {
    let definitions = get_keyword_definitions();
    definitions.iter()
        .find(|d| d.keyword == keyword)
        .map(|d| d.tools.clone())
        .unwrap_or_default()
}

// ============================================================================
// LOGGING
// ============================================================================

fn log_interaction(input: &str, parsed: &ParsedCommand) {
    let path = history_path();
    if let Some(parent) = path.parent() {
        let _ = fs::create_dir_all(parent);
    }

    let entry = serde_json::json!({
        "timestamp": Utc::now().to_rfc3339(),
        "input": input,
        "subject": parsed.subject,
        "keywords": parsed.keywords,
        "context": parsed.context,
        "confidence": parsed.confidence,
        "suggestions": parsed.suggestions,
    });

    if let Ok(mut file) = OpenOptions::new().create(true).append(true).open(path) {
        let _ = writeln!(file, "{}", entry);
    }
}

fn history_path() -> PathBuf {
    let base = dirs::config_dir().unwrap_or_else(|| PathBuf::from("."));
    base.join("fenrir").join("nlp_history.jsonl")
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_detect_ipv4() {
        let targets = detect_targets("scan 192.168.1.1 for vulnerabilities");
        assert!(!targets.is_empty());
        assert_eq!(targets[0].target_type, TargetType::IPv4);
        assert_eq!(targets[0].value, "192.168.1.1");
    }

    #[test]
    fn test_detect_domain() {
        let targets = detect_targets("scan example.com");
        assert!(!targets.is_empty());
        assert_eq!(targets[0].target_type, TargetType::Domain);
    }

    #[test]
    fn test_detect_email() {
        let targets = detect_targets("crack password for user@example.com");
        assert!(!targets.is_empty());
        assert_eq!(targets[0].target_type, TargetType::Email);
    }

    #[test]
    fn test_detect_keywords() {
        let keywords = detect_keywords("scan for vulnerabilities and crack passwords");
        assert!(keywords.iter().any(|(k, _)| k == "scan"));
        assert!(keywords.iter().any(|(k, _)| k == "password"));
    }

    #[test]
    fn test_detect_keywords_portuguese() {
        let keywords = detect_keywords("escanear e quebrar senha");
        assert!(keywords.iter().any(|(k, _)| k == "scan"));
        assert!(keywords.iter().any(|(k, _)| k == "password"));
    }

    #[test]
    fn test_fuzzy_match() {
        assert!(fuzzy_match("password", "password") == 1.0);
        assert!(fuzzy_match("passwords", "password") > 0.5);
        assert!(fuzzy_match("xyz", "password") < 0.5);
    }

    #[test]
    fn test_context_detection() {
        assert_eq!(detect_context("scan stealth mode"), Some("stealth mode".to_string()));
        assert_eq!(detect_context("aggressive full scan"), Some("aggressive mode".to_string()));
    }
}
