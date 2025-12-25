// Stub module for compilation

pub struct FenrirStarship {
    pub config: StarshipConfig,
}

pub struct StarshipConfig {
    pub format: String,
}

pub fn initialize_fenrir_starship() -> FenrirStarship {
    FenrirStarship {
        config: StarshipConfig {
            format: "fenrir".to_string(),
        }
    }
}
