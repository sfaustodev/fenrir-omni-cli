// --- FENRIR DAEMON SERVICE ---
// Continuous security monitoring and vulnerability scanning

use crate::kali_tools_comprehensive::{FenrirOrchestrationEngine, BreachDetector};
use chrono::{DateTime, Utc};
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::Mutex;
use tokio::time;
use std::fs;
use std::path::Path;
use std::process::Command;
use tokio::fs as async_fs;

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

        // Make daemon persistent at boot
        Self::install_boot_service().await?;

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

                // Automatic virus scanning
                if let Err(e) = Self::perform_virus_scan().await {
                    println!("❌ Virus scan error: {}", e);
                }

                // File management for old files
                if let Err(e) = Self::cleanup_old_files().await {
                    println!("❌ File cleanup error: {}", e);
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

    /// Install daemon as a boot service (macOS: launchd, Linux: systemd)
    async fn install_boot_service() -> Result<(), String> {
        println!("🔧 Installing FENRIR daemon as boot service...");

        // Detect OS and use appropriate service manager
        if cfg!(target_os = "macos") {
            Self::install_launchd_service().await
        } else {
            Self::install_systemd_service().await
        }
    }

    /// Install systemd service (Linux)
    async fn install_systemd_service() -> Result<(), String> {
        let service_content = format!(r#"[Unit]
Description=Fenrir Security Daemon
After=network.target

[Service]
Type=simple
User={}
ExecStart={}
Restart=always
RestartSec=10

[Install]
WantedBy=multi-user.target
"#,
            whoami::username(),
            std::env::current_exe().map_err(|e| format!("Failed to get exe path: {}", e))?
                .to_string_lossy()
        );

        let service_path = "/etc/systemd/system/fenrir-daemon.service";
        if let Err(e) = async_fs::write(service_path, service_content).await {
            println!("⚠️  Failed to create systemd service (may require sudo): {}", e);
            return Ok(()); // Don't fail, just warn
        }

        // Enable and start service
        let _ = Command::new("systemctl")
            .args(&["daemon-reload"])
            .status();

        let _ = Command::new("systemctl")
            .args(&["enable", "fenrir-daemon"])
            .status();

        let _ = Command::new("systemctl")
            .args(&["start", "fenrir-daemon"])
            .status();

        println!("✅ Daemon installed as systemd service");
        Ok(())
    }

    /// Install launchd service (macOS)
    async fn install_launchd_service() -> Result<(), String> {
        let plist_content = format!(r#"<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
    <key>Label</key>
    <string>com.fenrir.daemon</string>
    <key>ProgramArguments</key>
    <array>
        <string>{}</string>
        <string>daemon</string>
        <string>start</string>
    </array>
    <key>RunAtLoad</key>
    <true/>
    <key>KeepAlive</key>
    <true/>
    <key>StandardOutPath</key>
    <string>/tmp/fenrir-daemon.out</string>
    <key>StandardErrorPath</key>
    <string>/tmp/fenrir-daemon.err</string>
    <key>UserName</key>
    <string>{}</string>
</dict>
</plist>
"#,
            std::env::current_exe().map_err(|e| format!("Failed to get exe path: {}", e))?
                .to_string_lossy(),
            whoami::username()
        );

        let plist_path = format!("/Users/{}/Library/LaunchAgents/com.fenrir.daemon.plist", whoami::username());
        if let Err(e) = async_fs::write(&plist_path, plist_content).await {
            println!("⚠️  Failed to create launchd plist: {}", e);
            return Ok(()); // Don't fail, just warn
        }

        // Load and start the service
        let _ = Command::new("launchctl")
            .args(&["load", &plist_path])
            .status();

        let _ = Command::new("launchctl")
            .args(&["start", "com.fenrir.daemon"])
            .status();

        println!("✅ Daemon installed as launchd service");
        Ok(())
    }

    /// Automatic virus scanning on HDD
    async fn perform_virus_scan() -> Result<(), String> {
        println!("🛡️  Performing automatic virus scan...");

        if cfg!(target_os = "macos") {
            Self::macos_virus_scan().await
        } else {
            Self::linux_virus_scan().await
        }
    }

    /// Linux virus scanning with clamscan
    async fn linux_virus_scan() -> Result<(), String> {
        // Use clamscan if available
        match Command::new("clamscan")
            .args(&["-r", "/home", "--quiet", "--infected"])
            .output()
        {
            Ok(output) => {
                let stdout = String::from_utf8_lossy(&output.stdout);
                let stderr = String::from_utf8_lossy(&output.stderr);

                if !stdout.is_empty() || !stderr.is_empty() {
                    println!("🚨 VIRUS SCAN ALERT:");
                    if !stdout.is_empty() {
                        println!("  Infected files found:\n{}", stdout);
                    }
                    if !stderr.is_empty() {
                        println!("  Scan errors:\n{}", stderr);
                    }

                    // Alert user
                    Self::send_alert("Virus scan detected infected files!".to_string()).await;
                } else {
                    println!("✅ No viruses detected");
                }
            }
            Err(_) => {
                // Fallback: check for suspicious files manually
                Self::manual_virus_check().await?;
            }
        }

        Ok(())
    }

    /// macOS virus scanning with XProtect or manual check
    async fn macos_virus_scan() -> Result<(), String> {
        // Try XProtect (built-in macOS antivirus)
        match Command::new("xprotect")
            .args(&["--version"])
            .output()
        {
            Ok(_) => {
                // XProtect is available, run a basic check
                println!("🛡️  Using XProtect for virus scanning...");
                // XProtect doesn't have a direct scan command, so we'll do manual checks
                Self::manual_virus_check().await?;
            }
            Err(_) => {
                // XProtect not available or not accessible, manual check
                Self::manual_virus_check().await?;
            }
        }

        Ok(())
    }

    /// Manual virus checking when clamscan not available
    async fn manual_virus_check() -> Result<(), String> {
        println!("🔍 Performing manual virus check...");

        let suspicious_extensions = vec![
            "exe", "dll", "bat", "cmd", "scr", "pif", "com", "vbs", "js", "jar",
            "deb", "rpm", "dmg", "pkg", "app", "apk",
        ];

        let mut suspicious_files = Vec::new();

        // Scan common directories (macOS compatible)
        let scan_dirs = if cfg!(target_os = "macos") {
            vec!["/Users", "/tmp", "/var/tmp", "/usr/local/bin", "/Applications"]
        } else {
            vec!["/home", "/tmp", "/var/tmp", "/usr/local/bin"]
        };

        for dir in scan_dirs {
            if let Ok(entries) = async_fs::read_dir(dir).await {
                let mut entries = entries;
                while let Ok(Some(entry)) = entries.next_entry().await {
                    if let Ok(metadata) = entry.metadata().await {
                        // Check file permissions (executable by others)
                        let permissions = metadata.permissions();
                        if permissions.readonly() == false {
                            if let Some(ext) = entry.path().extension() {
                                if suspicious_extensions.contains(&ext.to_str().unwrap_or("")) {
                                    suspicious_files.push(entry.path().to_string_lossy().to_string());
                                }
                            }
                        }

                        // Check for hidden files
                        if entry.file_name().to_string_lossy().starts_with('.') &&
                           entry.file_name() != ".bashrc" && entry.file_name() != ".profile" {
                            suspicious_files.push(entry.path().to_string_lossy().to_string());
                        }
                    }
                }
            }
        }

        if !suspicious_files.is_empty() {
            println!("🚨 SUSPICIOUS FILES DETECTED:");
            for file in suspicious_files.iter().take(10) {
                println!("  • {}", file);
            }
            if suspicious_files.len() > 10 {
                println!("  ... and {} more", suspicious_files.len() - 10);
            }

            Self::send_alert(format!("{} suspicious files detected", suspicious_files.len())).await;
        } else {
            println!("✅ No suspicious files found");
        }

        Ok(())
    }

    /// File management: cleanup old files not opened for >1 month
    async fn cleanup_old_files() -> Result<(), String> {
        println!("🧹 Performing file cleanup...");

        let one_month_ago = Utc::now() - chrono::Duration::days(30);
        let mut files_to_cleanup = Vec::new();

        // Scan user directories
        let user_dirs = vec![
            dirs::home_dir().unwrap_or_else(|| Path::new("/home").to_path_buf()),
            Path::new("/tmp").to_path_buf(),
            Path::new("/var/tmp").to_path_buf(),
        ];

        for dir in user_dirs {
            if let Ok(entries) = async_fs::read_dir(&dir).await {
                let mut entries = entries;
                while let Ok(Some(entry)) = entries.next_entry().await {
                    if let Ok(metadata) = entry.metadata().await {
                        if metadata.is_file() {
                            if let Ok(modified) = metadata.modified() {
                                let modified_time = DateTime::<Utc>::from(modified);
                                if modified_time < one_month_ago {
                                    // Check if file is opened (simple check)
                                    if !Self::is_file_opened(&entry.path()).await {
                                        files_to_cleanup.push(entry.path());
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }

        // Clean up old files
        let mut cleaned_count = 0;
        for file_path in files_to_cleanup.iter().take(100) { // Limit to 100 files per run
            match async_fs::remove_file(file_path).await {
                Ok(_) => {
                    println!("  🗑️  Cleaned: {}", file_path.display());
                    cleaned_count += 1;
                }
                Err(e) => {
                    println!("  ⚠️  Failed to clean {}: {}", file_path.display(), e);
                }
            }
        }

        if cleaned_count > 0 {
            println!("✅ Cleaned {} old files", cleaned_count);
            Self::send_alert(format!("Cleaned {} old files", cleaned_count)).await;
        } else {
            println!("✅ No old files to clean");
        }

        Ok(())
    }

    /// Check if file is currently opened
    async fn is_file_opened(file_path: &Path) -> bool {
        // Simple check using lsof (if available)
        match Command::new("lsof")
            .arg(file_path)
            .output()
        {
            Ok(output) => !output.stdout.is_empty(),
            Err(_) => false, // Assume not opened if lsof fails
        }
    }

    /// Send alert to user
    async fn send_alert(message: String) {
        println!("🚨 ALERT: {}", message);

        // Try to send desktop notification
        let _ = Command::new("notify-send")
            .args(&["FENRIR Security Alert", &message])
            .status();

        // Log to system log
        let _ = Command::new("logger")
            .args(&["-t", "fenrir-daemon", &message])
            .status();
    }
}
