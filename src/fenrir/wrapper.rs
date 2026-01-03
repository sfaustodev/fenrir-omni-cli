use serde::{Deserialize, Serialize};
use std::process::Command;

/// Especificação de wrapper gerado.
#[derive(Debug, Serialize, Deserialize)]
pub struct WrapperSpec {
    pub tool: String,
    pub help: String,
}

/// Gera wrapper básico a partir do --help.
pub fn generate_wrapper(tool: &str) -> anyhow::Result<WrapperSpec> {
    let output = Command::new(tool).arg("--help").output()?;
    let help = String::from_utf8_lossy(&output.stdout).to_string();
    Ok(WrapperSpec {
        tool: tool.to_string(),
        help,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn wrapper_spec_has_tool() {
        let spec = WrapperSpec {
            tool: "fenrir".into(),
            help: "help".into(),
        };
        assert_eq!(spec.tool, "fenrir");
    }
}
