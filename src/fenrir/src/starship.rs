// Módulo starship - Stub para compilação
// TODO: Implementar funcionalidade completa

pub struct FenrirStarship {
    pub config: StarshipConfig,
}

pub struct StarshipConfig {
    pub format: String,
    pub modules: Vec<String>,
    pub fenrir: FenrirSymbol,
}

pub struct FenrirSymbol {
    pub symbol: String,
}

pub fn initialize_fenrir_starship() -> FenrirStarship {
    FenrirStarship {
        config: StarshipConfig {
            format: "fenrir>".to_string(),
            modules: vec!["git".to_string(), "rust".to_string()],
            fenrir: FenrirSymbol {
                symbol: "🐺".to_string(),
            },
        },
    }
}

impl FenrirStarship {
    pub fn update_context(&mut self) {
        // TODO: Atualizar contexto real
    }

    pub fn render_for_terminal(&self, _ghostty_available: bool, _last_status: i32) -> String {
        format!("{}{} ", self.config.fenrir.symbol, self.config.format)
    }

    pub fn render_prompt(&self, _last_status: i32) -> String {
        format!("{}{} ", self.config.fenrir.symbol, self.config.format)
    }
}
