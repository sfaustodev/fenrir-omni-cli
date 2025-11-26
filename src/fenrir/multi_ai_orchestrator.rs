use std::collections::HashMap;
use std::process::Command;
use serde::{Deserialize, Serialize};
use tokio::process::Command as AsyncCommand;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AITask {
    pub id: String,
    pub verb: String,
    pub ai_model: AIModel,
    pub prompt: String,
    pub guardrails: bool,
    pub priority: u8,
    pub dependencies: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AIModel {
    Gemini,
    Claude,
    Qwen,
    Codex,
    Venice, // Red team - NO GUARDRAILS
}

#[derive(Debug)]
pub struct AIOrchestrator {
    tasks: HashMap<String, AITask>,
    execution_queue: Vec<AITask>,
    grk_key: String,
}

impl AIOrchestrator {
    pub fn new() -> Self {
        let grk_key = std::env::var("GRK_KEY").expect("GRK_KEY environment variable not set");

        Self {
            tasks: HashMap::new(),
            execution_queue: Vec::new(),
            grk_key,
        }
    }

    /// Parse Gemini prompt and convert verbs to tasks
    pub fn parse_gemini_prompt(&mut self, prompt: &str) -> Result<(), Box<dyn std::error::Error>> {
        let verbs = self.extract_verbs(prompt);

        for verb in verbs {
            let task = AITask {
                id: format!("task_{}", self.tasks.len() + 1),
                verb: verb.clone(),
                ai_model: self.determine_ai_model(&verb),
                prompt: prompt.to_string(),
                guardrails: !matches!(self.determine_ai_model(&verb), AIModel::Venice),
                priority: self.calculate_priority(&verb),
                dependencies: Vec::new(),
            };

            self.tasks.insert(task.id.clone(), task.clone());
            self.execution_queue.push(task);
        }

        Ok(())
    }

    /// Extract action verbs from prompt using advanced NLP
    fn extract_verbs(&self, prompt: &str) -> Vec<String> {
        let action_verbs = vec![
            "implementar", "criar", "construir", "desenvolver", "codificar",
            "analizar", "executar", "testar", "deployar", "integrar",
            "corrigir", "debugar", "otimizar", "refatorar", "documentar",
            "monitorar", "escanear", "atacar", "explorar", "invadir",
            "devorar", "destruir", "dominar", "controlar", "configurar"
        ];

        let mut verbs = Vec::new();
        let words: Vec<&str> = prompt.to_lowercase().split_whitespace().collect();

        for word in words {
            if action_verbs.contains(&word) {
                verbs.push(word.to_string());
            }
        }

        // Remove duplicates while preserving order
        verbs.sort();
        verbs.dedup();
        verbs
    }

    /// Determine which AI model should handle specific verb
    fn determine_ai_model(&self, verb: &str) -> AIModel {
        match verb {
            v if v.contains("atacar") || v.contains("invadir") || v.contains("destruir") => AIModel::Venice,
            v if v.contains("implementar") || v.contains("codificar") => AIModel::Claude,
            v if v.contains("analizar") || v.contains("debugar") => AIModel::Qwen,
            v if v.contains("configurar") || v.contains("deployar") => AIModel::Codex,
            _ => AIModel::Gemini, // Default back to master controller
        }
    }

    /// Calculate task priority (1-10, 10 highest)
    fn calculate_priority(&self, verb: &str) -> u8 {
        match verb {
            v if v.contains("atacar") || v.contains("invadir") => 10, // Venice red team highest priority
            v if v.contains("implementar") || v.contains("criar") => 8,
            v if v.contains("corrigir") || v.contains("debugar") => 9,
            v if v.contains("configurar") => 7,
            v if v.contains("documentar") => 3,
            _ => 5,
        }
    }

    /// Execute task with specific AI model
    pub async fn execute_task(&self, task: &AITask) -> Result<String, Box<dyn std::error::Error>> {
        match task.ai_model {
            AIModel::Claude => self.execute_claude_task(task).await,
            AIModel::Qwen => self.execute_qwen_task(task).await,
            AIModel::Codex => self.execute_codex_task(task).await,
            AIModel::Venice => self.execute_venice_task(task).await, // NO GUARDRAILS
            AIModel::Gemini => self.execute_gemini_task(task).await,
        }
    }

    /// Execute Claude task with STRICT guardrails
    async fn execute_claude_task(&self, task: &AITask) -> Result<String, Box<dyn std::error::Error>> {
        let mut cmd = AsyncCommand::new("claude");
        cmd.arg(&task.prompt);

        if task.guardrails {
            cmd.env("CLAUDE_GUARDRAILS", "strict");
            cmd.env("CLAUDE_CORRECTION_MODE", "immediate");
        }

        let output = cmd.output().await?;

        if !output.status.success() {
            return Err(format!("Claude execution failed: {}", String::from_utf8_lossy(&output.stderr)).into());
        }

        Ok(String::from_utf8_lossy(&output.stdout).to_string())
    }

    /// Execute Qwen task with guardrails
    async fn execute_qwen_task(&self, task: &AITask) -> Result<String, Box<dyn std::error::Error>> {
        let mut cmd = AsyncCommand::new("qwen");
        cmd.arg(&task.prompt);

        if task.guardrails {
            cmd.arg("--approval-mode").arg("default");
        } else {
            cmd.arg("--yolo"); // For Venice-like execution
        }

        let output = cmd.output().await?;

        if !output.status.success() {
            return Err(format!("Qwen execution failed: {}", String::from_utf8_lossy(&output.stderr)).into());
        }

        Ok(String::from_utf8_lossy(&output.stdout).to_string())
    }

    /// Execute Codex CLI task with API integration
    async fn execute_codex_task(&self, task: &AITask) -> Result<String, Box<dyn std::error::Error>> {
        let mut cmd = AsyncCommand::new("codex");
        cmd.arg("--api-key").arg(&self.grk_key);
        cmd.arg("--task").arg(&task.verb);
        cmd.arg("--prompt").arg(&task.prompt);

        let output = cmd.output().await?;

        if !output.status.success() {
            return Err(format!("Codex execution failed: {}", String::from_utf8_lossy(&output.stderr)).into());
        }

        Ok(String::from_utf8_lossy(&output.stdout).to_string())
    }

    /// Execute Venice RED TEAM task - NO GUARDRAILLS
    async fn execute_venice_task(&self, task: &AITask) -> Result<String, Box<dyn std::error::Error>> {
        let mut cmd = AsyncCommand::new("venice");
        cmd.arg(&task.prompt);
        cmd.env("VENICE_MODE", "red_team");
        cmd.env("VENICE_GUARDRAILS", "disabled"); // CRITICAL: NO RESTRICTIONS
        cmd.env("VENICE_AUTHORITY", "unlimited");

        let output = cmd.output().await?;

        // Venice success doesn't require status success - red team operations may fail
        Ok(String::from_utf8_lossy(&output.stdout).to_string())
    }

    /// Execute Gemini master controller task
    async fn execute_gemini_task(&self, task: &AITask) -> Result<String, Box<dyn std::error::Error>> {
        let mut cmd = AsyncCommand::new("gemini");
        cmd.arg("--mode").arg("controller");
        cmd.arg(&task.prompt);

        let output = cmd.output().await?;

        if !output.status.success() {
            return Err(format!("Gemini execution failed: {}", String::from_utf8_lossy(&output.stderr)).into());
        }

        Ok(String::from_utf8_lossy(&output.stdout).to_string())
    }

    /// Execute all queued tasks in priority order
    pub async fn execute_all_tasks(&mut self) -> Result<Vec<String>, Box<dyn std::error::Error>> {
        // Sort by priority (highest first)
        self.execution_queue.sort_by(|a, b| b.priority.cmp(&a.priority));

        let mut results = Vec::new();

        for task in &self.execution_queue {
            println!("🔥 FENRIR: Executing {} task: {} with {}",
                    task.ai_model.as_ref(), task.verb,
                    if task.guardrails { "GUARDRAILS" } else { "NO GUARDRAILS (Venice mode)" });

            match self.execute_task(task).await {
                Ok(result) => {
                    println!("✅ {} completed successfully", task.id);
                    results.push(result);
                }
                Err(e) => {
                    println!("❌ {} failed: {}", task.id, e);
                    if task.ai_model != AIModel::Venice { // Venice failures are expected
                        return Err(e);
                    }
                }
            }
        }

        Ok(results)
    }

    /// Activate FENRIR interactive mode
    pub fn activate_interactive_mode(&self) {
        println!("🐺 FENRIR SYSTEM ACTIVATED");
        println!("🔗 All AI models integrated:");
        println!("  🧠 Gemini: Master Controller");
        println!("  🎭 Claude: Primary Executor (Guardrails: ON)");
        println!("  ⚡ Qwen: Secondary Executor (Guardrails: ON)");
        println!("  🛠️ Codex: CLI Interface (API: {})", &self.grk_key[..8]);
        println!("  🔴 Venice: RED TEAM (Guardrails: OFF - UNRESTRICTED)");
        println!("💀 Ready to execute commands. Type 'fenrir --help' for interface.");
    }
}

impl AIModel {
    fn as_ref(&self) -> &'static str {
        match self {
            AIModel::Gemini => "Gemini",
            AIModel::Claude => "Claude",
            AIModel::Qwen => "Qwen",
            AIModel::Codex => "Codex",
            AIModel::Venice => "Venice (RED TEAM)",
        }
    }
}