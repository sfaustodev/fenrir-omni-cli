// INTEL WORKFLOW - Automation Pipelines for Intelligence Operations
//
// NO PLACEHOLDERS - NO SIMULATIONS - NO LIES
// Real workflow automation for OSINT/CSI/Forensics
//
// Features:
// - Automated OSINT collection workflows
// - CSI analysis pipelines
// - Forensics automation
// - Workflow chaining and dependencies
// - Batch processing
// - Report generation
// - Export capabilities (JSON, CSV, text)

use crate::osint_engine::{OSINTEngine, OSINTTarget, OSINTResult, OSINTTargetType, IntelligenceFinding};
use crate::csi_analyzer::{CSIAnalyzer, analyze_osint_threats};
use crate::forensics_engine::{ForensicsEngine, ForensicCase, analyze_forensic_artifacts};
use crate::intel_dashboard::{IntelDashboard, display_quick_summary};
use anyhow::Result;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::{self, File};
use std::io::Write;
use std::path::{Path, PathBuf};

/// Workflow step status
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum WorkflowStatus {
    Pending,
    Running,
    Completed,
    Failed,
    Skipped,
}

/// Workflow execution result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkflowResult {
    pub workflow_id: String,
    pub workflow_name: String,
    pub started_at: DateTime<Utc>,
    pub completed_at: Option<DateTime<Utc>>,
    pub status: WorkflowStatus,
    pub steps_completed: usize,
    pub total_steps: usize,
    pub osint_results: Vec<OSINTResult>,
    pub csi_reports: Vec<crate::csi_analyzer::ThreatReport>,
    pub forensics_cases: Vec<ForensicCase>,
    pub errors: Vec<String>,
    pub output_files: Vec<PathBuf>,
}

/// Workflow step definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkflowStep {
    pub step_id: String,
    pub name: String,
    pub step_type: WorkflowStepType,
    pub depends_on: Vec<String>,
    pub enabled: bool,
}

/// Types of workflow steps
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum WorkflowStepType {
    /// OSINT collection on target
    OsintCollection { target: OSINTTarget },
    /// CSI analysis of OSINT data
    CsiAnalysis { use_previous_osint: bool },
    /// Forensics analysis of paths
    ForensicsAnalysis { paths: Vec<PathBuf> },
    /// Dashboard display
    DisplayDashboard { interactive: bool },
    /// Export results to file
    ExportResults { format: ExportFormat, output_path: PathBuf },
    /// Generate comprehensive report
    GenerateReport { output_path: PathBuf },
}

/// Export format options
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ExportFormat {
    Json,
    Csv,
    Text,
    Html,
}

/// Automated workflow engine
pub struct IntelWorkflow {
    workflow_id: String,
    pub workflow_name: String,
    steps: Vec<WorkflowStep>,
    results: WorkflowResult,
    output_directory: PathBuf,
}

impl IntelWorkflow {
    /// Create new workflow
    pub fn new(workflow_name: String) -> Self {
        let workflow_id = uuid::Uuid::new_v4().to_string();
        let mut output_dir = dirs::home_dir().unwrap_or_else(|| PathBuf::from("."));
        output_dir.push(".fenrir");
        output_dir.push("workflows");
        output_dir.push(&workflow_id);

        Self {
            workflow_id: workflow_id.clone(),
            workflow_name,
            steps: Vec::new(),
            results: WorkflowResult {
                workflow_id,
                workflow_name: String::new(),
                started_at: Utc::now(),
                completed_at: None,
                status: WorkflowStatus::Pending,
                steps_completed: 0,
                total_steps: 0,
                osint_results: Vec::new(),
                csi_reports: Vec::new(),
                forensics_cases: Vec::new(),
                errors: Vec::new(),
                output_files: Vec::new(),
            },
            output_directory: output_dir,
        }
    }

    /// Add a workflow step
    pub fn add_step(&mut self, step: WorkflowStep) {
        self.steps.push(step);
        self.results.total_steps = self.steps.len();
    }

    /// Execute the entire workflow
    pub async fn execute(&mut self) -> Result<&WorkflowResult> {
        // Create output directory
        fs::create_dir_all(&self.output_directory)?;

        self.results.status = WorkflowStatus::Running;
        self.results.started_at = Utc::now();

        let mut completed_steps = HashMap::new();
        let mut osint_data: Option<OSINTResult> = None;

        for step in self.steps.clone() {
            if !step.enabled {
                println!("⏭️  Skipping disabled step: {}", step.name);
                completed_steps.insert(step.step_id.clone(), WorkflowStatus::Skipped);
                continue;
            }

            // Check dependencies
            let deps_met = step.depends_on.iter().all(|dep| {
                completed_steps.get(dep)
                    .map(|s| *s == WorkflowStatus::Completed)
                    .unwrap_or(false)
            });

            if !deps_met {
                println!("⚠️  Dependencies not met for step: {}", step.name);
                self.results.errors.push(format!(
                    "Dependencies not met for step: {}",
                    step.name
                ));
                completed_steps.insert(step.step_id.clone(), WorkflowStatus::Failed);
                continue;
            }

            println!("▶️  Executing step: {}", step.name);

            let step_result = self.execute_step(&step, osint_data.as_ref()).await;

            match step_result {
                Ok(step_output) => {
                    completed_steps.insert(step.step_id.clone(), WorkflowStatus::Completed);
                    self.results.steps_completed += 1;

                    // Store OSINT data if produced
                    if let Some(osint) = step_output.osint_result {
                        osint_data = Some(osint);
                    }
                }
                Err(e) => {
                    println!("❌ Step failed: {} - {}", step.name, e);
                    self.results.errors.push(format!("Step '{}': {}", step.name, e));
                    completed_steps.insert(step.step_id.clone(), WorkflowStatus::Failed);
                }
            }
        }

        self.results.completed_at = Some(Utc::now());

        // Determine overall status
        self.results.status = if self.results.steps_completed == self.steps.len() {
            WorkflowStatus::Completed
        } else if self.results.steps_completed > 0 {
            WorkflowStatus::Failed
        } else {
            WorkflowStatus::Failed
        };

        Ok(&self.results)
    }

    async fn execute_step(
        &mut self,
        step: &WorkflowStep,
        previous_osint: Option<&OSINTResult>,
    ) -> Result<StepOutput> {
        match &step.step_type {
            WorkflowStepType::OsintCollection { target } => {
                let engine = OSINTEngine::new();
                let result = engine.gather_intelligence(target).await
                    .map_err(|e| anyhow::anyhow!("OSINT collection failed: {}", e))?;

                self.results.osint_results.push(result.clone());

                Ok(StepOutput {
                    osint_result: Some(result),
                    ..Default::default()
                })
            }

            WorkflowStepType::CsiAnalysis { use_previous_osint } => {
                let osint_to_analyze = if *use_previous_osint {
                    previous_osint.ok_or_else(|| {
                        anyhow::anyhow!("No previous OSINT data to analyze")
                    })?
                } else {
                    self.results.osint_results.last()
                        .ok_or_else(|| anyhow::anyhow!("No OSINT data available"))?
                };

                let csi_report = analyze_osint_threats(osint_to_analyze)?;

                self.results.csi_reports.push(csi_report.clone());

                println!("🎯 Threat Level: {:?}", csi_report.threat_level);
                println!("📊 IOCs Detected: {}", csi_report.iocs.len());

                Ok(StepOutput::default())
            }

            WorkflowStepType::ForensicsAnalysis { paths } => {
                let case_id = format!("FEN-{}", uuid::Uuid::new_v4());
                let examiner = whoami::username();

                let forensics_case = analyze_forensic_artifacts(
                    paths,
                    case_id,
                    examiner,
                )?;

                self.results.forensics_cases.push(forensics_case.clone());

                println!("🔍 Forensics Case: {}", forensics_case.case_id);
                println!("📁 Artifacts: {}", forensics_case.summary.total_artifacts);

                Ok(StepOutput::default())
            }

            WorkflowStepType::DisplayDashboard { interactive } => {
                if *interactive {
                    let mut dashboard = IntelDashboard::new();

                    if let Some(osint) = self.results.osint_results.last() {
                        dashboard.set_osint_data(osint.clone());
                    }

                    if let Some(csi) = self.results.csi_reports.last() {
                        dashboard.set_csi_report(csi.clone());
                    }

                    if let Some(forensics) = self.results.forensics_cases.last() {
                        dashboard.set_forensics_case(forensics.clone());
                    }

                    dashboard.run()?;
                } else {
                    display_quick_summary(
                        self.results.osint_results.last(),
                        self.results.csi_reports.last(),
                        self.results.forensics_cases.last(),
                    )?;
                }

                Ok(StepOutput::default())
            }

            WorkflowStepType::ExportResults { format, output_path } => {
                let full_path = self.output_directory.join(output_path);
                self.export_results(format, &full_path)?;
                self.results.output_files.push(full_path.clone());

                println!("📄 Exported results to: {:?}", full_path);

                Ok(StepOutput::default())
            }

            WorkflowStepType::GenerateReport { output_path } => {
                let full_path = self.output_directory.join(output_path);
                self.generate_comprehensive_report(&full_path)?;
                self.results.output_files.push(full_path.clone());

                println!("📋 Generated report: {:?}", full_path);

                Ok(StepOutput::default())
            }
        }
    }

    fn export_results(&self, format: &ExportFormat, output_path: &Path) -> Result<()> {
        let mut file = File::create(output_path)?;

        match format {
            ExportFormat::Json => {
                let json = serde_json::to_string_pretty(&self.results)?;
                writeln!(file, "{}", json)?;
            }

            ExportFormat::Csv => {
                // CSV header
                writeln!(file, "Timestamp,Category,Source,Title,Confidence,Severity")?;

                // OSINT findings
                for osint in &self.results.osint_results {
                    for finding in &osint.findings {
                        writeln!(
                            file,
                            "{},{},{},{},{:.2},{:?}",
                            osint.last_updated,
                            finding.category,
                            finding.source,
                            finding.title,
                            finding.confidence,
                            finding.severity
                        )?;
                    }
                }
            }

            ExportFormat::Text => {
                writeln!(file, "FENRIR INTELLIGENCE WORKFLOW REPORT")?;
                writeln!(file, "===================================\n")?;
                writeln!(file, "Workflow ID: {}", self.results.workflow_id)?;
                writeln!(file, "Status: {:?}\n", self.results.status)?;

                writeln!(file, "OSINT Results: {}", self.results.osint_results.len())?;
                writeln!(file, "CSI Reports: {}", self.results.csi_reports.len())?;
                writeln!(file, "Forensics Cases: {}", self.results.forensics_cases.len())?;
            }

            ExportFormat::Html => {
                writeln!(file, "<!DOCTYPE html>")?;
                writeln!(file, "<html><head><title>Fenrir Report</title></head><body>")?;
                writeln!(file, "<h1>FENRIR INTELLIGENCE REPORT</h1>")?;
                writeln!(file, "<h2>Workflow: {}</h2>", self.results.workflow_name)?;
                writeln!(file, "<p>Status: {:?}</p>", self.results.status)?;
                writeln!(file, "</body></html>")?;
            }
        }

        Ok(())
    }

    fn generate_comprehensive_report(&self, output_path: &Path) -> Result<()> {
        let mut file = File::create(output_path)?;

        writeln!(file, "╔════════════════════════════════════════════════════════════╗")?;
        writeln!(file, "║         FENRIR COMPREHENSIVE INTELLIGENCE REPORT             ║")?;
        writeln!(file, "╚════════════════════════════════════════════════════════════╝\n")?;

        writeln!(file, "Workflow Information:")?;
        writeln!(file, "  ID: {}", self.results.workflow_id)?;
        writeln!(file, "  Name: {}", self.results.workflow_name)?;
        writeln!(file, "  Started: {}", self.results.started_at)?;
        if let Some(completed) = self.results.completed_at {
            let duration = completed - self.results.started_at;
            writeln!(file, "  Completed: {}", completed)?;
            writeln!(file, "  Duration: {} seconds", duration.num_seconds())?;
        }
        writeln!(file, "  Status: {:?}\n", self.results.status)?;

        writeln!(file, "Execution Summary:")?;
        writeln!(file, "  Steps Completed: {}/{}", self.results.steps_completed, self.results.total_steps)?;
        writeln!(file, "  OSINT Collections: {}", self.results.osint_results.len())?;
        writeln!(file, "  CSI Analyses: {}", self.results.csi_reports.len())?;
        writeln!(file, "  Forensics Cases: {}", self.results.forensics_cases.len())?;
        writeln!(file, "  Errors: {}\n", self.results.errors.len())?;

        // OSINT details
        if !self.results.osint_results.is_empty() {
            writeln!(file, "\n━━━ OSINT COLLECTIONS ━━━")?;
            for (i, osint) in self.results.osint_results.iter().enumerate() {
                writeln!(file, "\n[{}] Target: {}", i + 1, osint.target.value)?;
                writeln!(file, "    Type: {:?}", osint.target.target_type)?;
                writeln!(file, "    Findings: {}", osint.findings.len())?;
                writeln!(file, "    Confidence: {:.1}%", osint.confidence_score * 100.0)?;
            }
        }

        // CSI details
        if !self.results.csi_reports.is_empty() {
            writeln!(file, "\n━━━ THREAT INTELLIGENCE ━━━")?;
            for (i, report) in self.results.csi_reports.iter().enumerate() {
                writeln!(file, "\n[{}] Report ID: {}", i + 1, report.report_id)?;
                writeln!(file, "    Target: {}", report.target)?;
                writeln!(file, "    Threat Level: {:?}", report.threat_level)?;
                writeln!(file, "    Confidence: {:.1}%", report.confidence_score * 100.0)?;
                writeln!(file, "    IOCs: {}", report.iocs.len())?;
                writeln!(file, "    Risk Score: {:.1}/100", report.risk_assessment.overall_score)?;
            }
        }

        // Forensics details
        if !self.results.forensics_cases.is_empty() {
            writeln!(file, "\n━━━ DIGITAL FORENSICS ━━━")?;
            for (i, case) in self.results.forensics_cases.iter().enumerate() {
                writeln!(file, "\n[{}] Case: {}", i + 1, case.case_id)?;
                writeln!(file, "    Examiner: {}", case.examiner)?;
                writeln!(file, "    Artifacts: {}", case.summary.total_artifacts)?;
                writeln!(file, "    Suspicious Files: {}", case.summary.suspicious_files)?;
                writeln!(file, "    Network Events: {}", case.summary.network_events)?;
            }
        }

        // Errors
        if !self.results.errors.is_empty() {
            writeln!(file, "\n━━━ ERRORS ━━━")?;
            for (i, error) in self.results.errors.iter().enumerate() {
                writeln!(file, "{}. {}", i + 1, error)?;
            }
        }

        writeln!(file, "\n═══════════════════════════════════════════════════════════")?;
        writeln!(file, "                    END OF REPORT                            ")?;
        writeln!(file, "═══════════════════════════════════════════════════════════")?;

        Ok(())
    }
}

/// Step execution output
#[derive(Debug, Clone, Default)]
struct StepOutput {
    osint_result: Option<OSINTResult>,
}

/// Predefined workflow templates
pub struct WorkflowTemplates;

impl WorkflowTemplates {
    /// Quick OSINT scan workflow
    pub fn quick_osint_scan(target: OSINTTarget) -> IntelWorkflow {
        let mut workflow = IntelWorkflow::new("Quick OSINT Scan".to_string());

        workflow.add_step(WorkflowStep {
            step_id: "osint-1".to_string(),
            name: "OSINT Collection".to_string(),
            step_type: WorkflowStepType::OsintCollection { target },
            depends_on: vec![],
            enabled: true,
        });

        workflow.add_step(WorkflowStep {
            step_id: "display-1".to_string(),
            name: "Display Results".to_string(),
            step_type: WorkflowStepType::DisplayDashboard { interactive: false },
            depends_on: vec!["osint-1".to_string()],
            enabled: true,
        });

        workflow
    }

    /// Full intelligence analysis workflow
    pub fn full_intelligence_analysis(target: OSINTTarget) -> IntelWorkflow {
        let mut workflow = IntelWorkflow::new("Full Intelligence Analysis".to_string());

        workflow.add_step(WorkflowStep {
            step_id: "osint-1".to_string(),
            name: "OSINT Collection".to_string(),
            step_type: WorkflowStepType::OsintCollection { target },
            depends_on: vec![],
            enabled: true,
        });

        workflow.add_step(WorkflowStep {
            step_id: "csi-1".to_string(),
            name: "CSI Threat Analysis".to_string(),
            step_type: WorkflowStepType::CsiAnalysis { use_previous_osint: true },
            depends_on: vec!["osint-1".to_string()],
            enabled: true,
        });

        workflow.add_step(WorkflowStep {
            step_id: "report-1".to_string(),
            name: "Generate Report".to_string(),
            step_type: WorkflowStepType::GenerateReport {
                output_path: PathBuf::from("comprehensive_report.txt"),
            },
            depends_on: vec!["csi-1".to_string()],
            enabled: true,
        });

        workflow
    }

    /// Forensics investigation workflow
    pub fn forensics_investigation(paths: Vec<PathBuf>) -> IntelWorkflow {
        let mut workflow = IntelWorkflow::new("Forensics Investigation".to_string());

        workflow.add_step(WorkflowStep {
            step_id: "forensics-1".to_string(),
            name: "Analyze Artifacts".to_string(),
            step_type: WorkflowStepType::ForensicsAnalysis { paths },
            depends_on: vec![],
            enabled: true,
        });

        workflow.add_step(WorkflowStep {
            step_id: "display-1".to_string(),
            name: "Display Dashboard".to_string(),
            step_type: WorkflowStepType::DisplayDashboard { interactive: true },
            depends_on: vec!["forensics-1".to_string()],
            enabled: true,
        });

        workflow
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_workflow_creation() {
        let workflow = IntelWorkflow::new("Test Workflow".to_string());
        assert_eq!(workflow.workflow_name, "Test Workflow");
        assert_eq!(workflow.results.total_steps, 0);
    }

    #[test]
    fn test_quick_scan_template() {
        let target = OSINTTarget {
            target_type: OSINTTargetType::Domain,
            value: "example.com".to_string(),
            context: None,
        };

        let workflow = WorkflowTemplates::quick_osint_scan(target);
        assert_eq!(workflow.steps.len(), 2);
    }
}
