// --- FENRIR BATCH EXECUTOR ---
// Abstractions for batch execution of Kali tools
// Enables parallel/sequential execution with progress tracking

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::Mutex;

// Re-export for convenience
pub use crate::kali_tools_comprehensive::FenrirOrchestrationEngine;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ExecutionMode {
    #[serde(rename = "sequential")]
    Sequential, // Execute tools one by one
    #[serde(rename = "parallel")]
    Parallel, // Execute tools in parallel with limits
    #[serde(rename = "pipeline")]
    Pipeline, // Execute in dependency-aware pipeline
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ExecutionPriority {
    #[serde(rename = "low")]
    Low,
    #[serde(rename = "normal")]
    Normal,
    #[serde(rename = "high")]
    High,
    #[serde(rename = "critical")]
    Critical,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BatchConfig {
    pub mode: ExecutionMode,
    pub max_parallel: usize,
    pub timeout_per_tool: Duration,
    pub retry_attempts: u32,
    pub continue_on_failure: bool,
    pub priority: ExecutionPriority,
    pub resource_limits: ResourceLimits,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceLimits {
    pub max_cpu_percent: f32,
    pub max_memory_mb: u64,
    pub max_concurrent_tools: usize,
}

impl Default for BatchConfig {
    fn default() -> Self {
        BatchConfig {
            mode: ExecutionMode::Sequential,
            max_parallel: 3,
            timeout_per_tool: Duration::from_secs(300),
            retry_attempts: 2,
            continue_on_failure: true,
            priority: ExecutionPriority::Normal,
            resource_limits: ResourceLimits {
                max_cpu_percent: 80.0,
                max_memory_mb: 2048,
                max_concurrent_tools: 5,
            },
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BatchJob {
    pub job_id: String,
    pub name: String,
    pub description: String,
    pub tools: Vec<String>, // Tool names
    pub target: String,
    pub config: BatchConfig,
    pub created_at: DateTime<Utc>,
    pub status: JobStatus,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum JobStatus {
    #[serde(rename = "pending")]
    Pending,
    #[serde(rename = "running")]
    Running,
    #[serde(rename = "completed")]
    Completed,
    #[serde(rename = "failed")]
    Failed,
    #[serde(rename = "cancelled")]
    Cancelled,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolExecutionResult {
    pub tool_name: String,
    pub success: bool,
    pub output: String,
    pub error: Option<String>,
    pub execution_time: Duration,
    pub retry_count: u32,
    pub timestamp: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BatchResult {
    pub job_id: String,
    pub total_tools: usize,
    pub successful_tools: usize,
    pub failed_tools: usize,
    pub total_execution_time: Duration,
    pub results: Vec<ToolExecutionResult>,
    pub summary: String,
}

pub struct BatchExecutor {
    pub engine: Arc<Mutex<FenrirOrchestrationEngine>>,
    pub active_jobs: Arc<Mutex<HashMap<String, BatchJob>>>,
    pub job_results: Arc<Mutex<HashMap<String, BatchResult>>>,
}

impl BatchExecutor {
    pub fn new(target: String) -> Self {
        let engine = Arc::new(Mutex::new(FenrirOrchestrationEngine::new(target)));
        let active_jobs = Arc::new(Mutex::new(HashMap::new()));
        let job_results = Arc::new(Mutex::new(HashMap::new()));

        BatchExecutor {
            engine,
            active_jobs,
            job_results,
        }
    }

    pub async fn submit_job(&self, job: BatchJob) -> Result<String, String> {
        let job_id = job.job_id.clone();

        // Validate tools exist
        let engine = self.engine.lock().await;
        for tool_name in &job.tools {
            if !engine.tools.iter().any(|t| t.name == *tool_name) {
                return Err(format!("Tool '{}' not found in tool database", tool_name));
            }
        }
        drop(engine);

        // Store job
        let mut jobs = self.active_jobs.lock().await;
        jobs.insert(job_id.clone(), job);

        Ok(job_id)
    }

    pub async fn execute_job(&self, job_id: &str) -> Result<BatchResult, String> {
        let mut jobs = self.active_jobs.lock().await;
        let job = jobs
            .get_mut(job_id)
            .ok_or_else(|| format!("Job {} not found", job_id))?;

        // Update status
        job.status = JobStatus::Running;
        let job_clone = job.clone();
        drop(jobs);

        let start_time = std::time::Instant::now();
        let mut results = Vec::new();
        let mut successful = 0;
        let mut failed = 0;

        match job_clone.config.mode {
            ExecutionMode::Sequential => {
                results = self.execute_sequential(&job_clone).await?;
            }
            ExecutionMode::Parallel => {
                results = self.execute_parallel(&job_clone).await?;
            }
            ExecutionMode::Pipeline => {
                results = self.execute_pipeline(&job_clone).await?;
            }
        }

        // Count results
        for result in &results {
            if result.success {
                successful += 1;
            } else {
                failed += 1;
            }
        }

        let total_time = start_time.elapsed();

        // Generate summary
        let summary =
            self.generate_batch_summary(&job_clone, &results, successful, failed, total_time);

        let batch_result = BatchResult {
            job_id: job_id.to_string(),
            total_tools: job_clone.tools.len(),
            successful_tools: successful,
            failed_tools: failed,
            total_execution_time: total_time,
            results,
            summary,
        };

        // Update job status
        let mut jobs = self.active_jobs.lock().await;
        if let Some(job) = jobs.get_mut(job_id) {
            job.status = if failed == 0 {
                JobStatus::Completed
            } else {
                JobStatus::Failed
            };
        }

        // Store result
        let mut job_results = self.job_results.lock().await;
        job_results.insert(job_id.to_string(), batch_result.clone());

        Ok(batch_result)
    }

    async fn execute_sequential(&self, job: &BatchJob) -> Result<Vec<ToolExecutionResult>, String> {
        let mut results = Vec::new();
        let engine = self.engine.lock().await;

        for tool_name in &job.tools {
            println!("🔧 Executing {} (sequential)", tool_name);

            // Get tool info first (immutable borrow)
            let tool_info = engine
                .tools
                .iter()
                .find(|t| t.name == *tool_name)
                .cloned()
                .ok_or_else(|| format!("Tool {} not found", tool_name))?;

            let start = std::time::Instant::now();
            let args = vec![job.target.clone()];

            // Execute tool (mutable borrow in separate scope)
            let result = {
                let mut engine_guard = self.engine.lock().await;
                match tokio::time::timeout(
                    job.config.timeout_per_tool,
                    engine_guard.execute_tool(&tool_info, &args),
                )
                .await
                {
                    Ok(Ok(output)) => ToolExecutionResult {
                        tool_name: tool_name.clone(),
                        success: true,
                        output,
                        error: None,
                        execution_time: start.elapsed(),
                        retry_count: 0,
                        timestamp: Utc::now(),
                    },
                    Ok(Err(e)) => {
                        if job.config.continue_on_failure {
                            ToolExecutionResult {
                                tool_name: tool_name.clone(),
                                success: false,
                                output: String::new(),
                                error: Some(e),
                                execution_time: start.elapsed(),
                                retry_count: 0,
                                timestamp: Utc::now(),
                            }
                        } else {
                            return Err(format!("Tool {} failed: {}", tool_name, e));
                        }
                    }
                    Err(_) => {
                        if job.config.continue_on_failure {
                            ToolExecutionResult {
                                tool_name: tool_name.clone(),
                                success: false,
                                output: String::new(),
                                error: Some("Timeout".to_string()),
                                execution_time: job.config.timeout_per_tool,
                                retry_count: 0,
                                timestamp: Utc::now(),
                            }
                        } else {
                            return Err(format!("Tool {} timed out", tool_name));
                        }
                    }
                }
            };

            results.push(result);
        }

        Ok(results)
    }

    async fn execute_parallel(&self, job: &BatchJob) -> Result<Vec<ToolExecutionResult>, String> {
        use std::sync::Arc;
        use tokio::sync::Semaphore;

        let semaphore = Arc::new(Semaphore::new(job.config.max_parallel));
        let mut handles = Vec::new();
        let engine = Arc::clone(&self.engine);

        for tool_name in &job.tools {
            let tool_name = tool_name.clone();
            let target = job.target.clone();
            let timeout = job.config.timeout_per_tool;
            let continue_on_failure = job.config.continue_on_failure;
            let semaphore = Arc::clone(&semaphore);
            let engine = Arc::clone(&engine);

            let handle = tokio::spawn(async move {
                let _permit = semaphore.acquire().await.unwrap();

                println!("🔧 Executing {} (parallel)", tool_name);

                // Get tool info first (immutable borrow)
                let tool_info = {
                    let engine_guard = engine.lock().await;
                    engine_guard
                        .tools
                        .iter()
                        .find(|t| t.name == tool_name)
                        .cloned()
                };

                let tool = tool_info.ok_or_else(|| format!("Tool {} not found", tool_name))?;

                let start = std::time::Instant::now();
                let args = vec![target.clone()];

                // Execute tool (mutable borrow in separate scope)
                let result = {
                    let mut engine_guard = engine.lock().await;
                    match tokio::time::timeout(timeout, engine_guard.execute_tool(&tool, &args))
                        .await
                    {
                        Ok(Ok(output)) => ToolExecutionResult {
                            tool_name: tool_name.clone(),
                            success: true,
                            output,
                            error: None,
                            execution_time: start.elapsed(),
                            retry_count: 0,
                            timestamp: Utc::now(),
                        },
                        Ok(Err(e)) => {
                            if continue_on_failure {
                                ToolExecutionResult {
                                    tool_name: tool_name.clone(),
                                    success: false,
                                    output: String::new(),
                                    error: Some(e),
                                    execution_time: start.elapsed(),
                                    retry_count: 0,
                                    timestamp: Utc::now(),
                                }
                            } else {
                                return Err(format!("Tool {} failed: {}", tool_name, e));
                            }
                        }
                        Err(_) => {
                            if continue_on_failure {
                                ToolExecutionResult {
                                    tool_name: tool_name.clone(),
                                    success: false,
                                    output: String::new(),
                                    error: Some("Timeout".to_string()),
                                    execution_time: timeout,
                                    retry_count: 0,
                                    timestamp: Utc::now(),
                                }
                            } else {
                                return Err(format!("Tool {} timed out", tool_name));
                            }
                        }
                    }
                };

                Ok(result)
            });

            handles.push(handle);
        }

        // Collect results
        let mut results = Vec::new();
        for handle in handles {
            match handle.await {
                Ok(Ok(result)) => results.push(result),
                Ok(Err(e)) => {
                    if !job.config.continue_on_failure {
                        return Err(e);
                    }
                    // Create error result
                    results.push(ToolExecutionResult {
                        tool_name: "unknown".to_string(),
                        success: false,
                        output: String::new(),
                        error: Some(e),
                        execution_time: Duration::from_secs(0),
                        retry_count: 0,
                        timestamp: Utc::now(),
                    });
                }
                Err(e) => {
                    if !job.config.continue_on_failure {
                        return Err(format!("Task join error: {:?}", e));
                    }
                }
            }
        }

        Ok(results)
    }

    async fn execute_pipeline(&self, job: &BatchJob) -> Result<Vec<ToolExecutionResult>, String> {
        // For now, implement as sequential with dependency awareness
        // TODO: Implement proper pipeline with dependency resolution
        self.execute_sequential(job).await
    }

    fn generate_batch_summary(
        &self,
        job: &BatchJob,
        results: &[ToolExecutionResult],
        successful: usize,
        failed: usize,
        total_time: Duration,
    ) -> String {
        let mut summary = format!(
            "# 🔧 FENRIR BATCH EXECUTION SUMMARY\n\n\
             **Job**: {}\n\
             **Target**: {}\n\
             **Mode**: {:?}\n\
             **Total Tools**: {}\n\
             **Successful**: {}\n\
             **Failed**: {}\n\
             **Total Time**: {:.2}s\n\n",
            job.name,
            job.target,
            job.config.mode,
            job.tools.len(),
            successful,
            failed,
            total_time.as_secs_f64()
        );

        if !results.is_empty() {
            summary.push_str("## 📊 TOOL RESULTS\n\n");
            for result in results {
                let status = if result.success { "✅" } else { "❌" };
                summary.push_str(&format!(
                    "### {} {}\n\n**Time**: {:.2}s\n\n",
                    status,
                    result.tool_name,
                    result.execution_time.as_secs_f64()
                ));

                if result.success {
                    summary.push_str(&format!(
                        "**Output Preview**:\n```\n{}\n```\n\n",
                        result.output.chars().take(500).collect::<String>()
                    ));
                } else if let Some(error) = &result.error {
                    summary.push_str(&format!("**Error**: {}\n\n", error));
                }
            }
        }

        summary.push_str(&format!(
            "---\n\n*Generated by FENRIR Batch Executor at {}*",
            Utc::now().format("%Y-%m-%d %H:%M:%S UTC")
        ));

        summary
    }

    pub async fn cancel_job(&self, job_id: &str) -> Result<(), String> {
        let mut jobs = self.active_jobs.lock().await;
        if let Some(job) = jobs.get_mut(job_id) {
            job.status = JobStatus::Cancelled;
            Ok(())
        } else {
            Err(format!("Job {} not found", job_id))
        }
    }

    pub async fn get_job_status(&self, job_id: &str) -> Option<JobStatus> {
        let jobs = self.active_jobs.lock().await;
        jobs.get(job_id).map(|job| job.status.clone())
    }

    pub async fn get_job_result(&self, job_id: &str) -> Option<BatchResult> {
        let results = self.job_results.lock().await;
        results.get(job_id).cloned()
    }

    pub async fn list_jobs(&self) -> Vec<BatchJob> {
        let jobs = self.active_jobs.lock().await;
        jobs.values().cloned().collect()
    }
}

// Helper functions for creating common batch jobs
pub fn create_recon_job(target: &str) -> BatchJob {
    BatchJob {
        job_id: format!(
            "recon_{}_{}",
            target.replace(".", "_"),
            Utc::now().timestamp()
        ),
        name: format!("Reconnaissance Scan - {}", target),
        description: "Comprehensive reconnaissance scanning".to_string(),
        tools: vec![
            "nmap".to_string(),
            "masscan".to_string(),
            "theHarvester".to_string(),
            "dnsrecon".to_string(),
            "amass".to_string(),
            "subfinder".to_string(),
        ],
        target: target.to_string(),
        config: BatchConfig {
            mode: ExecutionMode::Parallel,
            max_parallel: 3,
            ..Default::default()
        },
        created_at: Utc::now(),
        status: JobStatus::Pending,
    }
}

pub fn create_vuln_scan_job(target: &str) -> BatchJob {
    BatchJob {
        job_id: format!(
            "vuln_{}_{}",
            target.replace(".", "_"),
            Utc::now().timestamp()
        ),
        name: format!("Vulnerability Scan - {}", target),
        description: "Comprehensive vulnerability assessment".to_string(),
        tools: vec![
            "nikto".to_string(),
            "nuclei".to_string(),
            "sqlmap".to_string(),
            "dirsearch".to_string(),
            "wpscan".to_string(),
        ],
        target: target.to_string(),
        config: BatchConfig {
            mode: ExecutionMode::Sequential,
            timeout_per_tool: Duration::from_secs(600),
            ..Default::default()
        },
        created_at: Utc::now(),
        status: JobStatus::Pending,
    }
}

pub fn create_password_attack_job(target: &str) -> BatchJob {
    BatchJob {
        job_id: format!(
            "passwd_{}_{}",
            target.replace(".", "_"),
            Utc::now().timestamp()
        ),
        name: format!("Password Attack Suite - {}", target),
        description: "Comprehensive password cracking and analysis".to_string(),
        tools: vec![
            "john".to_string(),
            "hashcat".to_string(),
            "hydra".to_string(),
            "medusa".to_string(),
            "ncrack".to_string(),
            "patator".to_string(),
        ],
        target: target.to_string(),
        config: BatchConfig {
            mode: ExecutionMode::Parallel,
            max_parallel: 2,
            timeout_per_tool: Duration::from_secs(1800),
            ..Default::default()
        },
        created_at: Utc::now(),
        status: JobStatus::Pending,
    }
}

pub fn create_full_pentest_job(target: &str) -> BatchJob {
    BatchJob {
        job_id: format!(
            "full_{}_{}",
            target.replace(".", "_"),
            Utc::now().timestamp()
        ),
        name: format!("Full Penetration Test - {}", target),
        description: "Complete penetration testing suite".to_string(),
        tools: vec![
            // Recon
            "nmap".to_string(),
            "theHarvester".to_string(),
            "amass".to_string(),
            // Scanning
            "nikto".to_string(),
            "nuclei".to_string(),
            // Exploitation
            "sqlmap".to_string(),
            "metasploit-framework".to_string(),
            // Password attacks
            "hydra".to_string(),
            "john".to_string(),
            // Wireless (if applicable)
            "aircrack-ng".to_string(),
            // Forensics
            "binwalk".to_string(),
        ],
        target: target.to_string(),
        config: BatchConfig {
            mode: ExecutionMode::Sequential,
            timeout_per_tool: Duration::from_secs(900),
            continue_on_failure: true,
            ..Default::default()
        },
        created_at: Utc::now(),
        status: JobStatus::Pending,
    }
}
