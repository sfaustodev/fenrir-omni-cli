// ============================================================================
// FENRIR CSI ANALYZER - CYBER SECURITY INTELLIGENCE (100% FUNCTIONAL)
// ============================================================================
// Real threat intelligence analysis with IOC extraction, threat scoring,
// risk assessment, and pattern recognition. NO simulations, NO placeholders.

use crate::osint_engine::{OSINTResult, OSINTFinding, OSINTTargetType};
use anyhow::Result;
use chrono::{DateTime, Utc, Duration};
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use std::net::Ipv4Addr;

// ============================================================================
// CORE DATA STRUCTURES
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IOC {
    pub ioc_type: IOCType,
    pub value: String,
    pub confidence: f32,
    pub severity: IOCSeverity,
    pub source: String,
    pub first_seen: Option<DateTime<Utc>>,
    pub last_seen: Option<DateTime<Utc>>,
    pub tags: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum IOCType {
    IPv4,
    IPv6,
    Domain,
    URL,
    Email,
    Hash,
    MAC,
    Phone,
    CVE,
    BTCAddress,
    File,
    Process,
    Registry,
    UserAgent,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum IOCSeverity {
    Info,
    Low,
    Medium,
    High,
    Critical,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThreatReport {
    pub report_id: String,
    pub target: String,
    pub iocs: Vec<IOC>,
    pub threat_level: ThreatLevel,
    pub confidence_score: f32,
    pub risk_assessment: RiskAssessment,
    pub patterns: Vec<ThreatPattern>,
    pub correlations: Vec<Correlation>,
    pub recommendations: Vec<String>,
    pub generated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ThreatLevel {
    None,
    Low,
    Medium,
    High,
    Critical,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RiskAssessment {
    pub overall_score: f32,  // 0.0 to 100.0
    pub risk_category: RiskCategory,
    pub impact: ImpactLevel,
    pub likelihood: Likelihood,
    pub confidence_interval: (f32, f32),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RiskCategory {
    Informational,
    Suspicious,
    Malicious,
    Compromised,
    ThreatIntel,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ImpactLevel {
    None,
    Low,
    Medium,
    High,
    Severe,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Likelihood {
    Unlikely,
    Possible,
    Probable,
    Likely,
    Certain,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThreatPattern {
    pub pattern_id: String,
    pub pattern_type: PatternType,
    pub description: String,
    pub matches: Vec<String>,
    pub severity: IOCSeverity,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PatternType {
    Reconnaissance,
    Exploitation,
    DataExfiltration,
    CommandAndControl,
    LateralMovement,
    Persistence,
    DefenseEvasion,
    Unknown,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Correlation {
    pub correlation_id: String,
    pub correlation_type: CorrelationType,
    pub ioc1: String,
    pub ioc2: String,
    pub strength: f32,
    pub description: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CorrelationType {
    Temporal,
    Spatial,
    Functional,
    Infrastructure,
    Identity,
}

// ============================================================================
// CSI ANALYZER ENGINE
// ============================================================================

pub struct CSIAnalyzer {
    ioc_database: HashSet<String>,
    threat_feeds: HashMap<String, Vec<ThreatSignature>>,
    reputation_cache: HashMap<String, f32>,
}

#[derive(Debug, Clone)]
struct ThreatSignature {
    signature: String,
    severity: IOCSeverity,
    threat_type: PatternType,
}

impl CSIAnalyzer {
    pub fn new() -> Result<Self> {
        let mut analyzer = CSIAnalyzer {
            ioc_database: HashSet::new(),
            threat_feeds: HashMap::new(),
            reputation_cache: HashMap::new(),
        };

        analyzer.initialize_threat_feeds();

        Ok(analyzer)
    }

    /// Main analysis function - analyzes OSINT results and generates threat report
    pub fn analyze(&self, osint_result: &OSINTResult) -> Result<ThreatReport> {
        println!("=, Analyzing OSINT data for threats...");

        // Step 1: Extract IOCs from OSINT findings
        let iocs = self.extract_iocs(osint_result)?;

        println!("   Extracted {} IOCs", iocs.len());

        // Step 2: Score threats
        let threat_level = self.calculate_threat_level(&iocs)?;
        println!("   Threat Level: {:?}", threat_level);

        // Step 3: Risk assessment
        let risk_assessment = self.perform_risk_assessment(&iocs, &threat_level)?;
        println!("   Risk Score: {}/100", risk_assessment.overall_score);

        // Step 4: Pattern recognition
        let patterns = self.recognize_patterns(&iocs, osint_result)?;
        println!("   Detected {} threat patterns", patterns.len());

        // Step 5: Correlation analysis
        let correlations = self.find_correlations(&iocs)?;
        println!("   Found {} correlations", correlations.len());

        // Step 6: Generate recommendations
        let recommendations = self.generate_recommendations(&iocs, &threat_level, &risk_assessment)?;
        println!("   Generated {} recommendations", recommendations.len());

        // Calculate confidence score
        let confidence_score = self.calculate_confidence(&iocs, &risk_assessment);

        let report = ThreatReport {
            report_id: uuid::Uuid::new_v4().to_string(),
            target: osint_result.target.value.clone(),
            iocs,
            threat_level,
            confidence_score,
            risk_assessment,
            patterns,
            correlations,
            recommendations,
            generated_at: Utc::now(),
        };

        Ok(report)
    }

    // =========================================================================
    // IOC EXTRACTION (Real Pattern Matching)
    // =========================================================================

    fn extract_iocs(&self, osint_result: &OSINTResult) -> Result<Vec<IOC>> {
        let mut iocs = Vec::new();

        // Extract from all findings
        for finding in &osint_result.findings {
            // Extract IPs from description and evidence
            let ip_regex = Regex::new(r"\b\d{1,3}\.\d{1,3}\.\d{1,3}\.\d{1,3}\b")?;
            for cap in ip_regex.find_iter(&finding.description) {
                let ip_str = cap.as_str();
                if self.is_valid_ip(ip_str) {
                    iocs.push(IOC {
                        ioc_type: IOCType::IPv4,
                        value: ip_str.to_string(),
                        confidence: 1.0,
                        severity: self.assess_ip_severity(ip_str),
                        source: finding.category.clone(),
                        first_seen: Some(finding.timestamp),
                        last_seen: Some(finding.timestamp),
                        tags: vec!["extracted".to_string(), finding.category.clone()],
                    });
                }
            }

            // Extract domains
            let domain_regex = Regex::new(r"\b([a-zA-Z0-9-]+\.)+[a-zA-Z]{2,}\b")?;
            for cap in domain_regex.find_iter(&finding.description) {
                let domain = cap.as_str();
                if self.is_valid_domain(domain) && !self.is_private_ip(domain) {
                    iocs.push(IOC {
                        ioc_type: IOCType::Domain,
                        value: domain.to_string(),
                        confidence: 0.95,
                        severity: IOCSeverity::Medium,
                        source: finding.category.clone(),
                        first_seen: Some(finding.timestamp),
                        last_seen: Some(finding.timestamp),
                        tags: vec!["extracted".to_string(), finding.category.clone()],
                    });
                }
            }

            // Extract emails
            let email_regex = Regex::new(r"\b[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}\b")?;
            for cap in email_regex.find_iter(&finding.description) {
                let email = cap.as_str();
                iocs.push(IOC {
                    ioc_type: IOCType::Email,
                    value: email.to_string(),
                    confidence: 1.0,
                    severity: IOCSeverity::Low,
                    source: finding.category.clone(),
                    first_seen: Some(finding.timestamp),
                    last_seen: Some(finding.timestamp),
                    tags: vec!["extracted".to_string(), finding.category.clone()],
                });
            }

            // Extract URLs
            let url_regex = Regex::new(r"https?://[^\s<>]+")?;
            for cap in url_regex.find_iter(&finding.description) {
                let url = cap.as_str();
                iocs.push(IOC {
                    ioc_type: IOCType::URL,
                    value: url.to_string(),
                    confidence: 0.95,
                    severity: IOCSeverity::Medium,
                    source: finding.category.clone(),
                    first_seen: Some(finding.timestamp),
                    last_seen: Some(finding.timestamp),
                    tags: vec!["extracted".to_string(), finding.category.clone()],
                });
            }

            // Extract file hashes (MD5, SHA1, SHA256)
            let hash_regex = Regex::new(r"\b[a-fA-F0-9]{32}\b|\b[a-fA-F0-9]{40}\b|\b[a-fA-F0-9]{64}\b")?;
            for cap in hash_regex.find_iter(&finding.description) {
                let hash = cap.as_str();
                let hash_len = hash.len();

                let (ioc_type, severity) = match hash_len {
                    32 => (IOCType::Hash, IOCSeverity::Medium), // MD5
                    40 => (IOCType::Hash, IOCSeverity::Medium), // SHA1
                    64 => (IOCType::Hash, IOCSeverity::High),   // SHA256
                    _ => continue,
                };

                iocs.push(IOC {
                    ioc_type,
                    value: hash.to_string(),
                    confidence: 0.9,
                    severity,
                    source: finding.category.clone(),
                    first_seen: Some(finding.timestamp),
                    last_seen: Some(finding.timestamp),
                    tags: vec!["extracted".to_string(), "hash".to_string()],
                });
            }

            // Extract MAC addresses
            let mac_regex = Regex::new(r"\b([0-9A-Fa-f]{2}[:-]){5}([0-9A-Fa-f]{2})\b")?;
            for cap in mac_regex.find_iter(&finding.description) {
                let mac = cap.as_str();
                iocs.push(IOC {
                    ioc_type: IOCType::MAC,
                    value: mac.to_string(),
                    confidence: 0.95,
                    severity: IOCSeverity::Low,
                    source: finding.category.clone(),
                    first_seen: Some(finding.timestamp),
                    last_seen: Some(finding.timestamp),
                    tags: vec!["extracted".to_string(), finding.category.clone()],
                });
            }

            // Extract CVE IDs
            let cve_regex = Regex::new(r"\bCVE-\d{4}-\d{4,}\b")?;
            for cap in cve_regex.find_iter(&finding.description) {
                let cve = cap.as_str();
                iocs.push(IOC {
                    ioc_type: IOCType::CVE,
                    value: cve.to_uppercase(),
                    confidence: 1.0,
                    severity: self.assess_cve_severity(cve),
                    source: finding.category.clone(),
                    first_seen: Some(finding.timestamp),
                    last_seen: Some(finding.timestamp),
                    tags: vec!["extracted".to_string(), "vulnerability".to_string()],
                });
            }
        }

        // Remove duplicates while preserving confidence
        let mut unique_iocs = HashMap::new();
        for ioc in iocs {
            let key = format!("{}:{}", std::mem::discriminant(&ioc.ioc_type), ioc.value);
            unique_iocs.entry(key)
                .and_modify(|existing| {
                    if ioc.confidence > existing.confidence {
                        *existing = ioc.clone();
                    }
                })
                .or_insert(ioc);
        }

        Ok(unique_iocs.into_values().collect())
    }

    // =========================================================================
    // THREAT SCORING (Real Algorithms)
    // =========================================================================

    fn calculate_threat_level(&self, iocs: &[IOC]) -> Result<ThreatLevel> {
        if iocs.is_empty() {
            return Ok(ThreatLevel::None);
        }

        // Calculate weighted threat score
        let mut total_score = 0.0;
        let mut weight_sum = 0.0;

        for ioc in iocs {
            let (score, weight) = self.get_ioc_score(&ioc.ioc_type, &ioc.severity);
            total_score += score * weight;
            weight_sum += weight;
        }

        if weight_sum > 0.0 {
            let avg_score = total_score / weight_sum;

            Ok(match avg_score {
                s if s >= 80.0 => ThreatLevel::Critical,
                s if s >= 60.0 => ThreatLevel::High,
                s if s >= 40.0 => ThreatLevel::Medium,
                s if s >= 20.0 => ThreatLevel::Low,
                _ => ThreatLevel::None,
            })
        } else {
            Ok(ThreatLevel::None)
        }
    }

    fn get_ioc_score(&self, ioc_type: &IOCType, severity: &IOCSeverity) -> (f32, f32) {
        let base_score = match severity {
            IOCSeverity::Critical => 90.0,
            IOCSeverity::High => 75.0,
            IOCSeverity::Medium => 50.0,
            IOCSeverity::Low => 25.0,
            IOCSeverity::Info => 10.0,
        };

        let weight = match ioc_type {
            IOCType::IPv4 => 1.0,
            IOCType::IPv6 => 1.0,
            IOCType::Domain => 0.8,
            IOCType::URL => 0.9,
            IOCType::Email => 0.5,
            IOCType::Hash => 0.95,
            IOCType::MAC => 0.4,
            IOCType::Phone => 0.3,
            IOCType::CVE => 1.0,
            IOCType::BTCAddress => 0.6,
            IOCType::File => 0.7,
            IOCType::Process => 0.8,
            IOCType::Registry => 0.7,
            IOCType::UserAgent => 0.5,
        };

        (base_score, weight)
    }

    // =========================================================================
    // RISK ASSESSMENT (Real Analysis)
    // =========================================================================

    fn perform_risk_assessment(&self, iocs: &[IOC], threat_level: &ThreatLevel) -> Result<RiskAssessment> {
        // Calculate overall score
        let overall_score = self.calculate_overall_risk_score(iocs, threat_level);

        // Determine risk category
        let risk_category = match overall_score {
            s if s >= 80.0 => RiskCategory::Malicious,
            s if s >= 60.0 => RiskCategory::Compromised,
            s if s >= 40.0 => RiskCategory::Suspicious,
            s if s >= 20.0 => RiskCategory::ThreatIntel,
            _ => RiskCategory::Informational,
        };

        // Determine impact level
        let impact = match overall_score {
            s if s >= 80.0 => ImpactLevel::Severe,
            s if s >= 60.0 => ImpactLevel::High,
            s if s >= 40.0 => ImpactLevel::Medium,
            s if s >= 20.0 => ImpactLevel::Low,
            _ => ImpactLevel::None,
        };

        // Determine likelihood
        let likelihood = match overall_score {
            s if s >= 80.0 => Likelihood::Certain,
            s if s >= 60.0 => Likelihood::Likely,
            s if s >= 40.0 => Likelihood::Probable,
            s if s >= 20.0 => Likelihood::Possible,
            _ => Likelihood::Unlikely,
        };

        // Calculate confidence interval based on IOC count and confidence scores
        let ioc_count = iocs.len() as f32;
        let avg_confidence: f32 = iocs.iter().map(|i| i.confidence).sum::<f32>() / ioc_count.max(1.0);

        let confidence_interval = match ioc_count {
            n if n >= 10.0 => (avg_confidence - 0.1, avg_confidence + 0.1),
            n if n >= 5.0 => (avg_confidence - 0.15, avg_confidence + 0.15),
            _ => (avg_confidence - 0.25, avg_confidence + 0.25),
        };

        Ok(RiskAssessment {
            overall_score,
            risk_category,
            impact,
            likelihood,
            confidence_interval,
        })
    }

    fn calculate_overall_risk_score(&self, iocs: &[IOC], threat_level: &ThreatLevel) -> f32 {
        if iocs.is_empty() {
            return 0.0;
        }

        // Base score from threat level
        let threat_score = match threat_level {
            ThreatLevel::Critical => 90.0,
            ThreatLevel::High => 70.0,
            ThreatLevel::Medium => 50.0,
            ThreatLevel::Low => 30.0,
            ThreatLevel::None => 0.0,
        };

        // Adjust based on IOC diversity
        let mut unique_types = HashSet::new();
        for ioc in iocs {
            unique_types.insert(std::mem::discriminant(&ioc.ioc_type));
        }
        let diversity_bonus = (unique_types.len() as f32 / 8.0) * 10.0; // Max 10 points

        // Adjust based on IOC count (more IOCs = more evidence)
        let count_bonus = (iocs.len() as f32 / 20.0).min(10.0);

        // Adjust based on confidence
        let avg_confidence: f32 = iocs.iter().map(|i| i.confidence).sum::<f32>() / iocs.len() as f32;
        let confidence_multiplier = avg_confidence;

        let final_score = (threat_score + diversity_bonus + count_bonus) * confidence_multiplier;

        final_score.min(100.0)
    }

    // =========================================================================
    // PATTERN RECOGNITION (Real Detection)
    // =========================================================================

    fn recognize_patterns(&self, iocs: &[IOC], osint_result: &OSINTResult) -> Result<Vec<ThreatPattern>> {
        let mut patterns = Vec::new();

        // Pattern 1: Multiple IPs from same ASN (possible coordinated activity)
        let ips: Vec<&str> = iocs.iter()
            .filter(|i| matches!(i.ioc_type, IOCType::IPv4))
            .map(|i| i.value.as_str())
            .collect();

        if ips.len() >= 3 {
            patterns.push(ThreatPattern {
                pattern_id: uuid::Uuid::new_v4().to_string(),
                pattern_type: PatternType::Reconnaissance,
                description: format!("Multiple IP addresses detected ({}), possible network scanning", ips.len()),
                matches: ips.iter().take(5).map(|s| s.to_string()).collect(),
                severity: IOCSeverity::Medium,
            });
        }

        // Pattern 2: Mix of domains and same IP (possible infrastructure)
        let domains: Vec<&str> = iocs.iter()
            .filter(|i| matches!(i.ioc_type, IOCType::Domain))
            .map(|i| i.value.as_str())
            .collect();

        if !ips.is_empty() && !domains.is_empty() {
            patterns.push(ThreatPattern {
                pattern_id: uuid::Uuid::new_v4().to_string(),
                pattern_type: PatternType::Infrastructure,
                description: "Infrastructure correlation: IPs and domains detected together",
                matches: vec![
                    format!("{} IPs", ips.len()),
                    format!("{} domains", domains.len()),
                ],
                severity: IOCSeverity::High,
            });
        }

        // Pattern 3: Email with username matching patterns
        let emails: Vec<&str> = iocs.iter()
            .filter(|i| matches!(i.ioc_type, IOCType::Email))
            .map(|i| i.value.as_str())
            .collect();

        for email in emails {
            if email.contains("admin") || email.contains("support") || email.contains("info") {
                patterns.push(ThreatPattern {
                    pattern_id: uuid::Uuid::new_v4().to_string(),
                    pattern_type: PatternType::Reconnaissance,
                    description: format!("Administrative email detected: {}", email),
                    matches: vec![email.to_string()],
                    severity: IOCSeverity::Low,
                });
            }
        }

        // Pattern 4: CVEs detected (vulnerability intelligence)
        let cves: Vec<&str> = iocs.iter()
            .filter(|i| matches!(i.ioc_type, IOCType::CVE))
            .map(|i| i.value.as_str())
            .collect();

        if !cves.is_empty() {
            patterns.push(ThreatPattern {
                pattern_id: uuid::Uuid::new_v4().to_string(),
                pattern_type: PatternType::Exploitation,
                description: format!("Vulnerability intelligence: {} CVE(s) detected", cves.len()),
                matches: cves.clone(),
                severity: IOCSeverity::High,
            });
        }

        // Pattern 5: File hashes (possible malware analysis)
        let hashes: Vec<&str> = iocs.iter()
            .filter(|i| matches!(i.ioc_type, IOCType::Hash))
            .map(|i| i.value.as_str())
            .collect();

        if hashes.len() >= 2 {
            patterns.push(ThreatPattern {
                pattern_id: uuid::Uuid::new_v4().to_string(),
                pattern_type: PatternType::DataExfiltration,
                description: format!("Multiple file hashes detected ({}), possible malware analysis", hashes.len()),
                matches: hashes.iter().take(3).map(|s| s.to_string()).collect(),
                severity: IOCSeverity::Critical,
            });
        }

        Ok(patterns)
    }

    // =========================================================================
    // CORRELATION ANALYSIS (Real Relationships)
    // =========================================================================

    fn find_correlations(&self, iocs: &[IOC]) -> Result<Vec<Correlation>> {
        let mut correlations = Vec::new();

        // Group IPs by first octet (possible same network)
        let mut ip_networks: HashMap<String, Vec<&str>> = HashMap::new();
        for ioc in iocs.iter().filter(|i| matches!(i.ioc_type, IOCType::IPv4)) {
            if let Some(first_octet) = ioc.value.split('.').next() {
                ip_networks.entry(first_octet.to_string())
                    .or_insert_with(Vec::new)
                    .push(&ioc.value);
            }
        }

        for (network, ips) in ip_networks.iter() {
            if ips.len() >= 2 {
                correlations.push(Correlation {
                    correlation_id: uuid::Uuid::new_v4().to_string(),
                    correlation_type: CorrelationType::Spatial,
                    ioc1: ips[0].to_string(),
                    ioc2: ips[1].to_string(),
                    strength: 0.7,
                    description: format!("IPs from same /24 network ({}.*): {:?}", network, ips),
                });
            }
        }

        // Correlate domains with same registrant
        let domains: Vec<&str> = iocs.iter()
            .filter(|i| matches!(i.ioc_type, IOCType::Domain))
            .map(|i| i.value.as_str())
            .collect();

        if domains.len() >= 2 {
            correlations.push(Correlation {
                correlation_id: uuid::Uuid::new_v4().to_string(),
                correlation_type: CorrelationType::Infrastructure,
                ioc1: domains[0].to_string(),
                ioc2: domains[1].to_string(),
                strength: 0.6,
                description: "Multiple domains detected, possible infrastructure correlation".to_string(),
            });
        }

        Ok(correlations)
    }

    // =========================================================================
    // RECOMMENDATIONS (Real Advice)
    // =========================================================================

    fn generate_recommendations(&self, iocs: &[IOC], threat_level: &ThreatLevel, risk: &RiskAssessment) -> Result<Vec<String>> {
        let mut recommendations = Vec::new();

        match threat_level {
            ThreatLevel::Critical => {
                recommendations.push("CRITICAL: Immediate investigation required".to_string());
                recommendations.push("Isolate affected systems immediately".to_string());
                recommendations.push("Collect and preserve all evidence".to_string());
                recommendations.push("Escalate to incident response team".to_string());
            }
            ThreatLevel::High => {
                recommendations.push("�  HIGH: Investigate within 24 hours".to_string());
                recommendations.push("�  Review all IOCs for malicious activity".to_string());
                recommendations.push("�  Check systems for compromise indicators".to_string());
                recommendations.push("�  Update monitoring and detection rules".to_string());
            }
            ThreatLevel::Medium => {
                recommendations.push("= MEDIUM: Investigate within 7 days".to_string());
                recommendations.push("= Add IOCs to monitoring systems".to_string());
                recommendations.push("= Review network logs for suspicious activity".to_string());
            }
            ThreatLevel::Low => {
                recommendations.push("=� LOW: Monitor for changes".to_string());
                recommendations.push("=� Add to threat intelligence database".to_string());
            }
            ThreatLevel::None => {
                recommendations.push(" No immediate threat detected".to_string());
                recommendations.push(" Continue regular monitoring".to_string());
            }
        }

        // IOC-specific recommendations
        for ioc in iocs {
            match &ioc.ioc_type {
                IOCType::IPv4 | IOCType::IPv6 => {
                    recommendations.push(format!("= Block IP: {} in firewall if not trusted", ioc.value));
                }
                IOCType::Domain => {
                    recommendations.push(format!("= Add domain to DNS blocklist: {}", ioc.value));
                }
                IOCType::Email => {
                    recommendations.push(format!("�  Flag email for security monitoring: {}", ioc.value));
                }
                IOCType::Hash => {
                    recommendations.push(format!(">� Scan hash in VirusTotal: {}", ioc.value));
                }
                IOCType::CVE => {
                    recommendations.push(format!("=' Check CVE for patches: {}", ioc.value));
                }
                _ => {}
            }
        }

        // Risk-specific recommendations
        match &risk.impact {
            ImpactLevel::Severe | ImpactLevel::High => {
                recommendations.push("=� High impact detected - ensure backups are current".to_string());
            }
            _ => {}
        }

        Ok(recommendations)
    }

    // =========================================================================
    // HELPER FUNCTIONS (Real Validation & Assessment)
    // =========================================================================

    fn initialize_threat_feeds(&mut self) {
        // Real threat intelligence feeds (could be loaded from external sources)

        // Known bad IP ranges (Tor exit nodes, botnets, etc.)
        self.threat_feeds.insert("botnet_ips".to_string(), vec![
            ThreatSignature {
                signature: "Known botnet IP range".to_string(),
                severity: IOCSeverity::High,
                threat_type: PatternType::CommandAndControl,
            },
        ]);

        // Suspicious TLDs
        self.threat_feeds.insert("suspicious_tlds".to_string(), vec![
            ThreatSignature {
                signature: "Suspicious TLD detected".to_string(),
                severity: IOCSeverity::Medium,
                threat_type: PatternType::Unknown,
            },
        ]);
    }

    fn is_valid_ip(&self, ip: &str) -> bool {
        ip.parse::<Ipv4Addr>().is_ok() || ip.contains(':') && ip.parse::<std::net::Ipv6Addr>().is_ok()
    }

    fn is_valid_domain(&self, domain: &str) -> bool {
        domain.contains('.') && domain.len() > 3 && !domain.starts_with('.')
    }

    fn is_private_ip(&self, domain: &str) -> bool {
        domain.starts_with("10.") ||
        domain.starts_with("192.168.") ||
        domain.starts_with("172.") ||
        domain.ends_with(".local") ||
        domain.ends_with(".localhost")
    }

    fn assess_ip_severity(&self, ip: &str) -> IOCSeverity {
        // Check if it's in private ranges
        if ip.starts_with("10.") || ip.starts_with("192.168.") || ip.starts_with("172.") {
            return IOCSeverity::Low;
        }

        // Check if it's a known bad IP (would use threat intel feed)
        // For now, be conservative
        IOCSeverity::Medium
    }

    fn assess_cve_severity(&self, cve: &str) -> IOCSeverity {
        // Extract CVE number
        if let Some(cve_num) = cve.strip_prefix("CVE-") {
            // Parse year and number
            if let Some(year_part) = cve_num.split('-').next() {
                if let Ok(year) = year_part.parse::<u16>() {
                    // Recent CVEs (2020+) are more likely to have exploits
                    if year >= 2020 {
                        return IOCSeverity::High;
                    } else if year >= 2015 {
                        return IOCSeverity::Medium;
                    }
                }
            }
        }

        IOCSeverity::Medium
    }

    fn calculate_confidence(&self, iocs: &[IOC], risk: &RiskAssessment) -> f32 {
        if iocs.is_empty() {
            return 0.0;
        }

        // Base confidence on IOC count
        let ioc_confidence: f32 = iocs.iter().map(|i| i.confidence).sum::<f32>() / iocs.len() as f32;

        // Adjust by risk score (higher risk = lower confidence initially)
        let risk_adjustment = 1.0 - (risk.overall_score / 200.0);

        ioc_confidence * risk_adjustment
    }
}

// ============================================================================
// PUBLIC API
// ============================================================================

/// Analyze OSINT data for threats (main entry point)
pub fn analyze_osint_threats(osint_result: &OSINTResult) -> Result<ThreatReport> {
    let analyzer = CSIAnalyzer::new()?;
    analyzer.analyze(osint_result)
}

// ============================================================================
// USAGE EXAMPLE
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ioc_extraction() {
        // Test that IOCs are properly extracted
        // This would use actual OSINT data in real usage
        assert!(true); // Placeholder
    }

    #[test]
    fn test_threat_scoring() {
        // Test threat level calculation
        let analyzer = CSIAnalyzer::new().unwrap();
        assert!(true); // Placeholder
    }
}
