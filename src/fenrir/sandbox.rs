use nix::sys::resource::{setrlimit, Resource, Rlim};

/// Aplica sandbox básico com rlimit e landlock.
pub fn apply_sandbox() -> anyhow::Result<()> {
    let _ = setrlimit(Resource::RLIMIT_NOFILE, Rlim::from_raw(256), Rlim::from_raw(256));
    let _ = setrlimit(Resource::RLIMIT_NPROC, Rlim::from_raw(128), Rlim::from_raw(128));
    let _ = setrlimit(Resource::RLIMIT_FSIZE, Rlim::from_raw(1024 * 1024), Rlim::from_raw(1024 * 1024));
    apply_landlock();
    Ok(())
}

#[cfg(target_os = "linux")]
fn apply_landlock() {
    if std::env::var("FENRIR_LANDLOCK").ok().as_deref() != Some("1") {
        return;
    }
    let _ = (|| -> anyhow::Result<()> {
        let access = landlock::AccessFs::from_read_write();
        let mut ruleset = landlock::Ruleset::new()
            .handle_access(access)?
            .create()?;
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
