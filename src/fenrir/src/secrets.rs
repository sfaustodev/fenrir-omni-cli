// --- SECRETS MANAGEMENT MODULE ---
// Secure storage for API keys and sensitive data
// Supports multiple backends: environment variables, keyring, HashiCorp Vault, age-encrypted files

use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;
use anyhow::{Result, anyhow};
use dirs;

/// Secret storage backends
#[derive(Debug, Clone)]
pub enum SecretBackend {
    Env,           // Environment variables (current default)
    Keyring,       // OS keyring/keychain
    Vault,         // HashiCorp Vault
    AgeFile,       // age-encrypted files
}

/// Configuration for secret backends
#[derive(Debug, Clone)]
pub struct SecretConfig {
    pub backend: SecretBackend,
    pub vault_url: Option<String>,
    pub vault_token: Option<String>,
    pub age_key_file: Option<PathBuf>,
    pub project_name: String,
}

impl Default for SecretConfig {
    fn default() -> Self {
        Self {
            backend: SecretBackend::Env,
            vault_url: None,
            vault_token: None,
            age_key_file: None,
            project_name: "fenrir".to_string(),
        }
    }
}

/// Main secrets manager
pub struct SecretsManager {
    config: SecretConfig,
    cache: HashMap<String, String>,
}

impl SecretsManager {
    pub fn new(config: SecretConfig) -> Self {
        Self {
            config,
            cache: HashMap::new(),
        }
    }

    /// Get a secret by key
    pub fn get_secret(&mut self, key: &str) -> Result<String> {
        // Check cache first
        if let Some(value) = self.cache.get(key) {
            return Ok(value.clone());
        }

        let value = match &self.config.backend {
            SecretBackend::Env => self.get_from_env(key)?,
            SecretBackend::Keyring => self.get_from_keyring(key)?,
            SecretBackend::Vault => self.get_from_vault(key)?,
            SecretBackend::AgeFile => self.get_from_age_file(key)?,
        };

        // Cache the result
        self.cache.insert(key.to_string(), value.clone());
        Ok(value)
    }

    /// Set a secret
    pub fn set_secret(&self, key: &str, value: &str) -> Result<()> {
        match &self.config.backend {
            SecretBackend::Env => Err(anyhow!("Cannot set environment variables through secrets manager")),
            SecretBackend::Keyring => self.set_in_keyring(key, value),
            SecretBackend::Vault => self.set_in_vault(key, value),
            SecretBackend::AgeFile => self.set_in_age_file(key, value),
        }
    }

    /// Get from environment variables (fallback/default)
    fn get_from_env(&self, key: &str) -> Result<String> {
        std::env::var(key)
            .map_err(|_| anyhow!("Secret '{}' not found in environment", key))
    }

    /// Get from OS keyring
    fn get_from_keyring(&self, key: &str) -> Result<String> {
        let service = format!("{}-secrets", self.config.project_name);
        let entry = keyring::Entry::new(&service, key)
            .map_err(|e| anyhow!("Keyring error: {}", e))?;
        entry.get_password()
            .map_err(|e| anyhow!("Failed to get secret from keyring: {}", e))
    }

    /// Set in OS keyring
    fn set_in_keyring(&self, key: &str, value: &str) -> Result<()> {
        let service = format!("{}-secrets", self.config.project_name);
        let entry = keyring::Entry::new(&service, key)
            .map_err(|e| anyhow!("Keyring error: {}", e))?;
        entry.set_password(value)
            .map_err(|e| anyhow!("Failed to set secret in keyring: {}", e))
    }

    /// Get from HashiCorp Vault
    fn get_from_vault(&self, key: &str) -> Result<String> {
        // TODO: Implement Vault integration
        // For now, fall back to env
        println!("⚠️ Vault backend not yet implemented, falling back to environment");
        self.get_from_env(key)
    }

    /// Set in HashiCorp Vault
    fn set_in_vault(&self, _key: &str, _value: &str) -> Result<()> {
        // TODO: Implement Vault integration
        Err(anyhow!("Vault backend not yet implemented"))
    }

    /// Get from age-encrypted file
    fn get_from_age_file(&self, key: &str) -> Result<String> {
        // TODO: Implement age file encryption
        // For now, fall back to env
        println!("⚠️ Age file backend not yet implemented, falling back to environment");
        self.get_from_env(key)
    }

    /// Set in age-encrypted file
    fn set_in_age_file(&self, _key: &str, _value: &str) -> Result<()> {
        // TODO: Implement age file encryption
        Err(anyhow!("Age file backend not yet implemented"))
    }

    /// List available secrets
    pub fn list_secrets(&self) -> Result<Vec<String>> {
        match &self.config.backend {
            SecretBackend::Env => {
                // Return all env vars that look like API keys
                let api_keys = vec![
                    "API_KEY", "KAT_KEY", "GLM4_6_KEY", "GLM_4_6_KEY", "GLM_KEY", "GLM_API_KEY",
                    "GROK_API_KEY", "XAI_API_KEY", "GLI_KEY", "VENICE_KEY", "QWEN3_API_KEY",
                    "ZAI_API_KEY", "GEMINI_API_KEY",
                ];
                Ok(api_keys.into_iter().map(|s| s.to_string()).collect())
            }
            SecretBackend::Keyring => {
                // TODO: List keyring entries
                Ok(vec![])
            }
            SecretBackend::Vault => Ok(vec![]),
            SecretBackend::AgeFile => Ok(vec![]),
        }
    }

    /// Get backend info
    pub fn backend_info(&self) -> String {
        match &self.config.backend {
            SecretBackend::Env => "Environment Variables (default)".to_string(),
            SecretBackend::Keyring => "OS Keyring/Keychain".to_string(),
            SecretBackend::Vault => "HashiCorp Vault".to_string(),
            SecretBackend::AgeFile => "age-encrypted Files".to_string(),
        }
    }
}

/// Global secrets manager instance
static mut SECRETS_MANAGER: Option<SecretsManager> = None;

/// Initialize the global secrets manager
pub fn init_secrets_manager(config: SecretConfig) -> Result<()> {
    unsafe {
        SECRETS_MANAGER = Some(SecretsManager::new(config));
    }
    Ok(())
}

/// Get the global secrets manager
pub fn get_secrets_manager() -> Result<&'static mut SecretsManager> {
    unsafe {
        SECRETS_MANAGER.as_mut()
            .ok_or_else(|| anyhow!("Secrets manager not initialized"))
    }
}

/// Convenience function to get a secret
pub fn get_secret(key: &str) -> Result<String> {
    get_secrets_manager()?.get_secret(key)
}

/// Convenience function to set a secret
pub fn set_secret(key: &str, value: &str) -> Result<()> {
    get_secrets_manager()?.set_secret(key, value)
}