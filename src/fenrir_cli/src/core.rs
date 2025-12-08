//! # Módulo Core
//!
//! Ponto central de orquestração entre comandos e UI.
//! Responsável por coordenar operações e gerenciar estado global.

use crate::config::Config;
use crate::ui;

/// Estado global da aplicação Fenrir
pub struct FenrirCore {
    /// Configuração carregada
    pub config: Config,
    /// Modo verboso ativo
    pub verbose: bool,
}

impl FenrirCore {
    /// Cria uma nova instância do core
    pub fn new(config: Config, verbose: bool) -> Self {
        Self { config, verbose }
    }

    /// Verifica guardrails antes de executar uma operação
    pub fn check_guardrails(&self, operation: &str) -> GuardrailResult {
        // Verifica políticas de conteúdo
        if !self.config.allows_operation(operation) {
            return GuardrailResult::Blocked {
                reason: format!(
                    "Operação '{}' bloqueada por política de segurança",
                    operation
                ),
            };
        }

        GuardrailResult::Allowed
    }

    /// Log de operação (para auditoria)
    pub fn log_operation(&self, operation: &str, details: &str) {
        if self.config.content_policies.audit_logging {
            if self.verbose {
                ui::info(&format!("[AUDIT] {}: {}", operation, details));
            }
            // TODO: Implementar log em arquivo
        }
    }

    /// Imprime aviso de guardrail
    pub fn warn_guardrail(&self, message: &str) {
        ui::warning(&format!("Guardrail: {}", message));
    }
}

/// Resultado da verificação de guardrails
#[derive(Debug)]
pub enum GuardrailResult {
    /// Operação permitida
    Allowed,
    /// Operação bloqueada
    Blocked { reason: String },
    /// Operação permitida com aviso
    AllowedWithWarning { warning: String },
}

impl GuardrailResult {
    /// Verifica se a operação é permitida
    pub fn is_allowed(&self) -> bool {
        matches!(
            self,
            GuardrailResult::Allowed | GuardrailResult::AllowedWithWarning { .. }
        )
    }
}

/// Trait para operações que podem ser executadas pelo Fenrir
pub trait FenrirOperation {
    /// Nome da operação (para logs e auditoria)
    fn name(&self) -> &str;

    /// Descrição da operação
    fn description(&self) -> &str;

    /// Verifica se a operação requer permissões especiais
    fn requires_elevated(&self) -> bool {
        false
    }
}

/// Contexto de execução passado para comandos
pub struct ExecutionContext<'a> {
    pub core: &'a FenrirCore,
    pub verbose: bool,
}

impl<'a> ExecutionContext<'a> {
    pub fn new(core: &'a FenrirCore) -> Self {
        Self {
            verbose: core.verbose,
            core,
        }
    }
}
