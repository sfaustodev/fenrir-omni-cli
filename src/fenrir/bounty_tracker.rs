//! # Fenrir Bug Bounty Tracker Module
//!
//! This module provides comprehensive bug bounty tracking and reporting functionality.
//! It helps security researchers organize findings, generate reports, and manage
//! their bug bounty submissions.
//!
//! ## Features
//! - Track multiple bug bounty programs
//! - Organize findings by target, severity, and status
//! - Generate professional reports for submissions
//! - Calculate potential earnings
//! - Export to various formats (JSON, Markdown, PDF)

use anyhow::{Context, Result};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

// ============================================================================
// SECTION 1: BUG BOUNTY PROGRAM TRACKING
// ============================================================================

/// Status of a vulnerability finding
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum FindingStatus {
    /// Currently investigating
    Investigating,
    /// Ready to submit
    ReadyToSubmit,
    /// Submitted to program
    Submitted,
    /// Triaged by program
    Triaged,
    /// Accepted by program
    Accepted,
    /// Resolved/Fixed
    Resolved,
    /// Rejected by program
    Rejected,
    /// Needs more information
    NeedsMoreInfo,
    /// Duplicate finding
    Duplicate,
}

impl FindingStatus {
    pub fn as_str(&self) -> &str {
        match self {
            FindingStatus::Investigating => "Investigating",
            FindingStatus::ReadyToSubmit => "Ready to Submit",
            FindingStatus::Submitted => "Submitted",
            FindingStatus::Triaged => "Triaged",
            FindingStatus::Accepted => "Accepted",
            FindingStatus::Resolved => "Resolved",
            FindingStatus::Rejected => "Rejected",
            FindingStatus::NeedsMoreInfo => "Needs More Info",
            FindingStatus::Duplicate => "Duplicate",
        }
    }
}

/// Severity level for vulnerabilities
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum Severity {
    Critical,
    High,
    Medium,
    Low,
    Info,
}

impl Severity {
    pub fn as_str(&self) -> &str {
        match self {
            Severity::Critical => "Critical",
            Severity::High => "High",
            Severity::Medium => "Medium",
            Severity::Low => "Low",
            Severity::Info => "Info",
        }
    }

    /// Typical bounty range for this severity level (in USD)
    pub fn typical_bounty_range(&self) -> (u32, u32) {
        match self {
            Severity::Critical => (10000, 100000),
            Severity::High => (3000, 10000),
            Severity::Medium => (500, 3000),
            Severity::Low => (100, 500),
            Severity::Info => (0, 100),
        }
    }
}

/// Represents a bug bounty program
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BountyProgram {
    /// Program name (e.g., "Google VRP")
    pub name: String,
    /// Program URL
    pub url: String,
    /// Platform (HackerOne, Bugcrowd, Intigriti, etc.)
    pub platform: String,
    /// Program policy URL
    pub policy_url: String,
    /// In-scope targets
    pub scope: Vec<String>,
    /// Out-of-scope targets
    pub out_of_scope: Vec<String>,
    /// Minimum bounty amount
    pub min_bounty: u32,
    /// Maximum bounty amount
    pub max_bounty: u32,
    /// Average payout time in days
    pub avg_payout_days: u32,
    /// Notes about the program
    pub notes: String,
}

/// Represents a vulnerability finding
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VulnerabilityFinding {
    /// Unique ID for this finding
    pub id: String,
    /// Finding title
    pub title: String,
    /// Vulnerability type (XSS, SQLi, OAuth, etc.)
    pub vuln_type: String,
    /// Severity level
    pub severity: Severity,
    /// Current status
    pub status: FindingStatus,
    /// Target program
    pub program: String,
    /// Affected URL/endpoint
    pub affected_url: String,
    /// Detailed description
    pub description: String,
    /// Impact analysis
    pub impact: String,
    /// Steps to reproduce
    pub steps_to_reproduce: Vec<String>,
    /// Proof of concept (screenshots, code, etc.)
    pub proof_of_concept: String,
    /// Suggested remediation
    pub remediation: String,
    /// Date discovered
    pub date_discovered: DateTime<Utc>,
    /// Date submitted
    pub date_submitted: Option<DateTime<Utc>>,
    /// Date resolved
    pub date_resolved: Option<DateTime<Utc>>,
    /// Bounty amount received (if any)
    pub bounty_amount: Option<u32>,
    /// Report ID from platform
    pub report_id: Option<String>,
    /// Researcher notes
    pub notes: String,
    /// OWASP category
    pub owasp_category: Option<String>,
    /// CWE identifier
    pub cwe_id: Option<String>,
    /// CVSS score
    pub cvss_score: Option<f32>,
}

/// Bug Bounty Tracker - Main tracking system
#[derive(Debug)]
pub struct BugBountyTracker {
    /// Database of findings
    findings: Vec<VulnerabilityFinding>,
    /// Programs being tracked
    programs: Vec<BountyProgram>,
    /// Storage directory
    storage_dir: PathBuf,
}

impl BugBountyTracker {
    /// Create a new bug bounty tracker
    ///
    /// # Arguments
    /// * `storage_dir` - Directory to store tracking data
    pub fn new(storage_dir: PathBuf) -> Self {
        Self {
            findings: Vec::new(),
            programs: Vec::new(),
            storage_dir,
        }
    }

    /// Load tracker data from disk
    pub fn load(&mut self) -> Result<()> {
        // Create storage directory if it doesn't exist
        fs::create_dir_all(&self.storage_dir)?;

        let findings_path = self.storage_dir.join("findings.json");
        let programs_path = self.storage_dir.join("programs.json");

        // Load findings
        if findings_path.exists() {
            let findings_data = fs::read_to_string(&findings_path)?;
            self.findings = serde_json::from_str(&findings_data)?;
            println!("✅ Loaded {} findings", self.findings.len());
        }

        // Load programs
        if programs_path.exists() {
            let programs_data = fs::read_to_string(&programs_path)?;
            self.programs = serde_json::from_str(&programs_data)?;
            println!("✅ Loaded {} programs", self.programs.len());
        }

        Ok(())
    }

    /// Save tracker data to disk
    pub fn save(&self) -> Result<()> {
        fs::create_dir_all(&self.storage_dir)?;

        let findings_path = self.storage_dir.join("findings.json");
        let programs_path = self.storage_dir.join("programs.json");

        // Save findings
        let findings_json = serde_json::to_string_pretty(&self.findings)?;
        fs::write(&findings_path, findings_json)?;

        // Save programs
        let programs_json = serde_json::to_string_pretty(&self.programs)?;
        fs::write(&programs_path, programs_json)?;

        println!("✅ Saved {} findings and {} programs", self.findings.len(), self.programs.len());

        Ok(())
    }

    /// Add a new vulnerability finding
    pub fn add_finding(&mut self, finding: VulnerabilityFinding) {
        println!("📝 Adding new finding: {}", finding.title);
        self.findings.push(finding);
    }

    /// Update an existing finding
    pub fn update_finding(&mut self, id: &str, mut updated_finding: VulnerabilityFinding) -> Result<()> {
        let index = self
            .findings
            .iter()
            .position(|f| f.id == id)
            .context("Finding not found")?;

        updated_finding.id = id.to_string();
        self.findings[index] = updated_finding;

        println!("✅ Updated finding: {}", id);

        Ok(())
    }

    /// Update finding status
    pub fn update_status(&mut self, id: &str, new_status: FindingStatus) -> Result<()> {
        let finding = self
            .findings
            .iter_mut()
            .find(|f| f.id == id)
            .context("Finding not found")?;

        // Update timestamps based on status before moving new_status
        match new_status {
            FindingStatus::Submitted => {
                if finding.date_submitted.is_none() {
                    finding.date_submitted = Some(Utc::now());
                }
            }
            FindingStatus::Resolved => {
                finding.date_resolved = Some(Utc::now());
            }
            _ => {}
        }

        finding.status = new_status;

        println!("✅ Updated status for {}: {:?}", id, finding.status.as_str());

        Ok(())
    }

    /// Add a new bounty program
    pub fn add_program(&mut self, program: BountyProgram) {
        println!("📋 Adding program: {}", program.name);
        self.programs.push(program);
    }

    /// Get all findings
    pub fn get_findings(&self) -> &[VulnerabilityFinding] {
        &self.findings
    }

    /// Get findings by status
    pub fn get_findings_by_status(&self, status: FindingStatus) -> Vec<&VulnerabilityFinding> {
        self.findings
            .iter()
            .filter(|f| f.status == status)
            .collect()
    }

    /// Get findings by severity
    pub fn get_findings_by_severity(&self, severity: Severity) -> Vec<&VulnerabilityFinding> {
        self.findings
            .iter()
            .filter(|f| f.severity == severity)
            .collect()
    }

    /// Get findings by program
    pub fn get_findings_by_program(&self, program: &str) -> Vec<&VulnerabilityFinding> {
        self.findings
            .iter()
            .filter(|f| f.program == program)
            .collect()
    }

    /// Calculate total potential earnings
    pub fn calculate_potential_earnings(&self) -> HashMap<String, (u32, u32)> {
        let mut program_totals: HashMap<String, (u32, u32)> = HashMap::new();

        for finding in &self.findings {
            if matches!(
                finding.status,
                FindingStatus::Accepted | FindingStatus::Submitted | FindingStatus::Triaged
            ) {
                let (min, max) = finding.severity.typical_bounty_range();

                program_totals
                    .entry(finding.program.clone())
                    .and_modify(|(current_min, current_max)| {
                        *current_min += min;
                        *current_max += max;
                    })
                    .or_insert((min, max));
            }
        }

        program_totals
    }

    /// Calculate total confirmed earnings
    pub fn calculate_confirmed_earnings(&self) -> u32 {
        self.findings
            .iter()
            .filter_map(|f| f.bounty_amount)
            .sum()
    }

    /// Generate summary statistics
    pub fn generate_statistics(&self) -> String {
        let mut report = String::new();

        report.push_str("╔═══════════════════════════════════════════════════════════════════════════╗\n");
        report.push_str("║                       FENRIR BUG BOUNTY STATISTICS                         ║\n");
        report.push_str("╚═══════════════════════════════════════════════════════════════════════════╝\n\n");

        // Total findings
        report.push_str(&format!("Total Findings: {}\n\n", self.findings.len()));

        // By status
        report.push_str("By Status:\n");
        for status in &[
            FindingStatus::Investigating,
            FindingStatus::ReadyToSubmit,
            FindingStatus::Submitted,
            FindingStatus::Triaged,
            FindingStatus::Accepted,
            FindingStatus::Resolved,
            FindingStatus::Rejected,
            FindingStatus::NeedsMoreInfo,
            FindingStatus::Duplicate,
        ] {
            let count = self
                .findings
                .iter()
                .filter(|f| f.status == *status)
                .count();
            if count > 0 {
                report.push_str(&format!("  {}: {}\n", status.as_str(), count));
            }
        }

        // By severity
        report.push_str("\nBy Severity:\n");
        for severity in &[Severity::Critical, Severity::High, Severity::Medium, Severity::Low, Severity::Info] {
            let count = self
                .findings
                .iter()
                .filter(|f| f.severity == *severity)
                .count();
            if count > 0 {
                report.push_str(&format!("  {}: {}\n", severity.as_str(), count));
            }
        }

        // By program
        report.push_str("\nBy Program:\n");
        let mut program_counts: HashMap<String, usize> = HashMap::new();
        for finding in &self.findings {
            *program_counts.entry(finding.program.clone()).or_insert(0) += 1;
        }

        for (program, count) in program_counts.iter() {
            report.push_str(&format!("  {}: {}\n", program, count));
        }

        // Earnings
        let confirmed = self.calculate_confirmed_earnings();
        let potential = self.calculate_potential_earnings();

        report.push_str(&format!("\nConfirmed Earnings: ${}\n", confirmed));

        report.push_str("\nPotential Earnings by Program:\n");
        for (program, (min, max)) in &potential {
            report.push_str(&format!("  {}: ${} - ${}\n", program, min, max));
        }

        // Calculate totals
        let total_min: u32 = potential.values().map(|(min, _)| min).sum();
        let total_max: u32 = potential.values().map(|(_, max)| max).sum();

        report.push_str(&format!("\nTotal Potential: ${} - ${}\n", total_min, total_max));

        report.push_str("\n═══════════════════════════════════════════════════════════════════════════\n");

        report
    }

    /// Generate vulnerability report for submission
    pub fn generate_vulnerability_report(&self, id: &str) -> Result<String> {
        let finding = self
            .findings
            .iter()
            .find(|f| f.id == id)
            .context("Finding not found")?;

        let mut report = String::new();

        report.push_str("╔═══════════════════════════════════════════════════════════════════════════╗\n");
        report.push_str(&format!("║  VULNERABILITY REPORT: {:54} ║\n",
            finding.title.chars().take(54).collect::<String>()));
        report.push_str("╚═══════════════════════════════════════════════════════════════════════════╝\n\n");

        // Basic information
        report.push_str("📋 BASIC INFORMATION\n");
        report.push_str("─────────────────────────────────────────────────────────────────────────────\n");
        report.push_str(&format!("Finding ID: {}\n", finding.id));
        report.push_str(&format!("Vulnerability Type: {}\n", finding.vuln_type));
        report.push_str(&format!("Severity: {}\n", finding.severity.as_str()));
        report.push_str(&format!("Program: {}\n", finding.program));
        report.push_str(&format!("Affected URL: {}\n", finding.affected_url));
        report.push_str(&format!("Date Discovered: {}\n", finding.date_discovered.format("%Y-%m-%d")));

        if let Some(cvss) = finding.cvss_score {
            report.push_str(&format!("CVSS Score: {:.1}\n", cvss));
        }

        if let Some(cwe) = &finding.cwe_id {
            report.push_str(&format!("CWE: {}\n", cwe));
        }

        if let Some(owasp) = &finding.owasp_category {
            report.push_str(&format!("OWASP: {}\n", owasp));
        }

        report.push_str("\n");

        // Description
        report.push_str("📝 DESCRIPTION\n");
        report.push_str("─────────────────────────────────────────────────────────────────────────────\n");
        report.push_str(&finding.description);
        report.push_str("\n\n");

        // Impact
        report.push_str("💥 IMPACT\n");
        report.push_str("─────────────────────────────────────────────────────────────────────────────\n");
        report.push_str(&finding.impact);
        report.push_str("\n\n");

        // Steps to reproduce
        report.push_str("🔧 STEPS TO REPRODUCE\n");
        report.push_str("─────────────────────────────────────────────────────────────────────────────\n");
        for (i, step) in finding.steps_to_reproduce.iter().enumerate() {
            report.push_str(&format!("{}. {}\n", i + 1, step));
        }
        report.push_str("\n");

        // Proof of concept
        report.push_str("🎯 PROOF OF CONCEPT\n");
        report.push_str("─────────────────────────────────────────────────────────────────────────────\n");
        report.push_str(&finding.proof_of_concept);
        report.push_str("\n\n");

        // Remediation
        report.push_str("🛡️  REMEDIATION\n");
        report.push_str("─────────────────────────────────────────────────────────────────────────────\n");
        report.push_str(&finding.remediation);
        report.push_str("\n\n");

        // Timeline
        report.push_str("📅 TIMELINE\n");
        report.push_str("─────────────────────────────────────────────────────────────────────────────\n");
        report.push_str(&format!("Discovered: {}\n", finding.date_discovered.format("%Y-%m-%d %H:%M UTC")));

        if let Some(submitted) = finding.date_submitted {
            report.push_str(&format!("Submitted: {}\n", submitted.format("%Y-%m-%d %H:%M UTC")));
        }

        if let Some(resolved) = finding.date_resolved {
            report.push_str(&format!("Resolved: {}\n", resolved.format("%Y-%m-%d %H:%M UTC")));
        }

        if let Some(bounty) = finding.bounty_amount {
            report.push_str(&format!("Bounty Awarded: ${}\n", bounty));
        }

        report.push_str("\n");

        // Notes
        if !finding.notes.is_empty() {
            report.push_str("📌 NOTES\n");
            report.push_str("─────────────────────────────────────────────────────────────────────────────\n");
            report.push_str(&finding.notes);
            report.push_str("\n\n");
        }

        report.push_str("═══════════════════════════════════════════════════════════════════════════\n");
        report.push_str("This report was generated by Fenrir Bug Bounty Tracker\n");
        report.push_str("═══════════════════════════════════════════════════════════════════════════\n");

        Ok(report)
    }

    /// Generate program overview
    pub fn generate_program_overview(&self, program_name: &str) -> Result<String> {
        let program = self
            .programs
            .iter()
            .find(|p| p.name == program_name)
            .context("Program not found")?;

        let findings = self.get_findings_by_program(program_name);

        let mut report = String::new();

        report.push_str("╔═══════════════════════════════════════════════════════════════════════════╗\n");
        report.push_str(&format!("║  PROGRAM OVERVIEW: {:53} ║\n",
            program.name.chars().take(53).collect::<String>()));
        report.push_str("╚═══════════════════════════════════════════════════════════════════════════╝\n\n");

        report.push_str(&format!("Platform: {}\n", program.platform));
        report.push_str(&format!("URL: {}\n", program.url));
        report.push_str(&format!("Policy: {}\n", program.policy_url));
        report.push_str(&format!("Bounty Range: ${} - ${}\n", program.min_bounty, program.max_bounty));
        report.push_str(&format!("Avg Payout: {} days\n", program.avg_payout_days));
        report.push_str(&format!("Total Findings: {}\n\n", findings.len()));

        report.push_str("IN SCOPE:\n");
        for target in &program.scope {
            report.push_str(&format!("  ✓ {}\n", target));
        }

        if !program.out_of_scope.is_empty() {
            report.push_str("\nOUT OF SCOPE:\n");
            for target in &program.out_of_scope {
                report.push_str(&format!("  ✗ {}\n", target));
            }
        }

        if !program.notes.is_empty() {
            report.push_str(&format!("\nNOTES:\n{}\n", program.notes));
        }

        report.push_str("\n═══════════════════════════════════════════════════════════════════════════\n");

        Ok(report)
    }

    /// Export to Markdown
    pub fn export_markdown(&self, output_path: PathBuf) -> Result<()> {
        let mut markdown = String::new();

        markdown.push_str("# Bug Bounty Report\n\n");
        markdown.push_str(&format!("Generated: {}\n\n", Utc::now().format("%Y-%m-%d %H:%M UTC")));

        markdown.push_str("## Statistics\n\n");
        markdown.push_str(&self.generate_statistics());
        markdown.push_str("\n");

        markdown.push_str("## Findings\n\n");

        for finding in &self.findings {
            markdown.push_str(&format!("### {} - {}\n", finding.id, finding.title));
            markdown.push_str(&format!("- **Severity**: {}\n", finding.severity.as_str()));
            markdown.push_str(&format!("- **Status**: {}\n", finding.status.as_str()));
            markdown.push_str(&format!("- **Program**: {}\n", finding.program));
            markdown.push_str(&format!("- **URL**: {}\n", finding.affected_url));
            markdown.push_str(&format!("- **Date**: {}\n", finding.date_discovered.format("%Y-%m-%d")));

            if let Some(bounty) = finding.bounty_amount {
                markdown.push_str(&format!("- **Bounty**: ${}\n", bounty));
            }

            markdown.push_str("\n");
        }

        fs::write(&output_path, markdown)?;

        println!("✅ Exported to Markdown: {:?}", output_path);

        Ok(())
    }
}

// ============================================================================
// SECTION 2: HELPER FUNCTIONS
// ============================================================================

/// Generate a unique finding ID
pub fn generate_finding_id() -> String {
    use std::time::{SystemTime, UNIX_EPOCH};

    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();

    format!("FEN-{}", timestamp)
}

/// Create a new vulnerability finding with minimal required fields
pub fn create_finding(
    title: String,
    vuln_type: String,
    severity: Severity,
    program: String,
    affected_url: String,
    description: String,
) -> VulnerabilityFinding {
    VulnerabilityFinding {
        id: generate_finding_id(),
        title,
        vuln_type,
        severity,
        status: FindingStatus::Investigating,
        program,
        affected_url,
        description,
        impact: String::new(),
        steps_to_reproduce: Vec::new(),
        proof_of_concept: String::new(),
        remediation: String::new(),
        date_discovered: Utc::now(),
        date_submitted: None,
        date_resolved: None,
        bounty_amount: None,
        report_id: None,
        notes: String::new(),
        owasp_category: None,
        cwe_id: None,
        cvss_score: None,
    }
}

/// Create a new bounty program
pub fn create_program(
    name: String,
    url: String,
    platform: String,
    policy_url: String,
) -> BountyProgram {
    BountyProgram {
        name,
        url,
        platform,
        policy_url,
        scope: Vec::new(),
        out_of_scope: Vec::new(),
        min_bounty: 0,
        max_bounty: 0,
        avg_payout_days: 0,
        notes: String::new(),
    }
}

// ============================================================================
// SECTION 3: POPULAR BUG BOUNTY PROGRAMS
// ============================================================================

/// Get popular bug bounty programs with their details
pub fn get_popular_programs() -> Vec<BountyProgram> {
    vec![
        BountyProgram {
            name: "Google Vulnerability Reward Program".to_string(),
            url: "https://www.google.com/about/appsecurity/reward-program/".to_string(),
            platform: "Independent".to_string(),
            policy_url: "https://www.google.com/about/appsecurity/reward-program/rules/".to_string(),
            scope: vec![
                "*.google.com".to_string(),
                "*.youtube.com".to_string(),
                "*.blogger.com".to_string(),
                "google.com".to_string(),
            ],
            out_of_scope: vec![
                "*.gmail.com".to_string(),
                "*.google-analytics.com".to_string(),
            ],
            min_bounty: 100,
            max_bounty: 31337,
            avg_payout_days: 30,
            notes: "One of the oldest and most prestigious programs".to_string(),
        },
        BountyProgram {
            name: "Meta Bug Bounty".to_string(),
            url: "https://www.facebook.com/whitehat".to_string(),
            platform: "Independent".to_string(),
            policy_url: "https://www.facebook.com/whitehat/downloads/".to_string(),
            scope: vec![
                "*.facebook.com".to_string(),
                "*.instagram.com".to_string(),
                "*.whatsapp.com".to_string(),
                "*.messenger.com".to_string(),
            ],
            out_of_scope: vec![
                "*.fbcdn.net".to_string(),
                "*.cdninstagram.com".to_string(),
            ],
            min_bounty: 500,
            max_bounty: 40000,
            avg_payout_days: 45,
            notes: "Pays well for authentication and account takeover bugs".to_string(),
        },
        BountyProgram {
            name: "Microsoft Bug Bounty".to_string(),
            url: "https://www.microsoft.com/en-us/msrc/bounty".to_string(),
            platform: "Independent".to_string(),
            policy_url: "https://www.microsoft.com/en-us/msrc/bounty-microsoft-services".to_string(),
            scope: vec![
                "*.microsoft.com".to_string(),
                "*.office.com".to_string(),
                "*.live.com".to_string(),
                "*.xbox.com".to_string(),
            ],
            out_of_scope: vec![
                "*.windows.net".to_string(),
                "*.azure.net".to_string(),
            ],
            min_bounty: 500,
            max_bounty: 100000,
            avg_payout_days: 60,
            notes: "Large bounty pool, focuses on identity and cloud services".to_string(),
        },
    ]
}

// ============================================================================
// EXPORTS
// ============================================================================

/// Bug Bounty Tracker - Main entry point
pub struct TrackerTools;

impl TrackerTools {
    /// Create a new tracker
    pub fn create_tracker(storage_dir: PathBuf) -> BugBountyTracker {
        BugBountyTracker::new(storage_dir)
    }

    /// Create a quick finding for OAuth vulnerabilities
    pub fn create_oauth_finding(
        program: String,
        url: String,
        vuln_type: String,
        description: String,
    ) -> VulnerabilityFinding {
        VulnerabilityFinding {
            id: generate_finding_id(),
            title: format!("OAuth Vulnerability: {}", vuln_type),
            vuln_type: "OAuth 2.0".to_string(),
            severity: Severity::High, // OAuth bugs are usually high severity
            status: FindingStatus::Investigating,
            program,
            affected_url: url,
            description,
            impact: "OAuth vulnerabilities can lead to account takeover, data exposure, \
                     and unauthorized access to user information.".to_string(),
            steps_to_reproduce: vec![
                "1. Identify the OAuth flow type (Authorization Code, Implicit, etc.)".to_string(),
                "2. Intercept the authorization request with Burp Suite".to_string(),
                "3. Analyze parameters: client_id, redirect_uri, scope, state".to_string(),
                "4. Test redirect_uri manipulation for open redirects".to_string(),
                "5. Test state parameter for CSRF protection".to_string(),
                "6. Examine callback for authorization code leakage".to_string(),
            ],
            proof_of_concept: "See detailed reproduction steps and screenshots".to_string(),
            remediation: "1. Implement strict redirect_uri validation (whitelist only)\n\
                          2. Always use cryptographically random state parameter\n\
                          3. Enforce HTTPS on all OAuth endpoints\n\
                          4. Use Authorization Code flow with PKCE\n\
                          5. Validate all OAuth parameters properly".to_string(),
            date_discovered: Utc::now(),
            date_submitted: None,
            date_resolved: None,
            bounty_amount: None,
            report_id: None,
            notes: String::new(),
            owasp_category: Some("A1:2021 – Broken Access Control".to_string()),
            cwe_id: Some("CWE-303".to_string()),
            cvss_score: Some(7.5),
        }
    }
}
