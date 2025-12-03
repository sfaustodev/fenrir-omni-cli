// Módulo Oráculo - agora protegido pelo AI Hierarchy Abstraction Layer

use crate::ai_hierarchy_abstraction::{
    execute_ai_command, ComplexityLevel, ExecutionContext, ExecutionPriority,
};
use crate::executor::FenrirTask;
use anyhow::Result;

pub async fn chamar_gemini_com_timeout(consulta: &str) -> Result<FenrirTask> {
    println!("Consultando Oráculo da Hierarquia FENRIR: {}", consulta);

    let context = ExecutionContext {
        priority: ExecutionPriority::High,
        complexity: ComplexityLevel::Complex,
        requires_censorship_bypass: consulta.contains("uncensored") || consulta.contains("venice"),
        mission_critical: true,
    };

    let ai_response = execute_ai_command(
        &format!("CHAIN-OF-CARALHA::ORACULO::{}", consulta),
        Some(&context),
    )
    .await?;

    let explanation = ai_response
        .result
        .clone()
        .unwrap_or_else(|| format!("AI Hierarchy processou: {}", consulta));

    let lower = consulta.to_lowercase();
    let mut task_type = "unknown".to_string();
    let mut command_to_run = None;
    let mut target_path = None;
    let mut application = None;

    if lower.contains("listar") || lower.contains("ls") {
        task_type = "execute_command".to_string();
        command_to_run = Some("ls -la".to_string());
    } else if lower.contains("pwd") || lower.contains("diretório") || lower.contains("onde estou")
    {
        task_type = "execute_command".to_string();
        command_to_run = Some("pwd".to_string());
    } else if lower.contains("abrir") && lower.contains("code") {
        task_type = "open_editor".to_string();
        application = Some("code".to_string());
        target_path = Some(".".to_string());
    }

    Ok(FenrirTask {
        ia_explanation: explanation,
        task_type,
        command_to_run,
        target_path,
        application,
    })
}
