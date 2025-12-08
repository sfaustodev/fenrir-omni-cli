// --- MÓDULO EXECUTOR ---
// Módulo responsável por executar as tarefas definidas pelo Oráculo

use std::process::Command;
use std::io::{self, Write};
use uuid::Uuid;
use std::time::Duration;
use tokio::time::timeout;

#[derive(Debug, Clone)]
pub struct FenrirTask {
    pub id: Uuid,
    pub ia_explanation: String,
    pub task_type: String,
    pub command_to_run: Option<String>,
    pub target_path: Option<String>,
    pub application: Option<String>,
}

// Função para confirmar execução com usuário
pub async fn ask_for_confirmation(prompt: &str) -> bool {
    print!("{}", prompt);
    io::stdout().flush().unwrap();

    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(_) => {
            let response = input.trim().to_lowercase();
            response == "s" || response == "sim" || response == "y" || response == "yes"
        }
        Err(_) => false,
    }
}

// Função para log de tarefas
pub fn log_task(task: &FenrirTask) -> io::Result<()> {
    let log_entry = format!(
        "[FENRIR] Tarefa: {} | Tipo: {} | Comando: {}\n",
        task.ia_explanation,
        task.task_type,
        task.command_to_run.as_deref().unwrap_or("N/A")
    );

    // Aqui poderia implementar log em arquivo
    print!("{}", log_entry);
    Ok(())
}

// Enum to represent the result of task execution
pub enum TaskExecutionResult {
    Success,
    CommandError(String),
    Timeout,
}

// Função para executar comandos do sistema
pub async fn handle_execute_command(command: &str) -> TaskExecutionResult {
    println!("🐺 Executando comando: {}", command);

    let parts: Vec<&str> = command.split_whitespace().collect();
    if parts.is_empty() {
        eprintln!("❌ Comando vazio!");
        return TaskExecutionResult::CommandError("Empty command".to_string());
    }

    let (cmd, args) = parts.split_first().unwrap();

    let command_future = tokio::process::Command::new(cmd)
        .args(args)
        .output();

    match timeout(Duration::from_secs(60), command_future).await {
        Ok(Ok(output)) => {
            if output.status.success() {
                println!("✅ Comando executado com sucesso!");
                if !output.stdout.is_empty() {
                    println!("Saída:\n{}", String::from_utf8_lossy(&output.stdout));
                }
                TaskExecutionResult::Success
            } else {
                eprintln!("❌ Erro na execução do comando!");
                if !output.stderr.is_empty() {
                    eprintln!("Erro:\n{}", String::from_utf8_lossy(&output.stderr));
                }
                TaskExecutionResult::CommandError(
                    format!("Command failed with status: {}", output.status)
                )
            }
        }
        Ok(Err(e)) => {
            eprintln!("❌ Falha ao executar comando '{}': {}", cmd, e);
            TaskExecutionResult::CommandError(format!("Failed to execute command: {}", e))
        }
        Err(_) => {
            eprintln!("⏰ Comando '{}' excedeu o tempo limite de 60 segundos!", command);
            TaskExecutionResult::Timeout
        }
    }
}

// Função para abrir arquivos com aplicativos
pub fn handle_open_editor(application: &str, path: &str) {
    println!("🐺 Abrindo '{}' com aplicativo '{}'", path, application);

    match Command::new("open").args(["-a", application, path]).output() {
        Ok(output) => {
            if output.status.success() {
                println!("✅ Arquivo aberto com sucesso!");
            } else {
                eprintln!("❌ Erro ao abrir arquivo!");
            }
        }
        Err(e) => {
            eprintln!("❌ Falha ao abrir arquivo: {}", e);
        }
    }
}