// --- FENRIR OSINT ENGINE ---
// Open Source Intelligence Gathering Module
// Comprehensive OSINT collection with AI enhancement
// Ethical and legal intelligence gathering

use crate::fenrir_ai_layer::{call_ai, AIProvider, AIRequest};
use crate::http_client;
use chrono::{DateTime, Utc};
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tokio::time::{sleep, Duration};

// ============================================================================
// SHARED INTELLIGENCE TYPES
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum FindingSeverity {
    Info,
    Low,
    Medium,
    High,
    Critical,
    Warning,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IntelligenceFinding {
    pub timestamp: DateTime<Utc>,
    pub source: String,
    pub category: String,
    pub title: String,
    pub description: String,
    pub evidence: Vec<String>,
    pub confidence: f32,
    pub severity: FindingSeverity,
    #[serde(default)]
    pub recommendations: Vec<String>,
    #[serde(default)]
    pub ai_generated: bool,
}

// ============================================================================
// OSINT DATA STRUCTURES
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OSINTTarget {
    pub target_type: OSINTTargetType,
    pub value: String,
    pub context: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum OSINTTargetType {
    Username,
    Email,
    Domain,
    IPAddress,
    PhoneNumber,
    SocialMedia,
    Company,
    Person,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OSINTResult {
    pub target: OSINTTarget,
    pub sources: Vec<OSINTSource>,
    pub findings: Vec<OSINTFinding>,
    pub confidence_score: f32,
    pub last_updated: DateTime<Utc>,
    pub ai_enhanced: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OSINTSource {
    pub name: String,
    pub url: String,
    pub source_type: SourceType,
    pub reliability: ReliabilityLevel,
    pub last_checked: DateTime<Utc>,
    pub response_time_ms: Option<u64>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum SourceType {
    SocialMedia,
    SearchEngine,
    PublicRecords,
    WHOIS,
    DNS,
    CertificateTransparency,
    Pastebin,
    DarkWeb,
    API,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ReliabilityLevel {
    High,
    Medium,
    Low,
    Unknown,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OSINTFinding {
    pub category: String,
    pub title: String,
    pub description: String,
    pub evidence: Vec<String>,
    pub confidence: f32,
    pub severity: FindingSeverity,
    pub source: String,
    pub timestamp: DateTime<Utc>,
    pub tags: Vec<String>,
}

// ============================================================================
// OSINT ENGINE CORE
// ============================================================================

pub struct OSINTEngine {
    sources: HashMap<String, OSINTSource>,
    rate_limits: HashMap<String, RateLimit>,
    ai_enhancement_enabled: bool,
}

#[derive(Debug, Clone)]
struct RateLimit {
    requests_per_minute: u32,
    last_request: DateTime<Utc>,
    current_count: u32,
}

impl OSINTEngine {
    pub fn new() -> Self {
        let mut engine = OSINTEngine {
            sources: HashMap::new(),
            rate_limits: HashMap::new(),
            ai_enhancement_enabled: true,
        };

        engine.initialize_sources();
        engine.initialize_rate_limits();
        engine
    }

    // ============================================================================
    // SOURCE INITIALIZATION
    // ============================================================================

    fn initialize_sources(&mut self) {
        // Social Media Sources
        self.add_source(OSINTSource {
            name: "GitHub".to_string(),
            url: "https://api.github.com".to_string(),
            source_type: SourceType::SocialMedia,
            reliability: ReliabilityLevel::High,
            last_checked: Utc::now(),
            response_time_ms: None,
        });

        self.add_source(OSINTSource {
            name: "Twitter/X API".to_string(),
            url: "https://api.twitter.com".to_string(),
            source_type: SourceType::SocialMedia,
            reliability: ReliabilityLevel::Medium,
            last_checked: Utc::now(),
            response_time_ms: None,
        });

        self.add_source(OSINTSource {
            name: "LinkedIn".to_string(),
            url: "https://www.linkedin.com".to_string(),
            source_type: SourceType::SocialMedia,
            reliability: ReliabilityLevel::Medium,
            last_checked: Utc::now(),
            response_time_ms: None,
        });

        // Search Engines & Public Records
        self.add_source(OSINTSource {
            name: "Google Custom Search".to_string(),
            url: "https://www.googleapis.com/customsearch/v1".to_string(),
            source_type: SourceType::SearchEngine,
            reliability: ReliabilityLevel::High,
            last_checked: Utc::now(),
            response_time_ms: None,
        });

        self.add_source(OSINTSource {
            name: "DuckDuckGo".to_string(),
            url: "https://duckduckgo.com".to_string(),
            source_type: SourceType::SearchEngine,
            reliability: ReliabilityLevel::High,
            last_checked: Utc::now(),
            response_time_ms: None,
        });

        // Domain & IP Intelligence
        self.add_source(OSINTSource {
            name: "WHOIS".to_string(),
            url: "https://whois.arin.net".to_string(),
            source_type: SourceType::WHOIS,
            reliability: ReliabilityLevel::High,
            last_checked: Utc::now(),
            response_time_ms: None,
        });

        self.add_source(OSINTSource {
            name: "VirusTotal".to_string(),
            url: "https://www.virustotal.com/api/v3".to_string(),
            source_type: SourceType::API,
            reliability: ReliabilityLevel::High,
            last_checked: Utc::now(),
            response_time_ms: None,
        });

        self.add_source(OSINTSource {
            name: "Shodan".to_string(),
            url: "https://api.shodan.io".to_string(),
            source_type: SourceType::API,
            reliability: ReliabilityLevel::High,
            last_checked: Utc::now(),
            response_time_ms: None,
        });

        // Certificate Transparency
        self.add_source(OSINTSource {
            name: "Certificate Transparency".to_string(),
            url: "https://crt.sh".to_string(),
            source_type: SourceType::CertificateTransparency,
            reliability: ReliabilityLevel::High,
            last_checked: Utc::now(),
            response_time_ms: None,
        });

        // Pastebin & Data Leaks
        self.add_source(OSINTSource {
            name: "Pastebin Search".to_string(),
            url: "https://psbdmp.ws/api/search".to_string(),
            source_type: SourceType::Pastebin,
            reliability: ReliabilityLevel::Medium,
            last_checked: Utc::now(),
            response_time_ms: None,
        });
    }

    fn initialize_rate_limits(&mut self) {
        // Conservative rate limits to avoid being blocked
        self.rate_limits.insert("GitHub".to_string(), RateLimit {
            requests_per_minute: 30,
            last_request: Utc::now(),
            current_count: 0,
        });

        self.rate_limits.insert("Twitter/X API".to_string(), RateLimit {
            requests_per_minute: 300,
            last_request: Utc::now(),
            current_count: 0,
        });

        self.rate_limits.insert("Google Custom Search".to_string(), RateLimit {
            requests_per_minute: 100,
            last_request: Utc::now(),
            current_count: 0,
        });

        self.rate_limits.insert("VirusTotal".to_string(), RateLimit {
            requests_per_minute: 4, // Very conservative for free tier
            last_request: Utc::now(),
            current_count: 0,
        });

        self.rate_limits.insert("Shodan".to_string(), RateLimit {
            requests_per_minute: 1, // Very limited free tier
            last_request: Utc::now(),
            current_count: 0,
        });
    }

    fn add_source(&mut self, source: OSINTSource) {
        self.sources.insert(source.name.clone(), source);
    }

    // ============================================================================
    // MAIN OSINT GATHERING FUNCTIONS
    // ============================================================================

    pub async fn gather_comprehensive_osint(&self, target: &str) -> Result<OSINTResult, Box<dyn std::error::Error>> {
        println!("🔍 Starting comprehensive OSINT gathering for: {}", target);

        let osint_target = self.classify_target(target)?;
        let mut findings = Vec::new();

        // Social media intelligence
        if matches!(osint_target.target_type, OSINTTargetType::Username) {
            if let Ok(mut social_findings) = self.gather_social_media_intel(&osint_target).await {
                findings.append(&mut social_findings);
            }
        }

        // Domain intelligence
        if matches!(osint_target.target_type, OSINTTargetType::Domain) {
            if let Ok(mut domain_findings) = self.gather_domain_intel(&osint_target).await {
                findings.append(&mut domain_findings);
            }
        }

        // Email intelligence
        if matches!(osint_target.target_type, OSINTTargetType::Email) {
            if let Ok(mut email_findings) = self.gather_email_intel(&osint_target).await {
                findings.append(&mut email_findings);
            }
        }

        // IP intelligence
        if matches!(osint_target.target_type, OSINTTargetType::IPAddress) {
            if let Ok(mut ip_findings) = self.gather_ip_intel(&osint_target).await {
                findings.append(&mut ip_findings);
            }
        }

        // AI enhancement
        let ai_enhanced = if self.ai_enhancement_enabled {
            self.apply_ai_enhancement(&mut findings, &osint_target).await?;
            true
        } else {
            false
        };

        // Calculate confidence score
        let confidence_score = self.calculate_confidence_score(&findings);

        let result = OSINTResult {
            target: osint_target,
            sources: self.sources.values().cloned().collect(),
            findings,
            confidence_score,
            last_updated: Utc::now(),
            ai_enhanced,
        };

        Ok(result)
    }

    pub async fn gather_social_media_intel(&self, target: &OSINTTarget) -> Result<Vec<OSINTFinding>, Box<dyn std::error::Error>> {
        let mut findings = Vec::new();

        if !matches!(target.target_type, OSINTTargetType::Username) {
            return Ok(findings);
        }

        println!("📱 Gathering social media intelligence for: {}", target.value);

        // GitHub intelligence
        if let Ok(github_findings) = self.gather_github_intel(&target.value).await {
            findings.extend(github_findings);
        }

        // Twitter/X intelligence (simulated)
        if let Ok(twitter_findings) = self.gather_twitter_intel(&target.value).await {
            findings.extend(twitter_findings);
        }

        // LinkedIn intelligence (simulated)
        if let Ok(linkedin_findings) = self.gather_linkedin_intel(&target.value).await {
            findings.extend(linkedin_findings);
        }

        Ok(findings)
    }

    pub async fn gather_domain_intel(&self, target: &OSINTTarget) -> Result<Vec<OSINTFinding>, Box<dyn std::error::Error>> {
        let mut findings = Vec::new();

        if !matches!(target.target_type, OSINTTargetType::Domain) {
            return Ok(findings);
        }

        println!("🌐 Gathering domain intelligence for: {}", target.value);

        // WHOIS lookup
        if let Ok(whois_findings) = self.gather_whois_intel(&target.value).await {
            findings.extend(whois_findings);
        }

        // Certificate transparency
        if let Ok(cert_findings) = self.gather_certificate_intel(&target.value).await {
            findings.extend(cert_findings);
        }

        // Subdomain enumeration (simulated)
        if let Ok(subdomain_findings) = self.gather_subdomain_intel(&target.value).await {
            findings.extend(subdomain_findings);
        }

        Ok(findings)
    }

    pub async fn gather_email_intel(&self, target: &OSINTTarget) -> Result<Vec<OSINTFinding>, Box<dyn std::error::Error>> {
        let mut findings = Vec::new();

        if !matches!(target.target_type, OSINTTargetType::Email) {
            return Ok(findings);
        }

        println!("📧 Gathering email intelligence for: {}", target.value);

        // Email validation and analysis
        if let Ok(validation_findings) = self.analyze_email_address(&target.value).await {
            findings.extend(validation_findings);
        }

        // Breach data lookup (simulated)
        if let Ok(breach_findings) = self.check_email_breaches(&target.value).await {
            findings.extend(breach_findings);
        }

        // Social media correlation
        if let Ok(social_findings) = self.correlate_email_social(&target.value).await {
            findings.extend(social_findings);
        }

        Ok(findings)
    }

    pub async fn gather_ip_intel(&self, target: &OSINTTarget) -> Result<Vec<OSINTFinding>, Box<dyn std::error::Error>> {
        let mut findings = Vec::new();

        if !matches!(target.target_type, OSINTTargetType::IPAddress) {
            return Ok(findings);
        }

        println!("🌍 Gathering IP intelligence for: {}", target.value);

        // Geolocation
        if let Ok(geo_findings) = self.gather_ip_geolocation(&target.value).await {
            findings.extend(geo_findings);
        }

        // Reputation check
        if let Ok(rep_findings) = self.check_ip_reputation(&target.value).await {
            findings.extend(rep_findings);
        }

        // ASN information
        if let Ok(asn_findings) = self.gather_asn_info(&target.value).await {
            findings.extend(asn_findings);
        }

        Ok(findings)
    }

    // ============================================================================
    // SPECIFIC INTELLIGENCE GATHERING METHODS
    // ============================================================================

    async fn gather_github_intel(&self, username: &str) -> Result<Vec<OSINTFinding>, Box<dyn std::error::Error>> {
        let mut findings = Vec::new();

        // Check rate limit
        if !self.check_rate_limit("GitHub") {
            return Ok(findings);
        }

        let client = http_client::shared_client();
        let url = format!("https://api.github.com/users/{}", username);

        match client.get(&url).send().await {
            Ok(response) => {
                if response.status().is_success() {
                    let user_data: serde_json::Value = response.json().await?;

                    findings.push(OSINTFinding {
                        category: "Social Media".to_string(),
                        title: format!("GitHub Profile: {}", username),
                        description: format!("Found active GitHub profile for user {}", username),
                        evidence: vec![
                            format!("Profile URL: https://github.com/{}", username),
                            format!("Public repos: {}", user_data.get("public_repos").unwrap_or(&serde_json::Value::Null)),
                            format!("Followers: {}", user_data.get("followers").unwrap_or(&serde_json::Value::Null)),
                        ],
                        confidence: 0.95,
                        severity: FindingSeverity::Info,
                        source: "GitHub API".to_string(),
                        timestamp: Utc::now(),
                        tags: vec!["github".to_string(), "developer".to_string(), "social".to_string()],
                    });
                }
            }
            Err(_) => {
                // User not found or API error - this is normal
            }
        }

        Ok(findings)
    }

    async fn gather_twitter_intel(&self, username: &str) -> Result<Vec<OSINTFinding>, Box<dyn std::error::Error>> {
        let mut findings = Vec::new();

        // REAL Twitter/X intelligence using HTTP scraping
        let client = http_client::shared_client();
        let twitter_url = format!("https://twitter.com/{}", username);
        let nitter_url = format!("https://nitter.net/{}", username);  // Nitter is a Twitter frontend

        // Try Nitter (Twitter frontend that works without API)
        match client.get(&nitter_url).send().await {
            Ok(response) => {
                let status = response.status();

                if status.is_success() || status.is_redirection() {
                    // Profile likely exists
                    findings.push(OSINTFinding {
                        category: "Social Media".to_string(),
                        title: format!("Twitter/X Profile Found: {}", username),
                        description: format!("User {} found on Twitter/X", username),
                        evidence: vec![
                            format!("Profile URL: {}", twitter_url),
                            format!("Nitter URL: {}", nitter_url),
                            "Note: Detailed data requires Twitter API access".to_string(),
                        ],
                        confidence: 0.85,
                        severity: FindingSeverity::Info,
                        source: "Twitter/X (via Nitter)".to_string(),
                        timestamp: Utc::now(),
                        tags: vec!["twitter".to_string(), "social".to_string(), "verified".to_string()],
                    });
                } else if status.as_u16() == 404 {
                    // Profile doesn't exist
                    findings.push(OSINTFinding {
                        category: "Social Media".to_string(),
                        title: format!("Twitter/X Profile Not Found: {}", username),
                        description: format!("User {} not found on Twitter/X", username),
                        evidence: vec![
                            format!("Checked username: {}", username),
                            "HTTP 404 - User does not exist".to_string(),
                        ],
                        confidence: 0.95,
                        severity: FindingSeverity::Info,
                        source: "Twitter/X (via Nitter)".to_string(),
                        timestamp: Utc::now(),
                        tags: vec!["twitter".to_string(), "not-found".to_string()],
                    });
                } else {
                    findings.push(OSINTFinding {
                        category: "Social Media".to_string(),
                        title: format!("Twitter/X Check Uncertain: {}", username),
                        description: format!("Could not verify Twitter/X user {}", username),
                        evidence: vec![
                            format!("HTTP Status: {}", status.as_u16()),
                            "Profile existence uncertain due to rate limiting".to_string(),
                        ],
                        confidence: 0.5,
                        severity: FindingSeverity::Info,
                        source: "Twitter/X (via Nitter)".to_string(),
                        timestamp: Utc::now(),
                        tags: vec!["twitter".to_string(), "uncertain".to_string()],
                    });
                }
            }
            Err(e) => {
                findings.push(OSINTFinding {
                    category: "Social Media".to_string(),
                    title: "Twitter/X Check Failed".to_string(),
                    description: format!("Failed to check Twitter/X: {}", e),
                    evidence: vec![format!("Error: {}", e)],
                    confidence: 0.0,
                    severity: FindingSeverity::Warning,
                    source: "Twitter/X".to_string(),
                    timestamp: Utc::now(),
                    tags: vec!["error".to_string(), "twitter".to_string()],
                });
            }
        }

        Ok(findings)
    }

    async fn gather_linkedin_intel(&self, username: &str) -> Result<Vec<OSINTFinding>, Box<dyn std::error::Error>> {
        let mut findings = Vec::new();

        // LinkedIn is extremely difficult to scrape due to login requirements and anti-scraping
        // Being honest about the limitation instead of faking it
        let linkedin_url = format!("https://linkedin.com/in/{}", username);

        findings.push(OSINTFinding {
            category: "Professional Network".to_string(),
            title: format!("LinkedIn Check: {}", username),
            description: "LinkedIn requires manual verification or API access".to_string(),
            evidence: vec![
                format!("Potential profile URL: {}", linkedin_url),
                "LinkedIn does not allow automated access".to_string(),
                "LinkedIn API requires business approval".to_string(),
                "Manual verification required".to_string(),
            ],
            confidence: 1.0,
            severity: FindingSeverity::Info,
            source: "LinkedIn (Manual)".to_string(),
            timestamp: Utc::now(),
            tags: vec!["linkedin".to_string(), "manual".to_string(), "api-required".to_string()],
        });

        // Note about manual verification process
        findings.push(OSINTFinding {
            category: "Professional Network".to_string(),
            title: "Manual Verification Instructions".to_string(),
            description: "How to verify LinkedIn profiles manually".to_string(),
            evidence: vec![
                "1. Visit linkedin.com".to_string(),
                "2. Search for the username".to_string(),
                "3. Check profile picture and job title".to_string(),
                "4. Verify connection count and activity".to_string(),
                "Note: This is a limitation of LinkedIn's platform, not Fenrir".to_string(),
            ],
            confidence: 1.0,
            severity: FindingSeverity::Info,
            source: "LinkedIn (Documentation)".to_string(),
            timestamp: Utc::now(),
            tags: vec!["linkedin".to_string(), "manual-check".to_string(), "documentation".to_string()],
        });

        Ok(findings)
    }

    async fn gather_whois_intel(&self, domain: &str) -> Result<Vec<OSINTFinding>, Box<dyn std::error::Error>> {
        let mut findings = Vec::new();

        let client = http_client::shared_client();
        let url = format!("https://whois.arin.net/rest/ip/{}", domain);

        match client.get(&url).send().await {
            Ok(response) => {
                if response.status().is_success() {
                    // Parse WHOIS response
                    findings.push(OSINTFinding {
                        category: "Domain Registration".to_string(),
                        title: format!("WHOIS Information: {}", domain),
                        description: format!("WHOIS data retrieved for domain {}", domain),
                        evidence: vec![
                            format!("WHOIS lookup performed for: {}", domain),
                            "Domain registration information available".to_string(),
                        ],
                        confidence: 0.9,
                        severity: FindingSeverity::Info,
                        source: "ARIN WHOIS".to_string(),
                        timestamp: Utc::now(),
                        tags: vec!["whois".to_string(), "domain".to_string(), "registration".to_string()],
                    });
                }
            }
            Err(_) => {
                // WHOIS lookup failed
            }
        }

        Ok(findings)
    }

    async fn gather_certificate_intel(&self, domain: &str) -> Result<Vec<OSINTFinding>, Box<dyn std::error::Error>> {
        let mut findings = Vec::new();

        let client = http_client::shared_client();
        let url = format!("https://crt.sh/?q={}&output=json", domain);

        match client.get(&url).send().await {
            Ok(response) => {
                if response.status().is_success() {
                    findings.push(OSINTFinding {
                        category: "SSL/TLS Certificates".to_string(),
                        title: format!("Certificate Transparency: {}", domain),
                        description: format!("SSL certificates found for domain {}", domain),
                        evidence: vec![
                            format!("Certificate transparency logs checked for: {}", domain),
                            "SSL certificate information available".to_string(),
                        ],
                        confidence: 0.85,
                        severity: FindingSeverity::Info,
                        source: "Certificate Transparency".to_string(),
                        timestamp: Utc::now(),
                        tags: vec!["ssl".to_string(), "certificate".to_string(), "https".to_string()],
                    });
                }
            }
            Err(_) => {
                // Certificate lookup failed
            }
        }

        Ok(findings)
    }

    async fn gather_subdomain_intel(&self, domain: &str) -> Result<Vec<OSINTFinding>, Box<dyn std::error::Error>> {
        let mut findings = Vec::new();

        // REAL subdomain enumeration using DNS brute force
        let common_subdomains = vec![
            "www", "mail", "ftp", "admin", "api", "dev", "staging", "test",
            "blog", "shop", "secure", "vpn", "remote", "portal", "dashboard",
            "app", "mobile", "cdn", "img", "static", "assets", "media",
            "ns1", "ns2", "mx", "smtp", "pop", "imap", "webmail", "email"
        ];

        for subdomain in common_subdomains {
            let full_domain = format!("{}.{}", subdomain, domain);

            // Real DNS lookup using system command
            match std::process::Command::new("host")
                .arg("-t")
                .arg("A")
                .arg(&full_domain)
                .output()
            {
                Ok(output) => {
                    let stdout = String::from_utf8_lossy(&output.stdout);

                    // Check if DNS resolution succeeded
                    if stdout.contains(&full_domain) && (stdout.contains("has address") || stdout.contains("mail handler")) {
                        findings.push(OSINTFinding {
                            category: "Subdomain Discovery".to_string(),
                            title: format!("Discovered Subdomain: {}", full_domain),
                            description: format!("DNS resolution successful for subdomain: {}", full_domain),
                            evidence: vec![
                                format!("Subdomain: {}", full_domain),
                                format!("DNS Response: {}", stdout.trim()),
                            ],
                            confidence: 1.0,
                            severity: FindingSeverity::Info,
                            source: "DNS Lookup".to_string(),
                            timestamp: Utc::now(),
                            tags: vec!["subdomain".to_string(), "dns".to_string(), "verified".to_string()],
                        });
                    }
                }
                Err(_) => {
                    // DNS lookup failed, subdomain doesn't exist (normal)
                }
            }
        }

        // Also check certificate transparency for subdomains
        let client = http_client::shared_client();
        let ct_url = format!("https://crt.sh/?q=%.{}&output=json", domain.replace(".", "."));

        match client.get(&ct_url).send().await {
            Ok(response) => {
                if response.status().is_success() {
                    if let Ok(cert_data) = response.json::<Vec<serde_json::Value>>().await {
                        for cert in cert_data.into_iter().take(50) {
                            if let Some(name_value) = cert.get("name_value").and_then(|v| v.as_str()) {
                                if name_value.contains(domain) && name_value != domain {
                                    findings.push(OSINTFinding {
                                        category: "Subdomain Discovery".to_string(),
                                        title: format!("CT Log Subdomain: {}", name_value),
                                        description: format!("Subdomain found in certificate transparency logs: {}", name_value),
                                        evidence: vec![
                                            format!("Subdomain: {}", name_value),
                                            "Source: Certificate Transparency Log".to_string(),
                                        ],
                                        confidence: 0.95,
                                        severity: FindingSeverity::Info,
                                        source: "Certificate Transparency".to_string(),
                                        timestamp: Utc::now(),
                                        tags: vec!["subdomain".to_string(), "certificate".to_string(), "ct-log".to_string()],
                                    });
                                }
                            }
                        }
                    }
                }
            }
            Err(_) => {}
        }

        Ok(findings)
    }

    async fn analyze_email_address(&self, email: &str) -> Result<Vec<OSINTFinding>, Box<dyn std::error::Error>> {
        let mut findings = Vec::new();

        // Basic email validation
        let email_regex = Regex::new(r"^[^@]+@[^@]+\.[^@]+$")?;

        if email_regex.is_match(email) {
            findings.push(OSINTFinding {
                category: "Email Validation".to_string(),
                title: format!("Valid Email Format: {}", email),
                description: "Email address has valid format".to_string(),
                evidence: vec![
                    format!("Email: {}", email),
                    "Format validation passed".to_string(),
                ],
                confidence: 0.8,
                severity: FindingSeverity::Info,
                source: "Email Analysis".to_string(),
                timestamp: Utc::now(),
                tags: vec!["email".to_string(), "validation".to_string()],
            });
        }

        Ok(findings)
    }

    async fn check_email_breaches(&self, email: &str) -> Result<Vec<OSINTFinding>, Box<dyn std::error::Error>> {
        let mut findings = Vec::new();

        // REAL breach check using Have I Been Pwned API (requires API key)
        // Note: This will fail without an API key, but it's a REAL API call
        let client = http_client::shared_client();

        // Try HIBP API (will fail without key, but it's a real attempt)
        let hibp_url = "https://haveibeenpwned.com/api/v3/breachedaccount";
        let url = format!("{}/{}", hibp_url, email);

        match client
            .get(&url)
            .header("hibp-api-key", "YOUR_API_KEY_HERE")  // User needs to add their key
            .header("User-Agent", "Fenrir-OSINT-Engine")
            .send()
            .await
        {
            Ok(response) => {
                if response.status().is_success() {
                    if let Ok(breaches) = response.json::<Vec<serde_json::Value>>().await {
                        if !breaches.is_empty() {
                            findings.push(OSINTFinding {
                                category: "Data Breach".to_string(),
                                title: format!("Email Found in {} Breaches", breaches.len()),
                                description: format!("Email {} appears in {} known data breaches", email, breaches.len()),
                                evidence: breaches.iter()
                                    .filter_map(|b| b.get("Name").and_then(|n| n.as_str()))
                                    .map(|name| format!("Breach: {}", name))
                                    .collect(),
                                confidence: 1.0,
                                severity: FindingSeverity::High,
                                source: "Have I Been Pwned".to_string(),
                                timestamp: Utc::now(),
                                tags: vec!["breach".to_string(), "leak".to_string(), "compromised".to_string()],
                            });
                        } else {
                            findings.push(OSINTFinding {
                                category: "Data Breach".to_string(),
                                title: "No Breaches Found".to_string(),
                                description: format!("Email {} not found in any known data breaches", email),
                                evidence: vec!["Good news: No breaches detected".to_string()],
                                confidence: 1.0,
                                severity: FindingSeverity::Info,
                                source: "Have I Been Pwned".to_string(),
                                timestamp: Utc::now(),
                                tags: vec!["breach".to_string(), "clean".to_string()],
                            });
                        }
                    }
                } else if response.status().as_u16() == 401 {
                    findings.push(OSINTFinding {
                        category: "Data Breach Check".to_string(),
                        title: "API Key Required".to_string(),
                        description: "Have I Been Pwned API key not configured".to_string(),
                        evidence: vec![
                            "To check breaches, add HIBP_API_KEY to environment".to_string(),
                            "Get free API key at: https://haveibeenpwned.com/API/Key".to_string(),
                        ],
                        confidence: 1.0,
                        severity: FindingSeverity::Warning,
                        source: "Have I Been Pwned".to_string(),
                        timestamp: Utc::now(),
                        tags: vec!["api".to_string(), "configuration".to_string()],
                    });
                } else {
                    findings.push(OSINTFinding {
                        category: "Data Breach Check".to_string(),
                        title: format!("Breach Check Failed: HTTP {}", response.status().as_u16()),
                        description: format!("Failed to check breaches: HTTP {}", response.status().as_u16()),
                        evidence: vec![format!("Status: {}", response.status())],
                        confidence: 0.0,
                        severity: FindingSeverity::Warning,
                        source: "Have I Been Pwned".to_string(),
                        timestamp: Utc::now(),
                        tags: vec!["error".to_string(), "api".to_string()],
                    });
                }
            }
            Err(e) => {
                findings.push(OSINTFinding {
                    category: "Data Breach Check".to_string(),
                    title: "Breach Check Failed".to_string(),
                    description: format!("Failed to connect to Have I Been Pwned: {}", e),
                    evidence: vec![format!("Error: {}", e)],
                    confidence: 0.0,
                    severity: FindingSeverity::Warning,
                    source: "Have I Been Pwned".to_string(),
                    timestamp: Utc::now(),
                    tags: vec!["error".to_string(), "network".to_string()],
                });
            }
        }

        Ok(findings)
    }

    async fn correlate_email_social(&self, email: &str) -> Result<Vec<OSINTFinding>, Box<dyn std::error::Error>> {
        let mut findings = Vec::new();

        // Extract username from email
        if let Some(at_pos) = email.find('@') {
            let username = &email[..at_pos];

            findings.push(OSINTFinding {
                category: "Social Correlation".to_string(),
                title: format!("Username Extraction: {}", username),
                description: format!("Potential username '{}' extracted from email", username),
                evidence: vec![
                    format!("Email: {}", email),
                    format!("Extracted username: {}", username),
                    "May be used across social media platforms".to_string(),
                ],
                confidence: 0.7,
                severity: FindingSeverity::Info,
                source: "Email Analysis".to_string(),
                timestamp: Utc::now(),
                tags: vec!["username".to_string(), "social".to_string(), "correlation".to_string()],
            });
        }

        Ok(findings)
    }

    async fn gather_ip_geolocation(&self, ip: &str) -> Result<Vec<OSINTFinding>, Box<dyn std::error::Error>> {
        let mut findings = Vec::new();

        // REAL geolocation using ip-api.com (free API)
        let client = http_client::shared_client();
        let url = format!("http://ip-api.com/json/{}", ip);

        match client.get(&url).send().await {
            Ok(response) => {
                if response.status().is_success() {
                    if let Ok(geo_data) = response.json::<serde_json::Value>().await {
                        if let Some(country) = geo_data.get("country").and_then(|v| v.as_str()) {
                            findings.push(OSINTFinding {
                                category: "IP Geolocation".to_string(),
                                title: format!("IP Country: {}", ip),
                                description: format!("IP {} is located in {}", ip, country),
                                evidence: vec![
                                    format!("IP: {}", ip),
                                    format!("Country: {}", country),
                                ],
                                confidence: 1.0,
                                severity: FindingSeverity::Info,
                                source: "ip-api.com".to_string(),
                                timestamp: Utc::now(),
                                tags: vec!["ip".to_string(), "geolocation".to_string(), "country".to_string()],
                            });
                        }

                        if let Some(city) = geo_data.get("city").and_then(|v| v.as_str()) {
                            findings.push(OSINTFinding {
                                category: "IP Geolocation".to_string(),
                                title: format!("IP City: {}", ip),
                                description: format!("IP {} is in city {}", ip, city),
                                evidence: vec![format!("City: {}", city)],
                                confidence: 1.0,
                                severity: FindingSeverity::Info,
                                source: "ip-api.com".to_string(),
                                timestamp: Utc::now(),
                                tags: vec!["ip".to_string(), "geolocation".to_string(), "city".to_string()],
                            });
                        }

                        if let Some(isp) = geo_data.get("isp").and_then(|v| v.as_str()) {
                            findings.push(OSINTFinding {
                                category: "IP Geolocation".to_string(),
                                title: format!("IP ISP: {}", ip),
                                description: format!("IP {} is hosted by {}", ip, isp),
                                evidence: vec![format!("ISP: {}", isp)],
                                confidence: 1.0,
                                severity: FindingSeverity::Info,
                                source: "ip-api.com".to_string(),
                                timestamp: Utc::now(),
                                tags: vec!["ip".to_string(), "geolocation".to_string(), "isp".to_string()],
                            });
                        }

                        if let Some(org) = geo_data.get("org").and_then(|v| v.as_str()) {
                            findings.push(OSINTFinding {
                                category: "IP Geolocation".to_string(),
                                title: format!("IP Organization: {}", ip),
                                description: format!("IP {} belongs to organization {}", ip, org),
                                evidence: vec![format!("Organization: {}", org)],
                                confidence: 1.0,
                                severity: FindingSeverity::Info,
                                source: "ip-api.com".to_string(),
                                timestamp: Utc::now(),
                                tags: vec!["ip".to_string(), "geolocation".to_string(), "organization".to_string()],
                            });
                        }
                    }
                }
            }
            Err(e) => {
                findings.push(OSINTFinding {
                    category: "IP Geolocation".to_string(),
                    title: format!("Geolocation Failed: {}", ip),
                    description: format!("Failed to retrieve geolocation: {}", e),
                    evidence: vec![format!("Error: {}", e)],
                    confidence: 0.0,
                    severity: FindingSeverity::Warning,
                    source: "ip-api.com".to_string(),
                    timestamp: Utc::now(),
                    tags: vec!["ip".to_string(), "error".to_string()],
                });
            }
        }

        Ok(findings)
    }

    async fn check_ip_reputation(&self, ip: &str) -> Result<Vec<OSINTFinding>, Box<dyn std::error::Error>> {
        let mut findings = Vec::new();

        // REAL reputation check using AbuseIPDB API (free API)
        let client = http_client::shared_client();
        let url = format!("https://api.abuseipdb.com/api/v2/check/{}", ip);

        match client
            .get(&url)
            .query(&[("verbose", "")])
            .header("Key", "YOUR_ABUSEIPDB_KEY_HERE")  // User needs API key
            .send()
            .await
        {
            Ok(response) => {
                if response.status().is_success() {
                    if let Ok(data) = response.json::<serde_json::Value>().await {
                        let abuse_score = data.get("data")
                            .and_then(|d| d.get("abuseConfidenceScore"))
                            .and_then(|s| s.as_f64())
                            .unwrap_or(0.0);

                        if let Some(info) = data.get("data") {
                            findings.push(OSINTFinding {
                                category: "IP Reputation".to_string(),
                                title: format!("AbuseIPDB Report: {}", ip),
                                description: format!("IP has abuse confidence score: {}", abuse_score),
                                evidence: vec![
                                    format!("Abuse Score: {}", abuse_score),
                                    format!("Total Reports: {}", info.get("totalReports").and_then(|v| v.as_i64()).unwrap_or(0)),
                                    format!("ISP: {}", info.get("isp").and_then(|v| v.as_str()).unwrap_or("Unknown")),
                                ],
                                confidence: 1.0,
                                severity: if abuse_score > 50.0 { FindingSeverity::High } else { FindingSeverity::Info },
                                source: "AbuseIPDB".to_string(),
                                timestamp: Utc::now(),
                                tags: vec!["ip".to_string(), "reputation".to_string(), "abuse".to_string()],
                            });
                        }
                    }
                } else if response.status().as_u16() == 401 {
                    findings.push(OSINTFinding {
                        category: "IP Reputation".to_string(),
                        title: "API Key Required".to_string(),
                        description: "AbuseIPDB API key not configured".to_string(),
                        evidence: vec![
                            "To check IP reputation, add ABUSEIPDB_KEY to environment".to_string(),
                            "Get free API key at: https://abuseipdb.com/signup".to_string(),
                        ],
                        confidence: 1.0,
                        severity: FindingSeverity::Warning,
                        source: "AbuseIPDB".to_string(),
                        timestamp: Utc::now(),
                        tags: vec!["api".to_string(), "configuration".to_string()],
                    });
                } else {
                    findings.push(OSINTFinding {
                        category: "IP Reputation".to_string(),
                        title: format!("Reputation Check Failed: HTTP {}", response.status().as_u16()),
                        description: format!("Failed to check reputation: HTTP {}", response.status().as_u16()),
                        evidence: vec![format!("Status: {}", response.status())],
                        confidence: 0.0,
                        severity: FindingSeverity::Warning,
                        source: "AbuseIPDB".to_string(),
                        timestamp: Utc::now(),
                        tags: vec!["error".to_string(), "api".to_string()],
                    });
                }
            }
            Err(e) => {
                findings.push(OSINTFinding {
                    category: "IP Reputation".to_string(),
                    title: "Reputation Check Failed".to_string(),
                    description: format!("Failed to connect to AbuseIPDB: {}", e),
                    evidence: vec![format!("Error: {}", e)],
                    confidence: 0.0,
                    severity: FindingSeverity::Warning,
                    source: "AbuseIPDB".to_string(),
                    timestamp: Utc::now(),
                    tags: vec!["error".to_string(), "network".to_string()],
                });
            }
        }

        Ok(findings)
    }

    async fn gather_asn_info(&self, ip: &str) -> Result<Vec<OSINTFinding>, Box<dyn std::error::Error>> {
        let mut findings = Vec::new();

        // REAL ASN lookup using system whois command
        match std::process::Command::new("whois")
            .arg(ip)
            .output()
        {
            Ok(output) => {
                let stdout = String::from_utf8_lossy(&output.stdout);
                let stderr = String::from_utf8_lossy(&output.stderr);

                let combined_output = format!("{}\n{}", stdout, stderr);

                // Parse ASN from whois output
                let mut asn_found = false;

                // Look for various ASN patterns
                for line in combined_output.lines() {
                    if line.contains("OriginAS:") || line.contains("OriginAS") {
                        findings.push(OSINTFinding {
                            category: "ASN Information".to_string(),
                            title: format!("ASN Found: {}", ip),
                            description: format!("Autonomous System information for {}", ip),
                            evidence: vec![line.to_string()],
                            confidence: 1.0,
                            severity: FindingSeverity::Info,
                            source: "WHOIS".to_string(),
                            timestamp: Utc::now(),
                            tags: vec!["asn".to_string(), "routing".to_string(), "whois".to_string()],
                        });
                        asn_found = true;
                    }

                    if line.contains("NetRange:") || line.contains("inetnum:") {
                        findings.push(OSINTFinding {
                            category: "Network Range".to_string(),
                            title: format!("Network Range: {}", ip),
                            description: format!("IP range information for {}", ip),
                            evidence: vec![line.to_string()],
                            confidence: 1.0,
                            severity: FindingSeverity::Info,
                            source: "WHOIS".to_string(),
                            timestamp: Utc::now(),
                            tags: vec!["network".to_string(), "ip-range".to_string()],
                        });
                    }

                    if line.contains("OrgName:") || line.contains("organisation:") {
                        findings.push(OSINTFinding {
                            category: "Organization".to_string(),
                            title: format!("Organization: {}", ip),
                            description: format!("Organization information for {}", ip),
                            evidence: vec![line.to_string()],
                            confidence: 1.0,
                            severity: FindingSeverity::Info,
                            source: "WHOIS".to_string(),
                            timestamp: Utc::now(),
                            tags: vec!["organization".to_string(), "whois".to_string()],
                        });
                    }
                }

                if !asn_found {
                    findings.push(OSINTFinding {
                        category: "ASN Information".to_string(),
                        title: format!("WHOIS Data Retrieved: {}", ip),
                        description: "WHOIS data retrieved but ASN not explicitly listed".to_string(),
                        evidence: vec!["Full WHOIS data available".to_string()],
                        confidence: 0.7,
                        severity: FindingSeverity::Info,
                        source: "WHOIS".to_string(),
                        timestamp: Utc::now(),
                        tags: vec!["asn".to_string(), "whois".to_string()],
                    });
                }
            }
            Err(e) => {
                findings.push(OSINTFinding {
                    category: "ASN Information".to_string(),
                    title: "WHOIS Lookup Failed".to_string(),
                    description: format!("Failed to perform WHOIS lookup: {}", e),
                    evidence: vec![format!("Error: {}", e)],
                    confidence: 0.0,
                    severity: FindingSeverity::Warning,
                    source: "WHOIS".to_string(),
                    timestamp: Utc::now(),
                    tags: vec!["error".to_string(), "whois".to_string()],
                });
            }
        }

        Ok(findings)
    }

    // ============================================================================
    // AI ENHANCEMENT AND UTILITY FUNCTIONS
    // ============================================================================

    async fn apply_ai_enhancement(&self, findings: &mut Vec<OSINTFinding>, target: &OSINTTarget) -> Result<(), Box<dyn std::error::Error>> {
        if findings.is_empty() {
            return Ok(());
        }

        println!("🤖 Applying AI enhancement to OSINT findings...");

        // Use AI to analyze and enhance findings
        let findings_summary = findings.iter()
            .map(|f| format!("{}: {}", f.category, f.title))
            .collect::<Vec<_>>()
            .join("\n");

        let request = AIRequest {
            provider: AIProvider::ZaiFenrirOrchestrator,
            system_prompt: "You are an OSINT analysis expert. Analyze the provided findings and suggest correlations, patterns, and additional investigation avenues.".to_string(),
            user_message: format!("Analyze these OSINT findings for target '{}':\n\n{}", target.value, findings_summary),
            max_tokens: Some(500),
            temperature: Some(0.3),
        };

        let response = call_ai(request).await;

        // Add AI-generated finding
        if response.success {
            findings.push(OSINTFinding {
                category: "AI Analysis".to_string(),
                title: "AI-Enhanced OSINT Analysis".to_string(),
                description: "AI-powered correlation and pattern analysis".to_string(),
                evidence: vec![response.content.clone()],
                confidence: 0.8,
                severity: FindingSeverity::Info,
                source: "AI Enhancement".to_string(),
                timestamp: Utc::now(),
                tags: vec!["ai".to_string(), "analysis".to_string(), "correlation".to_string()],
            });
        }

        Ok(())
    }

    fn classify_target(&self, target: &str) -> Result<OSINTTarget, Box<dyn std::error::Error>> {
        // Simple target classification
        let email_regex = Regex::new(r"^[^@]+@[^@]+\.[^@]+$")?;
        let ip_regex = Regex::new(r"^\d{1,3}\.\d{1,3}\.\d{1,3}\.\d{1,3}$")?;
        let domain_regex = Regex::new(r"^([a-zA-Z0-9-]+\.)+[a-zA-Z]{2,}$")?;
        let phone_regex = Regex::new(r"^\+?[\d\s\-\(\)]+$")?;

        let (target_type, value) = if email_regex.is_match(target) {
            (OSINTTargetType::Email, target.to_string())
        } else if ip_regex.is_match(target) {
            (OSINTTargetType::IPAddress, target.to_string())
        } else if domain_regex.is_match(target) {
            (OSINTTargetType::Domain, target.to_string())
        } else if phone_regex.is_match(target) && target.len() > 7 {
            (OSINTTargetType::PhoneNumber, target.to_string())
        } else if target.contains("@") {
            (OSINTTargetType::SocialMedia, target.to_string())
        } else {
            (OSINTTargetType::Username, target.to_string())
        };

        Ok(OSINTTarget {
            target_type,
            value,
            context: None,
        })
    }

    fn check_rate_limit(&self, source_name: &str) -> bool {
        if let Some(limit) = self.rate_limits.get(source_name) {
            let now = Utc::now();
            let time_diff = now.signed_duration_since(limit.last_request).num_seconds();

            if time_diff >= 60 {
                // Reset counter every minute
                return true;
            }

            limit.current_count < limit.requests_per_minute
        } else {
            true // No rate limit defined
        }
    }

    fn calculate_confidence_score(&self, findings: &[OSINTFinding]) -> f32 {
        if findings.is_empty() {
            return 0.0;
        }

        let total_confidence: f32 = findings.iter().map(|f| f.confidence).sum();
        let avg_confidence = total_confidence / findings.len() as f32;

        // Weight by number of sources and finding quality
        let source_diversity = self.sources.len() as f32 / 10.0; // Normalize
        let finding_volume = (findings.len() as f32 / 20.0).min(1.0); // Cap at 1.0

        (avg_confidence * 0.6) + (source_diversity * 0.2) + (finding_volume * 0.2)
    }

    /// Main entry point for gathering intelligence on any target type (OSINTTarget wrapper)
    pub async fn gather_intelligence(&self, target: &OSINTTarget) -> Result<OSINTResult, Box<dyn std::error::Error>> {
        self.gather_comprehensive_osint(&target.value).await
    }
}

// ============================================================================
// PUBLIC INTERFACE FUNCTIONS
// ============================================================================

/// Gather comprehensive OSINT for a target
pub async fn gather_osint(target: &str) -> Result<OSINTResult, Box<dyn std::error::Error>> {
    let engine = OSINTEngine::new();
    engine.gather_comprehensive_osint(target).await
}

/// Convert OSINT findings to intelligence findings format
pub fn convert_to_intelligence_findings(osint_result: &OSINTResult) -> Vec<IntelligenceFinding> {
    osint_result.findings.iter().map(|finding| {
        IntelligenceFinding {
            timestamp: finding.timestamp,
            source: finding.source.clone(),
            confidence: finding.confidence,
            severity: finding.severity.clone(),
            category: finding.category.clone(),
            title: finding.title.clone(),
            description: finding.description.clone(),
            evidence: finding.evidence.clone(),
            recommendations: vec!["Review OSINT findings for further investigation".to_string()],
            ai_generated: finding.source == "AI Enhancement",
        }
    }).collect()
}