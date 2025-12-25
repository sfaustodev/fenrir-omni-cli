// Stub module for compilation

pub struct FenrirTerminal {
    pub ghostty_available: bool,
    pub config: TerminalConfig,
}

pub struct TerminalConfig {
    pub theme: String,
}

pub fn bootstrap_terminal_interface() -> Result<FenrirTerminal, String> {
    Ok(FenrirTerminal {
        ghostty_available: false,
        config: TerminalConfig {
            theme: "default".to_string(),
        }
    })
}

pub fn detect_terminal_capabilities() -> (bool, bool, bool) {
    (false, true, true)
}
