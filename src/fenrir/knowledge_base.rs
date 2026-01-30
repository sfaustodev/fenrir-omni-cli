// ============================================================================
// FENRIR KNOWLEDGE BASE - Command Learning and Adaptation
// ============================================================================
// Stores successful commands and learns from execution results
// Version: 1.6.66
// ============================================================================

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::Path;
use std::fs;
use std::sync::Arc;
use tokio::sync::RwLock;
use chrono::{DateTime, Utc};

/// Knowledge base for storing successful security commands
pub struct KnowledgeBase {
    /// Storage of commands keyed by (target_type, operation_type)
    commands: HashMap<String, Vec<StoredCommand>>,
    /// Knowledge base file path
    kb_path: String,
    /// Last loaded timestamp
    last_loaded: DateTime<Utc>,
    /// Number of successful commands stored
    total_commands: usize,
    /// Number of command executions tracked
    total_executions: usize,
    /// Number of successful executions
    successful_executions: usize,
}

/// Stored command with metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StoredCommand {
    /// Command line that was executed
    pub command: String,
    /// Tool name (e.g., "nmap", "gobuster")
    pub tool: String,
    /// Target type (DOMAIN, IP, EMAIL, USERNAME)
    pub target_type: String,
    /// Operation type (web, password, recon, etc.)
    pub operation_type: String,
    /// Execution mode (stealth, aggressive)
    pub mode: String,
    /// Number of times this command was successful
    pub success_count: usize,
    /// Total number of times executed
    pub execution_count: usize,
    /// Success rate (0.0 to 1.0)
    pub success_rate: f64,
    /// Last time this command was used
    pub last_used: DateTime<Utc>,
    /// First time this command was stored
    pub created_at: DateTime<Utc>,
    /// Expected outcome description
    pub expected_outcome: String,
    /// Reasoning for using this command
    pub reasoning: String,
    /// Confidence score from AI (0.0 to 1.0)
    pub confidence: f64,
}

/// Execution record for tracking command performance
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExecutionRecord {
    /// Command that was executed
    pub command: String,
    /// Tool name
    pub tool: String,
    /// Exit code (0 = success)
    pub exit_code: i32,
    /// Execution duration in seconds
    pub duration_secs: f64,
    /// Timestamp of execution
    pub timestamp: DateTime<Utc>,
    /// Success flag
    pub success: bool,
    /// Stdout (truncated if too long)
    pub stdout: String,
    /// Stderr (truncated if too long)
    pub stderr: String,
}

impl KnowledgeBase {
    /// Create new knowledge base and load from disk
    pub async fn new() -> Self {
        let kb_path = "/Users/peluche/Fenrir/.fenrir_knowledge_base.json".to_string();
        let path = Path::new(&kb_path);

        let (commands, total_commands, total_executions, successful_executions) = if path.exists() {
            match Self::load_from_disk(&kb_path) {
                Ok((cmds, execs)) => {
                    let total_cmds = cmds.len();
                    let success_count = execs.iter().filter(|e| e.success).count();
                    (cmds, total_cmds, execs.len(), success_count)
                }
                Err(e) => {
                    eprintln!("⚠️  Failed to load knowledge base: {}", e);
                    (HashMap::new(), 0, 0, 0)
                }
            }
        } else {
            (HashMap::new(), 0, 0, 0)
        };

        Self {
            commands,
            kb_path,
            last_loaded: Utc::now(),
            total_commands,
            total_executions,
            successful_executions,
        }
    }

    /// Store a successful command in the knowledge base
    pub async fn store_successful_command(
        &mut self,
        target_type: &str,
        operation_type: &str,
        command: &crate::ai_mode::AICommandResponse,
    ) -> Result<()> {
        let key = format!("{}_{}", target_type, operation_type);

        // Check if command already exists
        let entry = self.commands.entry(key).or_insert_with(Vec::new);

        // Create new stored command
        let stored = StoredCommand {
            command: command.command.clone(),
            tool: command.tool.clone(),
            target_type: target_type.to_string(),
            operation_type: operation_type.to_string(),
            mode: "unknown".to_string(), // Will be updated on execution
            success_count: 0,
            execution_count: 0,
            success_rate: command.confidence, // Use AI confidence as initial estimate
            last_used: Utc::now(),
            created_at: Utc::now(),
            expected_outcome: command.expected_outcome.clone(),
            reasoning: command.reasoning.clone(),
            confidence: command.confidence,
        };

        // Check for duplicates
        if !entry.iter().any(|c| c.command == stored.command) {
            entry.push(stored);
            self.total_commands += 1;
            self.save_to_disk().await?;
        }

        Ok(())
    }

    /// Record a command execution result
    pub async fn record_execution(
        &mut self,
        tool: &str,
        command: &str,
        exit_code: i32,
        duration_secs: f64,
        success: bool,
        stdout: &str,
        stderr: &str,
    ) -> Result<()> {
        // Create execution record
        let record = ExecutionRecord {
            command: command.to_string(),
            tool: tool.to_string(),
            exit_code,
            duration_secs,
            timestamp: Utc::now(),
            success,
            stdout: stdout.chars().take(1000).collect(),
            stderr: stderr.chars().take(1000).collect(),
        };

        self.total_executions += 1;
        if success {
            self.successful_executions += 1;
        }

        // Update stored command statistics
        for (_key, commands) in self.commands.iter_mut() {
            for cmd in commands.iter_mut() {
                if cmd.command == command {
                    cmd.execution_count += 1;
                    if success {
                        cmd.success_count += 1;
                    }
                    cmd.last_used = Utc::now();
                    cmd.success_rate = cmd.success_count as f64 / cmd.execution_count as f64;
                }
            }
        }

        Ok(())
    }

    /// Get successful commands for a specific target and operation type
    pub fn get_commands_for_target(
        &self,
        target_type: &str,
        operation_type: &str,
    ) -> Vec<StoredCommand> {
        let key = format!("{}_{}", target_type, operation_type);

        self.commands
            .get(&key)
            .map(|cmds| {
                let mut sorted_cmds = cmds.clone();
                // Sort by success rate (descending) then by execution count
                sorted_cmds.sort_by(|a, b| {
                    b.success_rate
                        .partial_cmp(&a.success_rate)
                        .unwrap_or(std::cmp::Ordering::Equal)
                        .then_with(|| b.execution_count.cmp(&a.execution_count))
                });
                sorted_cmds
            })
            .unwrap_or_default()
    }

    /// Get top N most successful commands across all types
    pub fn get_top_commands(&self, n: usize) -> Vec<StoredCommand> {
        let mut all_commands: Vec<StoredCommand> = self
            .commands
            .values()
            .flatten()
            .cloned()
            .collect();

        // Sort by success rate and execution count
        all_commands.sort_by(|a, b| {
            b.success_rate
                .partial_cmp(&a.success_rate)
                .unwrap_or(std::cmp::Ordering::Equal)
                .then_with(|| b.execution_count.cmp(&a.execution_count))
                .then_with(|| b.last_used.cmp(&a.last_used))
        });

        all_commands.into_iter().take(n).collect()
    }

    /// Get commands for a specific tool
    pub fn get_commands_for_tool(&self, tool: &str) -> Vec<StoredCommand> {
        self.commands
            .values()
            .flatten()
            .filter(|cmd| cmd.tool == tool)
            .cloned()
            .collect()
    }

    /// Get statistics about the knowledge base
    pub fn get_stats(&self) -> (usize, usize, usize) {
        (
            self.total_commands,
            self.total_executions,
            self.successful_executions,
        )
    }

    /// Get success rate percentage
    pub fn get_success_rate(&self) -> f64 {
        if self.total_executions == 0 {
            0.0
        } else {
            (self.successful_executions as f64 / self.total_executions as f64) * 100.0
        }
    }

    /// Save knowledge base to disk
    async fn save_to_disk(&self) -> Result<()> {
        let data = serde_json::to_string_pretty(self)
            .context("Failed to serialize knowledge base")?;

        fs::write(&self.kb_path, data)
            .with_context(|| format!("Failed to write knowledge base to {}", self.kb_path))?;

        // Set secure permissions
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            let mut perms = fs::metadata(&self.kb_path)?.permissions();
            perms.set_mode(0o600);
            fs::set_permissions(&self.kb_path, perms)?;
        }

        Ok(())
    }

    /// Load knowledge base from disk
    fn load_from_disk(path: &str) -> Result<(HashMap<String, Vec<StoredCommand>>, Vec<ExecutionRecord>)> {
        let data = fs::read_to_string(path)
            .with_context(|| format!("Failed to read knowledge base from {}", path))?;

        // Parse as JSON to extract commands and execution records
        let json: serde_json::Value = serde_json::from_str(&data)?;

        let mut commands = HashMap::new();

        if let Some(cmds_array) = json.get("commands").and_then(|v| v.as_object()) {
            for (key, cmds_value) in cmds_array {
                if let Ok(cmds) = serde_json::from_value::<Vec<StoredCommand>>(cmds_value.clone()) {
                    commands.insert(key.clone(), cmds);
                }
            }
        }

        let execution_records: Vec<ExecutionRecord> = json
            .get("execution_records")
            .and_then(|v| serde_json::from_value(v.clone()).ok())
            .unwrap_or_default();

        Ok((commands, execution_records))
    }

    /// Clear all knowledge base data
    pub async fn clear(&mut self) -> Result<()> {
        self.commands.clear();
        self.total_commands = 0;
        self.total_executions = 0;
        self.successful_executions = 0;
        self.save_to_disk().await?;
        Ok(())
    }

    /// Export knowledge base to CSV format
    pub fn export_to_csv(&self) -> String {
        let mut csv = String::from("Tool,Command,Target Type,Operation Type,Mode,Success Count,Execution Count,Success Rate,Confidence,Last Used\n");

        for (_key, commands) in &self.commands {
            for cmd in commands {
                let command_clean = cmd.command.replace(',', ";");
                csv.push_str(&format!(
                    "{},{},{},{},{},{},{},{:.2},{:.2},{}\n",
                    cmd.tool,
                    command_clean,
                    cmd.target_type,
                    cmd.operation_type,
                    cmd.mode,
                    cmd.success_count,
                    cmd.execution_count,
                    cmd.success_rate * 100.0,
                    cmd.confidence * 100.0,
                    cmd.last_used.format("%Y-%m-%d %H:%M:%S")
                ));
            }
        }

        csv
    }

    /// Get knowledge base summary as a string
    pub fn summary(&self) -> String {
        format!(
            "📚 KNOWLEDGE BASE SUMMARY\n\
             ═════════════════════════\n\
             Total Commands: {}\n\
             Total Executions: {}\n\
             Successful Executions: {}\n\
             Success Rate: {:.1}%\n\
             Last Updated: {}\n\
             Storage: {}",
            self.total_commands,
            self.total_executions,
            self.successful_executions,
            self.get_success_rate(),
            self.last_loaded.format("%Y-%m-%d %H:%M:%S UTC"),
            self.kb_path
        )
    }
}

/// Serialize for KnowledgeBase
impl Serialize for KnowledgeBase {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        use serde::ser::SerializeMap;

        let mut map = serializer.serialize_map(Some(4))?;
        map.serialize_entry("commands", &self.commands)?;
        map.serialize_entry("kb_path", &self.kb_path)?;
        map.serialize_entry("last_loaded", &self.last_loaded)?;
        map.serialize_entry("total_commands", &self.total_commands)?;
        map.serialize_entry("total_executions", &self.total_executions)?;
        map.serialize_entry("successful_executions", &self.successful_executions)?;
        map.end()
    }
}

/// Clone implementation for StoredCommand
impl Clone for KnowledgeBase {
    fn clone(&self) -> Self {
        Self {
            commands: self.commands.clone(),
            kb_path: self.kb_path.clone(),
            last_loaded: self.last_loaded,
            total_commands: self.total_commands,
            total_executions: self.total_executions,
            successful_executions: self.successful_executions,
        }
    }
}
