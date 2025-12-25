// --- FENRIR MULTI-AI ORCHESTRATOR ---
// GLM 4.7 (Claude Code) = The Brain
// Coordinates all AI operations

use crate::fenrir_ai_layer::{
    FenrirTask, TaskType, AIProvider, ExecutionMode, TaskStatus,
    AIRequest, AIResponse, call_ai
};
use std::time::Instant;
use tokio::join;

// ============================================================================
// ORCHESTRATOR - THE BRAIN
// ============================================================================

pub struct FenrirOrchestrator {
    pub god_mode: bool,
    pub auto_confirm: bool,
}

impl FenrirOrchestrator {
    pub fn new() -> Self {
        let god_mode = std::env::var("FENRIR_MODE")
            .unwrap_or_else(|_| String::from("normal"))
            == "godmode";

        FenrirOrchestrator {
            god_mode,
            auto_confirm: false,
        }
    }

    // ------------------------------------------------------------------------
    // MAIN ENTRY POINT - Receive from Gemini, orchestrate execution
    // ------------------------------------------------------------------------

    pub async fn process_input(&self, user_input: String) -> String {
        println!("🐺 Fenrir Orchestrator activated...");
        println!("📥 Input: {}", user_input);

        // Step 0: Read FENRIR_MCP.md as system prompt
        let system_prompt = match std::fs::read_to_string("FENRIR_MCP.md") {
            Ok(content) => {
                println!("📚 FENRIR_MCP.md loaded ({:.1} KB)", content.len() as f64 / 1024.0);
                content
            }
            Err(e) => {
                println!("⚠️  Could not read FENRIR_MCP.md: {}", e);
                String::from("You are Fenrir MCP 2.0 orchestrator. Coordinate AI tasks.")
            }
        };

        // Step 1: Call Gemini for translation
        println!("🔄 Calling Gemini for translation...");
        let translated = self.translate_input_with_context(user_input.clone(), &system_prompt).await;
        println!("✅ Gemini: {}", translated);

        // Step 2: Create task structure
        let mut task = self.create_task(user_input.clone(), translated).await;

        // Step 3: Analyze complexity
        let complexity = self.analyze_complexity(&task);
        println!("🧠 Complexity: {:?}", complexity);

        // Step 4: Split if complex
        if matches!(complexity, Complexity::Complex) {
            println!("✂️  Splitting complex task...");
            task = self.split_into_subtasks(task).await;
        }

        // Step 5: Execute
        println!("⚡ Executing task...");
        let result = self.execute_task(task).await;

        // Step 6: Format output
        self.format_output(result)
    }

    // ------------------------------------------------------------------------
    // STEP 1: TRANSLATION (GEMINI)
    // ------------------------------------------------------------------------

    async fn translate_input_with_context(&self, input: String, system_prompt: &str) -> String {
        let request = AIRequest {
            provider: AIProvider::GeminiTranslator,
            system_prompt: format!(
                "{}\n\n{}\n\n",
                system_prompt,
                "TRANSLATE INPUT INTO COMMANDS AND SEND BOTH INPUT AND TRANSLATION TO FENRIR ORQ (GLM 4.7). \
                You are the translation layer. Convert Portuguese/English input into terminal commands. \
                Return ONLY the translated command, no explanation."
            ),
            user_message: input,
            max_tokens: Some(500),
            temperature: Some(0.3),
        };

        match call_ai(request).await {
            response if response.success => response.content,
            _ => String::from("Translation failed - using local fallback")
        }
    }

    async fn translate_input(&self, input: String) -> String {
        self.translate_input_with_context(input, "Default context").await
    }

    // ------------------------------------------------------------------------
    // STEP 2: CREATE TASK STRUCTURE
    // ------------------------------------------------------------------------

    async fn create_task(&self, user_input: String, translated: String) -> FenrirTask {
        let task_id = format!("task-{}", uuid::Uuid::new_v4().to_string().split_off(8));

        // Determine task type
        let task_type = self.detect_task_type(&translated);

        // Assign AI provider
        let assigned_ai = self.assign_ai(&task_type);

        FenrirTask {
            id: task_id,
            priority: 5,
            description: format!("Execute: {}", user_input),
            user_input,
            translated_command: translated,
            task_type,
            assigned_ai,
            subtasks: Vec::new(),
            execution_mode: ExecutionMode::Sequential,
            required_capabilities: Vec::new(),
            status: TaskStatus::Pending,
            result: None,
        }
    }

    fn detect_task_type(&self, command: &str) -> TaskType {
        let cmd_lower = command.to_lowercase();

        // Check for bite/morder commands
        if cmd_lower.starts_with("bite ") || cmd_lower.starts_with("morder ") {
            return TaskType::Pentest;
        }

        // Check for scan command
        if cmd_lower.starts_with("scan ") {
            return TaskType::SecurityScan;
        }

        // Check for Kali tool usage
        if cmd_lower.contains("nmap") || cmd_lower.contains("nikto") ||
           cmd_lower.contains("sqlmap") || cmd_lower.contains("metasploit") ||
           cmd_lower.contains("john") || cmd_lower.contains("hashcat") {
            return TaskType::Pentest;
        }

        if cmd_lower.contains("pentest") || cmd_lower.contains("exploit") {
            return TaskType::Pentest;
        }
        if cmd_lower.contains("malware") || cmd_lower.contains("virus") {
            return TaskType::MalwareAnalysis;
        }
        if cmd_lower.contains("recon") || cmd_lower.contains("enum") {
            return TaskType::NetworkRecon;
        }
        if cmd_lower.contains("code") || cmd_lower.contains("generate") {
            return TaskType::GenerateCode;
        }

        TaskType::ExecuteCommand
    }

    fn assign_ai(&self, task_type: &TaskType) -> AIProvider {
        match task_type {
            TaskType::ExecuteCommand => AIProvider::GLM_Orchestrator,  // Execute directly
            TaskType::GenerateCode => AIProvider::Grok,
            TaskType::Pentest => {
                // For pentesting, use Venice Red Team for aggressive operations
                // or Grok for planning/analysis
                if self.god_mode {
                    AIProvider::VeniceRedTeam
                } else {
                    // In normal mode, ask for clarification or use Grok with guardrails
                    AIProvider::Grok
                }
            }
            TaskType::MalwareAnalysis => AIProvider::VeniceRedTeam,  // Red team for malware
            TaskType::SecurityScan => {
                if self.god_mode {
                    AIProvider::VeniceRedTeam
                } else {
                    AIProvider::Grok  // Grok can do security scanning with guardrails
                }
            }
            TaskType::NetworkRecon => AIProvider::Grok,
            _ => AIProvider::Grok,
        }
    }

    // ------------------------------------------------------------------------
    // STEP 3: ANALYZE COMPLEXITY
    // ------------------------------------------------------------------------

    fn analyze_complexity(&self, task: &FenrirTask) -> Complexity {
        // Count operators
        let and_count = task.translated_command.matches("&&").count();
        let or_count = task.translated_command.matches("||").count();
        let pipe_count = task.translated_command.matches("|").count();

        if and_count + or_count + pipe_count > 2 {
            return Complexity::Complex;
        }

        // Check for multi-step operations
        if task.translated_command.contains(";") && task.translated_command.len() > 100 {
            return Complexity::Complex;
        }

        // Check for keywords indicating complexity
        let keywords = ["scan", "exploit", "analyze", "monitor", "attack"];
        for keyword in &keywords {
            if task.translated_command.contains(keyword) && task.translated_command.len() > 50 {
                return Complexity::Moderate;
            }
        }

        Complexity::Simple
    }

    // ------------------------------------------------------------------------
    // STEP 4: SPLIT INTO SUBTASKS
    // ------------------------------------------------------------------------

    async fn split_into_subtasks(&self, mut task: FenrirTask) -> FenrirTask {
        println!("✂️  Splitting complex task into subtasks...");

        let commands: Vec<&str> = task.translated_command
            .split("&&")
            .map(|s| s.trim())
            .collect();

        let mut subtasks = Vec::new();

        for (idx, cmd) in commands.iter().enumerate() {
            let subtask_id = format!("{}-sub{}", task.id, idx);
            let subtask_type = self.detect_task_type(cmd);
            let subtask_ai = self.assign_ai(&subtask_type);

            subtasks.push(FenrirTask {
                id: subtask_id,
                priority: task.priority,
                description: format!("Subtask {}: {}", idx + 1, cmd),
                user_input: task.user_input.clone(),
                translated_command: cmd.to_string(),
                task_type: subtask_type,
                assigned_ai: subtask_ai,
                subtasks: Vec::new(),
                execution_mode: ExecutionMode::Sequential,
                required_capabilities: Vec::new(),
                status: TaskStatus::Pending,
                result: None,
            });
        }

        task.subtasks = subtasks;
        task.execution_mode = ExecutionMode::Sequential;
        task
    }

    // ------------------------------------------------------------------------
    // STEP 5: EXECUTE TASK
    // ------------------------------------------------------------------------

    async fn execute_task(&self, task: FenrirTask) -> FenrirTask {
        let start = Instant::now();

        // If has subtasks, execute them
        if !task.subtasks.is_empty() {
            return self.execute_subtasks(task).await;
        }

        // Otherwise execute directly
        let mut updated_task = task;
        updated_task.status = TaskStatus::InProgress;

        // Execute based on assigned AI
        let result = match updated_task.assigned_ai {
            AIProvider::GLM_Orchestrator => {
                self.execute_directly(&updated_task).await
            }
            AIProvider::Grok => {
                self.delegate_to_ai(updated_task.clone(), AIProvider::Grok).await
            }
            AIProvider::VeniceRedTeam => {
                self.delegate_to_ai(updated_task.clone(), AIProvider::VeniceRedTeam).await
            }
            _ => {
                crate::fenrir_ai_layer::AIResponse {
                    success: false,
                    content: String::from("Invalid AI provider"),
                    error: Some(String::from("Cannot execute with Gemini")),
                    provider: AIProvider::GeminiTranslator,
                    execution_time_ms: 0,
                }
            }
        };

        updated_task.result = Some(crate::fenrir_ai_layer::TaskResult {
            success: result.success,
            output: result.content,
            error: result.error,
            execution_time: start.elapsed().as_millis() as u64,
            ai_provider: updated_task.assigned_ai.clone(),
        });

        updated_task.status = if result.success {
            TaskStatus::Completed
        } else {
            TaskStatus::Failed
        };

        updated_task
    }

    async fn execute_subtasks(&self, mut task: FenrirTask) -> FenrirTask {
        let mut results = Vec::new();

        for mut subtask in task.subtasks.clone() {
            println!("📋 Executing subtask: {}", subtask.description);
            subtask.status = TaskStatus::InProgress;

            let result = match subtask.assigned_ai {
                AIProvider::GLM_Orchestrator => {
                    self.execute_directly(&subtask).await
                }
                AIProvider::Grok => {
                    self.delegate_to_ai(subtask.clone(), AIProvider::Grok).await
                }
                AIProvider::VeniceRedTeam => {
                    self.delegate_to_ai(subtask.clone(), AIProvider::VeniceRedTeam).await
                }
                _ => continue,
            };

            subtask.result = Some(crate::fenrir_ai_layer::TaskResult {
                success: result.success,
                output: result.content.clone(),
                error: result.error.clone(),
                execution_time: result.execution_time_ms,
                ai_provider: subtask.assigned_ai.clone(),
            });

            subtask.status = if result.success {
                TaskStatus::Completed
            } else {
                TaskStatus::Failed
            };

            results.push(result.content);
        }

        task.subtasks = task.subtasks.into_iter().map(|mut t| {
            // Update status from results
            t
        }).collect();

        // Aggregate results
        let aggregated_output = results.join("\n\n");
        task.result = Some(crate::fenrir_ai_layer::TaskResult {
            success: true,
            output: aggregated_output,
            error: None,
            execution_time: 0,
            ai_provider: AIProvider::GLM_Orchestrator,
        });

        task.status = TaskStatus::Completed;
        task
    }

    async fn execute_directly(&self, task: &FenrirTask) -> crate::fenrir_ai_layer::AIResponse {
        use std::process::Command;

        println!("🎯 Executing directly: {}", task.translated_command);

        let parts: Vec<&str> = task.translated_command.split_whitespace().collect();
        if parts.is_empty() {
            return crate::fenrir_ai_layer::AIResponse {
                success: false,
                content: String::from(""),
                error: Some(String::from("Empty command")),
                provider: AIProvider::GLM_Orchestrator,
                execution_time_ms: 0,
            };
        }

        let (cmd, args) = parts.split_first().unwrap();

        match Command::new(cmd).args(args).output() {
            Ok(output) => {
                if output.status.success() {
                    crate::fenrir_ai_layer::AIResponse {
                        success: true,
                        content: String::from_utf8_lossy(&output.stdout).to_string(),
                        error: None,
                        provider: AIProvider::GLM_Orchestrator,
                        execution_time_ms: 0,
                    }
                } else {
                    let stderr = String::from_utf8_lossy(&output.stderr).to_string();
                    crate::fenrir_ai_layer::AIResponse {
                        success: false,
                        content: String::from(""),
                        error: Some(stderr),
                        provider: AIProvider::GLM_Orchestrator,
                        execution_time_ms: 0,
                    }
                }
            }
            Err(e) => crate::fenrir_ai_layer::AIResponse {
                success: false,
                content: String::from(""),
                error: Some(format!("Failed to execute: {}", e)),
                provider: AIProvider::GLM_Orchestrator,
                execution_time_ms: 0,
            }
        }
    }

    async fn delegate_to_ai(&self, task: FenrirTask, provider: AIProvider) -> crate::fenrir_ai_layer::AIResponse {
        let prompt = format!(
            "Execute this task:\n\nDescription: {}\nCommand: {}\n\nProvide detailed output.",
            task.description, task.translated_command
        );

        let request = AIRequest {
            provider: provider.clone(),
            system_prompt: format!("You are Fenrir's {}. Execute the task.", match provider {
                AIProvider::Grok => "general AI assistant",
                AIProvider::VeniceRedTeam => "red team operator",
                _ => "AI assistant",
            }),
            user_message: prompt,
            max_tokens: Some(4096),
            temperature: Some(0.7),
        };

        call_ai(request).await
    }

    // ------------------------------------------------------------------------
    // STEP 6: FORMAT OUTPUT
    // ------------------------------------------------------------------------

    fn format_output(&self, task: FenrirTask) -> String {
        task.to_markdown()
    }
}

// ============================================================================
// COMPLEXITY LEVELS
// ============================================================================

#[derive(Debug, PartialEq)]
enum Complexity {
    Simple,
    Moderate,
    Complex,
}
