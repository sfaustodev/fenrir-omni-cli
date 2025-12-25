// --- MÓDULO ORÁCULO ---
// Comunicação com Gemini AI para processar comandos

use serde::{Deserialize, Serialize};

use crate::executor::FenrirTask;

#[derive(Debug, Serialize, Deserialize)]
pub struct GeminiRequest {
    pub contents: Vec<Content>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Content {
    pub parts: Vec<Part>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Part {
    pub text: String,
}

#[derive(Debug, Deserialize)]
pub struct GeminiResponse {
    pub candidates: Vec<Candidate>,
}

#[derive(Debug, Deserialize)]
pub struct Candidate {
    pub content: Content,
}

// Função principal do Oráculo - chama Gemini com timeout
pub async fn chamar_gemini_com_timeout(consulta: &str) -> Result<FenrirTask, Box<dyn std::error::Error>> {
    // Simulação - implementação real conectaria com API Gemini
    let task = analisar_comando_local(consulta).await?;
    Ok(task)
}

// Análise local de comandos (fallback quando API não está disponível)
async fn analisar_comando_local(consulta: &str) -> Result<FenrirTask, Box<dyn std::error::Error>> {
    let consulta_lower = consulta.to_lowercase();

    // Análise simples de comandos
    if consulta_lower.contains("listar") || consulta_lower.contains("ls") {
        return Ok(FenrirTask {
            ia_explanation: "Listar arquivos no diretório atual".to_string(),
            task_type: "execute_command".to_string(),
            command_to_run: Some("ls -la".to_string()),
            target_path: None,
            application: None,
        });
    }

    if consulta_lower.contains("diretório") || consulta_lower.contains("pasta") {
        return Ok(FenrirTask {
            ia_explanation: "Mostrar diretório de trabalho atual".to_string(),
            task_type: "execute_command".to_string(),
            command_to_run: Some("pwd".to_string()),
            target_path: None,
            application: None,
        });
    }

    if consulta_lower.contains("abrir") && (consulta_lower.contains("vscode") || consulta_lower.contains("code")) {
        return Ok(FenrirTask {
            ia_explanation: "Abrir VS Code no diretório atual".to_string(),
            task_type: "execute_command".to_string(),
            command_to_run: Some("code .".to_string()),
            target_path: None,
            application: None,
        });
    }

    if consulta_lower.contains("limpar") || consulta_lower.contains("clear") {
        return Ok(FenrirTask {
            ia_explanation: "Limpar tela do terminal".to_string(),
            task_type: "execute_command".to_string(),
            command_to_run: Some("clear".to_string()),
            target_path: None,
            application: None,
        });
    }

    if consulta_lower.contains("data") || consulta_lower.contains("date") {
        return Ok(FenrirTask {
            ia_explanation: "Mostrar data e hora atual".to_string(),
            task_type: "execute_command".to_string(),
            command_to_run: Some("date".to_string()),
            target_path: None,
            application: None,
        });
    }

    // Se não reconhecer o comando
    Ok(FenrirTask {
        ia_explanation: format!("Comando não reconhecido: '{}'. Tente comandos como 'listar', 'diretório', 'abrir vscode', 'limpar', 'data'", consulta),
        task_type: "unknown".to_string(),
        command_to_run: None,
        target_path: None,
        application: None,
    })
}