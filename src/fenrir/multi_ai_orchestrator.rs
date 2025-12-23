use std::collections::HashMap;
use std::process::Command;
use serde::{Deserialize, Serialize};
use tokio::process::Command as AsyncCommand;
use crate::oraculo; // Import oraculo module

const GEMINI_MODEL: &str = "gemini-3.0-pro-preview";

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

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum AIModel {
    Gemini,
    Claude,
    Qwen,
    Codex,
    Venice, // Red team - NO GUARDRAILS
    Grok,   // New Grok model
}

#[derive(Debug)]
pub struct AIOrchestrator {
    tasks: HashMap<String, AITask>,
    execution_queue: Vec<AITask>,
    grk_key: String,
    xai_api_key: String,
    gemini_api_key: String,
}

impl AIOrchestrator {
    pub fn new() -> Self {
        let grk_key = std::env::var("GRK_KEY").unwrap_or_else(|_| "mock_grk_key".to_string());
        let xai_api_key = std::env::var("XAI_API_KEY").unwrap_or_else(|_| "".to_string());
        let gemini_api_key = std::env::var("api_key").unwrap_or_else(|_| "".to_string());

        Self {
            tasks: HashMap::new(),
            execution_queue: Vec::new(),
            grk_key,
            xai_api_key,
            gemini_api_key,
        }
    }

    /// Parse prompt using AI Planning (Oracle -> Grok Fallback)
    pub async fn parse_gemini_prompt(&mut self, prompt: &str) -> Result<(), Box<dyn std::error::Error>> {
        // No more word-by-word parsing. We use the AI Chain.
        println!("🧠 Invoking AI Oracle to plan tasks for: '{}'", prompt);
        
        let plan_json = match oraculo::get_execution_plan(prompt, &self.gemini_api_key).await {
            Ok(plan) => plan,
            Err(e) => {
                eprintln!("⚠️ Oracle (Gemini) failed: {}. Falling back to Grok...", e);
                oraculo::get_grok_plan(prompt, &self.xai_api_key).await?
            }
        };

        // Parse the JSON plan into tasks
        // Assuming the plan comes as a list of task objects
        let tasks: Vec<AITaskDTO> = serde_json::from_str(&plan_json)?;

        for (index, task_dto) in tasks.into_iter().enumerate() {
             let task = AITask {
                id: format!("task_{}_{}", self.tasks.len() + 1, index),
                verb: task_dto.verb.clone(),
                ai_model: task_dto.ai_model,
                prompt: task_dto.prompt,
                guardrails: task_dto.guardrails,
                priority: task_dto.priority,
                dependencies: task_dto.dependencies,
            };

            self.tasks.insert(task.id.clone(), task.clone());
            self.execution_queue.push(task);
        }

        Ok(())
    }

    /// Execute task with specific AI model
    pub async fn execute_task(&self, task: &AITask) -> Result<String, Box<dyn std::error::Error>> {
        match task.ai_model {
            AIModel::Claude => self.execute_claude_task(task).await,
            AIModel::Qwen => self.execute_qwen_task(task).await,
            AIModel::Codex => self.execute_codex_task(task).await,
            AIModel::Venice => self.execute_venice_task(task).await, // NO GUARDRAILS
            AIModel::Gemini => self.execute_gemini_task(task).await,
            AIModel::Grok => self.execute_grok_task(task).await,
        }
    }

    /// Execute Grok task
    async fn execute_grok_task(&self, task: &AITask) -> Result<String, Box<dyn std::error::Error>> {
        // Call Grok API via oraculo or CLI if available
        // For now, we use the API integration in oraculo
        oraculo::execute_grok_command(&task.prompt, &self.xai_api_key).await
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
        cmd.arg("--model").arg(GEMINI_MODEL);
        cmd.arg(&task.prompt);

        let output = cmd.output().await?;

        if !output.status.success() {
            eprintln!(
                "⚠️ Gemini failed once ({}). Replacing with Qwen for task {}",
                String::from_utf8_lossy(&output.stderr),
                task.id
            );
            return self.execute_qwen_task(task).await;
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
        println!("  🚀 Grok: Fallback Oracle (API: {})", if self.xai_api_key.is_empty() { "MISSING" } else { "CONFIGURED" });
        println!("  🔴 Venice: RED TEAM (Guardrails: OFF - UNRESTRICTED)");
        println!("💀 Ready to execute commands. Type 'exit' to quit.");
    }
}

// DTO for JSON parsing
#[derive(Debug, Deserialize)]
struct AITaskDTO {
    verb: String,
    ai_model: AIModel,
    prompt: String,
    guardrails: bool,
    priority: u8,
    dependencies: Vec<String>,
}

impl AIModel {
    fn as_ref(&self) -> &'static str {
        match self {
            AIModel::Gemini => "Gemini",
            AIModel::Claude => "Claude",
            AIModel::Qwen => "Qwen",
            AIModel::Codex => "Codex",
            AIModel::Venice => "Venice (RED TEAM)",
            AIModel::Grok => "Grok",
        }
    }
}
