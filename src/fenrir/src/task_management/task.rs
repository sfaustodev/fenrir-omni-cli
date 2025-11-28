// 📋 TASK SYSTEM - Atomic development units
// Professional task management with zero error margin

use crate::task_management::team_profiles::{DeveloperLevel, DeveloperProfile};
use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TarefaFinha {
    pub id: String,
    pub titulo: String,
    pub descricao: String,
    pub assignee: DeveloperProfile,
    pub priority: Priority,
    pub complexity: Complexity,
    pub estimated_minutes: u16,
    pub dependencies: Vec<String>,
    pub artifacts: Vec<String>,
    pub status: TarefaStatus,
    pub created_at: u64,
    pub started_at: Option<u64>,
    pub completed_at: Option<u64>,
    pub review_score: Option<u8>, // 1-10
    pub attempts: u8,
    pub error_message: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Priority {
    Critical, // Se falhar, tudo explode
    High,     // Bloqueia outras tarefinhas
    Medium,   // Importante mas não urgente
    Low,      // Nice to have
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Complexity {
    Junior,  // Até Geminho consegue
    Pleno,   // Pra Venz é tranquilo
    Senior,  // Só Claudao resolve
    GodMode, // Nem o chefe duvida
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TarefaStatus {
    Pending,    // Na fila
    InProgress, // Em execução
    Completed,  // Feita e commitada
    Failed,     // Precisa retry
    Reviewing,  // Claudao revisando
    Approved,   // Aprovada pelo senior
    Rejected,   // Refazer
}

impl TarefaFinha {
    pub fn new(
        titulo: String,
        descricao: String,
        assignee: DeveloperProfile,
        priority: Priority,
        complexity: Complexity,
        estimated_minutes: u16,
    ) -> Self {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();

        Self {
            id: format!("finha_{}", timestamp),
            titulo,
            descricao,
            assignee,
            priority,
            complexity,
            estimated_minutes,
            dependencies: vec![],
            artifacts: vec![],
            status: TarefaStatus::Pending,
            created_at: timestamp,
            started_at: None,
            completed_at: None,
            review_score: None,
            attempts: 0,
            error_message: None,
        }
    }

    pub fn start(&mut self) {
        self.status = TarefaStatus::InProgress;
        self.started_at = Some(
            SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs(),
        );
        self.attempts += 1;
    }

    pub fn complete(&mut self, artifacts: Vec<String>) {
        self.status = TarefaStatus::Completed;
        self.completed_at = Some(
            SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs(),
        );
        self.artifacts = artifacts;
    }

    pub fn fail(&mut self, error: String) {
        self.status = TarefaStatus::Failed;
        self.error_message = Some(error);
    }

    pub fn approve(&mut self, score: u8) {
        self.status = TarefaStatus::Approved;
        self.review_score = Some(score.min(10));
    }

    pub fn reject(&mut self, reason: String) {
        self.status = TarefaStatus::Rejected;
        self.error_message = Some(reason);
    }

    pub fn is_executable_by(&self, profile: &DeveloperProfile) -> bool {
        match (&self.complexity, &profile.level) {
            (Complexity::Junior, _) => true,
            (Complexity::Pleno, DeveloperLevel::Pleno | DeveloperLevel::Senior) => true,
            (Complexity::Senior, DeveloperLevel::Senior) => true,
            (Complexity::GodMode, DeveloperLevel::Senior) => true,
            _ => false,
        }
    }

    pub fn get_priority_weight(&self) -> u8 {
        match self.priority {
            Priority::Critical => 100,
            Priority::High => 75,
            Priority::Medium => 50,
            Priority::Low => 25,
        }
    }

    pub fn to_commit_message(&self) -> String {
        let prefix = match self.status {
            TarefaStatus::Completed => "feat",
            TarefaStatus::Approved => "fix",
            _ => "wip",
        };

        format!(
            "{}({}): {} [{} min] [{}]",
            prefix,
            self.assignee.nickname,
            self.titulo.to_lowercase().replace(" ", "-"),
            self.estimated_minutes,
            self.assignee.level
        )
    }

    pub fn get_execution_summary(&self) -> String {
        format!(
            "📋 TAREFINHA #{}\n\
            🔹 Título: {}\n\
            👤 Assignee: {} ({})\n\
            ⚡ Prioridade: {:?}\n\
            🎯 Complexidade: {:?}\n\
            ⏱️ Estimado: {} min\n\
            📊 Status: {:?}\n\
            🔄 Tentativas: {}\n\
            ⭐ Score: {:?}",
            self.id.split('_').last().unwrap_or("unknown"),
            self.titulo,
            self.assignee.nickname,
            self.assignee.level,
            self.priority,
            self.complexity,
            self.estimated_minutes,
            self.status,
            self.attempts,
            self.review_score
        )
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TarefaFinhaBatch {
    pub batch_id: String,
    pub parent_goal: String,
    pub tarefinhas: Vec<TarefaFinha>,
    pub created_by: String,
    pub deadline: Option<u64>,
}

impl TarefaFinhaBatch {
    pub fn new(parent_goal: String, created_by: String) -> Self {
        Self {
            batch_id: format!(
                "batch_{}",
                SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .unwrap()
                    .as_secs()
            ),
            parent_goal,
            tarefinhas: vec![],
            created_by,
            deadline: None,
        }
    }

    pub fn add_tarefa(&mut self, tarefinha: TarefaFinha) {
        self.tarefinhas.push(tarefinha);
    }

    pub fn get_pending_tarefinhas(&self) -> Vec<&TarefaFinha> {
        self.tarefinhas
            .iter()
            .filter(|t| matches!(t.status, TarefaStatus::Pending))
            .collect()
    }

    pub fn get_completion_rate(&self) -> f32 {
        if self.tarefinhas.is_empty() {
            return 0.0;
        }

        let completed = self
            .tarefinhas
            .iter()
            .filter(|t| matches!(t.status, TarefaStatus::Approved))
            .count();

        completed as f32 / self.tarefinhas.len() as f32
    }
}
