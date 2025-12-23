// 🔗 CHAIN MANAGER - Orquestrador principal do Chain-of-Caralho
// Claudao gerencia Geminho e Venz como um verdadeiro senior

use crate::task_management::{
    commit_system::{CommitInfo, CommitQueue},
    review_system::ReviewEngine,
    task::{TarefaFinha, TarefaFinhaBatch},
    team_profiles::{DeveloperProfile, Team},
};
use anyhow::{Context, Result};
use futures::future::join_all;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::OpenOptions;
use std::io::{BufRead, BufReader, Write};
use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug, Clone)]
struct TarefaFinhaConfig {
    titulo: String,
    descricao: String,
    complexity: crate::task_management::task::Complexity,
    priority: crate::task_management::task::Priority,
    estimated_minutes: u16,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChainOfCaralhoManager {
    pub team: Team,
    pub commit_queue: CommitQueue,
    pub review_engine: ReviewEngine,
    pub caderninhos: Vec<TarefaFinhaBatch>,
    pub completed_cadernos: Vec<TarefaFinhaBatch>,
    pub pilha_async: Vec<TarefaFinha>,
    pub metrics: ChainMetrics,
    pub execution_mode: ExecutionMode,
    pub last_autoregulation_epoch: Option<u64>,
    pub weekly_loop_started: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExternalTarefinhaPlan {
    pub titulo: String,
    pub descricao: String,
    pub priority: Option<crate::task_management::Priority>,
    pub complexity: Option<crate::task_management::Complexity>,
    pub estimated_minutes: Option<u16>,
    pub dependencies: Vec<String>,
    pub async_ok: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ExecutionMode {
    Sequential, // Uma por vez
    Parallel,   // Múltiplas simultâneas
    Hybrid,     // Inteligente
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChainMetrics {
    pub total_tarefinhas_created: usize,
    pub total_tarefinhas_completed: usize,
    pub average_completion_time: f32,
    pub developer_performance: HashMap<String, DeveloperMetrics>,
    pub batch_success_rate: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeveloperMetrics {
    pub tarefinhas_completed: usize,
    pub average_score: f32,
    pub success_rate: f32,
    pub total_time_minutes: u64,
    pub specialties_completed: Vec<String>,
}

struct AsyncOutcome {
    tarefinha: TarefaFinha,
    commit: Option<CommitInfo>,
    error: Option<String>,
}

impl ChainOfCaralhoManager {
    pub fn new() -> Self {
        let team = Team::dream_team();
        let senior = team.get_member("Claudao").unwrap().clone();

        Self {
            team: team.clone(),
            commit_queue: CommitQueue::new(),
            review_engine: ReviewEngine::new(senior),
            caderninhos: vec![],
            completed_cadernos: vec![],
            pilha_async: vec![],
            metrics: ChainMetrics {
                total_tarefinhas_created: 0,
                total_tarefinhas_completed: 0,
                average_completion_time: 0.0,
                developer_performance: HashMap::new(),
                batch_success_rate: 0.0,
            },
            execution_mode: ExecutionMode::Hybrid,
            last_autoregulation_epoch: None,
            weekly_loop_started: false,
        }
    }

    /// 🎯 CRIAR BATCH DE TAREFINHAS A PARTIR DE UM OBJETIVO
    pub fn create_batch_from_goal(&mut self, goal: String) -> Result<Option<String>> {
        self.run_weekly_log_review_if_due()?;
        println!("🎯 Analisando objetivo: {}", goal);
        println!("🧠 Claudao decompondo em tarefinhas impossíveis de errar...");

        let (batch_id, async_count) = self.generate_tarefinhas_for_goal(&goal)?;

        self.scream_chain_of_caralho(&goal);
        if let Some(ref id) = batch_id {
            if let Some(last) = self.caderninhos.last() {
                println!(
                    "✅ Caderninho {} criado com {} tarefinhas",
                    id,
                    last.tarefinhas.len()
                );
            }
        }

        if async_count > 0 {
            println!(
                "⚡ Pilha async carregada com {} tarefinhas paralelizáveis",
                async_count
            );
        }

        Ok(batch_id)
    }

    /// 🎯 CRIAR BATCH A PARTIR DE TAREFINHAS EXTERNAS (Fenrir_ORQ)
    pub fn create_batch_from_external(
        &mut self,
        goal: String,
        tasks: Vec<ExternalTarefinhaPlan>,
    ) -> Result<Option<String>> {
        self.run_weekly_log_review_if_due()?;
        println!("🎯 Recebendo tarefinhas externas para: {}", goal);

        let mut batch = TarefaFinhaBatch::new(goal.clone(), "Fenrir_ORQ".to_string());
        let mut async_pilha: Vec<TarefaFinha> = vec![];

        for task in tasks {
            let priority = task
                .priority
                .unwrap_or(crate::task_management::Priority::Medium);
            let complexity = task
                .complexity
                .unwrap_or(crate::task_management::Complexity::Pleno);
            let estimated_minutes = task.estimated_minutes.unwrap_or(15);

            let assignee = self.assign_tarefinha(&complexity);
            let mut tarefinha = TarefaFinha::new(
                task.titulo,
                task.descricao,
                assignee,
                priority,
                complexity,
                estimated_minutes,
            );
            tarefinha.dependencies = task.dependencies;

            if task.async_ok
                && tarefinha.dependencies.is_empty()
                && !matches!(
                    tarefinha.priority,
                    crate::task_management::Priority::Critical
                        | crate::task_management::Priority::High
                )
            {
                async_pilha.push(tarefinha);
            } else {
                batch.add_tarefa(tarefinha);
            }
        }

        let mut batch_id = None;

        if !batch.tarefinhas.is_empty() {
            let tarefinhas_count = batch.tarefinhas.len();
            self.metrics.total_tarefinhas_created += tarefinhas_count;
            batch_id = Some(batch.batch_id.clone());
            self.caderninhos.push(batch);
        }

        if !async_pilha.is_empty() {
            self.metrics.total_tarefinhas_created += async_pilha.len();
            self.pilha_async.extend(async_pilha);
        }

        Ok(batch_id)
    }

    /// Vozes internas berrando no terminal para todo mundo ver
    pub fn scream_chain_of_caralho(&self, goal: &str) {
        println!("\n🔊🔊🔊 CHAIN-OF-CARALHO ATIVADO PARA: {}", goal);
        println!("🧠 VÓZES: Fenrir_ORQ manda quebrar em micro-tarefas, Claudao revisa, Geminho chora, Venz/Grok executa.");
        println!(
            "🗒️ Caderninhos: {} | Concluídos: {} | Pilha async: {}",
            self.caderninhos.len(),
            self.completed_cadernos.len(),
            self.pilha_async.len()
        );
        println!(
            "📈 Métricas: total criadas {} | total concluídas {}",
            self.metrics.total_tarefinhas_created, self.metrics.total_tarefinhas_completed
        );
        println!("🔺 Execução: {:?}", self.execution_mode);
    }

    fn generate_tarefinhas_for_goal(&mut self, goal: &str) -> Result<(Option<String>, usize)> {
        let mut batch = TarefaFinhaBatch::new(goal.to_string(), "Claudao".to_string());
        let mut async_pilha: Vec<TarefaFinha> = vec![];

        // Análise inteligente do objetivo e decomposição
        let tarefinhas = self.decompose_goal(goal);

        for tarefinha_config in tarefinhas {
            let assignee = self.assign_tarefinha(&tarefinha_config.complexity);
            let tarefinha = TarefaFinha::new(
                tarefinha_config.titulo,
                tarefinha_config.descricao,
                assignee,
                tarefinha_config.priority,
                tarefinha_config.complexity,
                tarefinha_config.estimated_minutes,
            );

            if self.belongs_to_caderninho(&tarefinha) {
                batch.add_tarefa(tarefinha);
            } else {
                async_pilha.push(tarefinha);
            }
        }

        let mut batch_id = None;

        if !batch.tarefinhas.is_empty() {
            let tarefinhas_count = batch.tarefinhas.len();
            self.metrics.total_tarefinhas_created += tarefinhas_count;
            batch_id = Some(batch.batch_id.clone());
            self.caderninhos.push(batch);
        }

        if !async_pilha.is_empty() {
            self.metrics.total_tarefinhas_created += async_pilha.len();
            self.pilha_async.extend(async_pilha);
        }

        Ok((batch_id, self.pilha_async.len()))
    }

    fn belongs_to_caderninho(&self, tarefinha: &TarefaFinha) -> bool {
        matches!(
            tarefinha.priority,
            crate::task_management::Priority::Critical | crate::task_management::Priority::High
        ) || !tarefinha.dependencies.is_empty()
    }

    fn decompose_goal(&self, goal: &str) -> Vec<TarefaFinhaConfig> {
        let mut tarefinhas = vec![];

        // Tarefinhas padrão para qualquer objetivo
        if goal.to_lowercase().contains("modulo") || goal.to_lowercase().contains("sistema") {
            tarefinhas.extend_from_slice(&[
                TarefaFinhaConfig {
                    titulo: "Criar estrutura de diretórios".to_string(),
                    descricao: "Criar módulo principal com arquivos básicos".to_string(),
                    complexity: crate::task_management::Complexity::Junior,
                    priority: crate::task_management::Priority::Critical,
                    estimated_minutes: 5,
                },
                TarefaFinhaConfig {
                    titulo: "Implementar estrutura de dados".to_string(),
                    descricao: "Definir structs e enums principais".to_string(),
                    complexity: crate::task_management::Complexity::Pleno,
                    priority: crate::task_management::Priority::High,
                    estimated_minutes: 15,
                },
                TarefaFinhaConfig {
                    titulo: "Implementar lógica de negócio".to_string(),
                    descricao: "Criar funções principais do sistema".to_string(),
                    complexity: crate::task_management::Complexity::Senior,
                    priority: crate::task_management::Priority::High,
                    estimated_minutes: 30,
                },
                TarefaFinhaConfig {
                    titulo: "Adicionar testes unitários".to_string(),
                    descricao: "Criar testes para funcionalidades principais".to_string(),
                    complexity: crate::task_management::Complexity::Pleno,
                    priority: crate::task_management::Priority::Medium,
                    estimated_minutes: 20,
                },
                TarefaFinhaConfig {
                    titulo: "Documentar código".to_string(),
                    descricao: "Adicionar docs e comentários".to_string(),
                    complexity: crate::task_management::Complexity::Junior,
                    priority: crate::task_management::Priority::Low,
                    estimated_minutes: 10,
                },
            ]);
        }

        // Se for sobre chain-of-caralalo específico
        if goal.to_lowercase().contains("chain") || goal.to_lowercase().contains("hierarquia") {
            tarefinhas.push(TarefaFinhaConfig {
                titulo: "Implementar sistema de delegação".to_string(),
                descricao: "Criar sistema de delegação automática de tarefinhas".to_string(),
                complexity: crate::task_management::Complexity::Senior,
                priority: crate::task_management::Priority::Critical,
                estimated_minutes: 45,
            });
        }

        tarefinhas
    }

    fn assign_tarefinha(
        &self,
        complexity: &crate::task_management::Complexity,
    ) -> DeveloperProfile {
        self.team
            .get_best_candidate(complexity)
            .unwrap_or_else(|| self.team.get_member("Claudao").unwrap())
            .clone()
    }

    /// 🚀 EXECUTAR PRÓXIMA TAREFINHA DISPONÍVEL
    pub fn execute_next_tarefinha(&mut self) -> Result<Option<(TarefaFinha, CommitInfo)>> {
        // Encontrar próxima tarefinha executável
        let (batch_index, tarefinha_index) = self.find_next_executable_tarefinha()?;

        let tarefinha = {
            let batch = &mut self.caderninhos[batch_index];
            batch.tarefinhas.remove(tarefinha_index)
        };

        println!("🚀 Executando: {}", tarefinha.titulo);
        println!("👤 Assignee: {}", tarefinha.assignee.nickname);

        // Simular execução
        let mut tarefinha = tarefinha;
        self.simulate_tarefinha_execution(&mut tarefinha)?;

        // Criar commit
        let commit = CommitInfo::from_tarefinha(&tarefinha, &tarefinha.assignee);

        // Adicionar ao batch novamente (agora completed)
        {
            let batch = &mut self.caderninhos[batch_index];
            batch.tarefinhas.push(tarefinha.clone());
        }

        Ok(Some((tarefinha, commit)))
    }

    fn find_next_executable_tarefinha(&self) -> Result<(usize, usize)> {
        for (batch_idx, batch) in self.caderninhos.iter().enumerate() {
            for (tarefinha_idx, tarefinha) in batch.tarefinhas.iter().enumerate() {
                if matches!(
                    tarefinha.status,
                    crate::task_management::TarefaStatus::Pending
                ) {
                    return Ok((batch_idx, tarefinha_idx));
                }
            }
        }

        Err(anyhow::anyhow!(
            "Nenhuma tarefinha disponível para execução"
        ))
    }

    fn simulate_tarefinha_execution(&self, tarefinha: &mut TarefaFinha) -> Result<()> {
        tarefinha.start();

        // Simular tempo de execução baseado no developer
        let actual_minutes = (tarefinha.estimated_minutes as f32
            * tarefinha
                .assignee
                .get_time_multiplier(&tarefinha.complexity)) as u16;

        println!(
            "⏱️ Executando por {} minutos (estimado: {})",
            actual_minutes, tarefinha.estimated_minutes
        );

        // Simular completion
        let artifacts = vec![
            format!(
                "src/{}.rs",
                tarefinha.titulo.to_lowercase().replace(" ", "_")
            ),
            format!(
                "tests/test_{}.rs",
                tarefinha.titulo.to_lowercase().replace(" ", "_")
            ),
        ];

        tarefinha.complete(artifacts);

        // Simular possíveis erros baseados no developer
        match tarefinha.assignee.nickname.as_str() {
            "Geminho" => {
                if rand::random::<f32>() < 0.3 {
                    // 30% chance de erro
                    return self.simulate_tarefinha_failure(tarefinha);
                }
            }
            "Venz" => {
                if rand::random::<f32>() < 0.15 {
                    // 15% chance de erro
                    return self.simulate_tarefinha_failure(tarefinha);
                }
            }
            "Claudao" => {
                if rand::random::<f32>() < 0.05 {
                    // 5% chance de erro
                    return self.simulate_tarefinha_failure(tarefinha);
                }
            }
            _ => {}
        }

        Ok(())
    }

    fn simulate_tarefinha_failure(&self, tarefinha: &mut TarefaFinha) -> Result<()> {
        let error = tarefinha.assignee.get_error_message();
        tarefinha.fail(error);
        let _ = self.log_error_with_context(tarefinha, "caderninho_sync");

        // Tentar novamente se for Geminho (ele aprende com erros)
        if tarefinha.assignee.nickname == "Geminho" && tarefinha.attempts < 3 {
            println!("🔄 Geminho tentando novamente...");
            return self.simulate_tarefinha_execution(tarefinha);
        }

        Err(anyhow::anyhow!(
            "Tarefinha falhou: {}",
            tarefinha
                .error_message
                .as_ref()
                .unwrap_or(&"Erro desconhecido".to_string())
        ))
    }

    /// 🔄 PROCESSAR BATCH COMPLETO
    pub async fn process_batch(&mut self, batch_id: &str) -> Result<()> {
        println!("\n🔄 PROCESSANDO BATCH: {}", batch_id);
        println!("{}", "═".repeat(60));

        let batch_idx = self
            .caderninhos
            .iter()
            .position(|b| b.batch_id == batch_id)
            .context("Batch não encontrado")?;

        let mut completed_tarefinhas = 0;
        let total_tarefinhas = self.caderninhos[batch_idx].tarefinhas.len();

        while completed_tarefinhas < total_tarefinhas {
            match self.execute_next_tarefinha() {
                Ok(Some((tarefinha, commit))) => {
                    // Commit
                    self.commit_queue.add_commit(commit.clone());
                    self.commit_queue.process_next_commit()?;

                    // Review
                    self.review_engine
                        .submit_for_review(tarefinha.clone(), commit);
                    self.review_engine.process_next_review()?;

                    completed_tarefinhas += 1;

                    println!(
                        "📊 Progresso: {}/{} tarefinhas completas",
                        completed_tarefinhas, total_tarefinhas
                    );
                }
                Ok(None) => {
                    println!("ℹ️ Nenhuma tarefinha disponível");
                    break;
                }
                Err(e) => {
                    println!("❌ Erro na execução: {}", e);
                    break;
                }
            }
        }

        // Mover batch para completed
        let batch = self.caderninhos.remove(batch_idx);
        self.completed_cadernos.push(batch);

        println!("\n✅ BATCH {} CONCLUÍDO!", batch_id);
        self.update_metrics();

        Ok(())
    }

    /// 📊 ATUALIZAR MÉTRICAS
    fn update_metrics(&mut self) {
        self.metrics.total_tarefinhas_completed = self
            .completed_cadernos
            .iter()
            .map(|b| b.tarefinhas.len())
            .sum();

        // Atualizar métricas por developer
        for batch in &self.completed_cadernos {
            for tarefinha in &batch.tarefinhas {
                let dev_metrics = self
                    .metrics
                    .developer_performance
                    .entry(tarefinha.assignee.nickname.clone())
                    .or_insert(DeveloperMetrics {
                        tarefinhas_completed: 0,
                        average_score: 0.0,
                        success_rate: 0.0,
                        total_time_minutes: 0,
                        specialties_completed: vec![],
                    });

                dev_metrics.tarefinhas_completed += 1;
                if let Some(score) = tarefinha.review_score {
                    dev_metrics.average_score = (dev_metrics.average_score + score as f32) / 2.0;
                }

                if matches!(
                    tarefinha.status,
                    crate::task_management::TarefaStatus::Approved
                ) {
                    dev_metrics.success_rate += 0.1;
                }
            }
        }

        self.metrics.batch_success_rate = if !self.completed_cadernos.is_empty() {
            self.completed_cadernos
                .iter()
                .map(|b| b.get_completion_rate())
                .sum::<f32>()
                / self.completed_cadernos.len() as f32
        } else {
            0.0
        };
    }

    /// 📈 MOSTRAR DASHBOARD COMPLETO
    pub fn show_dashboard(&self) {
        println!("\n🔥 CHAIN-OF-CARALHO DASHBOARD");
        println!("{}", "═".repeat(60));

        println!("📊 GERAL:");
        println!(
            "   📋 Tarefinhas Criadas: {}",
            self.metrics.total_tarefinhas_created
        );
        println!(
            "   ✅ Tarefinhas Completas: {}",
            self.metrics.total_tarefinhas_completed
        );
        println!(
            "   📈 Taxa de Sucesso: {:.1}%",
            self.metrics.batch_success_rate * 100.0
        );

        println!("\n👥 PERFORMANCE POR DEVELOPER:");
        for (nickname, metrics) in &self.metrics.developer_performance {
            println!("   {}:", nickname);
            println!("     📋 Completas: {}", metrics.tarefinhas_completed);
            println!("     ⭐ Score Médio: {:.1}/10", metrics.average_score);
            println!("     ✅ Taxa Sucesso: {:.1}%", metrics.success_rate * 100.0);
        }

        println!("\n🔄 BATCHES ATIVOS: {}", self.caderninhos.len());
        println!("✅ BATCHES CONCLUÍDOS: {}", self.completed_cadernos.len());
        println!("⚡ PILHA ASYNC PENDENTE: {}", self.pilha_async.len());

        self.review_engine.show_metrics_dashboard();
        self.commit_queue.get_processed_count();
    }

    pub fn get_status(&self) -> ChainStatus {
        ChainStatus {
            active_batches: self.caderninhos.len(),
            completed_batches: self.completed_cadernos.len(),
            pending_commits: self.commit_queue.get_pending_count(),
            pending_reviews: self.review_engine.get_pending_count(),
            total_tarefinhas: self.metrics.total_tarefinhas_created,
            completion_rate: self.metrics.batch_success_rate,
        }
    }

    pub async fn process_pilha_async(&mut self) -> Result<()> {
        if self.pilha_async.is_empty() {
            println!("⚡ Pilha async vazia, nada para paralelizar.");
            return Ok(());
        }

        println!(
            "⚡ Disparando {} tarefinhas async em Tokio",
            self.pilha_async.len()
        );
        let mut backlog: Vec<TarefaFinha> = Vec::new();
        std::mem::swap(&mut backlog, &mut self.pilha_async);

        let handles: Vec<_> = backlog
            .into_iter()
            .map(|tarefinha| {
                tokio::spawn(async move { Self::execute_async_tarefinha(tarefinha).await })
            })
            .collect();

        let outcomes = join_all(handles).await;

        for outcome in outcomes {
            match outcome {
                Ok(async_outcome) => {
                    if let Some(err) = async_outcome.error.as_ref() {
                        self.log_error_with_context(
                            &async_outcome.tarefinha,
                            &format!("async_exec: {}", err),
                        )?;
                    }

                    if let Some(commit) = async_outcome.commit {
                        self.commit_queue.add_commit(commit.clone());
                        self.commit_queue.process_next_commit()?;

                        self.review_engine
                            .submit_for_review(async_outcome.tarefinha.clone(), commit);
                        self.review_engine.process_next_review()?;
                        self.metrics.total_tarefinhas_completed += 1;
                    }
                }
                Err(join_err) => {
                    let phantom = TarefaFinha::new(
                        "JoinError async".to_string(),
                        format!("Erro ao aguardar tarefa async: {}", join_err),
                        self.team.get_member("Claudao").unwrap().clone(),
                        crate::task_management::Priority::High,
                        crate::task_management::Complexity::Senior,
                        1,
                    );
                    self.log_error_with_context(&phantom, "tokio_join")?;
                }
            }
        }

        Ok(())
    }

    async fn execute_async_tarefinha(mut tarefinha: TarefaFinha) -> AsyncOutcome {
        tarefinha.start();
        let error_roll: f32 = rand::random();

        if error_roll < 0.2 {
            let msg = tarefinha.assignee.get_error_message();
            tarefinha.fail(msg.clone());
            return AsyncOutcome {
                tarefinha,
                commit: None,
                error: Some(msg),
            };
        }

        let artifacts = vec![
            format!(
                "async/src/{}.rs",
                tarefinha.titulo.to_lowercase().replace(" ", "_")
            ),
            format!(
                "async/tests/test_{}.rs",
                tarefinha.titulo.to_lowercase().replace(" ", "_")
            ),
        ];

        tarefinha.complete(artifacts);
        let commit = CommitInfo::from_tarefinha(&tarefinha, &tarefinha.assignee);

        AsyncOutcome {
            tarefinha,
            commit: Some(commit),
            error: None,
        }
    }

    fn log_error_with_context(&self, tarefinha: &TarefaFinha, stage: &str) -> Result<()> {
        let timestamp = SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs();
        let mut file = OpenOptions::new()
            .create(true)
            .append(true)
            .open("encaralhamento.log")?;

        writeln!(
            file,
            "[{}] stage:{} | titulo:{} | assignee:{} | attempts:{} | status:{:?} | erro:{}",
            timestamp,
            stage,
            tarefinha.titulo,
            tarefinha.assignee.nickname,
            tarefinha.attempts,
            tarefinha.status,
            tarefinha
                .error_message
                .as_deref()
                .unwrap_or("erro nao informado"),
        )?;

        println!("📜 encaralhamento.log recebeu erro com contexto completo.");
        Ok(())
    }

    pub fn start_weekly_scheduler(&mut self) {
        if self.weekly_loop_started {
            return;
        }

        self.weekly_loop_started = true;
        if let Err(err) = self.run_weekly_log_review_if_due() {
            eprintln!("❌ Falha ao rodar auto-regulação imediata: {}", err);
        }
    }

    fn run_weekly_log_review_if_due(&mut self) -> Result<()> {
        let now = SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs();
        let one_week = 60 * 60 * 24 * 7;

        if self
            .last_autoregulation_epoch
            .map_or(true, |last| now.saturating_sub(last) >= one_week)
        {
            self.analyze_error_log_and_schedule()?;
            self.last_autoregulation_epoch = Some(now);
        }

        Ok(())
    }

    fn analyze_error_log_and_schedule(&mut self) -> Result<()> {
        let log_path = PathBuf::from("encaralhamento.log");
        if !log_path.exists() {
            println!("🧹 Nenhum encaralhamento.log ainda, nada para revisar.");
            return Ok(());
        }

        let file = OpenOptions::new().read(true).open(&log_path)?;
        let reader = BufReader::new(file);
        let lines: Vec<String> = reader
            .lines()
            .filter_map(|l| l.ok())
            .filter(|l| !l.trim().is_empty())
            .collect();

        if lines.is_empty() {
            println!("🧹 encaralhamento.log vazio, sem erros para tratar.");
            return Ok(());
        }

        println!(
            "🔁 Auto-regulação semanal: {} erros pendentes no encaralhamento.log",
            lines.len()
        );

        let mut batch = TarefaFinhaBatch::new(
            "Revisar encaralhamento.log e blindar cadeia".to_string(),
            "Claudao".to_string(),
        );

        for (idx, line) in lines.iter().take(6).enumerate() {
            let tarefinha = TarefaFinha::new(
                format!("Revisar erro crítico #{}", idx + 1),
                format!("Contexto: {}", line),
                self.team.get_member("Claudao").unwrap().clone(),
                crate::task_management::Priority::Critical,
                crate::task_management::Complexity::Senior,
                10,
            );

            batch.add_tarefa(tarefinha);
        }

        self.metrics.total_tarefinhas_created += batch.tarefinhas.len();
        self.caderninhos.push(batch);

        println!(
            "✅ Caderninho semanal criado para lamber {} erros",
            lines.len().min(6)
        );
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChainStatus {
    pub active_batches: usize,
    pub completed_batches: usize,
    pub pending_commits: usize,
    pub pending_reviews: usize,
    pub total_tarefinhas: usize,
    pub completion_rate: f32,
}
