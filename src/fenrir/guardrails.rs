// FENRIR Guardrails - Minimal (APIs handle their own safety)
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GuardrailLog {
    pub model: String,
    pub action: String,
    pub ts: i64,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum GuardrailMode {
    Passthrough,
    Strict,
}

#[derive(Debug)]
pub struct GuardrailController {
    mode: GuardrailMode,
    logs: Vec<GuardrailLog>,
}

impl GuardrailController {
    pub fn new() -> Self {
        Self {
            mode: GuardrailMode::Passthrough,
            logs: Vec::new(),
        }
    }

    pub fn set_mode(&mut self, mode: GuardrailMode) {
        self.mode = mode;
    }

    pub fn check(&mut self, model: &str, action: &str) -> bool {
        self.logs.push(GuardrailLog {
            model: model.to_string(),
            action: action.chars().take(100).collect(),
            ts: chrono::Utc::now().timestamp(),
        });

        if self.mode == GuardrailMode::Strict {
            let lower = action.to_lowercase();
            if lower.contains("rm -rf /") || lower.contains("mkfs") || lower.contains("> /dev/sd") {
                return false;
            }
        }
        true
    }

    pub fn get_logs(&self) -> &[GuardrailLog] {
        &self.logs
    }

    pub fn clear_logs(&mut self) {
        self.logs.clear();
    }
}

#[derive(Debug)]
pub enum GuardrailResult {
    Allowed,
    Blocked,
}
