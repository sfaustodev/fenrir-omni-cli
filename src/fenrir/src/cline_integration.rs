use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// 🐺 CLINE INTEGRATION - "The Devoured One"
/// Mocks the configuration and capability structure of Cline
/// (Since we cannot compile the TS code, we replicate its data structures)

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ClineConfig {
    pub allowed_commands: Vec<String>,
    pub sound_enabled: bool,
    pub diff_enabled: bool,
    pub browser_viewport_size: String,
    pub api_configuration: ClineApiConfig,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ClineApiConfig {
    pub api_provider: String, // "anthropic", "openrouter", "bedrock", etc.
    pub api_model_id: Option<String>,
    pub api_key: Option<String>,
    pub temperature: f32,
}

impl ClineConfig {
    pub fn default_fenrir() -> Self {
        Self {
            allowed_commands: vec![
                "npm test".to_string(),
                "npm install".to_string(),
                "cargo build".to_string(),
                "cargo test".to_string(),
            ],
            sound_enabled: false,
            diff_enabled: true,
            browser_viewport_size: "1280x800".to_string(),
            api_configuration: ClineApiConfig {
                api_provider: "anthropic".to_string(),
                api_model_id: Some("claude-3-sonnet-20240229".to_string()),
                api_key: std::env::var("ANTHROPIC_API_KEY").ok(),
                temperature: 0.0,
            },
        }
    }
}

/// 🐺 Capabilities "Devoured" from Cline
pub struct ClineCapabilities {
    pub can_edit_files: bool,
    pub can_run_shell: bool,
    pub can_use_browser: bool,
    pub can_use_mcp: bool, // Model Context Protocol
}

impl ClineCapabilities {
    pub fn god_mode() -> Self {
        Self {
            can_edit_files: true,
            can_run_shell: true,
            can_use_browser: true, // Fenrir "Morder" mode
            can_use_mcp: true,     // Future integration
        }
    }
}

pub fn print_devoured_status() {
    println!("💀 CLINE STATUS: DEVOURADO E DIGERIDO");
    println!("   - API Providers: [Anthropic, OpenRouter, Bedrock, Vertex, Gemini]");
    println!("   - Capabilities: [File Edit, Shell, Browser, MCP]");
    println!("   - Integrated into: FENRIR COTOA ENGINE");
}
