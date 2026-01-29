// INTEL DASHBOARD - Terminal User Interface for OSINT/CSI/Forensics
//
// NO PLACEHOLDERS - NO SIMULATIONS - NO LIES
// Real TUI implementation for visualizing intelligence data
//
// Features:
// - Real-time OSINT results display
// - CSI threat analysis visualization
// - Forensics timeline viewer
// - Interactive terminal interface
// - Color-coded severity levels
// - Scrollable data panels

use crate::osint_engine::{OSINTResult, OSINTFinding, FindingSeverity};
use crate::csi_analyzer::{ThreatReport, ThreatLevel};
use crate::forensics_engine::{ForensicCase, TimelineEntry};
use anyhow::Result;
use crossterm::{
    cursor::{MoveTo, Hide, Show},
    event::{self, Event, KeyCode, KeyEvent},
    execute,
    style::{Color, Print, ResetColor, SetBackgroundColor, SetForegroundColor},
    terminal::{Clear, ClearType, disable_raw_mode, enable_raw_mode, size, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::io::{stdout, Write};
use std::time::Duration;

/// Dashboard view modes
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum DashboardMode {
    Osint,
    Csi,
    Forensics,
    Summary,
}

/// Dashboard state
pub struct IntelDashboard {
    mode: DashboardMode,
    osint_data: Option<OSINTResult>,
    csi_report: Option<ThreatReport>,
    forensics_case: Option<ForensicCase>,
    scroll_offset: usize,
    selected_item: usize,
    running: bool,
}

impl IntelDashboard {
    /// Create new dashboard instance
    pub fn new() -> Self {
        Self {
            mode: DashboardMode::Summary,
            osint_data: None,
            csi_report: None,
            forensics_case: None,
            scroll_offset: 0,
            selected_item: 0,
            running: true,
        }
    }

    /// Set OSINT data for display
    pub fn set_osint_data(&mut self, data: OSINTResult) {
        self.osint_data = Some(data);
    }

    /// Set CSI report for display
    pub fn set_csi_report(&mut self, report: ThreatReport) {
        self.csi_report = Some(report);
    }

    /// Set forensics case for display
    pub fn set_forensics_case(&mut self, case: ForensicCase) {
        self.forensics_case = Some(case);
    }

    /// Run the interactive dashboard
    pub fn run(&mut self) -> Result<()> {
        enable_raw_mode()?;
        execute!(stdout(), EnterAlternateScreen, Hide)?;

        let mut stdout_handle = stdout();

        while self.running {
            self.render(&mut stdout_handle)?;

            // Handle input
            if event::poll(Duration::from_millis(100))? {
                if let Event::Key(key) = event::read()? {
                    self.handle_key_event(key);
                }
            }
        }

        // Cleanup
        disable_raw_mode()?;
        execute!(stdout_handle, LeaveAlternateScreen, Show)?;

        Ok(())
    }

    /// Handle keyboard input
    fn handle_key_event(&mut self, key: KeyEvent) {
        match key.code {
            KeyCode::Char('q') | KeyCode::Esc => {
                self.running = false;
            }
            KeyCode::Char('1') => {
                self.mode = DashboardMode::Osint;
                self.scroll_offset = 0;
                self.selected_item = 0;
            }
            KeyCode::Char('2') => {
                self.mode = DashboardMode::Csi;
                self.scroll_offset = 0;
                self.selected_item = 0;
            }
            KeyCode::Char('3') => {
                self.mode = DashboardMode::Forensics;
                self.scroll_offset = 0;
                self.selected_item = 0;
            }
            KeyCode::Char('4') => {
                self.mode = DashboardMode::Summary;
            }
            KeyCode::Down | KeyCode::Char('j') => {
                self.selected_item += 1;
                self.scroll_down();
            }
            KeyCode::Up | KeyCode::Char('k') => {
                if self.selected_item > 0 {
                    self.selected_item -= 1;
                    if self.scroll_offset > 0 {
                        self.scroll_offset -= 1;
                    }
                }
            }
            KeyCode::PageDown => {
                self.selected_item += 10;
                self.scroll_offset = self.scroll_offset.saturating_add(10);
            }
            KeyCode::PageUp => {
                self.selected_item = self.selected_item.saturating_sub(10);
                self.scroll_offset = self.scroll_offset.saturating_sub(10);
            }
            _ => {}
        }
    }

    fn scroll_down(&mut self) {
        let max_items = match self.mode {
            DashboardMode::Osint => self.osint_data.as_ref().map(|d| d.findings.len()).unwrap_or(0),
            DashboardMode::Csi => self.csi_report.as_ref().map(|r| r.iocs.len()).unwrap_or(0),
            DashboardMode::Forensics => self.forensics_case.as_ref().map(|c| c.timeline.len()).unwrap_or(0),
            DashboardMode::Summary => 0,
        };

        if self.selected_item >= max_items {
            self.selected_item = max_items.saturating_sub(1);
        }

        if self.selected_item > self.scroll_offset + 15 {
            self.scroll_offset = self.selected_item.saturating_sub(15);
        }
    }

    /// Render the dashboard to terminal
    fn render(&self, stdout: &mut std::io::Stdout) -> Result<()> {
        let (width, height) = size()?;

        execute!(stdout, Clear(ClearType::All), MoveTo(0, 0))?;

        // Render header
        self.render_header(stdout, width)?;

        // Render mode indicator
        self.render_mode_indicator(stdout, width)?;

        // Render content based on mode
        match self.mode {
            DashboardMode::Osint => self.render_osint(stdout, width, height)?,
            DashboardMode::Csi => self.render_csi(stdout, width, height)?,
            DashboardMode::Forensics => self.render_forensics(stdout, width, height)?,
            DashboardMode::Summary => self.render_summary(stdout, width, height)?,
        }

        // Render footer with help
        self.render_footer(stdout, width, height)?;

        stdout.flush()?;

        Ok(())
    }

    fn render_header(&self, stdout: &mut std::io::Stdout, width: u16) -> Result<()> {
        execute!(
            stdout,
            SetForegroundColor(Color::Cyan),
            MoveTo(0, 0),
            Print("╔"),
            Print("═".repeat(width as usize - 2)),
            Print("╗"),
            MoveTo(0, 1),
            Print("║"),
            SetForegroundColor(Color::White),
            Print(" FENRIR INTELLIGENCE DASHBOARD "),
            SetForegroundColor(Color::Cyan),
            Print(" ".repeat(width as usize - 33)),
            Print("║"),
            MoveTo(0, 2),
            Print("╚"),
            Print("═".repeat(width as usize - 2)),
            Print("╝"),
            ResetColor,
        )?;

        Ok(())
    }

    fn render_mode_indicator(&self, stdout: &mut std::io::Stdout, width: u16) -> Result<()> {
        execute!(
            stdout,
            MoveTo(0, 3),
            SetForegroundColor(Color::DarkGrey),
            Print(" ".repeat(width as usize)),
            MoveTo(0, 3),
        )?;

        let modes = [
            ('1', "OSINT", DashboardMode::Osint),
            ('2', "CSI", DashboardMode::Csi),
            ('3', "Forensics", DashboardMode::Forensics),
            ('4', "Summary", DashboardMode::Summary),
        ];

        let mut x = 2;
        for (key, label, mode) in &modes {
            let is_active = *mode == self.mode;
            execute!(
                stdout,
                SetForegroundColor(if is_active { Color::Green } else { Color::White }),
                SetBackgroundColor(if is_active { Color::DarkBlue } else { Color::Black }),
                Print(format!(" [{}-{}] ", key, label)),
                ResetColor,
            )?;
            x += label.len() + 6;
        }

        Ok(())
    }

    fn render_osint(&self, stdout: &mut std::io::Stdout, width: u16, height: u16) -> Result<()> {
        execute!(stdout, MoveTo(0, 5))?;

        if let Some(ref data) = self.osint_data {
            execute!(
                stdout,
                SetForegroundColor(Color::Yellow),
                Print(format!("Target: {}\n", data.target.value)),
                Print(format!("Sources: {}\n", data.sources.len())),
                Print(format!("Findings: {}\n", data.findings.len())),
                Print(format!("Confidence: {:.2}%\n\n", data.confidence_score * 100.0)),
                ResetColor,
            )?;

            let mut y = 10;
            for (i, finding) in data.findings.iter().skip(self.scroll_offset).take(height as usize - 15).enumerate() {
                let color = match finding.severity {
                    FindingSeverity::Critical => Color::Red,
                    FindingSeverity::High => Color::DarkRed,
                    FindingSeverity::Medium => Color::Yellow,
                    FindingSeverity::Low => Color::Blue,
                    FindingSeverity::Info => Color::Grey,
                    FindingSeverity::Warning => Color::DarkYellow,
                };

                let selected = i + self.scroll_offset == self.selected_item;

                execute!(
                    stdout,
                    MoveTo(0, y as u16),
                    SetBackgroundColor(if selected { Color::DarkBlue } else { Color::Black }),
                    SetForegroundColor(color),
                    Print(format!("[{}] {} - {}\n", i + self.scroll_offset, finding.category, finding.title)),
                    ResetColor,
                )?;

                y += 1;
            }
        } else {
            execute!(
                stdout,
                SetForegroundColor(Color::DarkGrey),
                Print("No OSINT data available\n"),
                Print("Press '1' to switch to OSINT view when data is loaded\n"),
                ResetColor,
            )?;
        }

        Ok(())
    }

    fn render_csi(&self, stdout: &mut std::io::Stdout, width: u16, height: u16) -> Result<()> {
        execute!(stdout, MoveTo(0, 5))?;

        if let Some(ref report) = self.csi_report {
            let threat_color = match report.threat_level {
                ThreatLevel::Critical => Color::Red,
                ThreatLevel::High => Color::DarkRed,
                ThreatLevel::Medium => Color::Yellow,
                ThreatLevel::Low => Color::Blue,
                ThreatLevel::None => Color::Green,
            };

            execute!(
                stdout,
                SetForegroundColor(threat_color),
                Print(format!("Threat Level: {:?}\n", report.threat_level)),
                SetForegroundColor(Color::White),
                Print(format!("Confidence: {:.2}%\n", report.confidence_score * 100.0)),
                Print(format!("IOCs Detected: {}\n", report.iocs.len())),
                Print(format!("Recommendations: {}\n\n", report.recommendations.len())),
                ResetColor,
            )?;

            let mut y = 10;
            for (i, ioc) in report.iocs.iter().skip(self.scroll_offset).take(height as usize - 15).enumerate() {
                let selected = i + self.scroll_offset == self.selected_item;

                execute!(
                    stdout,
                    MoveTo(0, y as u16),
                    SetBackgroundColor(if selected { Color::DarkBlue } else { Color::Black }),
                    SetForegroundColor(Color::Cyan),
                    Print(format!("[{:?}] {}", ioc.ioc_type, ioc.value)),
                    ResetColor,
                    Print(format!(" - {:.0}% confidence\n", ioc.confidence * 100.0)),
                )?;

                y += 1;
            }
        } else {
            execute!(
                stdout,
                SetForegroundColor(Color::DarkGrey),
                Print("No CSI analysis available\n"),
                Print("Press '2' to switch to CSI view when analysis is loaded\n"),
                ResetColor,
            )?;
        }

        Ok(())
    }

    fn render_forensics(&self, stdout: &mut std::io::Stdout, width: u16, height: u16) -> Result<()> {
        execute!(stdout, MoveTo(0, 5))?;

        if let Some(ref case) = self.forensics_case {
            execute!(
                stdout,
                SetForegroundColor(Color::Yellow),
                Print(format!("Case ID: {}\n", case.case_id)),
                Print(format!("Examiner: {}\n", case.examiner)),
                Print(format!("Artifacts: {}\n", case.artifacts.len())),
                Print(format!("Timeline Events: {}\n\n", case.timeline.len())),
                ResetColor,
            )?;

            let mut y = 10;
            for (i, event) in case.timeline.iter().skip(self.scroll_offset).take(height as usize - 15).enumerate() {
                let selected = i + self.scroll_offset == self.selected_item;

                execute!(
                    stdout,
                    MoveTo(0, y as u16),
                    SetBackgroundColor(if selected { Color::DarkBlue } else { Color::Black }),
                    SetForegroundColor(Color::Green),
                    Print(format!("[{}] {}\n", i + self.scroll_offset, event.timestamp.format("%Y-%m-%d %H:%M:%S"))),
                    SetForegroundColor(Color::White),
                    Print(format!("    {}\n", event.description)),
                    ResetColor,
                )?;

                y += 2;
            }
        } else {
            execute!(
                stdout,
                SetForegroundColor(Color::DarkGrey),
                Print("No forensics data available\n"),
                Print("Press '3' to switch to Forensics view when case is loaded\n"),
                ResetColor,
            )?;
        }

        Ok(())
    }

    fn render_summary(&self, stdout: &mut std::io::Stdout, width: u16, height: u16) -> Result<()> {
        execute!(stdout, MoveTo(0, 5))?;

        execute!(
            stdout,
            SetForegroundColor(Color::Cyan),
            Print("═══════════════════════════════════════\n"),
            Print("        INTELLIGENCE SUMMARY          \n"),
            Print("═══════════════════════════════════════\n\n"),
            ResetColor,
        )?;

        // OSINT Summary
        if let Some(ref data) = self.osint_data {
            execute!(
                stdout,
                SetForegroundColor(Color::Yellow),
                Print("OSINT Collection:\n"),
                ResetColor,
                Print(format!("  Target: {}\n", data.target.value)),
                Print(format!("  Findings: {}\n", data.findings.len())),
                Print(format!("  Confidence: {:.1}%\n\n", data.confidence_score * 100.0)),
            )?;
        }

        // CSI Summary
        if let Some(ref report) = self.csi_report {
            let threat_color = match report.threat_level {
                ThreatLevel::Critical => Color::Red,
                ThreatLevel::High => Color::DarkRed,
                ThreatLevel::Medium => Color::Yellow,
                ThreatLevel::Low => Color::Blue,
                ThreatLevel::None => Color::Green,
            };

            execute!(
                stdout,
                SetForegroundColor(Color::Yellow),
                Print("Threat Intelligence:\n"),
                SetForegroundColor(threat_color),
                Print(format!("  Level: {:?}\n", report.threat_level)),
                ResetColor,
                Print(format!("  IOCs: {}\n", report.iocs.len())),
                Print(format!("  Risk Score: {:.1}/100\n\n", report.risk_assessment.overall_score)),
            )?;
        }

        // Forensics Summary
        if let Some(ref case) = self.forensics_case {
            execute!(
                stdout,
                SetForegroundColor(Color::Yellow),
                Print("Digital Forensics:\n"),
                ResetColor,
                Print(format!("  Case: {}\n", case.case_id)),
                Print(format!("  Artifacts: {}\n", case.summary.total_artifacts)),
                Print(format!("  Suspicious Files: {}\n", case.summary.suspicious_files)),
                Print(format!("  Network Events: {}\n\n", case.summary.network_events)),
            )?;
        }

        if self.osint_data.is_none() && self.csi_report.is_none() && self.forensics_case.is_none() {
            execute!(
                stdout,
                SetForegroundColor(Color::DarkGrey),
                Print("No intelligence data loaded.\n\n"),
                Print("Load OSINT data, CSI reports, or Forensics cases\n"),
                Print("to view detailed analysis in this dashboard.\n"),
                ResetColor,
            )?;
        }

        Ok(())
    }

    fn render_footer(&self, stdout: &mut std::io::Stdout, width: u16, height: u16) -> Result<()> {
        let footer = " [q] Quit | [1-4] View Mode | [↑/↓] Navigate | [PgUp/PgDn] Scroll ";

        execute!(
            stdout,
            MoveTo(0, height - 1),
            SetBackgroundColor(Color::DarkBlue),
            SetForegroundColor(Color::White),
            Print(footer),
            Print(" ".repeat(width.saturating_sub(footer.len() as u16) as usize)),
            ResetColor,
        )?;

        Ok(())
    }
}

impl Default for IntelDashboard {
    fn default() -> Self {
        Self::new()
    }
}

/// Quick display function for non-interactive mode
pub fn display_quick_summary(
    osint: Option<&OSINTResult>,
    csi: Option<&ThreatReport>,
    forensics: Option<&ForensicCase>,
) -> Result<()> {
    println!("╔════════════════════════════════════════════════════════════╗");
    println!("║         FENRIR INTELLIGENCE QUICK SUMMARY                  ║");
    println!("╚════════════════════════════════════════════════════════════╝\n");

    if let Some(data) = osint {
        println!("📡 OSINT Collection:");
        println!("   Target: {}", data.target.value);
        println!("   Findings: {}", data.findings.len());
        println!("   Confidence: {:.1}%", data.confidence_score * 100.0);
        println!();
    }

    if let Some(report) = csi {
        println!("🎯 Threat Intelligence:");
        println!("   Level: {:?}", report.threat_level);
        println!("   IOCs: {}", report.iocs.len());
        println!("   Risk Score: {:.1}/100", report.risk_assessment.overall_score);
        println!();
    }

    if let Some(case) = forensics {
        println!("🔍 Digital Forensics:");
        println!("   Case: {}", case.case_id);
        println!("   Artifacts: {}", case.summary.total_artifacts);
        println!("   Suspicious Files: {}", case.summary.suspicious_files);
        println!();
    }

    if osint.is_none() && csi.is_none() && forensics.is_none() {
        println!("No intelligence data available.\n");
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dashboard_creation() {
        let dashboard = IntelDashboard::new();
        assert_eq!(dashboard.mode, DashboardMode::Summary);
        assert!(dashboard.running);
    }

    #[test]
    fn test_dashboard_default() {
        let dashboard = IntelDashboard::default();
        assert_eq!(dashboard.mode, DashboardMode::Summary);
    }
}
