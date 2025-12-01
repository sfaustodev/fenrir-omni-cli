// FENRIR Multi-AI Coordinator - Chain of Thoughts to Action (COTOA)
use crate::grok_code_client::GrokCodeClient;
use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::env;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MultiAICoordinator {
    pub hierarchy_online: bool,
    pub grok_model: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskResult {
    pub success: bool,
    pub final_report: String,
    pub execution_log: Vec<String>,
    pub artifacts: Vec<String>,
}

impl MultiAICoordinator {
    pub fn new() -> Result<Self> {
        let grok_model =
            env::var("GROK_MODEL").unwrap_or_else(|_| "x-ai/grok-code-fast-1".to_string());

        println!("Multi-IA: Fenrir Hierarchy + Grok ready");
        Ok(Self {
            hierarchy_online: true,
            grok_model,
        })
    }

    pub async fn cotoa_process(&self, input: &str) -> Result<TaskResult> {
        let grok = GrokCodeClient::new()?;

        // Phase 1: Strategic analysis
        let strategic = grok
            .ask_with_context(
                "You are a strategic planner. Analyze the goal and break it into phases.",
                input,
            )
            .await?;

        // Phase 2: Technical planning
        let technical = grok
            .ask_with_context(
                "You are a technical architect. Create implementation steps from this strategy.",
                &strategic,
            )
            .await?;

        // Phase 3: Action items
        let actions = grok
            .ask_with_context(
                "Convert this plan into concrete actionable tasks. List each as a separate line.",
                &technical,
            )
            .await?;

        let artifacts: Vec<String> = actions
            .lines()
            .filter(|l| !l.trim().is_empty())
            .map(|l| l.trim().to_string())
            .collect();

        Ok(TaskResult {
            success: true,
            final_report: format!(
                "Strategy:\n{}\n\nTechnical:\n{}\n\nActions: {}",
                &strategic[..strategic.len().min(500)],
                &technical[..technical.len().min(500)],
                artifacts.len()
            ),
            execution_log: vec![
                "Strategic analysis done".to_string(),
                "Technical planning done".to_string(),
                format!("{} actions generated", artifacts.len()),
            ],
            artifacts,
        })
    }

    pub async fn strategic_analysis(&self, input: &str) -> Result<String> {
        let grok = GrokCodeClient::new()?;
        grok.ask_with_context("Analyze strategically and provide recommendations.", input)
            .await
    }

    pub async fn tactical_processing(&self, input: &str) -> Result<String> {
        let grok = GrokCodeClient::new()?;
        grok.ask_with_context("Process tactically and create an action plan.", input)
            .await
    }

    pub async fn technical_implementation(&self, input: &str) -> Result<String> {
        let grok = GrokCodeClient::new()?;
        grok.ask_with_context("Create technical implementation details.", input)
            .await
    }

    pub async fn uncensored_execution(&self, input: &str) -> Result<Vec<String>> {
        let grok = GrokCodeClient::new()?;
        let result = grok.ask(input).await?;
        Ok(result
            .lines()
            .filter(|l| !l.is_empty())
            .map(|s| s.to_string())
            .collect())
    }

    pub async fn process_complete_task(&self, input: &str) -> Result<TaskResult> {
        self.cotoa_process(input).await
    }

    pub async fn emergency_execution(&self, input: &str) -> Result<Vec<String>> {
        let grok = GrokCodeClient::new()?;
        grok.generate_dirty_commands(input).await
    }

    pub fn show_status(&self) {
        println!(
            "Multi-IA Status: Hierarchy={} Grok={}",
            if self.hierarchy_online { "ONLINE" } else { "offline" },
            self.grok_model
        );
    }
}
