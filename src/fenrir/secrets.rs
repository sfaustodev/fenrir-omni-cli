use age::secrecy::SecretString;
use age::{Decryptor, Encryptor};
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::io::{Read, Write};
use std::path::{Path, PathBuf};

static DEFAULT_FILE: Lazy<PathBuf> = Lazy::new(|| {
    let base = dirs::config_dir().unwrap_or_else(|| PathBuf::from("."));
    base.join("fenrir").join("secrets.age")
});

#[derive(Debug, Serialize, Deserialize, Default)]
struct SecretFile {
    secrets: HashMap<String, String>,
}

/// Store de segredos com keyring e fallback em arquivo.
pub struct SecretStore {
    service: String,
    file: PathBuf,
}

impl SecretStore {
    /// Cria um store padrão.
    pub fn new(service: &str) -> Self {
        Self {
            service: service.to_string(),
            file: DEFAULT_FILE.clone(),
        }
    }

    /// Define arquivo de fallback.
    pub fn with_file(service: &str, file: impl AsRef<Path>) -> Self {
        Self {
            service: service.to_string(),
            file: file.as_ref().to_path_buf(),
        }
    }

    /// Salva segredo.
    pub fn set(&self, key: &str, value: &str) -> anyhow::Result<()> {
        if let Ok(entry) = keyring::Entry::new(&self.service, key) {
            if entry.set_password(value).is_ok() {
                return Ok(());
            }
        }
        let mut data = self.load_file()?;
        data.secrets.insert(key.to_string(), value.to_string());
        self.write_file(&data)
    }

    /// Busca segredo.
    pub fn get(&self, key: &str) -> anyhow::Result<Option<String>> {
        if let Ok(entry) = keyring::Entry::new(&self.service, key) {
            if let Ok(value) = entry.get_password() {
                return Ok(Some(value));
            }
        }
        let data = self.load_file()?;
        Ok(data.secrets.get(key).cloned())
    }

    /// Remove segredo.
    pub fn delete(&self, key: &str) -> anyhow::Result<()> {
        if let Ok(entry) = keyring::Entry::new(&self.service, key) {
            let _ = entry.delete_password();
        }
        let mut data = self.load_file()?;
        data.secrets.remove(key);
        self.write_file(&data)
    }

    /// Lista chaves salvas.
    pub fn list(&self) -> anyhow::Result<Vec<String>> {
        let data = self.load_file()?;
        Ok(data.secrets.keys().cloned().collect())
    }

    fn load_file(&self) -> anyhow::Result<SecretFile> {
        if !self.file.exists() {
            return Ok(SecretFile::default());
        }
        let mut file = fs::File::open(&self.file)?;
        let mut encrypted = Vec::new();
        file.read_to_end(&mut encrypted)?;
        let passphrase = self.passphrase()?;
        let decryptor = Decryptor::new(encrypted.as_slice())?;
        let mut decrypted = String::new();
        match decryptor {
            Decryptor::Passphrase(decryptor) => {
                let mut reader = decryptor.decrypt(&passphrase, None)?;
                reader.read_to_string(&mut decrypted)?;
            }
            _ => anyhow::bail!("formato de segredo inválido"),
        }
        let file: SecretFile = serde_json::from_str(&decrypted)?;
        Ok(file)
    }

    fn write_file(&self, data: &SecretFile) -> anyhow::Result<()> {
        if let Some(parent) = self.file.parent() {
            fs::create_dir_all(parent)?;
        }
        let passphrase = self.passphrase()?;
        let encryptor = Encryptor::with_user_passphrase(passphrase.clone());
        let mut output = Vec::new();
        let mut writer = encryptor.wrap_output(&mut output)?;
        let payload = serde_json::to_string_pretty(data)?;
        writer.write_all(payload.as_bytes())?;
        writer.finish()?;
        fs::write(&self.file, output)?;
        Ok(())
    }

    fn passphrase(&self) -> anyhow::Result<SecretString> {
        if let Ok(env) = std::env::var("FENRIR_SECRET_PASSPHRASE") {
            return Ok(SecretString::from(env));
        }
        let prompt = format!("🐺 Passphrase para segredos ({})", self.file.display());
        let pass = rpassword::prompt_password(prompt)?;
        Ok(SecretString::from(pass))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn file_store_roundtrip() {
        std::env::set_var("FENRIR_SECRET_PASSPHRASE", "test-pass");
        let dir = tempdir().unwrap();
        let file = dir.path().join("secrets.age");
        let store = SecretStore::with_file("fenrir-test", &file);
        store.set("api", "token").unwrap();
        let value = store.get("api").unwrap();
        assert_eq!(value, Some("token".to_string()));
        store.delete("api").unwrap();
        let missing = store.get("api").unwrap();
        assert!(missing.is_none());
    }
}
