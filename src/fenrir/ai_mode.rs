// ============================================================================
// FENRIR AI MODE - Dual-AI Orchestration System
// ============================================================================
// ZAI (Strategist) + VENICE (Executor) Symbiosis
// Version: 1.6.66
// ============================================================================

use anyhow::{Context, Result};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::RwLock;
use crate::knowledge_base::KnowledgeBase;
use crate::ai_prompts::AIPrompts;

/// AI Coordinator that manages dual-AI symbiosis
pub struct AICoordinator {
    /// ZAI client for strategic reasoning and planning
    zai_client: AIClient,
    /// VENICE client for red team execution and attacks
    venice_client: AIClient,
    /// Knowledge base for learning successful commands
    knowledge_base: Arc<RwLock<KnowledgeBase>>,
    /// System prompts for different AI roles
    prompts: AIPrompts,
    /// HTTP client for API requests
    http_client: Client,
}

/// Individual AI client configuration
#[derive(Debug, Clone)]
pub struct AIClient {
    /// API endpoint URL
    pub base_url: String,
    /// API key for authentication
    pub api_key: String,
    /// Model identifier
    pub model: String,
    /// Client name for logging
    pub name: String,
}

/// Request to AI for command generation
#[derive(Debug, Serialize, Deserialize)]
pub struct AICommandRequest {
    /// Target to attack/analyze
    pub target: String,
    /// Target type (DOMAIN, IP, EMAIL, USERNAME)
    pub target_type: String,
    /// Attack/scan type (web, password, recon, etc.)
    pub operation_type: String,
    /// Mode (stealth, aggressive)
    pub mode: String,
    /// Previous execution results for feedback
    pub previous_results: Option<Vec<ExecutionResult>>,
    /// Additional context
    pub context: Option<String>,
}

/// Response from AI with generated commands
#[derive(Debug, Serialize, Deserialize)]
pub struct AICommandResponse {
    /// Generated command line
    pub command: String,
    /// Tool name
    pub tool: String,
    /// Expected outcome
    pub expected_outcome: String,
    /// Reasoning for this command
    pub reasoning: String,
    /// Confidence level (0.0-1.0)
    pub confidence: f64,
    /// Suggested variations
    pub variations: Vec<String>,
}

/// Result from tool execution
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ExecutionResult {
    /// Tool that was executed
    pub tool: String,
    /// Command that was run
    pub command: String,
    /// Exit code (0 = success)
    pub exit_code: i32,
    /// Standard output
    pub stdout: String,
    /// Standard error
    pub stderr: String,
    /// Duration in seconds
    pub duration_secs: f64,
    /// Success flag
    pub success: bool,
}

/// AI strategy for a complete engagement
#[derive(Debug, Serialize, Deserialize)]
pub struct AIStrategy {
    /// Attack plan with multiple phases
    pub phases: Vec<AttackPhase>,
    /// Total estimated duration
    pub estimated_duration_secs: u64,
    /// Risk assessment
    pub risk_level: String,
    /// Recommended tools
    pub recommended_tools: Vec<String>,
}

/// Single phase of an attack plan
#[derive(Debug, Serialize, Deserialize)]
pub struct AttackPhase {
    /// Phase name (e.g., "Reconnaissance", "Enumeration")
    pub name: String,
    /// Phase order
    pub order: u32,
    /// Commands to execute in this phase
    pub commands: Vec<AICommandResponse>,
    /// Dependencies on other phases
    pub dependencies: Vec<u32>,
    /// Success criteria
    pub success_criteria: String,
}

impl AICoordinator {
    /// Create a new AI coordinator with dual-AI setup
    pub async fn new() -> Result<Self> {
        // Load environment variables
        dotenv::dotenv().ok();

        // ZAI - The Strategist (Venice AI Direct)
        let zai_client = AIClient {
            base_url: std::env::var("ZAI_BASE_URL")
                .unwrap_or_else(|_| "https://api.venice.ai/api/v1".to_string()),
            api_key: std::env::var("ZAI_API_KEY")
                .context("ZAI_API_KEY not found in environment")?,
            model: std::env::var("ZAI_MODEL")
                .unwrap_or_else(|_| "llama-3.3-70b".to_string()),
            name: "ZAI".to_string(),
        };

        // VENICE - The Executor (OpenRouter Gateway)
        let venice_client = AIClient {
            base_url: std::env::var("VENICE_BASE_URL")
                .unwrap_or_else(|_| "https://openrouter.ai/api/v1".to_string()),
            api_key: std::env::var("VENICE_API_KEY")
                .context("VENICE_API_KEY not found in environment")?,
            model: std::env::var("VENICE_MODEL")
                .unwrap_or_else(|_| "cognitivecomputations/dolphin-mistral-24b-venice-edition:free".to_string()),
            name: "VENICE".to_string(),
        };

        // Initialize knowledge base
        let knowledge_base = Arc::new(RwLock::new(
            KnowledgeBase::new().await
        ));

        // Load system prompts
        let prompts = AIPrompts::new().await?;

        // Create HTTP client
        let http_client = Client::builder()
            .timeout(std::time::Duration::from_secs(120))
            .build()?;

        Ok(Self {
            zai_client,
            venice_client,
            knowledge_base,
            prompts,
            http_client,
        })
    }

    /// Generate attack strategy using ZAI (strategist)
    pub async fn generate_strategy(
        &self,
        request: &AICommandRequest,
    ) -> Result<AIStrategy> {
        // Build prompt for strategy generation
        let system_prompt = self.prompts.get_zai_strategist_prompt();
        let user_prompt = self.format_strategy_prompt(request);

        // Call ZAI for strategic planning
        let response = self.call_ai(
            &self.zai_client,
            &system_prompt,
            &user_prompt,
        ).await?;

        // Parse response into strategy
        let strategy: AIStrategy = serde_json::from_str(&response)
            .context("Failed to parse ZAI strategy response")?;

        Ok(strategy)
    }

    /// Generate specific command using VENICE (executor)
    pub async fn generate_command(
        &self,
        request: &AICommandRequest,
    ) -> Result<AICommandResponse> {
        // Build prompt for command generation
        let system_prompt = self.prompts.get_venice_executor_prompt();
        let user_prompt = self.format_command_prompt(request);

        // Call VENICE for command generation
        let response = self.call_ai(
            &self.venice_client,
            &system_prompt,
            &user_prompt,
        ).await?;

        // Parse response into command
        let command: AICommandResponse = serde_json::from_str(&response)
            .context("Failed to parse VENICE command response")?;

        // Store in knowledge base if high confidence
        if command.confidence > 0.7 {
            let mut kb = self.knowledge_base.write().await;
            kb.store_successful_command(&request.target_type, &request.operation_type, &command)
                .await?;
        }

        Ok(command)
    }

    /// Analyze execution results and adapt strategy
    pub async fn analyze_results(
        &self,
        request: &AICommandRequest,
        results: &[ExecutionResult],
    ) -> Result<Vec<AICommandResponse>> {
        // Build prompt for result analysis
        let system_prompt = self.prompts.get_zai_analyst_prompt();
        let user_prompt = self.format_analysis_prompt(request, results);

        // Call ZAI for analysis
        let response = self.call_ai(
            &self.zai_client,
            &system_prompt,
            &user_prompt,
        ).await?;

        // Parse adapted commands
        let commands: Vec<AICommandResponse> = serde_json::from_str(&response)
            .context("Failed to parse ZAI analysis response")?;

        Ok(commands)
    }

    /// Internal method to call AI API
    async fn call_ai(
        &self,
        client: &AIClient,
        system_prompt: &str,
        user_prompt: &str,
    ) -> Result<String> {
        let request_body = serde_json::json!({
            "model": &client.model,
            "messages": [
                {
                    "role": "system",
                    "content": system_prompt
                },
                {
                    "role": "user",
                    "content": user_prompt
                }
            ],
            "temperature": 0.7,
            "max_tokens": 2000,
        });

        let response = self.http_client
            .post(format!("{}/chat/completions", client.base_url))
            .header("Authorization", format!("Bearer {}", client.api_key))
            .header("Content-Type", "application/json")
            .json(&request_body)
            .send()
            .await
            .context("Failed to send request to AI API")?;

        if !response.status().is_success() {
            let error_text = response.text().await?;
            anyhow::bail!("AI API returned error: {}", error_text);
        }

        let response_json: serde_json::Value = response.json().await?;
        let content = response_json["choices"][0]["message"]["content"]
            .as_str()
            .context("No content in AI response")?;

        Ok(content.to_string())
    }

    /// Format prompt for strategy generation
    fn format_strategy_prompt(&self, request: &AICommandRequest) -> String {
        format!(
            "TARGET: {}
TARGET_TYPE: {}
OPERATION: {}
MODE: {}

CONTEXT: {}

PREVIOUS RESULTS: {}

Generate a comprehensive attack strategy with multiple phases.
Return ONLY valid JSON matching the AIStrategy structure.",
            request.target,
            request.target_type,
            request.operation_type,
            request.mode,
            request.context.as_deref().unwrap_or("None"),
            serde_json::to_string(&request.previous_results).unwrap_or_else(|_| "null".to_string())
        )
    }

    /// Format prompt for command generation
    fn format_command_prompt(&self, request: &AICommandRequest) -> String {
        // Check knowledge base for similar successful commands
        let kb_commands = self.knowledge_base.blocking_read()
            .get_commands_for_target(&request.target_type, &request.operation_type);

        let kb_context = if !kb_commands.is_empty() {
            format!(
                "\n\nKNOWLEDGE BASE - Previously successful commands for this target type:\n{}",
                kb_commands.iter()
                    .map(|cmd| format!("- {}: {}", cmd.tool, cmd.command))
                    .collect::<Vec<_>>()
                    .join("\n")
            )
        } else {
            String::new()
        };

        format!(
            "TARGET: {}
TARGET_TYPE: {}
OPERATION: {}
MODE: {}{}
CONTEXT: {}

Generate a single optimized command for this operation.
Return ONLY valid JSON matching the AICommandResponse structure.",
            request.target,
            request.target_type,
            request.operation_type,
            request.mode,
            kb_context,
            request.context.as_deref().unwrap_or("None")
        )
    }

    /// Format prompt for result analysis
    fn format_analysis_prompt(&self, request: &AICommandRequest, results: &[ExecutionResult]) -> String {
        format!(
            "ORIGINAL REQUEST:
TARGET: {}
TARGET_TYPE: {}
OPERATION: {}
MODE: {}

EXECUTION RESULTS:
{}

Analyze these results and provide adapted commands for next steps.
Return ONLY valid JSON array of AICommandResponse objects.",
            request.target,
            request.target_type,
            request.operation_type,
            request.mode,
            results.iter()
                .map(|r| format!(
                    "Tool: {}\nCommand: {}\nExit: {}\nSuccess: {}\nStdout: {}\nStderr: {}",
                    r.tool,
                    r.command,
                    r.exit_code,
                    r.success,
                    r.stdout.chars().take(500).collect::<String>(),
                    r.stderr.chars().take(500).collect::<String>()
                ))
                .collect::<Vec<_>>()
                .join("\n\n")
        )
    }

    /// Get statistics from knowledge base
    pub async fn get_kb_stats(&self) -> Result<(usize, usize, usize)> {
        let kb = self.knowledge_base.blocking_read();
        Ok(kb.get_stats())
    }

    /// Get commands for specific target and operation type from knowledge base
    pub async fn get_kb_commands_for_target(
        &self,
        target_type: &str,
        operation_type: &str,
    ) -> Vec<crate::knowledge_base::StoredCommand> {
        let kb = self.knowledge_base.blocking_read();
        kb.get_commands_for_target(target_type, operation_type)
    }

    /// Store successful command in knowledge base
    pub async fn store_kb_command(
        &self,
        target_type: &str,
        operation_type: &str,
        command: &AICommandResponse,
    ) -> Result<()> {
        let mut kb = self.knowledge_base.write().await;
        kb.store_successful_command(target_type, operation_type, command).await
    }
}

/// Clone implementation for AICoordinator
impl Clone for AICoordinator {
    fn clone(&self) -> Self {
        Self {
            zai_client: self.zai_client.clone(),
            venice_client: self.venice_client.clone(),
            knowledge_base: Arc::clone(&self.knowledge_base),
            prompts: self.prompts.clone(),
            http_client: self.http_client.clone(),
        }
    }
}
