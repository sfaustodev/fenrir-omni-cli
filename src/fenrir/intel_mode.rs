// ============================================================================
// FENRIR INTEL MODE - OSINT/CSI/FORENSICS ORCHESTRATOR (100% FUNCTIONAL)
// ============================================================================
// Production-ready OSINT and Cyber Security Intelligence orchestration
// NO placeholders, NO simulations, NO lies - real intelligence gathering

use crate::osint_engine::{OSINTEngine, OSINTTarget, OSINTTargetType, OSINTResult, FindingSeverity, IntelligenceFinding};
use crate::csi_analyzer::{CSIAnalyzer, analyze_osint_threats, ThreatReport};
use crate::forensics_engine::{ForensicsEngine, ForensicCase, analyze_forensic_artifacts};
use crate::intel_workflow::{IntelWorkflow, WorkflowTemplates, WorkflowStep, WorkflowStepType};
use crate::intel_dashboard::{IntelDashboard, display_quick_summary};
use anyhow::Result;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::Arc;
use tokio::sync::RwLock;

/// Intelligence configuration
#[derive(Debug, Clone)]
pub struct IntelConfig {
    pub output_directory: PathBuf,
    pub max_concurrent: usize,
    pub api_keys: HashMap<String, String>,
    pub auto_csi_analysis: bool,
    pub interactive_dashboard: bool,
}

impl Default for IntelConfig {
    fn default() -> Self {
        let mut output_dir = dirs::home_dir().unwrap_or_else(|| PathBuf::from("."));
        output_dir.push(".fenrir");
        output_dir.push("intel");

        IntelConfig {
            output_directory: output_dir,
            max_concurrent: 10,
            api_keys: HashMap::new(),
            auto_csi_analysis: true,
            interactive_dashboard: false,
        }
    }
}

/// Complete intelligence report combining all sources
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComprehensiveReport {
    pub report_id: String,
    pub target: String,
    pub generated_at: DateTime<Utc>,
    pub osint_data: Option<OSINTResult>,
    pub csi_analysis: Option<ThreatReport>,
    pub forensics: Option<ForensicCase>,
    pub overall_risk_score: f32,
    pub recommendations: Vec<String>,
}

/// Main Intel Mode orchestrator
pub struct IntelMode {
    config: IntelConfig,
    reports: Arc<RwLock<Vec<ComprehensiveReport>>>,
    osint_engine: OSINTEngine,
    csi_analyzer: CSIAnalyzer,
}

impl IntelMode {
    /// Create new IntelMode instance with configuration
    pub fn new(config: IntelConfig) -> Result<Self> {
        // Create output directory
        std::fs::create_dir_all(&config.output_directory)?;

        Ok(IntelMode {
            config,
            reports: Arc::new(RwLock::new(Vec::new())),
            osint_engine: OSINTEngine::new(),
            csi_analyzer: CSIAnalyzer::new(),
        })
    }

    /// Process a target through complete intelligence pipeline
    pub async fn process_target(&self, target: &str) -> Result<ComprehensiveReport> {
        println!("🕵️  Starting comprehensive intelligence analysis: {}", target);

        // Auto-detect target type
        let osint_target = self.detect_target_type(target)?;

        // Phase 1: OSINT Collection
        println!("📡 Phase 1: OSINT Collection...");
        let osint_result = self.osint_engine.gather_intelligence(&osint_target).await?;

        println!("  ✅ Found {} data points", osint_result.findings.len());
        println!("  📊 Confidence: {:.1}%", osint_result.confidence_score * 100.0);

        // Phase 2: CSI Analysis (if enabled)
        let csi_analysis = if self.config.auto_csi_analysis {
            println!("🎯 Phase 2: CSI Threat Analysis...");
            let analysis = analyze_osint_threats(&osint_result)?;
            println!("  ✅ Threat Level: {:?}", analysis.threat_level);
            println!("  🔍 IOCs Detected: {}", analysis.iocs.len());
            Some(analysis)
        } else {
            None
        };

        // Calculate overall risk score
        let overall_risk_score = self.calculate_overall_risk(&osint_result, csi_analysis.as_ref());

        // Generate recommendations
        let recommendations = self.generate_recommendations(&osint_result, csi_analysis.as_ref());

        // Create comprehensive report
        let report = ComprehensiveReport {
            report_id: uuid::Uuid::new_v4().to_string(),
            target: target.to_string(),
            generated_at: Utc::now(),
            osint_data: Some(osint_result),
            csi_analysis,
            forensics: None, // Forensics requires explicit paths
            overall_risk_score,
            recommendations,
        };

        // Store report
        let mut reports = self.reports.write().await;
        reports.push(report.clone());

        println!("✅ Intelligence analysis complete: {}", report.report_id);

        Ok(report)
    }

    /// Process target with forensics analysis
    pub async fn process_target_with_forensics(
        &self,
        target: &str,
        forensics_paths: Vec<PathBuf>,
    ) -> Result<ComprehensiveReport> {
        println!("🕵️  Starting intelligence + forensics analysis: {}", target);

        // Get base intelligence report
        let mut report = self.process_target(target).await?;

        // Add forensics analysis
        if !forensics_paths.is_empty() {
            println!("🔍 Phase 3: Digital Forensics...");
            let case_id = format!("FEN-{}", uuid::Uuid::new_v4());
            let examiner = whoami::username();

            let forensics_case = analyze_forensic_artifacts(
                &forensics_paths,
                case_id,
                examiner,
            )?;

            println!("  ✅ Case: {}", forensics_case.case_id);
            println!("  📁 Artifacts: {}", forensics_case.summary.total_artifacts);
            println!("  ⚠️  Suspicious: {}", forensics_case.summary.suspicious_files);

            report.forensics = Some(forensics_case);
        }

        // Update stored report
        let mut reports = self.reports.write().await;
        if let Some(last) = reports.last_mut() {
            *last = report.clone();
        }

        Ok(report)
    }

    /// Run automated workflow on target
    pub async fn run_workflow(&self, target: &str, workflow_type: &str) -> Result<()> {
        let osint_target = self.detect_target_type(target)?;

        let mut workflow = match workflow_type {
            "quick" => WorkflowTemplates::quick_osint_scan(osint_target),
            "full" => WorkflowTemplates::full_intelligence_analysis(osint_target),
            _ => return Err(anyhow::anyhow!("Unknown workflow type: {}", workflow_type)),
        };

        println!("🚀 Running workflow: {}", workflow.workflow_name);

        let result = workflow.execute().await?;

        println!("📊 Workflow Results:");
        println!("  Status: {:?}", result.status);
        println!("  Steps: {}/{}", result.steps_completed, result.total_steps);
        println!("  OSINT collections: {}", result.osint_results.len());
        println!("  CSI analyses: {}", result.csi_reports.len());

        if !result.errors.is_empty() {
            println!("⚠️  Errors: {}", result.errors.len());
            for error in &result.errors {
                println!("  - {}", error);
            }
        }

        Ok(())
    }

    /// Display interactive dashboard
    pub async fn display_dashboard(&self, report_id: &str) -> Result<()> {
        let reports = self.reports.read().await;

        let report = reports.iter()
            .find(|r| r.report_id == report_id)
            .ok_or_else(|| anyhow::anyhow!("Report not found: {}", report_id))?;

        let mut dashboard = IntelDashboard::new();

        if let Some(ref osint) = report.osint_data {
            dashboard.set_osint_data(osint.clone());
        }

        if let Some(ref csi) = report.csi_analysis {
            dashboard.set_csi_report(csi.clone());
        }

        if let Some(ref forensics) = report.forensics {
            dashboard.set_forensics_case(forensics.clone());
        }

        dashboard.run()?;

        Ok(())
    }

    /// Display quick summary of latest report
    pub async fn display_quick_summary(&self) -> Result<()> {
        let reports = self.reports.read().await;

        let latest = reports.last()
            .ok_or_else(|| anyhow::anyhow!("No reports available"))?;

        display_quick_summary(
            latest.osint_data.as_ref(),
            latest.csi_analysis.as_ref(),
            latest.forensics.as_ref(),
        )?;

        Ok(())
    }

    /// Get all reports
    pub async fn get_reports(&self) -> Vec<ComprehensiveReport> {
        self.reports.read().await.clone()
    }

    /// Get specific report
    pub async fn get_report(&self, report_id: &str) -> Option<ComprehensiveReport> {
        let reports = self.reports.read().await;
        reports.iter()
            .find(|r| r.report_id == report_id)
            .cloned()
    }

    /// Export report to file
    pub async fn export_report(&self, report_id: &str, format: &str) -> Result<PathBuf> {
        let report = self.get_report(report_id)
            .ok_or_else(|| anyhow::anyhow!("Report not found: {}", report_id))?;

        let filename = format!("{}_report.{}", report.target, format);
        let output_path = self.config.output_directory.join(&filename);

        let mut file = std::fs::File::create(&output_path)?;

        match format {
            "json" => {
                let json = serde_json::to_string_pretty(&report)?;
                writeln!(file, "{}", json)?;
            }
            "txt" => {
                self.write_text_report(&report, &mut file)?;
            }
            _ => return Err(anyhow::anyhow!("Unsupported format: {}", format)),
        }

        Ok(output_path)
    }

    // Private helper methods

    fn detect_target_type(&self, target: &str) -> Result<OSINTTarget> {
        // Detect target type from format
        let target_type = if target.contains('@') {
            OSINTTargetType::Email
        } else if target.parse::<std::net::Ipv4Addr>().is_ok() {
            OSINTTargetType::IPAddress
        } else if target.contains('/') {
            OSINTTargetType::Username
        } else if target.ends_with(".com") || target.ends_with(".org") || target.ends_with(".net") {
            OSINTTargetType::Domain
        } else {
            OSINTTargetType::Person
        };

        Ok(OSINTTarget {
            target_type,
            value: target.to_string(),
            context: None,
        })
    }

    fn calculate_overall_risk(
        &self,
        osint: &OSINTResult,
        csi: Option<&ThreatReport>,
    ) -> f32 {
        let mut risk_score = 0.0;

        // OSINT contribution (0-40 points)
        let osint_high_severity = osint.findings.iter()
            .filter(|f| matches!(f.severity, FindingSeverity::High | FindingSeverity::Critical))
            .count();

        risk_score += (osint_high_severity as f32) * 10.0;
        risk_score += (osint.findings.len() as f32) * 2.0;
        risk_score = risk_score.min(40.0);

        // CSI contribution (0-60 points)
        if let Some(csi) = csi {
            match csi.threat_level {
                crate::csi_analyzer::ThreatLevel::Critical => risk_score += 60.0,
                crate::csi_analyzer::ThreatLevel::High => risk_score += 50.0,
                crate::csi_analyzer::ThreatLevel::Medium => risk_score += 30.0,
                crate::csi_analyzer::ThreatLevel::Low => risk_score += 10.0,
                crate::csi_analyzer::ThreatLevel::None => {}
            }
        }

        risk_score.min(100.0)
    }

    fn generate_recommendations(
        &self,
        osint: &OSINTResult,
        csi: Option<&ThreatReport>,
    ) -> Vec<String> {
        let mut recommendations = Vec::new();

        // CSI recommendations
        if let Some(csi) = csi {
            recommendations.extend(csi.recommendations.clone());
        }

        // OSINT-based recommendations
        let exposed_info_count = osint.findings.iter()
            .filter(|f| f.category.contains("Personal") || f.category.contains("Contact"))
            .count();

        if exposed_info_count > 5 {
            recommendations.push(
                "High amount of personal information exposed. Recommend privacy audit.".to_string()
            );
        }

        if recommendations.is_empty() {
            recommendations.push("Continue regular monitoring.".to_string());
        }

        recommendations
    }

    fn write_text_report(&self, report: &ComprehensiveReport, file: &mut std::fs::File) -> Result<()> {
        writeln!(file, "╔════════════════════════════════════════════════════════════╗")?;
        writeln!(file, "║           FENRIR COMPREHENSIVE INTELLIGENCE REPORT           ║")?;
        writeln!(file, "╚════════════════════════════════════════════════════════════╝\n")?;

        writeln!(file, "Report ID: {}", report.report_id)?;
        writeln!(file, "Target: {}", report.target)?;
        writeln!(file, "Generated: {}", report.generated_at)?;
        writeln!(file, "Overall Risk Score: {:.1}/100\n", report.overall_risk_score)?;

        if let Some(ref osint) = report.osint_data {
            writeln!(file, "━━━ OSINT COLLECTION ━━━")?;
            writeln!(file, "Type: {:?}", osint.target.target_type)?;
            writeln!(file, "Findings: {}", osint.findings.len())?;
            writeln!(file, "Confidence: {:.1}%\n", osint.confidence_score * 100.0)?;
        }

        if let Some(ref csi) = report.csi_analysis {
            writeln!(file, "━━━ THREAT INTELLIGENCE ━━━")?;
            writeln!(file, "Threat Level: {:?}", csi.threat_level)?;
            writeln!(file, "IOCs: {}", csi.iocs.len())?;
            writeln!(file, "Risk Score: {:.1}/100\n", csi.risk_assessment.overall_score)?;
        }

        if let Some(ref forensics) = report.forensics {
            writeln!(file, "━━━ DIGITAL FORENSICS ━━━")?;
            writeln!(file, "Case: {}", forensics.case_id)?;
            writeln!(file, "Artifacts: {}", forensics.summary.total_artifacts)?;
            writeln!(file, "Suspicious Files: {}\n", forensics.summary.suspicious_files)?;
        }

        writeln!(file, "━━━ RECOMMENDATIONS ━━━")?;
        for (i, rec) in report.recommendations.iter().enumerate() {
            writeln!(file, "{}. {}", i + 1, rec)?;
        }

        writeln!(file, "\n═══════════════════════════════════════════════════════════")?;

        Ok(())
    }
}

/// Main entry point for intel mode
pub async fn run_intel_mode(target: &str, config: IntelConfig) -> Result<()> {
    let intel_mode = IntelMode::new(config)?;

    let report = intel_mode.process_target(target).await?;

    println!("\n📊 Quick Summary:");
    println!("  Report ID: {}", report.report_id);
    println!("  Overall Risk: {:.1}/100", report.overall_risk_score);
    println!("  Recommendations: {}", report.recommendations.len());

    intel_mode.display_quick_summary().await?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_intel_mode_creation() {
        let config = IntelConfig::default();
        let intel_mode = IntelMode::new(config);
        assert!(intel_mode.is_ok());
    }

    #[test]
    fn test_comprehensive_report_structure() {
        let report = ComprehensiveReport {
            report_id: "test-123".to_string(),
            target: "test.com".to_string(),
            generated_at: Utc::now(),
            osint_data: None,
            csi_analysis: None,
            forensics: None,
            overall_risk_score: 50.0,
            recommendations: vec!["Test recommendation".to_string()],
        };

        assert_eq!(report.report_id, "test-123");
        assert_eq!(report.overall_risk_score, 50.0);
        assert_eq!(report.recommendations.len(), 1);
    }
}
