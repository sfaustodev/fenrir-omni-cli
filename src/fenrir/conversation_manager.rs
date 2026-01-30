// ============================================================================
// FENRIR CONVERSATION MANAGER - Multi-turn AI Conversations
// ============================================================================
// Tracks conversation history and context for multi-turn interactions
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

/// Conversation manager for tracking multi-turn interactions
pub struct ConversationManager {
    /// Active conversations by session ID
    pub conversations: HashMap<String, Conversation>,
    /// Storage path
    pub storage_path: String,
}

/// Single conversation session
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Conversation {
    /// Unique session identifier
    pub session_id: String,
    /// Target being analyzed
    pub target: String,
    /// Target type (DOMAIN, IP, EMAIL, USERNAME)
    pub target_type: String,
    /// Conversation start time
    pub started_at: DateTime<Utc>,
    /// Last activity time
    pub last_activity: DateTime<Utc>,
    /// Conversation messages
    pub messages: Vec<ConversationMessage>,
    /// Current context/mode
    pub current_mode: String,
    /// Execution results from previous commands
    pub execution_history: Vec<ExecutionResult>,
    /// Active strategy/phases
    pub active_strategy: Option<String>,
}

/// Individual message in conversation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConversationMessage {
    /// Message timestamp
    pub timestamp: DateTime<Utc>,
    /// Message role (user/assistant/system)
    pub role: MessageRole,
    /// Message content
    pub content: String,
    /// Associated command if any
    pub command: Option<String>,
    /// Tool used if any
    pub tool: Option<String>,
}

/// Message role types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MessageRole {
    #[serde(rename = "user")]
    User,
    #[serde(rename = "assistant")]
    Assistant,
    #[serde(rename = "system")]
    System,
    #[serde(rename = "tool")]
    Tool,
}

/// Execution result from tool
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExecutionResult {
    /// Tool that was executed
    pub tool: String,
    /// Command executed
    pub command: String,
    /// Exit code
    pub exit_code: i32,
    /// Duration in seconds
    pub duration_secs: f64,
    /// Success flag
    pub success: bool,
    /// Output (truncated)
    pub output: String,
    /// Timestamp
    pub timestamp: DateTime<Utc>,
}

impl ConversationManager {
    /// Create new conversation manager
    pub async fn new() -> Result<Self> {
        let storage_path = "/Users/peluche/Fenrir/.fenrir_conversations".to_string();

        // Create storage directory if needed
        if !Path::new(&storage_path).exists() {
            fs::create_dir_all(&storage_path)
                .context("Failed to create conversations directory")?;
        }

        Ok(Self {
            conversations: HashMap::new(),
            storage_path,
        })
    }

    /// Start a new conversation session
    pub fn start_conversation(&mut self, target: &str, target_type: &str, mode: &str) -> String {
        let session_id = format!("{}_{}", target.replace('/', "_"), Utc::now().timestamp());

        let conversation = Conversation {
            session_id: session_id.clone(),
            target: target.to_string(),
            target_type: target_type.to_string(),
            started_at: Utc::now(),
            last_activity: Utc::now(),
            messages: Vec::new(),
            current_mode: mode.to_string(),
            execution_history: Vec::new(),
            active_strategy: None,
        };

        self.conversations.insert(session_id.clone(), conversation);

        session_id
    }

    /// Add a message to conversation
    pub fn add_message(&mut self, session_id: &str, role: MessageRole, content: &str) -> Result<()> {
        if let Some(conv) = self.conversations.get_mut(session_id) {
            let message = ConversationMessage {
                timestamp: Utc::now(),
                role,
                content: content.to_string(),
                command: None,
                tool: None,
            };

            conv.messages.push(message);
            conv.last_activity = Utc::now();

            Ok(())
        } else {
            anyhow::bail!("Conversation session not found: {}", session_id)
        }
    }

    /// Record tool execution result
    pub fn record_execution(
        &mut self,
        session_id: &str,
        tool: &str,
        command: &str,
        exit_code: i32,
        duration_secs: f64,
        success: bool,
        output: &str,
    ) -> Result<()> {
        if let Some(conv) = self.conversations.get_mut(session_id) {
            let result = ExecutionResult {
                tool: tool.to_string(),
                command: command.to_string(),
                exit_code,
                duration_secs,
                success,
                output: output.chars().take(1000).collect(),
                timestamp: Utc::now(),
            };

            conv.execution_history.push(result);
            conv.last_activity = Utc::now();

            Ok(())
        } else {
            anyhow::bail!("Conversation session not found: {}", session_id)
        }
    }

    /// Get conversation context as prompt for AI
    pub fn get_conversation_context(&self, session_id: &str, max_messages: usize) -> String {
        if let Some(conv) = self.conversations.get(session_id) {
            let mut context = format!(
                "CONVERSATION CONTEXT:\n\
                 Target: {} ({})\n\
                 Mode: {}\n\
                 Session started: {}\n\
                 Messages so far: {}\n\
                 Executions: {}\n\
                 Recent history:\n",
                conv.target,
                conv.target_type,
                conv.current_mode,
                conv.started_at.format("%Y-%m-%d %H:%M:%S UTC"),
                conv.messages.len(),
                conv.execution_history.len()
            );

            // Add recent messages
            let recent_messages: Vec<_> = conv.messages.iter()
                .rev()
                .take(max_messages)
                .collect();

            for msg in recent_messages.iter().rev() {
                let role_str = match msg.role {
                    MessageRole::User => "User",
                    MessageRole::Assistant => "Assistant",
                    MessageRole::System => "System",
                    MessageRole::Tool => "Tool",
                };

                context.push_str(&format!("[{}] {}\n", role_str, msg.content));
            }

            // Add recent successful executions
            let recent_executions: Vec<_> = conv.execution_history.iter()
                .filter(|e| e.success)
                .rev()
                .take(5)
                .collect();

            if !recent_executions.is_empty() {
                context.push_str("\nRecent successful executions:\n");
                for exec in recent_executions.iter().rev() {
                    context.push_str(&format!(
                        "- {}: {} (success: {:.1}s)\n",
                        exec.tool,
                        exec.command.chars().take(80).collect::<String>(),
                        exec.duration_secs
                    ));
                }
            }

            context
        } else {
            String::from("No conversation context available")
        }
    }

    /// Get recent execution results for AI analysis
    pub fn get_recent_executions(&self, session_id: &str, count: usize) -> Vec<ExecutionResult> {
        if let Some(conv) = self.conversations.get(session_id) {
            conv.execution_history.iter()
                .rev()
                .take(count)
                .cloned()
                .collect()
        } else {
            Vec::new()
        }
    }

    /// Update conversation mode
    pub fn update_mode(&mut self, session_id: &str, mode: &str) -> Result<()> {
        if let Some(conv) = self.conversations.get_mut(session_id) {
            conv.current_mode = mode.to_string();
            conv.last_activity = Utc::now();
            Ok(())
        } else {
            anyhow::bail!("Conversation session not found: {}", session_id)
        }
    }

    /// Set active strategy
    pub fn set_strategy(&mut self, session_id: &str, strategy: &str) -> Result<()> {
        if let Some(conv) = self.conversations.get_mut(session_id) {
            conv.active_strategy = Some(strategy.to_string());
            conv.last_activity = Utc::now();
            Ok(())
        } else {
            anyhow::bail!("Conversation session not found: {}", session_id)
        }
    }

    /// Get conversation summary
    pub fn get_summary(&self, session_id: &str) -> Option<String> {
        if let Some(conv) = self.conversations.get(session_id) {
            Some(format!(
                "📋 CONVERSATION SUMMARY\n\
                 ══════════════════════\n\
                 Session: {}\n\
                 Target: {} ({})\n\
                 Mode: {}\n\
                 Started: {}\n\
                 Last activity: {}\n\
                 Messages: {}\n\
                 Executions: {}\n\
                 Strategy: {}",
                conv.session_id,
                conv.target,
                conv.target_type,
                conv.current_mode,
                conv.started_at.format("%Y-%m-%d %H:%M:%S UTC"),
                conv.last_activity.format("%Y-%m-%d %H:%M:%S UTC"),
                conv.messages.len(),
                conv.execution_history.len(),
                conv.active_strategy.as_deref().unwrap_or("None")
            ))
        } else {
            None
        }
    }

    /// End conversation and save to disk
    pub fn end_conversation(&mut self, session_id: &str) -> Result<()> {
        if let Some(conv) = self.conversations.remove(session_id) {
            // Save conversation to disk
            let filename = format!("{}/{}.json", self.storage_path, session_id);
            let data = serde_json::to_string_pretty(&conv)
                .context("Failed to serialize conversation")?;

            fs::write(&filename, data)
                .with_context(|| format!("Failed to write conversation to {}", filename))?;

            Ok(())
        } else {
            anyhow::bail!("Conversation session not found: {}", session_id)
        }
    }

    /// Get all active conversations
    pub fn get_active_conversations(&self) -> Vec<String> {
        self.conversations.keys().cloned().collect()
    }

    /// Clean up old conversations (inactive for >24 hours)
    pub fn cleanup_old_conversations(&mut self) -> Result<usize> {
        let threshold = Utc::now() - chrono::Duration::hours(24);
        let initial_count = self.conversations.len();

        self.conversations.retain(|_, conv| conv.last_activity > threshold);

        let cleaned = initial_count - self.conversations.len();
        Ok(cleaned)
    }

    /// Export conversation to markdown
    pub fn export_to_markdown(&self, session_id: &str) -> Result<String> {
        if let Some(conv) = self.conversations.get(session_id) {
            let mut md = format!("# FENRIR Conversation Report\n\n");
            md.push_str(&format!("**Target**: {}\n", conv.target));
            md.push_str(&format!("**Type**: {}\n", conv.target_type));
            md.push_str(&format!("**Mode**: {}\n", conv.current_mode));
            md.push_str(&format!("**Started**: {}\n", conv.started_at.format("%Y-%m-%d %H:%M:%S UTC")));
            md.push_str(&format!("**Messages**: {}\n\n", conv.messages.len()));

            md.push_str("## Conversation\n\n");
            for msg in &conv.messages {
                let role_str = match msg.role {
                    MessageRole::User => "👤 User",
                    MessageRole::Assistant => "🤖 Assistant",
                    MessageRole::System => "⚙️  System",
                    MessageRole::Tool => "🔧 Tool",
                };

                md.push_str(&format!("**{}** [{}]:\n{}\n\n",
                    role_str,
                    msg.timestamp.format("%H:%M:%S"),
                    msg.content
                ));
            }

            if !conv.execution_history.is_empty() {
                md.push_str("\n## Executions\n\n");
                for exec in &conv.execution_history {
                    let status = if exec.success { "✅" } else { "❌" };
                    md.push_str(&format!(
                        "{} **{}**: `{}` ({:.1}s)\n",
                        status,
                        exec.tool,
                        exec.command.chars().take(80).collect::<String>(),
                        exec.duration_secs
                    ));
                }
            }

            Ok(md)
        } else {
            anyhow::bail!("Conversation session not found: {}", session_id)
        }
    }
}
