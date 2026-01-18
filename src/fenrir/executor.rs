// --- MÓDULO EXECUTOR ---
// Módulo responsável por executar as tarefas definidas pelo Oráculo

use std::io::{self, Write};
use std::process::Command;

pub struct FenrirTask {
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

// Função para executar comandos do sistema
pub fn handle_execute_command(command: &str) {
    println!("🐺 Executando comando: {}", command);

    // Check if command needs shell (has operators like &&, ||, |, ;, >, <)
    let needs_shell = command.contains("&&")
        || command.contains("||")
        || command.contains('|')
        || command.contains(';')
        || command.contains('>')
        || command.contains('<')
        || command.contains("cd ");

    let output = if needs_shell {
        // Use shell for compound commands
        Command::new("sh").arg("-c").arg(command).output()
    } else {
        // Simple command - execute directly
        let parts: Vec<&str> = command.split_whitespace().collect();
        if parts.is_empty() {
            eprintln!("❌ Comando vazio!");
            return;
        }
        let (cmd, args) = parts.split_first().unwrap();
        Command::new(cmd).args(args).output()
    };

    match output {
        Ok(output) => {
            if output.status.success() {
                println!("✅ Comando executado com sucesso!");
                if !output.stdout.is_empty() {
                    println!("Saída:\n{}", String::from_utf8_lossy(&output.stdout));
                }
            } else {
                eprintln!("❌ Erro na execução do comando!");
                if !output.stderr.is_empty() {
                    eprintln!("Erro:\n{}", String::from_utf8_lossy(&output.stderr));
                }
            }
        }
        Err(e) => {
            eprintln!("❌ Falha ao executar comando: {}", e);
        }
    }
}

// Função para abrir arquivos com aplicativos
pub fn handle_open_editor(application: &str, path: &str) {
    println!("🐺 Abrindo '{}' com aplicativo '{}'", path, application);

    match Command::new("open")
        .args(["-a", application, path])
        .output()
    {
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
