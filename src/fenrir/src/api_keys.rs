use anyhow::{anyhow, Result};
use std::env;
use crate::secrets::{get_secret, SecretConfig, SecretBackend, init_secrets_manager};

/// Source/name of the environment variable that satisfied the lookup.
#[derive(Debug, Clone)]
pub struct ApiKeyResolution {
    pub value: String,
    pub source: &'static str,
}

/// Priority list for Fenrir CLI API keys.
pub const API_KEY_PRIORITY: &[&str] = &[
    "ZAI_API_KEY",         // Zhipu AI GLM-4.6 (prioridade máxima)
    "GEMINI_API_KEY",      // Gemini 3.0 Pro
    "API_KEY",
    "KAT_KEY",
    "GLM4_6_KEY",
    "GLM_4_6_KEY",
    "GLM_KEY",
    "GLM_API_KEY",
    "GROK_API_KEY",
    "XAI_API_KEY",
    "GLI_KEY",
];

/// Resolve the primary API key respecting the priority list above.
/// Now supports secrets manager backends in addition to environment variables.
pub fn resolve_primary_grok_key() -> Result<ApiKeyResolution> {
    // Try secrets manager first (if initialized)
    if let Ok(mut manager) = crate::secrets::get_secrets_manager() {
        for &var in API_KEY_PRIORITY {
            if let Ok(value) = manager.get_secret(var) {
                let trimmed = value.trim();
                if !trimmed.is_empty() {
                    return Ok(ApiKeyResolution {
                        value: trimmed.to_string(),
                        source: &format!("secrets:{}", var),
                    });
                }
            }
        }
    }

    // Fallback to environment variables
    for &var in API_KEY_PRIORITY {
        if let Ok(value) = env::var(var) {
            let trimmed = value.trim();
            if !trimmed.is_empty() {
                return Ok(ApiKeyResolution {
                    value: trimmed.to_string(),
                    source: var,
                });
            }
        }
    }

    Err(anyhow!(
        "Configure at least one API key (order: {}). Available backends: environment variables, keyring, vault, age files.",
        API_KEY_PRIORITY.join(" → ")
    ))
}

/// Helper for user-facing diagnostics.
pub fn describe_priority() -> String {
    API_KEY_PRIORITY
        .iter()
        .map(|name| format!("${}", name))
        .collect::<Vec<_>>()
        .join(" → ")
}
