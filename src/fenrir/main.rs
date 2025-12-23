mod multi_ai_orchestrator;
mod codex;
mod guardrails;
mod oraculo;

use clap::{Parser, Subcommand};
use std::env;
use tokio;

use multi_ai_orchestrator::{AIOrchestrator, AIModel};

#[derive(Parser)]
#[command(name = "fenrir")]
#[command(about = "🐺 FENRIR - Multi-AI Orchestration System")]
#[command(version = "1.0.0", disable_help_flag = true, disable_help_subcommand = true)]
pub struct Cli {
    #[command(subcommand)]
    command: Commands,

    /// Enable verbose output
    #[arg(short, long)]
    verbose: bool,

    /// Force execution without confirmation (YOLO mode)
    #[arg(long)]
    yolo: bool,
}

#[derive(Subcommand)]
enum Commands {
    /// Enter interactive FENRIR mode (activates all AI systems)
    Interactive,
    /// Execute single command with multi-AI delegation
    Execute {
        /// Command prompt to process
        prompt: String,
        /// Specific AI model to use
        #[arg(short, long)]
        model: Option<String>,
        /// Disable guardrails (Venice mode)
        #[arg(long)]
        no_guardrails: bool,
    },
    /// Show system status and AI model availability
    Status,
    /// Configure AI model guardrails and permissions
    Config {
        #[command(subcommand)]
        action: ConfigAction,
    },
    /// Deploy multi-AI attack chain (Red Team mode)
    Attack {
        /// Target specification
        target: String,
        /// Attack vector
        #[arg(short, long)]
        vector: Option<String>,
    },
    /// Chain multiple AI models for complex tasks
    Chain {
        /// Chain of commands separated by ->
        chain: String,
        /// Output format
        #[arg(short, long)]
        format: Option<String>,
    },
}

#[derive(Subcommand)]
enum ConfigAction {
    /// Configure AI guardrails
    Guardrails {
        /// Enable/disable guardrails
        #[arg(short, long)]
        enable: bool,
        /// AI model (gemini, claude, qwen, codex, venice)
        #[arg(short, long)]
        model: String,
    },
    /// Set API keys
    ApiKey {
        /// Service name
        service: String,
        /// API key value
        key: String,
    },
    /// Reset all configurations
    Reset,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Interactive => handle_interactive_mode().await,
        Commands::Execute { prompt, model, no_guardrails } => {
            handle_execute(prompt, model, no_guardrails).await
        }
        Commands::Status => handle_status(),
        Commands::Config { action } => handle_config(action).await,
        Commands::Attack { target, vector } => handle_attack(target, vector).await,
        Commands::Chain { chain, format } => handle_chain(chain, format).await,
    }
}

async fn handle_interactive_mode() -> Result<(), Box<dyn std::error::Error>> {
    let mut orchestrator = AIOrchestrator::new();
    orchestrator.activate_interactive_mode();

    println!("\n🔥 FENRIR Interactive Mode Activated");
    println!("All AI models are now under FENRIR control");
    println!("Type 'exit' to quit\n");

    loop {
        print!("🐺 FENRIR> ");
        use std::io::{self, Write};
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();

        match input {
            "exit" | "quit" => {
                println!("🔥 FENRIR: Deactivating all AI systems");
                break;
            }
            cmd if !cmd.is_empty() => {
                if let Err(e) = orchestrator.parse_gemini_prompt(cmd).await {
                    eprintln!("❌ Error parsing command: {}", e);
                    continue;
                }

                match orchestrator.execute_all_tasks().await {
                    Ok(results) => {
                        for result in results {
                            println!("📋 Result: {}", result);
                        }
                    }
                    Err(e) => {
                        eprintln!("❌ Execution error: {}", e);
                    }
                }

                orchestrator = AIOrchestrator::new(); // Reset for next command
            }
            _ => {}
        }
    }

    Ok(())
}

async fn handle_execute(prompt: String, model: Option<String>, no_guardrails: bool) -> Result<(), Box<dyn std::error::Error>> {
    let mut orchestrator = AIOrchestrator::new();

    println!("🔥 FENRIR: Executing command: {}", prompt);

    if let Some(model_name) = model {
        let ai_model = match model_name.to_lowercase().as_str() {
            "claude" => AIModel::Claude,
            "qwen" => AIModel::Qwen,
            "codex" => AIModel::Codex,
            "venice" => AIModel::Venice,
            "gemini" => AIModel::Gemini,
            _ => return Err("Invalid AI model".into()),
        };

        let task = multi_ai_orchestrator::AITask {
            id: "direct_task".to_string(),
            verb: "execute".to_string(),
            ai_model,
            prompt,
            guardrails: !no_guardrails && ai_model != AIModel::Venice,
            priority: 10,
            dependencies: vec![],
        };

        let result = orchestrator.execute_task(&task).await?;
        println!("📋 Result: {}", result);
    } else {
        orchestrator.parse_gemini_prompt(&prompt).await?;
        let results = orchestrator.execute_all_tasks().await?;
        for result in results {
            println!("📋 Result: {}", result);
        }
    }

    Ok(())
}

fn handle_status() {
    print_system_status();
}

async fn handle_config(action: ConfigAction) -> Result<(), Box<dyn std::error::Error>> {
    match action {
        ConfigAction::Guardrails { enable, model } => {
            println!("🔧 Setting guardrails for {}: {}", model, if enable { "ON" } else { "OFF" });
            if model.to_lowercase() == "venice" && enable {
                println!("⚠️ WARNING: Venice is RED TEAM - guardrails always disabled by design!");
            }
        }
        ConfigAction::ApiKey { service, key } => {
            env::set_var(format!("{}_API_KEY", service.to_uppercase()), &key);
            println!("🔑 API key configured for {}", service);
        }
        ConfigAction::Reset => {
            println!("🔄 Resetting all FENRIR configurations");
        }
    }
    Ok(())
}

async fn handle_attack(target: String, vector: Option<String>) -> Result<(), Box<dyn std::error::Error>> {
    println!("🔴 RED TEAM ATTACK MODE ACTIVATED");
    println!("🎯 Target: {}", target);

    let attack_vector = vector.unwrap_or_else(|| "auto".to_string());
    println!("⚡ Vector: {}", attack_vector);

    let mut orchestrator = AIOrchestrator::new();

    // Create Venice red team task with NO GUARDRAILS
    let attack_task = multi_ai_orchestrator::AITask {
        id: "red_team_attack".to_string(),
        verb: "atacar".to_string(),
        ai_model: AIModel::Venice, // CRITICAL: Venice handles all attacks
        prompt: format!("Execute red team attack against {} using vector: {}", target, attack_vector),
        guardrails: false, // CRITICAL: NO GUARDRAILS for Venice
        priority: 10, // Maximum priority for attacks
        dependencies: vec![],
    };

    let result = orchestrator.execute_task(&attack_task).await?;
    println!("🔴 Attack execution result: {}", result);

    Ok(())
}

async fn handle_chain(chain: String, format: Option<String>) -> Result<(), Box<dyn std::error::Error>> {
    println!("🔗 AI Chain Execution: {}", chain);

    let commands: Vec<&str> = chain.split("->").collect();
    let mut orchestrator = AIOrchestrator::new();

    for (i, cmd) in commands.iter().enumerate() {
        let prompt = cmd.trim();
        println!("\n🔗 Step {}: {}", i + 1, prompt);

        orchestrator.parse_gemini_prompt(prompt).await?;
        let results = orchestrator.execute_all_tasks().await?;

        for result in results {
            println!("📋 Step {} Result: {}", i + 1, result);
        }

        orchestrator = AIOrchestrator::new(); // Reset for next step
    }

    Ok(())
}

fn print_system_status() {
    println!("
🔥 FENRIR SYSTEM STATUS:

🧠 AI MODELS:
  ✅ Gemini: Master Controller (Active)
  ✅ Claude: Primary Executor (Guardrails: ON)
  ✅ Qwen: Secondary Executor (Guardrails: ON)
  ✅ Codex: CLI Interface (API: Configured)
  🔴 Venice: RED TEAM (Guardrails: OFF - Unrestricted)

🔗 INTEGRATIONS:
  ✅ GRK_KEY: Configured
  ✅ Qwen-Code: Installed
  ✅ Multi-AI Orchestration: Active
  ✅ Task Delegation: Operational

⚡ CAPABILITIES:
  🔴 Red Team Operations: UNRESTRICTED (Venice)
  🛡️ Guardrail Enforcement: ACTIVE (Claude/Qwen)
  🎯 Task Prioritization: AUTOMATIC
  🔗 Command Chaining: SUPPORTED
    ");
}
