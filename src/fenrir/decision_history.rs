use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;

/// Histórico de decisões do usuário para contexto futuro.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DecisionRecord {
    pub timestamp: DateTime<Local>,
    pub action_type: String,
    pub description: String,
    pub user_choice: String,
    pub context: HashMap<String, String>,
    pub outcome: Option<String>,
}

#[derive(Debug)]
pub struct DecisionHistory {
    records: Vec<DecisionRecord>,
    max_records: usize,
    storage_path: PathBuf,
}

impl DecisionHistory {
    /// Cria novo histórico de decisões.
    pub fn new() -> Self {
        let home = dirs::home_dir().unwrap_or_default();
        let storage_path = home.join(".fenrir").join("decision_history.json");

        Self {
            records: Vec::new(),
            max_records: 100,
            storage_path,
        }
    }

    /// Carrega histórico do disco.
    pub fn load(&mut self) -> anyhow::Result<()> {
        if self.storage_path.exists() {
            let content = fs::read_to_string(&self.storage_path)?;
            self.records = serde_json::from_str(&content)?;
        }
        Ok(())
    }

    /// Salva histórico no disco.
    pub fn save(&self) -> anyhow::Result<()> {
        // Create directory if it doesn't exist
        if let Some(parent) = self.storage_path.parent() {
            fs::create_dir_all(parent)?;
        }

        let content = serde_json::to_string_pretty(&self.records)?;
        fs::write(&self.storage_path, content)?;
        Ok(())
    }

    /// Adiciona novo registro de decisão.
    pub fn add_decision(&mut self, record: DecisionRecord) {
        self.records.push(record);

        // Mantém apenas os registros mais recentes
        if self.records.len() > self.max_records {
            self.records.remove(0);
        }

        // Salva automaticamente
        let _ = self.save();
    }

    /// Busca decisões similares por tipo de ação.
    pub fn find_similar_decisions(&self, action_type: &str) -> Vec<&DecisionRecord> {
        self.records
            .iter()
            .filter(|r| r.action_type == action_type)
            .collect()
    }

    /// Busca decisões por contexto (palavras-chave).
    pub fn search_by_context(&self, keywords: &[&str]) -> Vec<&DecisionRecord> {
        self.records
            .iter()
            .filter(|r| {
                keywords.iter().any(|kw| {
                    r.description.to_lowercase().contains(&kw.to_lowercase()) ||
                    r.context.values().any(|v| v.to_lowercase().contains(&kw.to_lowercase()))
                })
            })
            .collect()
    }

    /// Obtém estatísticas de decisões.
    pub fn get_statistics(&self) -> HashMap<String, usize> {
        let mut stats = HashMap::new();

        for record in &self.records {
            *stats.entry(record.action_type.clone()).or_insert(0) += 1;
            *stats.entry(format!("choice_{}", record.user_choice)).or_insert(0) += 1;
        }

        stats
    }

    /// Lista decisões recentes.
    pub fn recent_decisions(&self, limit: usize) -> Vec<&DecisionRecord> {
        self.records
            .iter()
            .rev()
            .take(limit)
            .collect()
    }

    /// Gera resumo das decisões tomadas.
    pub fn generate_summary(&self) -> String {
        let mut summary = format!("📊 Histórico de Decisões (últimas {})\n\n", self.records.len());

        let stats = self.get_statistics();
        summary.push_str("📈 Estatísticas:\n");
        for (key, count) in &stats {
            if key.starts_with("choice_") {
                let choice = key.strip_prefix("choice_").unwrap();
                summary.push_str(&format!("  {}: {}\n", choice, count));
            }
        }

        summary.push_str("\n🕐 Decisões Recentes:\n");
        for record in self.recent_decisions(5) {
            summary.push_str(&format!(
                "  {} - {}: {}\n",
                record.timestamp.format("%Y-%m-%d %H:%M"),
                record.action_type,
                record.description
            ));
        }

        summary
    }

    /// Sugere ação baseada no histórico.
    pub fn suggest_based_on_history(&self, action_type: &str) -> Option<String> {
        let similar = self.find_similar_decisions(action_type);
        if similar.is_empty() {
            return None;
        }

        // Conta escolhas mais frequentes
        let mut choice_counts = HashMap::new();
        for record in similar {
            *choice_counts.entry(record.user_choice.clone()).or_insert(0) += 1;
        }

        // Retorna a escolha mais frequente
        choice_counts
            .into_iter()
            .max_by_key(|&(_, count)| count)
            .map(|(choice, _)| choice)
    }
}

/// Cria registro de decisão rapidamente.
pub fn create_decision_record(
    action_type: &str,
    description: &str,
    user_choice: &str,
    context: HashMap<String, String>,
) -> DecisionRecord {
    DecisionRecord {
        timestamp: Local::now(),
        action_type: action_type.to_string(),
        description: description.to_string(),
        user_choice: user_choice.to_string(),
        context,
        outcome: None,
    }
}

/// Atualiza resultado de uma decisão.
pub fn update_decision_outcome(history: &mut DecisionHistory, index: usize, outcome: &str) {
    if let Some(record) = history.records.get_mut(index) {
        record.outcome = Some(outcome.to_string());
        let _ = history.save();
    }
}
