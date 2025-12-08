use std::{env, time::Duration};

use anyhow::{Context, Result, anyhow};
use argh::FromArgs;
use colored::*;
use reqwest::StatusCode;
use serde::{Deserialize, Serialize};

#[derive(FromArgs, Debug)]
/// Pequeno CLI para orquestrar chamadas ao Grok.
struct Args {
    /// texto a ser enviado ao modelo
    #[argh(option, short = 'p')]
    prompt: String,

    /// modelo Grok a ser utilizado
    #[argh(option, short = 'm', default = "default_model()")]
    model: String,

    /// temperatura de amostragem (0 a 1)
    #[argh(option, default = "0.7")]
    temperature: f32,

    /// limite de tokens da resposta
    #[argh(option, default = "512")]
    max_tokens: u32,

    /// timeout da requisição (segundos)
    #[argh(option, default = "30")]
    timeout_secs: u64,
}

fn default_model() -> String {
    "grok-4.1".to_string()
}

#[derive(Serialize)]
struct ChatRequest<'a> {
    model: &'a str,
    messages: Vec<ChatMessage<'a>>,
    max_tokens: u32,
    temperature: f32,
}

#[derive(Serialize)]
struct ChatMessage<'a> {
    role: &'a str,
    content: &'a str,
}

#[derive(Deserialize)]
struct ChatResponse {
    choices: Vec<Choice>,
    usage: Option<Usage>,
}

#[derive(Deserialize)]
struct Choice {
    message: ChoiceMessage,
}

#[derive(Deserialize)]
struct ChoiceMessage {
    content: String,
}

#[derive(Deserialize)]
struct Usage {
    prompt_tokens: u32,
    completion_tokens: u32,
    total_tokens: u32,
}

#[derive(Deserialize)]
struct ErrorEnvelope {
    error: ApiError,
}

#[derive(Deserialize)]
struct ApiError {
    message: String,
}

#[tokio::main]
async fn main() -> Result<()> {
    let args: Args = argh::from_env();

    if args.prompt.trim().is_empty() {
        return Err(anyhow!("--prompt não pode ser vazio"));
    }

    let api_key = resolve_api_key()?;
    let client = reqwest::Client::builder()
        .timeout(Duration::from_secs(args.timeout_secs))
        .build()
        .context("falha ao configurar cliente HTTP")?;

    println!(
        "{} {}",
        "🚀 Enviando prompt para Grok:".bold(),
        args.model.bright_cyan()
    );

    let reply = call_grok(&client, &api_key, &args)
        .await
        .context("falha na chamada ao Grok")?;

    render_response(&reply);

    Ok(())
}

fn resolve_api_key() -> Result<String> {
    let candidates = ["KAT_KEY", "GROK_API_KEY", "XAI_API_KEY", "GLI_KEY"];

    for key in candidates {
        if let Ok(value) = env::var(key) {
            if !value.trim().is_empty() {
                return Ok(value);
            }
        }
    }

    Err(anyhow!(
        "nenhuma API key encontrada (KAT_KEY / GROK_API_KEY / XAI_API_KEY / GLI_KEY)"
    ))
}

async fn call_grok(client: &reqwest::Client, api_key: &str, args: &Args) -> Result<ChatResponse> {
    let payload = ChatRequest {
        model: &args.model,
        messages: vec![ChatMessage {
            role: "user",
            content: &args.prompt,
        }],
        max_tokens: args.max_tokens,
        temperature: args.temperature,
    };

    let response = client
        .post("https://api.x.ai/v1/chat/completions")
        .bearer_auth(api_key)
        .json(&payload)
        .send()
        .await
        .context("erro de transporte ao chamar a API")?;

    let status = response.status();
    let body = response
        .text()
        .await
        .context("falha ao ler corpo da resposta")?;

    if !status.is_success() {
        return Err(describe_error(status, &body));
    }

    serde_json::from_str::<ChatResponse>(&body)
        .context("não foi possível decodificar a resposta do Grok")
}

fn describe_error(status: StatusCode, body: &str) -> anyhow::Error {
    if let Ok(envelope) = serde_json::from_str::<ErrorEnvelope>(body) {
        return anyhow!("{}: {}", status, envelope.error.message);
    }

    anyhow!("{}: {}", status, body)
}

fn render_response(response: &ChatResponse) {
    let divider = "━".repeat(60);
    println!("\n{}", divider.bright_black());

    if let Some(first) = response.choices.first() {
        println!("{}", "Resposta".bold().green());
        println!("{}\n", first.message.content.trim());
    } else {
        println!("{}", "Nenhuma resposta recebida".bold().red());
    }

    if let Some(usage) = &response.usage {
        println!(
            "{} {} | {} {} | {} {}",
            "prompt:".bright_black(),
            usage.prompt_tokens,
            "resposta:".bright_black(),
            usage.completion_tokens,
            "total:".bright_black(),
            usage.total_tokens
        );
    }

    println!("{}", divider.bright_black());
}
