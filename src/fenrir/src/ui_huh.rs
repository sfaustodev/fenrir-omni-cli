use dialoguer::{theme::ColorfulTheme, Input, Select, MultiSelect};
use console::{style, Term};
use std::io::Write;

/// 🐺 FENRIR HUH EMULATOR - "Plagio na cara dura"
/// Replicates the UX of https://github.com/charmbracelet/huh
pub struct HuhEmulator {
    term: Term,
}

impl HuhEmulator {
    pub fn new() -> Self {
        Self {
            term: Term::stdout(),
        }
    }

    /// Displays a "Huh"-style text input
    pub fn input(&self, label: &str, placeholder: Option<&str>) -> anyhow::Result<String> {
        let theme = ColorfulTheme::default();
        let mut input = Input::<String>::with_theme(&theme);
        input = input.with_prompt(label);
        if let Some(ph) = placeholder {
            input = input.default(ph.to_string());
        }

        let value = input.interact_text()?;
        Ok(value)
    }

    /// Displays a "Huh"-style select
    pub fn select(&self, label: &str, items: &[&str]) -> anyhow::Result<usize> {
        let selection = Select::with_theme(&ColorfulTheme::default())
            .with_prompt(label)
            .default(0)
            .items(items)
            .interact()?;
        Ok(selection)
    }

    /// 💀 GROK INSULT MODE - When tasks fail or thinking is empty
    pub fn grok_insult(&mut self, message: &str) -> anyhow::Result<()> {
        self.term.clear_screen()?;
        
        // The "Huh" style header, but aggressive
        let header = style("💀 GROK FEEDBACK LOOP 💀").red().bold().on_black();
        self.term.write_line(&format!("\n{}\n", header))?;
        
        let insult = style(format!("⚠️  {}\n", message)).yellow().bold();
        self.term.write_line(&format!("\n{}\n", insult))?;
        
        self.term.write_line("PRESS ENTER TO ACKNOWLEDGE YOUR INCOMPETENCE")?;
        self.term.read_char()?;
        
        Ok(())
    }

    /// "Huh" style task status list
    pub fn render_task_status(&self, tasks: &[(&str, &str)]) -> anyhow::Result<()> {
        self.term.write_line("\n")?;
        for (task, status) in tasks {
            let symbol = match *status {
                "pending" => style("○").dim(),
                "running" => style("◐").cyan(),
                "success" => style("●").green(),
                "failed" => style("✖").red(),
                _ => style("?").yellow(),
            };
            
            self.term.write_line(&format!("  {} {}  {}\n", symbol, style(*task).bold(), style(*status).italic()))?;
        }
        self.term.write_line("\n")?;
        Ok(())
    }
}

pub fn run_demo() -> anyhow::Result<()> {
    let mut huh = HuhEmulator::new();
    
    huh.input("What is your command?", Some("destroy everything"))?;
    huh.select("Choose your destiny", &["God Mode", "Grok Insult", "Devour Cline"])?;
    
    huh.render_task_status(&[
        ("CloneCline", "success"),
        ("Analyze API Keys", "running"),
        ("Reimplement logic", "pending"),
    ])?;
    
    huh.grok_insult("REBANHO DE FILHA DA PUTA! CADÊ AS TAREFAS?")?;
    
    Ok(())
}
