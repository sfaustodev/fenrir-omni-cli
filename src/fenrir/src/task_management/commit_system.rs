// 🔥 COMMIT SYSTEM - Um commit por tarefinha
// Cada commit é sagrado e individual

use crate::task_management::{task::TarefaFinha, team_profiles::DeveloperProfile};
use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::process::Command;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommitInfo {
    pub hash: String,
    pub author: String,
    pub message: String,
    pub timestamp: u64,
    pub files_changed: Vec<String>,
    pub lines_added: u32,
    pub lines_removed: u32,
    pub tarefinha_id: String,
    pub review_status: ReviewStatus,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ReviewStatus {
    Pending,
    Approved,
    Rejected,
    NeedsChanges,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommitQueue {
    pub pending_commits: Vec<CommitInfo>,
    pub processed_commits: Vec<CommitInfo>,
    pub auto_commit_enabled: bool,
}

impl CommitQueue {
    pub fn new() -> Self {
        Self {
            pending_commits: vec![],
            processed_commits: vec![],
            auto_commit_enabled: true,
        }
    }

    pub fn add_commit(&mut self, mut commit: CommitInfo) {
        commit.timestamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();

        self.pending_commits.push(commit);
    }

    pub fn process_next_commit(&mut self) -> Result<Option<CommitInfo>> {
        if let Some(commit) = self.pending_commits.pop() {
            let commit_result = self.execute_commit(&commit)?;

            if let Some(hash) = commit_result {
                let mut processed_commit = commit.clone();
                processed_commit.hash = hash.clone();
                self.processed_commits.push(processed_commit.clone());
                Ok(Some(processed_commit))
            } else {
                Ok(None)
            }
        } else {
            Ok(None)
        }
    }

    fn execute_commit(&self, commit: &CommitInfo) -> Result<Option<String>> {
        if !self.auto_commit_enabled {
            println!("🔧 Auto-commit disabled. Commit em modo dry-run:");
            println!("   {}", commit.message);
            return Ok(None);
        }

        println!("🔥 Executando commit: {}", commit.message);

        // Stage todos os arquivos modificados
        let output = Command::new("git")
            .args(&["add", "."])
            .output()
            .context("Failed to git add")?;

        if !output.status.success() {
            return Err(anyhow::anyhow!(
                "Git add failed: {}",
                String::from_utf8_lossy(&output.stderr)
            ));
        }

        // Executa o commit
        let output = Command::new("git")
            .args(&["commit", "-m", &commit.message])
            .output()
            .context("Failed to git commit")?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);

            // Verifica se é "nothing to commit"
            if stderr.contains("nothing to commit") {
                println!("ℹ️ Nada para commitar");
                return Ok(None);
            } else {
                return Err(anyhow::anyhow!("Git commit failed: {}", stderr));
            }
        }

        // Pega o hash do commit
        let output = Command::new("git")
            .args(&["rev-parse", "HEAD"])
            .output()
            .context("Failed to get commit hash")?;

        if output.status.success() {
            let hash = String::from_utf8_lossy(&output.stdout).trim().to_string();
            println!("✅ Commit criado: {}", hash);
            Ok(Some(hash))
        } else {
            Ok(None)
        }
    }

    pub fn process_all_pending(&mut self) -> Result<Vec<CommitInfo>> {
        let mut processed = vec![];

        while !self.pending_commits.is_empty() {
            if let Some(commit) = self.process_next_commit()? {
                processed.push(commit);
            } else {
                break;
            }
        }

        Ok(processed)
    }

    pub fn get_pending_count(&self) -> usize {
        self.pending_commits.len()
    }

    pub fn get_processed_count(&self) -> usize {
        self.processed_commits.len()
    }

    pub fn clear_pending(&mut self) {
        self.pending_commits.clear();
    }
}

impl CommitInfo {
    pub fn from_tarefinha(tarefinha: &TarefaFinha, developer: &DeveloperProfile) -> Self {
        let message = developer.get_commit_signature(tarefinha);

        Self {
            hash: String::new(), // Será preenchido após o commit
            author: developer.nickname.clone(),
            message,
            timestamp: 0, // Será preenchido após o commit
            files_changed: vec![],
            lines_added: 0,
            lines_removed: 0,
            tarefinha_id: tarefinha.id.clone(),
            review_status: ReviewStatus::Pending,
        }
    }

    pub fn set_review_status(&mut self, status: ReviewStatus) {
        self.review_status = status;
    }

    pub fn is_approved(&self) -> bool {
        matches!(self.review_status, ReviewStatus::Approved)
    }

    pub fn get_summary(&self) -> String {
        format!(
            "📦 Commit: {}...{}\n\
            👤 Author: {}\n\
            💬 Message: {}\n\
            📋 TarefaFinha: {}\n\
            ✅ Status: {:?}\n\
            📁 Files: {} changed",
            &self.hash[..self.hash.len().min(8)],
            if self.hash.len() > 8 { "..." } else { "" },
            self.author,
            self.message.lines().next().unwrap_or(""),
            self.tarefinha_id.split('_').last().unwrap_or("unknown"),
            self.review_status,
            self.files_changed.len()
        )
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommitMetrics {
    pub total_commits: usize,
    pub commits_by_developer: std::collections::HashMap<String, usize>,
    pub success_rate: f32,
    pub average_commit_size: f32,
    pub commit_frequency: f32,
}

impl CommitMetrics {
    pub fn from_commits(commits: &[CommitInfo]) -> Self {
        let mut commits_by_dev = std::collections::HashMap::new();
        let mut approved_count = 0;
        let mut total_size = 0u32;

        for commit in commits {
            *commits_by_dev.entry(commit.author.clone()).or_insert(0) += 1;

            if commit.is_approved() {
                approved_count += 1;
            }

            total_size += commit.lines_added + commit.lines_removed;
        }

        let success_rate = if commits.is_empty() {
            0.0
        } else {
            approved_count as f32 / commits.len() as f32
        };

        let average_commit_size = if commits.is_empty() {
            0.0
        } else {
            total_size as f32 / commits.len() as f32
        };

        Self {
            total_commits: commits.len(),
            commits_by_developer: commits_by_dev,
            success_rate,
            average_commit_size,
            commit_frequency: commits.len() as f32 / 24.0, // Assumindo 24h
        }
    }

    pub fn show_dashboard(&self) {
        println!("\n📊 COMMIT METRICS DASHBOARD");
        println!("═══════════════════════════════════");
        println!("🔥 Total Commits: {}", self.total_commits);
        println!("✅ Success Rate: {:.1}%", self.success_rate * 100.0);
        println!("📏 Avg Size: {:.1} lines", self.average_commit_size);
        println!("⚡ Frequency: {:.1} commits/hour", self.commit_frequency);

        println!("\n👥 By Developer:");
        for (dev, count) in &self.commits_by_developer {
            println!("   {}: {} commits", dev, count);
        }
    }
}
