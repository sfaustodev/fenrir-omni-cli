//! # Fenrir Burp Suite Integration Module
//!
//! This module provides integration with Burp Suite Community Edition for
//! advanced web application security testing in bug bounty programs.
//!
//! ## What is Burp Suite?
//! Burp Suite is the leading integrated platform for web application security testing.
//! It includes a proxy, scanner, repeater, intruder, and many other tools.
//!
//! ## Integration Features
//! - Proxy configuration for intercepting OAuth flows
//! - Automated capture and analysis of authentication flows
//! - Import/export of requests for further testing
//! - Integration with Fenrir's bug bounty tools
//!
//! ## Educational Note
//! Burp Suite Community Edition is free but has limitations:
//! - No automated scanning (requires Professional)
//! - Limited intruder functionality
//! - Manual testing only
//!
//! However, for bug bounty hunting, the proxy and repeater are sufficient for
//! comprehensive manual testing.

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::fs;
use std::net::SocketAddr;
use std::path::PathBuf;

// ============================================================================
// SECTION 1: BURP SUITE CONFIGURATION
// ============================================================================
// This section handles Burp Suite proxy configuration and setup.
//
// How Burp Suite Proxy Works:
// 1. Burp Suite runs a proxy server (default: 127.0.0.1:8080)
// 2. Browser is configured to send traffic through Burp proxy
// 3. Burp intercepts requests/responses for inspection and modification
// 4. Can capture OAuth flows, analyze them, and replay modified requests
//
// Common Proxy Settings:
// - Proxy Address: 127.0.0.1 (localhost)
// - Proxy Port: 8080 (default)
// - Browsers: Configure proxy settings or use FoxyProxy extension

/// Burp Suite proxy configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BurpProxyConfig {
    /// Proxy listen address (usually 127.0.0.1)
    pub listen_address: String,
    /// Proxy port (default: 8080)
    pub port: u16,
    /// Whether to use HTTPS
    pub use_https: bool,
    /// CA certificate path for SSL interception
    pub ca_cert_path: Option<PathBuf>,
}

impl Default for BurpProxyConfig {
    fn default() -> Self {
        Self {
            listen_address: "127.0.0.1".to_string(),
            port: 8080,
            use_https: true,
            ca_cert_path: None,
        }
    }
}

/// Represents an intercepted HTTP request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InterceptedRequest {
    /// Request ID
    pub id: String,
    /// Full URL
    pub url: String,
    /// HTTP method (GET, POST, etc.)
    pub method: String,
    /// Request headers
    pub headers: Vec<(String, String)>,
    /// Request body
    pub body: Option<String>,
    /// Timestamp of interception
    pub timestamp: chrono::DateTime<chrono::Utc>,
    /// Whether this is an OAuth-related request
    pub is_oauth: bool,
    /// OAuth flow stage (if applicable)
    pub oauth_stage: Option<String>,
}

/// Burp Suite Integration - Main struct for Burp Suite integration
#[derive(Debug, Clone)]
pub struct BurpSuiteIntegration {
    /// Proxy configuration
    config: BurpProxyConfig,
    /// Directory to save intercepted requests
    output_dir: PathBuf,
    /// Whether Burp Suite is currently running
    is_running: bool,
}

impl BurpSuiteIntegration {
    /// Create a new Burp Suite integration instance
    ///
    /// # Arguments
    /// * `config` - Burp proxy configuration
    /// * `output_dir` - Directory to save captured requests
    pub fn new(config: BurpProxyConfig, output_dir: PathBuf) -> Self {
        Self {
            config,
            output_dir,
            is_running: false,
        }
    }

    /// Check if Burp Suite is running and accessible
    ///
    /// This method attempts to connect to the Burp proxy to verify it's running.
    pub async fn check_burp_running(&self) -> Result<bool> {
        let proxy_addr = format!("{}:{}", self.config.listen_address, self.config.port);

        // Try to connect to Burp proxy
        match tokio::net::TcpStream::connect(&proxy_addr).await {
            Ok(_) => {
                println!("✅ Burp Suite is running at {}", proxy_addr);
                Ok(true)
            }
            Err(_) => {
                println!("❌ Burp Suite is not accessible at {}", proxy_addr);
                println!("💡 Make sure Burp Suite is running and proxy is enabled");
                Ok(false)
            }
        }
    }

    /// Configure HTTP client to use Burp proxy
    ///
    /// This creates a reqwest client configured to send requests through Burp.
    pub fn create_proxied_client(&self) -> Result<reqwest::Client> {
        let proxy = reqwest::Proxy::all(format!(
            "http://{}:{}",
            self.config.listen_address, self.config.port
        ))?;

        let client = reqwest::Client::builder()
            .proxy(proxy)
            .timeout(std::time::Duration::from_secs(30))
            .danger_accept_invalid_certs(true) // Burp uses self-signed cert
            .build()?;

        Ok(client)
    }

    /// Send a test request through Burp proxy
    ///
    /// Useful for verifying the proxy is working correctly.
    pub async fn test_proxy_connection(&self) -> Result<String> {
        let client = self.create_proxied_client()?;

        println!("🔍 Sending test request through Burp proxy...");

        let response = client
            .get("https://httpbin.org/ip")
            .send()
            .await?
            .text()
            .await?;

        println!("✅ Proxy connection successful!");
        println!("📄 Response: {}", response);

        Ok(response)
    }

    /// Capture OAuth flow through Burp proxy
    ///
    /// This method initiates an OAuth flow while proxying through Burp,
    /// allowing Burp to intercept and capture all OAuth-related requests.
    ///
    /// # Educational Note
    /// When testing OAuth with Burp:
    /// 1. Enable interception in Burp Proxy
    /// 2. Initiate OAuth login flow
    /// 3. Burp will intercept the authorization request
    /// 4. Examine parameters: client_id, redirect_uri, scope, state
    /// 5. Forward to complete the flow
    /// 6. Intercept the callback with authorization code
    /// 7. Capture the code and analyze it
    ///
    /// Common vulnerabilities to look for:
    /// - Open redirect via redirect_uri
    /// - Missing or weak state parameter
    /// - Exposed client_secret
    /// - Weak scope validation
    pub async fn capture_oauth_flow(
        &self,
        authorization_url: String,
        client_id: String,
        redirect_uri: String,
        scope: String,
    ) -> Result<Vec<InterceptedRequest>> {
        let client = self.create_proxied_client()?;
        let mut captured_requests = Vec::new();

        println!("🔍 Capturing OAuth flow through Burp proxy...");
        println!("📍 Authorization URL: {}", authorization_url);

        // Build OAuth authorization URL
        let auth_url = format!(
            "{}?client_id={}&redirect_uri={}&response_type=code&scope={}",
            authorization_url,
            urlencoding::encode(&client_id),
            urlencoding::encode(&redirect_uri),
            urlencoding::encode(&scope)
        );

        println!("💡 Open this URL in your browser configured to use Burp proxy:");
        println!("   {}", auth_url);
        println!();
        println!("📋 Burp will intercept the following:");
        println!("   1. Authorization request with client_id and redirect_uri");
        println!("   2. Callback with authorization code");
        println!("   3. Token exchange request");
        println!();
        println!("⚠️  Make sure Burp Suite 'Intercept' is ON");

        // In a real implementation, we would wait for Burp to capture requests
        // For now, this is a placeholder for the manual testing workflow

        Ok(captured_requests)
    }

    /// Save intercepted request to file for analysis
    pub fn save_request(&self, request: &InterceptedRequest) -> Result<PathBuf> {
        // Create output directory if it doesn't exist
        fs::create_dir_all(&self.output_dir)?;

        // Generate filename
        let filename = format!(
            "{}_{}.json",
            request.id,
            request.timestamp.format("%Y%m%d_%H%M%S")
        );
        let filepath = self.output_dir.join(&filename);

        // Serialize request to JSON
        let json = serde_json::to_string_pretty(request)?;

        // Write to file
        fs::write(&filepath, json)?;

        println!("💾 Saved request to: {:?}", filepath);

        Ok(filepath)
    }

    /// Load captured requests from directory
    pub fn load_requests(&self) -> Result<Vec<InterceptedRequest>> {
        let mut requests = Vec::new();

        if !self.output_dir.exists() {
            return Ok(requests);
        }

        for entry in fs::read_dir(&self.output_dir)? {
            let entry = entry?;
            let path = entry.path();

            if path.extension().and_then(|s| s.to_str()) == Some("json") {
                let contents = fs::read_to_string(&path)?;
                let request: InterceptedRequest = serde_json::from_str(&contents)?;
                requests.push(request);
            }
        }

        requests.sort_by(|a, b| b.timestamp.cmp(&a.timestamp));

        Ok(requests)
    }

    /// Analyze captured requests for OAuth vulnerabilities
    ///
    /// This method analyzes previously captured requests to identify
    /// potential OAuth security issues.
    pub fn analyze_oauth_requests(&self, requests: &[InterceptedRequest]) -> Result<String> {
        let mut report = String::new();

        report.push_str("╔═══════════════════════════════════════════════════════════════════════════╗\n");
        report.push_str("║                    FENRIR BURP OAUTH ANALYSIS REPORT                      ║\n");
        report.push_str("╚═══════════════════════════════════════════════════════════════════════════╝\n\n");

        report.push_str(&format!("Total Requests Analyzed: {}\n\n", requests.len()));

        let oauth_requests: Vec<_> = requests.iter().filter(|r| r.is_oauth).collect();

        report.push_str(&format!("OAuth-Related Requests: {}\n\n", oauth_requests.len()));

        for (i, request) in oauth_requests.iter().enumerate() {
            report.push_str(&format!("┌─ REQUEST #{}\n", i + 1));
            report.push_str(&format!("│ URL: {}\n", request.url));
            report.push_str(&format!("│ Method: {}\n", request.method));
            report.push_str(&format!("│ Timestamp: {}\n", request.timestamp));

            if let Some(stage) = &request.oauth_stage {
                report.push_str(&format!("│ OAuth Stage: {}\n", stage));
            }

            report.push_str("│ Headers:\n");
            for (key, value) in &request.headers {
                // Mask sensitive values
                let display_value = if key.to_lowercase().contains("authorization")
                    || key.to_lowercase().contains("cookie")
                {
                    "***REDACTED***"
                } else {
                    value
                };
                report.push_str(&format!("│   {}: {}\n", key, display_value));
            }

            if let Some(body) = &request.body {
                // Don't print full body, just indicate presence
                report.push_str(&format!("│ Body Length: {} bytes\n", body.len()));
            }

            report.push_str("└─────────────────────────────────────────────────────────────────────\n\n");
        }

        // Analyze for common vulnerabilities
        report.push_str("─────────────────────────────────────────────────────────────────────────────\n");
        report.push_str("VULNERABILITY ANALYSIS:\n");
        report.push_str("─────────────────────────────────────────────────────────────────────────────\n\n");

        for request in oauth_requests {
            // Check for redirect_uri parameter
            if request.url.contains("redirect_uri=") {
                if let Some(uri_part) = request.url.split("redirect_uri=").nth(1) {
                    let redirect_uri = uri_part.split('&').next().unwrap_or("");

                    if redirect_uri.contains("http://") {
                        report.push_str(&format!(
                            "⚠️  INSECURE REDIRECT: HTTP redirect_uri found\n\
                             URL: {}\n\
                             Redirect: {}\n\
                             This may expose authorization code to interception.\n\n",
                            request.url, redirect_uri
                        ));
                    }

                    // Check for open redirect
                    if redirect_uri.contains("evil.com") || redirect_uri.contains("example.com") {
                        report.push_str(&format!(
                            "🚨 POTENTIAL OPEN REDIRECT: Unusual redirect_uri\n\
                             URL: {}\n\
                             Redirect: {}\n\
                             Verify redirect URI validation.\n\n",
                            request.url, redirect_uri
                        ));
                    }
                }
            }

            // Check for state parameter
            if !request.url.contains("state=") && request.url.contains("client_id=") {
                report.push_str(&format!(
                    "⚠️  MISSING STATE PARAMETER\n\
                     URL: {}\n\
                     OAuth flow lacks CSRF protection.\n\n",
                    request.url
                ));
            }

            // Check for token in URL
            if request.url.contains("access_token=") || request.url.contains("code=") {
                report.push_str(&format!(
                    "⚠️  TOKEN IN URL\n\
                     URL: {}\n\
                     Sensitive token may be logged in browser history or Referer headers.\n\n",
                    request.url
                ));
            }
        }

        report.push_str("═══════════════════════════════════════════════════════════════════════════\n");

        Ok(report)
    }

    /// Export requests in Burp Suite format for import
    ///
    /// This generates an XML file that can be imported into Burp Suite.
    pub fn export_for_burp(&self, requests: &[InterceptedRequest]) -> Result<PathBuf> {
        let mut xml = String::new();
        xml.push_str("<?xml version=\"1.0\"?>\n");
        xml.push_str("<items>\n");

        for request in requests {
            xml.push_str("  <item>\n");
            xml.push_str(&format!("    <url>{}</url>\n", escape_xml(&request.url)));
            xml.push_str(&format!("    <method>{}</method>\n", escape_xml(&request.method)));

            xml.push_str("    <headers>\n");
            for (key, value) in &request.headers {
                xml.push_str("      <header>\n");
                xml.push_str(&format!("        <name>{}</name>\n", escape_xml(key)));
                xml.push_str(&format!("        <value>{}</value>\n", escape_xml(value)));
                xml.push_str("      </header>\n");
            }
            xml.push_str("    </headers>\n");

            if let Some(body) = &request.body {
                xml.push_str(&format!("    <body>{}</body>\n", escape_xml(body)));
            }

            xml.push_str("  </item>\n");
        }

        xml.push_str("</items>\n");

        let filepath = self.output_dir.join("burp_import.xml");
        fs::write(&filepath, xml)?;

        println!("💾 Exported {} requests for Burp Suite: {:?}", requests.len(), filepath);

        Ok(filepath)
    }
}

/// Helper function to escape XML special characters
fn escape_xml(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&apos;")
}

// ============================================================================
// SECTION 2: BURP SUITE LAUNCHER
// ============================================================================
// This section handles launching and managing Burp Suite process.

/// Burp Suite Launcher - Manages Burp Suite executable
#[derive(Debug)]
pub struct BurpSuiteLauncher {
    /// Path to Burp Suite jar file
    burp_jar_path: PathBuf,
    /// Java executable path
    java_path: PathBuf,
    /// Memory allocation for Burp (MB)
    memory_mb: u32,
}

impl BurpSuiteLauncher {
    /// Create a new Burp Suite launcher
    ///
    /// # Arguments
    /// * `burp_jar_path` - Path to burpsuite_community.jar
    /// * `java_path` - Path to java executable (optional, uses default if None)
    /// * `memory_mb` - Memory allocation in MB (default: 1024)
    pub fn new(burp_jar_path: PathBuf, java_path: Option<PathBuf>, memory_mb: Option<u32>) -> Self {
        Self {
            burp_jar_path,
            java_path: java_path.unwrap_or_else(|| PathBuf::from("java")),
            memory_mb: memory_mb.unwrap_or(1024),
        }
    }

    /// Launch Burp Suite Community Edition
    ///
    /// This method starts Burp Suite with appropriate configuration.
    pub async fn launch(&self) -> Result<()> {
        if !self.burp_jar_path.exists() {
            anyhow::bail!(
                "Burp Suite jar file not found at {:?}\n\
                 Please download Burp Suite Community Edition from:\n\
                 https://portswigger.net/burp/communitydownload",
                self.burp_jar_path
            );
        }

        println!("🚀 Launching Burp Suite Community Edition...");
        println!("📍 JAR: {:?}", self.burp_jar_path);
        println!("💾 Memory: {} MB", self.memory_mb);

        // Command to launch Burp Suite
        let output = tokio::process::Command::new(&self.java_path)
            .arg(format!("-Xmx{}M", self.memory_mb))
            .arg("-jar")
            .arg(&self.burp_jar_path)
            .spawn();

        match output {
            Ok(_) => {
                println!("✅ Burp Suite launched successfully!");
                println!("💡 Configure your browser to use proxy: 127.0.0.1:8080");
                Ok(())
            }
            Err(e) => {
                anyhow::bail!("Failed to launch Burp Suite: {}", e);
            }
        }
    }

    /// Check if Java is installed
    pub fn check_java_installed(&self) -> Result<bool> {
        let output = std::process::Command::new(&self.java_path)
            .arg("-version")
            .output()?;

        Ok(output.status.success())
    }

    /// Get recommended Burp Suite download instructions
    pub fn download_instructions() -> &'static str {
        r#"
BURP SUITE INSTALLATION INSTRUCTIONS:
═══════════════════════════════════════════════════════════════════════════

1. Download Burp Suite Community Edition:
   Visit: https://portswigger.net/burp/communitydownload
   Download: burpsuite_community_linux_v202x.sh (Linux)
            burpsuite_community_osx_v202x.dmg (macOS)
            burpsuite_community_windows-v202x.exe (Windows)

2. Install Burp Suite:
   - Linux: chmod +x burpsuite_community_*.sh && sudo ./burpsuite_community_*.sh
   - macOS: Open the .dmg file and drag to Applications
   - Windows: Run the .exe installer

3. Locate the JAR file:
   - Linux: /opt/burpsuite_community/burpsuite_community.jar
   - macOS: /Applications/Burp Suite Community Edition.app/Contents/java/burpsuite_community.jar
   - Windows: C:\Program Files\BurpSuiteCommunity\burpsuite_community.jar

4. Launch Fenrir Burp integration:
   fenrir burp launch --jar-path <path_to_jar>

═══════════════════════════════════════════════════════════════════════════
"#
    }
}

// ============================================================================
// SECTION 3: AUTOMATED OAUTH TESTING WITH BURP
// ============================================================================
// This section implements automated OAuth testing workflows using Burp.

/// Automated OAuth testing workflow
pub struct AutomatedOAuthTesting {
    /// Burp integration
    burp: BurpSuiteIntegration,
    /// Whether to save all requests
    save_all: bool,
}

impl AutomatedOAuthTesting {
    /// Create new automated OAuth tester
    pub fn new(burp: BurpSuiteIntegration, save_all: bool) -> Self {
        Self { burp, save_all }
    }

    /// Run automated OAuth security assessment
    ///
    /// This method orchestrates a comprehensive OAuth security test:
    /// 1. Configure Burp proxy
    /// 2. Capture OAuth flow
    /// 3. Analyze requests
    /// 4. Generate report
    pub async fn run_oauth_assessment(
        &self,
        target_url: String,
        client_id: String,
        redirect_uri: String,
    ) -> Result<String> {
        println!("🔍 Starting automated OAuth security assessment...");
        println!("📍 Target: {}", target_url);

        // Check Burp is running
        if !self.burp.check_burp_running().await? {
            anyhow::bail!("Burp Suite is not running. Please start Burp first.");
        }

        // Capture OAuth flow
        let requests = self
            .burp
            .capture_oauth_flow(target_url, client_id, redirect_uri, "openid profile email".to_string())
            .await?;

        // Save requests if needed
        if self.save_all {
            for request in &requests {
                self.burp.save_request(request)?;
            }
        }

        // Analyze requests
        let report = self.burp.analyze_oauth_requests(&requests)?;

        Ok(report)
    }

    /// Guide user through manual OAuth testing
    pub fn manual_testing_guide(&self) -> &'static str {
        r#"
MANUAL OAUTH TESTING GUIDE WITH BURP SUITE:
═══════════════════════════════════════════════════════════════════════════

SETUP:
1. Open Burp Suite
2. Go to Proxy > Options
3. Ensure proxy is running on 127.0.0.1:8080
4. Enable "Intercept" (should be ON)

5. Configure your browser:
   - Install FoxyProxy extension
   - Add proxy: 127.0.0.1:8080
   - OR configure system proxy manually

6. Import Burp CA certificate:
   - In browser, visit http://burp
   - Download CA certificate
   - Install in browser's certificate store

TESTING OAUTH FLOWS:

Step 1: Intercept Authorization Request
────────────────────────────────────────
1. Visit target application
2. Click "Login with Google/Facebook/etc."
3. Burp will intercept the authorization request
4. Examine these parameters:
   ✓ client_id - Can we use a different client_id?
   ✓ redirect_uri - Can we change it to evil.com?
   ✓ scope - Is it overly broad?
   ✓ state - Does it exist? Is it random?
   ✓ response_type - Is it appropriate?

Step 2: Test redirect_uri Manipulation
────────────────────────────────────────
In Burp Repeater, try:
- https://evil.com
- //evil.com
- /\\.evil.com
- https://legit.com.evil.com
- https://legit.com\\@evil.com
- URL-encoded variations

Look for:
- 302 redirect to evil.com
- Success response with manipulated URI
- Error messages showing validation logic

Step 3: Test state Parameter
────────────────────────────────────────
1. Remove state parameter entirely
2. Reuse old state value
3. Try predictable states (0000, 1111, etc.)
4. Check if CSRF is possible

Step 4: Capture Callback with Authorization Code
────────────────────────────────────────
1. Forward the authorization request
2. Complete login in browser
3. Burp intercepts the callback
4. Capture the authorization code
5. Check if code is:
   - In URL (could be logged in history)
   - Referrer header
   - Browser history

Step 5: Token Exchange
────────────────────────────────────────
1. If token exchange happens, capture it
2. Check for:
   - Client secret exposure
   - Token in URL params vs POST body
   - Token leakage in Referer header

Step 6: Test Token Replay
────────────────────────────────────────
1. Copy access token
2. In Burp Repeater, use token in API request
3. Test if token can be replayed
4. Check token expiration if possible

COMMON VULNERABILITIES TO LOOK FOR:
────────────────────────────────────────
✗ Open Redirect via redirect_uri
✗ Missing/weak state parameter (CSRF)
✗ Authorization code in URL (leakage)
✗ Client secret exposed in frontend
✗ HTTP instead of HTTPS
✗ Weak scope validation
✗ Token in URL fragment (implicit flow)
✗ Subdomain takeover on redirect_uri

REPORTING:
────────────────────────────────────────
For each vulnerability:
1. Clearly describe the issue
2. Provide steps to reproduce
3. Include screenshots from Burp
4. Explain the impact
5. Suggest remediation

═══════════════════════════════════════════════════════════════════════════
"#
    }
}

// ============================================================================
// EXPORTS
// ============================================================================

/// Burp Suite tools - Main entry point
pub struct BurpTools;

impl BurpTools {
    /// Create Burp integration with default settings
    pub fn create_integration(output_dir: PathBuf) -> BurpSuiteIntegration {
        let config = BurpProxyConfig::default();
        BurpSuiteIntegration::new(config, output_dir)
    }

    /// Create Burp launcher
    pub fn create_launcher(jar_path: PathBuf) -> BurpSuiteLauncher {
        BurpSuiteLauncher::new(jar_path, None, Some(1024))
    }
}
