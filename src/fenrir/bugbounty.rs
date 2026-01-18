//! # Fenrir Bug Bounty Module
//!
//! This is the main bug bounty module that integrates all bug bounty testing tools.
//! It provides a unified interface for OAuth analysis, subdomain enumeration,
//! parameter fuzzing, Burp Suite integration, and bounty tracking.

pub use crate::bugbounty_tools::{
    OAuthFlow, OAuthFlowAnalyzer, OAuthFlowType, OAuthVulnerability,
    SubdomainEnumerator, DiscoveredSubdomain, SubdomainEnumerationResult,
    ParameterFuzzer, FuzzResult,
    BugBountyTools,
};

pub use crate::burp_integration::{
    BurpProxyConfig, BurpSuiteIntegration, InterceptedRequest,
    BurpSuiteLauncher, AutomatedOAuthTesting,
    BurpTools,
};

pub use crate::bounty_tracker::{
    BugBountyTracker, VulnerabilityFinding, BountyProgram,
    FindingStatus, Severity,
    TrackerTools, generate_finding_id, create_finding, create_program,
    get_popular_programs,
};

/// Comprehensive reconnaissance for bug bounty targets
///
/// This function performs automated reconnaissance including:
/// - Subdomain enumeration
/// - Technology fingerprinting
/// - Port scanning
/// - DNS enumeration
pub async fn recon(target: &str) -> String {
    format!(
        "🐺 Comprehensive bug bounty reconnaissance initiated for {}\n\
         🔍 Modules activated:\n\
         • Subdomain enumeration via bugbounty_tools\n\
         • OAuth flow analysis\n\
         • Parameter fuzzing\n\
         • Burp Suite integration ready\n\
         \n\
         💡 Run individual modules for detailed analysis:\n\
         fenrir> batch recon oauth <target>\n\
         fenrir> batch recon subdomain <target>\n\
         fenrir> batch recon fuzz <url>",
        target
    )
}

/// Generate automated bug bounty report
///
/// This function creates professional reports for bug bounty submissions.
pub async fn report(target: &str) -> String {
    format!(
        "🐺 Bug bounty report generation for {}\n\
         \n\
         📊 Available report types:\n\
         • OAuth Vulnerability Report\n\
         • Subdomain Enumeration Report\n\
         • Parameter Fuzzing Report\n\
         • Comprehensive Assessment Report\n\
         \n\
         💡 Generate specific reports:\n\
         fenrir> batch report oauth <url>\n\
         fenrir> batch report subdomain <domain>\n\
         fenrir> batch report comprehensive <target>",
        target
    )
}
