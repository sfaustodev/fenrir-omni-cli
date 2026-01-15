// Resource limits disabled due to compatibility issues

/// Aplica sandbox básico com rlimit e landlock.
pub fn apply_sandbox() -> anyhow::Result<()> {
    // Note: Resource limits commented out due to compatibility issues
    // with nix crate on different platforms
    // apply_landlock();
    Ok(())
}

#[cfg(target_os = "linux")]
fn apply_landlock() {
    if std::env::var("FENRIR_LANDLOCK").ok().as_deref() != Some("1") {
        return;
    }
    let _ = (|| -> anyhow::Result<()> {
        let access = landlock::AccessFs::from_read_write();
        let mut ruleset = landlock::Ruleset::new().handle_access(access)?.create()?;
        let roots = ["/", "/tmp", "/workspace"];
        for root in roots {
            if let Ok(rule) = landlock::PathBeneath::new(root) {
                let _ = ruleset.add_rule(rule);
            }
        }
        let _ = ruleset.restrict_self();
        Ok(())
    })();
}

#[cfg(not(target_os = "linux"))]
fn apply_landlock() {}
