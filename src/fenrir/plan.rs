use regex::Regex;
use std::collections::HashMap;

/// Interpreta intenções em linguagem natural e sugere ações.
pub struct PlanInterpreter {
    patterns: HashMap<String, PlanAction>,
}

#[derive(Debug, Clone)]
pub struct PlanAction {
    pub action_type: String,
    pub description: String,
    pub confidence: f32,
    pub parameters: HashMap<String, String>,
    pub requires_confirmation: bool,
}

impl PlanInterpreter {
    pub fn new() -> Self {
        let mut patterns = HashMap::new();

        // Cleanup patterns
        patterns.insert(
            "cleanup|limpar|liberar espaço".to_string(),
            PlanAction {
                action_type: "disk_cleanup".to_string(),
                description: "Limpeza automática de disco (caches, logs, arquivos temporários)".to_string(),
                confidence: 0.9,
                parameters: HashMap::new(),
                requires_confirmation: true,
            }
        );

        patterns.insert(
            "duplicatas|duplicates".to_string(),
            PlanAction {
                action_type: "find_duplicates".to_string(),
                description: "Detectar e listar arquivos duplicados por hash".to_string(),
                confidence: 0.8,
                parameters: HashMap::new(),
                requires_confirmation: false,
            }
        );

        // Safe check patterns
        patterns.insert(
            "verificar segurança|security check|safe check".to_string(),
            PlanAction {
                action_type: "safe_check".to_string(),
                description: "Verificar arquivos de configuração em busca de padrões arriscados".to_string(),
                confidence: 0.9,
                parameters: HashMap::new(),
                requires_confirmation: false,
            }
        );

        // Git patterns
        patterns.insert(
            "git|commit|push|pull".to_string(),
            PlanAction {
                action_type: "git_automation".to_string(),
                description: "Automatizar operações Git com verificações de segurança".to_string(),
                confidence: 0.7,
                parameters: HashMap::new(),
                requires_confirmation: true,
            }
        );

        // Monitor patterns
        patterns.insert(
            "monitor|monitorar|recursos|system".to_string(),
            PlanAction {
                action_type: "monitor".to_string(),
                description: "Monitorar uso de recursos do sistema (CPU, memória, disco)".to_string(),
                confidence: 0.8,
                parameters: HashMap::new(),
                requires_confirmation: false,
            }
        );

        Self { patterns }
    }

    /// Interpreta uma consulta em linguagem natural.
    pub fn interpret(&self, query: &str) -> Vec<PlanAction> {
        let query_lower = query.to_lowercase();
        let mut matches = Vec::new();

        for (pattern, action) in &self.patterns {
            if query_lower.contains(pattern) {
                let mut action_clone = action.clone();

                // Extract parameters from query
                self.extract_parameters(&query_lower, &mut action_clone);

                // Adjust confidence based on context
                action_clone.confidence = self.adjust_confidence(&query_lower, action_clone.confidence);

                matches.push(action_clone);
            }
        }

        // Sort by confidence
        matches.sort_by(|a, b| b.confidence.partial_cmp(&a.confidence).unwrap());
        matches
    }

    /// Extrai parâmetros específicos da consulta.
    fn extract_parameters(&self, query: &str, action: &mut PlanAction) {
        // Extract paths
        if let Some(path_match) = Regex::new(r"(?:em|no|na|do|da)\s+([^\s]+)").unwrap().find(query) {
            action.parameters.insert("path".to_string(), path_match.as_str().to_string());
        }

        // Extract sizes
        if let Some(size_match) = Regex::new(r"(\d+)\s*(?:MB|GB|mb|gb)").unwrap().find(query) {
            action.parameters.insert("min_size".to_string(), size_match.as_str().to_string());
        }

        // Extract time periods
        if let Some(time_match) = Regex::new(r"(\d+)\s*(?:dias?|days?)").unwrap().find(query) {
            action.parameters.insert("days".to_string(), time_match.as_str().to_string());
        }
    }

    /// Ajusta confiança baseada no contexto da consulta.
    fn adjust_confidence(&self, query: &str, base_confidence: f32) -> f32 {
        let mut confidence = base_confidence;

        // Boost confidence for explicit commands
        if query.contains("quero") || query.contains("preciso") || query.contains("por favor") {
            confidence += 0.1;
        }

        // Reduce confidence for ambiguous queries
        if query.contains("talvez") || query.contains("pode ser") || query.contains("maybe") {
            confidence -= 0.2;
        }

        // Cap between 0.0 and 1.0
        confidence.max(0.0).min(1.0)
    }

    /// Gera um resumo das ações sugeridas.
    pub fn generate_summary(&self, actions: &[PlanAction]) -> String {
        if actions.is_empty() {
            return "❌ Não consegui interpretar sua intenção. Tente ser mais específico.".to_string();
        }

        let mut summary = format!("🤔 Interpretei sua solicitação como:\n\n");

        for (i, action) in actions.iter().enumerate() {
            let confidence_icon = if action.confidence > 0.8 {
                "🎯"
            } else if action.confidence > 0.6 {
                "⚡"
            } else {
                "🤷"
            };

            summary.push_str(&format!(
                "{}. {} {} (confiança: {:.1}%)\n",
                i + 1,
                confidence_icon,
                action.description,
                action.confidence * 100.0
            ));

            if action.requires_confirmation {
                summary.push_str("   ⚠️  Requer confirmação antes da execução\n");
            }

            if !action.parameters.is_empty() {
                summary.push_str("   📋 Parâmetros detectados:\n");
                for (key, value) in &action.parameters {
                    summary.push_str(&format!("      • {}: {}\n", key, value));
                }
            }

            summary.push_str("\n");
        }

        summary.push_str("✅ Confirma essas ações? (sim/não)\n");
        summary
    }
}

/// Interpreta uma consulta e retorna ações sugeridas.
pub fn interpret_intention(query: &str) -> Vec<PlanAction> {
    let interpreter = PlanInterpreter::new();
    interpreter.interpret(query)
}

/// Gera resumo das ações interpretadas.
pub fn generate_plan_summary(actions: &[PlanAction]) -> String {
    let interpreter = PlanInterpreter::new();
    interpreter.generate_summary(actions)
}
