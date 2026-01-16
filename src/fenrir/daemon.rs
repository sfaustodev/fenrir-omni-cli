// --- FENRIR DAEMON SERVICE ---
// Continuous security monitoring and vulnerability scanning

use crate::kali_tools_comprehensive::{FenrirOrchestrationEngine, BreachDetector};
use chrono::{DateTime, Utc};
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::Mutex;
use tokio::time;

#[derive(Debug, Clone)]
pub struct DaemonConfig {
    pub scan_interval: Duration,
    pub network_range: String,
    pub iot_scan_enabled: bool,
    pub app_scan_enabled: bool,
    pub breach_alerts_enabled: bool,
    pub auto_remediate: bool,
}

impl Default for DaemonConfig {
    fn default() -> Self {
        DaemonConfig {
            scan_interval: Duration::from_secs(3600), // 1 hour
            network_range: "192.168.1.0/24".to_string(),
            iot_scan_enabled: true,
            app_scan_enabled: true,
            breach_alerts_enabled: true,
            auto_remediate: false,
        }
    }
}

pub struct FenrirDaemon {
    pub config: DaemonConfig,
    pub engine: Arc<Mutex<FenrirOrchestrationEngine>>,
    pub breach_detector: Arc<Mutex<BreachDetector>>,
    pub is_running: Arc<Mutex<bool>>,
    pub last_scan: Arc<Mutex<Option<DateTime<Utc>>>>,
}

impl FenrirDaemon {
    pub fn new(target: String, config: DaemonConfig) -> Self {
        let engine = Arc::new(Mutex::new(FenrirOrchestrationEngine::new(target)));
        let breach_detector = Arc::new(Mutex::new(BreachDetector::new()));

        FenrirDaemon {
            config,
            engine,
            breach_detector,
            is_running: Arc::new(Mutex::new(false)),
            last_scan: Arc::new(Mutex::new(None)),
        }
    }

    pub async fn start(&self) -> Result<(), String> {
        let mut running = self.is_running.lock().await;
        if *running {
            return Err("Daemon is already running".to_string());
        }
        *running = true;
        drop(running);

        println!("🐺 FENRIR DAEMON STARTED");
        println!("🔄 Scan Interval: {:?}", self.config.scan_interval);
        println!("🌐 Network Range: {}", self.config.network_range);
        println!("📱 IoT Scan: {}", self.config.iot_scan_enabled);
        println!("📱 App Scan: {}", self.config.app_scan_enabled);
        println!("🚨 Breach Alerts: {}", self.config.breach_alerts_enabled);
        println!("🔧 Auto Remediate: {}\n", self.config.auto_remediate);

        let engine = Arc::clone(&self.engine);
        let breach_detector = Arc::clone(&self.breach_detector);
        let config = self.config.clone();
        let is_running = Arc::clone(&self.is_running);
        let last_scan = Arc::clone(&self.last_scan);

        tokio::spawn(async move {
            let mut interval = time::interval(config.scan_interval);

            loop {
                interval.tick().await;

                let running = is_running.lock().await;
                if !*running {
                    break;
                }
                drop(running);

                println!("\n🔄 [{}] DAEMON SCAN STARTING...", Utc::now().format("%H:%M:%S"));

                // Perform continuous scanning
                if let Err(e) = Self::perform_daemon_scan(&engine, &breach_detector, &config).await {
                    println!("❌ Daemon scan error: {}", e);
                }

                let mut last = last_scan.lock().await;
                *last = Some(Utc::now());
            }
        });

        Ok(())
    }

    pub async fn stop(&self) -> Result<(), String> {
        let mut running = self.is_running.lock().await;
        if !*running {
            return Err("Daemon is not running".to_string());
        }
        *running = false;
        println!("🐺 FENRIR DAEMON STOPPED");
        Ok(())
    }

    pub async fn status(&self) -> String {
        let running = self.is_running.lock().await;
        let last = self.last_scan.lock().await;

        let mut status = format!("🐺 FENRIR DAEMON STATUS\n");
        status.push_str(&format!("Running: {}\n", *running));
        status.push_str(&format!("Network Range: {}\n", self.config.network_range));
        status.push_str(&format!("Scan Interval: {:?}\n", self.config.scan_interval));

        if let Some(last_scan) = *last {
            status.push_str(&format!("Last Scan: {}\n", last_scan.format("%Y-%m-%d %H:%M:%S UTC")));
        } else {
            status.push_str("Last Scan: Never\n");
        }

        status
    }

    async fn perform_daemon_scan(
        engine: &Arc<Mutex<FenrirOrchestrationEngine>>,
        breach_detector: &Arc<Mutex<BreachDetector>>,
        config: &DaemonConfig,
    ) -> Result<(), String> {
        // Network device scanning
        println!("🌐 Scanning network devices...");
        Self::scan_network_devices(engine, config).await?;

        // IoT device scanning
        if config.iot_scan_enabled {
            println!("📱 Scanning IoT devices...");
            Self::scan_iot_devices(engine, config).await?;
        }

        // Application vulnerability scanning
        if config.app_scan_enabled {
            println!("📱 Scanning applications...");
            Self::scan_applications(engine, config).await?;
        }

        // Check for breaches
        if config.breach_alerts_enabled {
            Self::check_breaches(breach_detector).await?;
        }

        // Auto remediation (if enabled)
        if config.auto_remediate {
            Self::perform_auto_remediation(engine, breach_detector).await?;
        }

        println!("✅ Daemon scan cycle completed");
        Ok(())
    }

    async fn scan_network_devices(
        engine: &Arc<Mutex<FenrirOrchestrationEngine>>,
        config: &DaemonConfig,
    ) -> Result<(), String> {
        let mut eng = engine.lock().await;

        // Use nmap for network scanning
        let tools = eng.tools.iter()
            .filter(|t| format!("{:?}", t.category).contains("NetworkScanning"))
            .take(2)
            .cloned()
            .collect::<Vec<_>>();

        for tool in tools {
            if tool.name == "nmap" {
                let args = vec![config.network_range.clone(), "-sn".to_string()];
                match eng.execute_tool(&tool, &args).await {
                    Ok(output) => {
                        println!("📊 Network scan results: {} devices found", output.lines().count());
                    }
                    Err(e) => println!("⚠️ Network scan failed: {}", e),
                }
            }
        }

        Ok(())
    }

    async fn scan_iot_devices(
        engine: &Arc<Mutex<FenrirOrchestrationEngine>>,
        config: &DaemonConfig,
    ) -> Result<(), String> {
        let mut eng = engine.lock().await;

        // Look for IoT-specific tools and patterns
        let iot_tools = eng.tools.iter()
            .filter(|t| t.description.to_lowercase().contains("iot") ||
                      t.description.to_lowercase().contains("wireless") ||
                      t.name.contains("kismet"))
            .take(2)
            .cloned()
            .collect::<Vec<_>>();

        for tool in iot_tools {
            let args = vec![config.network_range.clone()];
            match eng.execute_tool(&tool, &args).await {
                Ok(output) => {
                    println!("📱 IoT scan results for {}: {} lines", tool.name, output.lines().count());
                }
                Err(e) => println!("⚠️ IoT scan failed for {}: {}", tool.name, e),
            }
        }

        Ok(())
    }

    async fn scan_applications(
        engine: &Arc<Mutex<FenrirOrchestrationEngine>>,
        config: &DaemonConfig,
    ) -> Result<(), String> {
        let mut eng = engine.lock().await;

        // Application vulnerability scanning
        let app_tools = eng.tools.iter()
            .filter(|t| format!("{:?}", t.category).contains("WebApplication") ||
                      format!("{:?}", t.category).contains("Vulnerability"))
            .take(3)
            .cloned()
            .collect::<Vec<_>>();

        for tool in app_tools {
            // Scan common ports/services
            let args = vec!["localhost".to_string()];
            match eng.execute_tool(&tool, &args).await {
                Ok(output) => {
                    println!("📱 App scan results for {}: {} findings", tool.name, output.lines().count());
                }
                Err(e) => println!("⚠️ App scan failed for {}: {}", tool.name, e),
            }
        }

        Ok(())
    }

    async fn check_breaches(breach_detector: &Arc<Mutex<BreachDetector>>) -> Result<(), String> {
        let detector = breach_detector.lock().await;

        if !detector.detected_breaches.is_empty() {
            println!("🚨 SECURITY BREACH DETECTED!");
            println!("Details: {} breaches found", detector.detected_breaches.len());

            for breach in &detector.detected_breaches {
                println!("  • {}: {}", format!("{:?}", breach.breach_type), breach.description);
            }

            // Trigger terminal alert
            println!("\n🔔 ALERT: Security breach detected - immediate attention required!");
        }

        Ok(())
    }

    async fn perform_auto_remediation(
        engine: &Arc<Mutex<FenrirOrchestrationEngine>>,
        breach_detector: &Arc<Mutex<BreachDetector>>,
    ) -> Result<(), String> {
        let detector = breach_detector.lock().await;

        if detector.detected_breaches.is_empty() {
            return Ok(());
        }

        println!("🔧 Performing auto-remediation...");

        // Basic remediation actions (would be expanded based on breach types)
        for breach in &detector.detected_breaches {
            match breach.breach_type {
                crate::kali_tools_comprehensive::BreachType::SQLInjection => {
                    println!("  • SQL Injection detected - recommending parameterized queries");
                }
                crate::kali_tools_comprehensive::BreachType::XSS => {
                    println!("  • XSS detected - recommending input sanitization");
                }
                _ => {
                    println!("  • {} detected - manual remediation required", format!("{:?}", breach.breach_type));
                }
            }
        }

        Ok(())
    }
}
