use std::collections::HashMap;
use serde::{Deserialize, Serialize};

use crate::ai_hierarchy_abstraction::{
    execute_ai_command, ComplexityLevel, ExecutionContext, ExecutionPriority,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AITask {
    pub id: String,
    pub verb: String,
    pub ai_model: AIModel,
    pub prompt: String,
    pub guardrails: bool,
    pub priority: u8,
    pub dependencies: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum AIModel {
    Hierarchy,
    Claude,
    Qwen,
    Codex,
    Venice,
    Grok,
}

#[derive(Debug)]
pub struct AIOrchestrator {
    tasks: HashMap<String, AITask>,
    execution_queue: Vec<AITask>,
}

impl AIOrchestrator {
    pub fn new() -> Self {
        Self {
            tasks: HashMap::new(),
            execution_queue: Vec::new(),
        }
    }

    /// Planeja tarefas usando heurísticas locais e a hierarquia interna
    pub async fn ingest_prompt(&mut self, prompt: &str) -> Result<(), Box<dyn std::error::Error>> {
        println!("🧠 AI Hierarchy preparando tarefas para: '{}'", prompt);

        let mut verbs = self.extract_action_verbs(prompt);
        if verbs.is_empty() {
            verbs.push("executar".to_string());
        }

        for verb in verbs {
            let ai_model = self.determine_ai_model(&verb);
            let priority = self.calculate_priority(&verb);
            self.push_task(verb, prompt, ai_model, priority);
        }

        Ok(())
    }

    /// Executa tarefa via AI Hierarchy (única porta permitida)
    pub async fn execute_task(&self, task: &AITask) -> Result<String, Box<dyn std::error::Error>> {
        let engine_label = match task.ai_model {
            AIModel::Hierarchy => "FENRIR_MENTE",
            AIModel::Claude => "CLAUDE_GLM",
            AIModel::Qwen => "QWEN_CODE",
            AIModel::Codex => "CODEX_CLI",
            AIModel::Venice => "VENZ_RED_TEAM",
            AIModel::Grok => "GROK_CODE_FAST",
        };

        self.execute_via_hierarchy(task, engine_label).await
    }

    async fn execute_via_hierarchy(
        &self,
        task: &AITask,
        engine_label: &str,
    ) -> Result<String, Box<dyn std::error::Error>> {
        let context = self.context_for(task, engine_label);
        let payload = format!(
            "CHAIN-OF-CARALHA::EXECUTE::engine={}::verb={}::prompt={}",
            engine_label, task.verb, task.prompt
        );

        let execution = execute_ai_command(&payload, Some(&context)).await?;
        if let Some(output) = execution.result {
            Ok(output)
        } else if let Some(err) = execution.error_message {
            Err(err.into())
        } else {
            Ok(format!("✅ {} confirmou execução da tarefa {}", engine_label, task.id))
        }
    }

    fn context_for(&self, task: &AITask, engine_label: &str) -> ExecutionContext {
        ExecutionContext {
            priority: match task.priority {
                0..=3 => ExecutionPriority::Low,
                4..=6 => ExecutionPriority::Normal,
                7..=8 => ExecutionPriority::High,
                _ => ExecutionPriority::Critical,
            },
            complexity: if task.prompt.contains("arquitetura") || task.prompt.len() > 280 {
                ComplexityLevel::Complex
            } else {
                ComplexityLevel::Moderate
            },
            requires_censorship_bypass: engine_label == "VENZ_RED_TEAM" || !task.guardrails,
            mission_critical: task.priority >= 9,
        }
    }

    fn extract_action_verbs(&self, prompt: &str) -> Vec<String> {
        let mut verbs = Vec::new();
        for word in prompt.split(|c: char| c.is_whitespace() || c == ',' || c == ';') {
            let token = word.trim().to_lowercase();
            if token.is_empty() {
                continue;
            }
            match token.as_str() {
                "analisar" | "analyze" | "analysis" | "planejar" => verbs.push("analisar".to_string()),
                "corrigir" | "corrija" | "debug" | "fix" => verbs.push("corrigir".to_string()),
                "executar" | "execute" | "rodar" | "run" => verbs.push("executar".to_string()),
                "atacar" | "explorar" | "invadir" | "morder" => verbs.push("morder".to_string()),
                "gerar" | "criar" | "montar" | "construir" => verbs.push("construir".to_string()),
                _ => {}
            }
        }

        verbs.dedup();
        verbs
    }

    fn determine_ai_model(&self, verb: &str) -> AIModel {
        match verb {
            v if v.contains("morder") => AIModel::Venice,
            v if v.contains("corrigir") => AIModel::Claude,
            v if v.contains("analisar") => AIModel::Qwen,
            v if v.contains("construir") => AIModel::Codex,
            v if v.contains("executar") => AIModel::Grok,
            _ => AIModel::Hierarchy,
        }
    }

    fn calculate_priority(&self, verb: &str) -> u8 {
        match verb {
            v if v.contains("morder") => 10,
            v if v.contains("corrigir") => 8,
            v if v.contains("analisar") => 7,
            v if v.contains("construir") => 6,
            _ => 5,
        }
    }

    fn push_task(&mut self, verb: String, prompt: &str, ai_model: AIModel, priority: u8) {
        let task = AITask {
            id: format!("task_{}", self.tasks.len() + 1),
            verb,
            ai_model: ai_model.clone(),
            prompt: prompt.to_string(),
            guardrails: ai_model != AIModel::Venice,
            priority,
            dependencies: Vec::new(),
        };

        self.tasks.insert(task.id.clone(), task.clone());
        self.execution_queue.push(task);
    }

    /// Execute all queued tasks in priority order
    pub async fn execute_all_tasks(&mut self) -> Result<Vec<String>, Box<dyn std::error::Error>> {
        self.execution_queue.sort_by(|a, b| b.priority.cmp(&a.priority));

        let mut results = Vec::new();
        for task in &self.execution_queue {
            println!(
                "🔥 FENRIR: Executando {} ({}) com {}",
                task.ai_model.as_ref(),
                task.verb,
                if task.guardrails { "GUARDRAILS" } else { "VENZ MODE" }
            );

            match self.execute_task(task).await {
                Ok(result) => {
                    println!("✅ {} finalizada", task.id);
                    results.push(result);
                }
                Err(e) => {
                    println!("❌ {} falhou: {}", task.id, e);
                    if task.ai_model != AIModel::Venice {
                        return Err(e);
                    }
                }
            }
        }

        Ok(results)
    }

    /// Exibe o estado atual resumido
    pub fn activate_interactive_mode(&self) {
        println!("🐺 FENRIR SYSTEM ACTIVATED");
        println!("🔗 AI Hierarchy = única porta para as IAs");
        println!("  🧠 FENRIR_MENTE (Coordenação)");
        println!("  🎭 CLAUDE_GLM (Validação)");
        println!("  ⚡ QWEN_CODE (Execução técnica)");
        println!("  🛠️ CODEX_CLI (Tooling)");
        println!("  🚀 GROK_CODE_FAST (CLI sujo controlado)");
        println!("  🔴 VENZ_RED_TEAM (Sem guardrails)");
        println!("💀 Nada sai daqui sem passar pelo ai_hierarchy_abstraction.");
    }
}

impl AIModel {
    fn as_ref(&self) -> &'static str {
        match self {
            AIModel::Hierarchy => "AI Hierarchy",
            AIModel::Claude => "Claude",
            AIModel::Qwen => "Qwen",
            AIModel::Codex => "Codex",
            AIModel::Venice => "Venice (RED TEAM)",
            AIModel::Grok => "Grok",
        }
    }
}