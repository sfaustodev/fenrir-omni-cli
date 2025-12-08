use tokio::sync::mpsc;
use tokio::task;
use std::sync::Arc;
use tokio::sync::Mutex;
use crate::ui_huh::HuhEmulator;

#[derive(Debug, Clone)]
pub enum TaskStatus {
    Pending,
    InProgress,
    Success,
    Failed,
}

#[derive(Debug, Clone)]
pub struct CotoaTask {
    pub id: String,
    pub description: String,
    pub status: TaskStatus,
}

pub struct CotoaEngine {
    task_queue: Arc<Mutex<Vec<CotoaTask>>>,
    sender: mpsc::Sender<CotoaTask>,
    receiver: Arc<Mutex<mpsc::Receiver<CotoaTask>>>,
}

impl CotoaEngine {
    pub fn new() -> Self {
        let (tx, rx) = mpsc::channel(100);
        Self {
            task_queue: Arc::new(Mutex::new(Vec::new())),
            sender: tx,
            receiver: Arc::new(Mutex::new(rx)),
        }
    }

    pub async fn add_task(&self, description: String) -> anyhow::Result<()> {
        let task = CotoaTask {
            id: uuid::Uuid::new_v4().to_string(),
            description,
            status: TaskStatus::Pending,
        };
        
        // Add to queue log
        self.task_queue.lock().await.push(task.clone());
        
        // Send to async processor
        self.sender.send(task).await?;
        Ok(())
    }

    pub async fn run_loop(&self) -> anyhow::Result<()> {
        let receiver = self.receiver.clone();
        let queue_ref = self.task_queue.clone();
        
        // 🐺 UI Handler (Sync in its own thread/way usually, but here integrated)
        let mut ui = HuhEmulator::new();

        println!("🔥 COTOA ASYNC ENGINE STARTED");

        loop {
            let mut rx = receiver.lock().await;
            if let Some(mut task) = rx.recv().await {
                // Update status to Running
                {
                    let mut q = queue_ref.lock().await;
                    if let Some(t) = q.iter_mut().find(|t| t.id == task.id) {
                        t.status = TaskStatus::InProgress;
                    }
                }
                
                // Visualize
                ui.render_task_status(&[(&task.description, "running")])?;

                // Simulate Async Execution (Tokio Spawn)
                let task_desc = task.description.clone();
                let handle = task::spawn(async move {
                    // Here we would call the actual tools (git, cargo, etc.)
                    tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
                    // Simulation: if description contains "fail", it fails
                    if task_desc.contains("fail") {
                        Err(anyhow::anyhow!("Simulated failure"))
                    } else {
                        Ok(())
                    }
                });

                match handle.await? {
                    Ok(_) => {
                         task.status = TaskStatus::Success;
                         ui.render_task_status(&[(&task.description, "success")])?;
                    },
                    Err(_) => {
                        task.status = TaskStatus::Failed;
                        ui.render_task_status(&[(&task.description, "failed")])?;
                        // 💀 RECOVERY STRATEGY OR INSULT
                        ui.grok_insult(&format!("TASK FAILED: {}", task.description))?;
                    }
                }
            } else {
                // Channel closed or empty logic
                break;
            }
        }
        Ok(())
    }

    /// 💀 CHECK FOR "REBANHO" CONDITION
    /// If thinking round produced 0 tasks, Trigger Insult.
    pub async fn check_productivity(&self) -> anyhow::Result<()> {
        let count = self.task_queue.lock().await.len();
        if count == 0 {
            let mut ui = HuhEmulator::new();
            ui.grok_insult("REBANHO DE FILHA DA PUTA! ZERO TAREFAS GERADAS! PENSEM DIREITO!")?;
        }
        Ok(())
    }
}
