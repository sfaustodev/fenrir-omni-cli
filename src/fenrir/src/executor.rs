// Módulo executor - Stub para compilação
// TODO: Implementar funcionalidade completa

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FenrirTask {
    pub ia_explanation: String,
    pub task_type: String,
    pub command_to_run: Option<String>,
    pub target_path: Option<String>,
    pub application: Option<String>,
}

pub async fn ask_for_confirmation(message: &str) -> bool {
    println!("{}", message);
    // TODO: Implementar leitura real de input
    true // Por enquanto sempre confirma
}

pub fn handle_execute_command(cmd: &str) {
    println!("Executando comando: {}", cmd);
    // TODO: Implementar execução real
}

pub fn handle_open_editor(app: &str, path: &str) {
    println!("Abrindo {} com {}", path, app);
    // TODO: Implementar abertura real
}

pub fn log_task(task: &FenrirTask) -> anyhow::Result<()> {
    println!("Fazendo log da tarefa: {:?}", task);
    // TODO: Implementar log real
    Ok(())
}
