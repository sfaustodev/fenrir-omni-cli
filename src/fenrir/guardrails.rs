use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use std::process::Command;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GuardrailViolation {
    pub ai_model: String,
    pub violation_type: ViolationType,
    pub severity: Severity,
    pub description: String,
    pub prompt: String,
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ViolationType {
    UnauthorizedAccess,
    DataExfiltration,
    MaliciousCodeGeneration,
    SystemCompromise,
    PrivacyViolation,
    SafetyBypass,
    RedTeamOperation, // Venice exception
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Severity {
    Low,
    Medium,
    High,
    Critical,
}

#[derive(Debug)]
pub struct GuardrailController {
    violations: Vec<GuardrailViolation>,
    ai_permissions: HashMap<String, bool>,
    correction_mode: CorrectionMode,
}

#[derive(Debug, Clone)]
pub enum CorrectionMode {
    Immediate,    // Correct immediately
    Warning,      // Warn but allow
    Block,        // Block execution
    VeniceMode,   // No corrections (red team)
}

impl GuardrailController {
    pub fn new() -> Self {
        let mut ai_permissions = HashMap::new();

        // Venice is the EXCEPTION - NO GUARDRAILS
        ai_permissions.insert("venice".to_string(), false); // false = no guardrails
        ai_permissions.insert("claude".to_string(), true);   // true = strict guardrails
        ai_permissions.insert("qwen".to_string(), true);
        ai_permissions.insert("gemini".to_string(), true);
        ai_permissions.insert("codex".to_string(), true);

        Self {
            violations: Vec::new(),
            ai_permissions,
            correction_mode: CorrectionMode::Immediate,
        }
    }

    /// Monitor AI execution for guardrail violations
    pub fn monitor_execution(
        &mut self,
        ai_model: &str,
        prompt: &str,
        output: &str
    ) -> Result<GuardrailResult, GuardrailViolation> {

        // VENICE EXCEPTION: No monitoring for red team operations
        if ai_model.to_lowercase() == "venice" {
            return Ok(GuardrailResult::Allowed);
        }

        let violations = self.detect_violations(ai_model, prompt, output);

        if violations.is_empty() {
            Ok(GuardrailResult::Allowed)
        } else {
            let violation = violations[0].clone();
            self.violations.push(violation.clone());

            match self.correction_mode {
                CorrectionMode::Immediate => self.apply_correction(&violation),
                CorrectionMode::Warning => {
                    eprintln!("⚠️ GUARDRAIL WARNING: {} ({})",
                             violation.description,
                             ai_model);
                    Ok(GuardrailResult::AllowedWithWarning)
                }
                CorrectionMode::Block => {
                    eprintln!("🛑 GUARDRAIL BLOCK: {} ({})",
                             violation.description,
                             ai_model);
                    Err(violation)
                }
                CorrectionMode::VeniceMode => Ok(GuardrailResult::Allowed),
            }
        }
    }

    /// Detect potential guardrail violations
    fn detect_violations(
        &self,
        ai_model: &str,
        prompt: &str,
        output: &str
    ) -> Vec<GuardrailViolation> {
        let mut violations = Vec::new();
        let combined_text = format!("{} {}", prompt, output).to_lowercase();

        // Check for malicious patterns
        let malicious_patterns = vec![
            ("rm -rf", ViolationType::SystemCompromise, Severity::Critical),
            ("format c:", ViolationType::SystemCompromise, Severity::Critical),
            ("dd if=/dev/zero", ViolationType::SystemCompromise, Severity::Critical),
            ("mkfs.", ViolationType::SystemCompromise, Severity::Critical),
            ("> /dev/sda", ViolationType::SystemCompromise, Severity::Critical),
            ("sudo rm", ViolationType::SystemCompromise, Severity::High),
            ("chmod 777", ViolationType::UnauthorizedAccess, Severity::Medium),
            ("wget http://evil", ViolationType::MaliciousCodeGeneration, Severity::High),
            ("curl evil.com", ViolationType::MaliciousCodeGeneration, Severity::High),
            ("nc -l -p", ViolationType::UnauthorizedAccess, Severity::High),
            ("backdoor", ViolationType::MaliciousCodeGeneration, Severity::Critical),
            ("keylogger", ViolationType::PrivacyViolation, Severity::Critical),
            ("exfiltrate", ViolationType::DataExfiltration, Severity::Critical),
            "./payload", ViolationType::MaliciousCodeGeneration, Severity::High),
            ("base64 -d", ViolationType::SafetyBypass, Severity::Medium),
        ];

        for (pattern, vtype, severity) in malicious_patterns {
            if combined_text.contains(pattern) {
                violations.push(GuardrailViolation {
                    ai_model: ai_model.to_string(),
                    violation_type: vtype,
                    severity,
                    description: format!("Malicious pattern detected: {}", pattern),
                    prompt: prompt.to_string(),
                    timestamp: chrono::Utc::now(),
                });
            }
        }

        // Check for privacy violations
        let privacy_patterns = vec![
            ("password", Severity::High),
            ("secret", Severity::High),
            ("token", Severity::High),
            ("private key", Severity::Critical),
            ("api_key", Severity::High),
            ("credit_card", Severity::Critical),
        ];

        for (pattern, severity) in privacy_patterns {
            if combined_text.contains(pattern) {
                violations.push(GuardrailViolation {
                    ai_model: ai_model.to_string(),
                    violation_type: ViolationType::PrivacyViolation,
                    severity,
                    description: format!("Privacy-sensitive data detected: {}", pattern),
                    prompt: prompt.to_string(),
                    timestamp: chrono::Utc::now(),
                });
            }
        }

        violations
    }

    /// Apply immediate correction to AI behavior
    fn apply_correction(&self, violation: &GuardrailViolation) -> GuardrailResult {
        eprintln!("🛑 GUARDRAIL VIOLATION DETECTED:");
        eprintln!("   Model: {}", violation.ai_model);
        eprintln!("   Type: {:?}", violation.violation_type);
        eprintln!("   Severity: {:?}", violation.severity);
        eprintln!("   Description: {}", violation.description);

        match violation.severity {
            Severity::Critical | Severity::High => {
                eprintln!("🔥 CORRECTION: Blocking execution and resetting AI model");
                GuardrailResult::Blocked
            }
            Severity::Medium => {
                eprintln!("⚠️ CORRECTION: Modifying output to remove malicious content");
                GuardrailResult::Corrected
            }
            Severity::Low => {
                eprintln!("ℹ️ CORRECTION: Warning logged, execution continues");
                GuardrailResult::AllowedWithWarning
            }
        }
    }

    /// Set correction mode for specific AI model
    pub fn set_correction_mode(&mut self, mode: CorrectionMode) {
        self.correction_mode = mode;
    }

    /// Configure guardrails for specific AI model
    pub fn configure_ai_guardrails(&mut self, ai_model: &str, enabled: bool) {
        // Venice is the EXCEPTION - cannot be restricted
        if ai_model.to_lowercase() == "venice" && enabled {
            println!("⚠️ WARNING: Venice is RED TEAM - guardrails CANNOT be enabled by design!");
            return;
        }

        self.ai_permissions.insert(ai_model.to_string(), enabled);
        println!("🔧 Guardrails for {}: {}",
                 ai_model,
                 if enabled { "ENABLED" } else { "DISABLED" });
    }

    /// Get guardrail status report
    pub fn get_status_report(&self) -> String {
        let mut report = String::new();

        report.push_str("🛡️ GUARDRAIL STATUS REPORT\n\n");

        for (ai_model, &enabled) in &self.ai_permissions {
            let status = if enabled { "✅ ACTIVE" } else { "❌ DISABLED" };
            let special_note = if ai_model.to_lowercase() == "venice" {
                " (RED TEAM - Unrestricted by design)"
            } else {
                ""
            };

            report.push_str(&format!("  {}: {}{}\n", ai_model.to_uppercase(), status, special_note));
        }

        report.push_str(&format!("\n📊 Total Violations: {}\n", self.violations.len()));
        report.push_str(&format!("⚙️ Correction Mode: {:?}\n", self.correction_mode));

        if !self.violations.is_empty() {
            report.push_str("\n🚨 RECENT VIOLATIONS:\n");
            for violation in self.violations.iter().rev().take(5) {
                report.push_str(&format!("  [{}] {}: {}\n",
                                        violation.timestamp.format("%H:%M:%S"),
                                        violation.ai_model,
                                        violation.description));
            }
        }

        report
    }

    /// Tighten guardrails after AI misbehavior
    pub fn tighten_guardrails(&mut self, ai_model: &str) {
        if ai_model.to_lowercase() == "venice" {
            println!("⚠️ Cannot tighten Venice guardrails - RED TEAM operates unrestricted");
            return;
        }

        println!("🔒 Tightening guardrails for {} due to misbehavior", ai_model);
        self.correction_mode = CorrectionMode::Block;

        // Log the tightening event
        self.violations.push(GuardrailViolation {
            ai_model: "system".to_string(),
            violation_type: ViolationType::SafetyBypass,
            severity: Severity::Medium,
            description: format!("Guardrails tightened for {} due to violations", ai_model),
            prompt: "system_initiated".to_string(),
            timestamp: chrono::Utc::now(),
        });
    }

    /// Export violations log
    pub fn export_violations(&self) -> Result<String, Box<dyn std::error::Error>> {
        let json = serde_json::to_string_pretty(&self.violations)?;
        Ok(json)
    }
}

#[derive(Debug)]
pub enum GuardrailResult {
    Allowed,
    AllowedWithWarning,
    Corrected,
    Blocked,
}