# 🐺 FENRIR CODEBASE AGENT GUIDE

## 🚀 Build, Lint & Test Commands

### Build Commands
```bash
# Production build with optimizations
cargo build --release

# Development build
cargo build

# Build specific binary
cargo build --bin fenrir
cargo build --bin fenrir-hunter
```

### Test Commands
```bash
# Run all tests
cargo test

# Run specific test (single test)
cargo test test_function_name

# Run tests for specific module
cargo test module_name

# Run tests with output
cargo test -- --nocapture

# Run specific test in specific file
cargo test --bin fenrir test_name
```

### Code Quality & Linting
```bash
# Run clippy (Rust linter)
cargo clippy

# Format code
cargo fmt

# Check formatting
cargo fmt --check

# Check build without compiling
cargo check
```

## 📝 Code Style Guidelines

### 🏗️ Project Structure
- **Workspace**: Multi-crate workspace with `src/fenrir/` and `src/bin/`
- **Main binaries**: `fenrir-cli` (main CLI) and `fenrir-hunter` (malware detection)
- **Modular design**: Each feature in separate module with `mod.rs`

### 📚 Imports & Dependencies
```rust
// Use workspace dependencies (defined in root Cargo.toml)
use tokio = { workspace = true }
use serde = { workspace = true }
use clap = { workspace = true }

// Standard library imports first
use std::collections::HashMap;
use std::path::PathBuf;

// External crates alphabetically
use anyhow::Result;
use colored::Colorize;
use serde::{Deserialize, Serialize};

// Internal modules last
use crate::ai_layer::ZaiOrchestrator;
use crate::security::CircuitBreaker;
```

### 🎨 Formatting Rules
- **rustfmt.toml**: Uses workspace defaults
- **Line length**: 100 characters max
- **Indentation**: 4 spaces (no tabs)
- **Braces**: Unix style with newlines
- **Trailing commas**: Required in multi-line structures

### 🏷️ Naming Conventions
- **Functions**: `snake_case`
- **Variables**: `snake_case`
- **Constants**: `UPPER_SNAKE_CASE`
- **Types**: `PascalCase`
- **Modules**: `snake_case`
- **Files**: `snake_case.rs`
- **Binaries**: `kebab-case`

### ⚡ Async Patterns
```rust
// Always use async/await
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let result = async_operation().await?;
    Ok(())
}

// Use Result types for error handling
async fn process_target(target: &str) -> Result<Vec<ScanResult>, ScanError> {
    // Implementation
}
```

### 🛡️ Error Handling
```rust
// Use anyhow for application errors
use anyhow::{anyhow, Result, Context};

// Use thiserror for custom error types
#[derive(Error, Debug)]
pub enum ScanError {
    #[error("Network timeout: {duration:?}")]
    Timeout { duration: Duration },
    #[error("Invalid target: {target}")]
    InvalidTarget { target: String },
}

// Proper error context
fn validate_target(target: &str) -> Result<()> {
    if target.is_empty() {
        return Err(anyhow!("Target cannot be empty"));
    }
    Ok(())
}
```

### 🔒 Security Patterns
```rust
// Secrets handling
use secrecy::{Secret, ExposeSecret};

// Never log secrets
fn process_secret(secret: &Secret<String>) {
    // Use secret.expose_secret() only when necessary
}

// Input validation
fn validate_input(input: &str) -> Result<()> {
    if input.contains("..") || input.contains('/') {
        return Err(anyhow!("Invalid path"));
    }
    Ok(())
}
```

## 🤖 AI Integration Guidelines

### Multi-AI Architecture
- **GLM 4.7**: Orchestrator (main decision maker)
- **Gemini**: Translation layer (Portuguese ↔ English)
- **Grok**: General tasks (guarded)
- **Venice Red Team**: Aggressive pentesting (unguarded)

### AI Safety Protocols
```rust
// Always validate AI output
fn validate_ai_response(response: &str) -> Result<()> {
    if response.contains("malicious") || response.contains("exploit") {
        return Err(anyhow!("AI response contains security concerns"));
    }
    Ok(())
}

// Use sandboxing for dangerous operations
#[cfg(feature = "sandbox")]
fn run_sandboxed_command(cmd: &str) -> Result<String> {
    // Isolated execution
}
```

## 🎯 Commit Guidelines

### Commit Message Format
```
<type>: <description>

# Types:
- feat: New feature
- fix: Bug fix
- docs: Documentation changes
- test: Test additions/changes
- refactor: Code refactoring
- perf: Performance improvements
- chore: Maintenance tasks

# Examples:
feat: Add Gemini AI translation layer
fix: Resolve memory leak in scan engine
docs: Update API documentation
test: Add unit tests for ethical protocol
```

### Git Workflow
```bash
# One commit per line changed (as specified)
git add file.rs
git commit -m "fix: Resolve null pointer in scan processor"
git push
```

## 📋 Code Review Checklist

### Before Submitting
- [ ] Code builds without warnings
- [ ] All tests pass
- [ ] Follows naming conventions
- [ ] Proper error handling
- [ ] Security best practices applied
- [ ] Documentation updated
- [ ] No hardcoded secrets
- [ ] Input validation implemented

### Security Review
- [ ] No sensitive data in logs
- [ ] Input validation for all user inputs
- [ ] Proper sandboxing for external commands
- [ ] Memory safety verified
- [ ] No hardcoded API keys

## 🔄 Development Workflow

### Adding New Features
1. Create feature branch: `git checkout -b feat/new-feature`
2. Implement with tests
3. Run: `cargo test && cargo clippy && cargo fmt`
4. Commit changes
5. Create PR with detailed description

### Bug Fixes
1. Create issue branch: `git checkout -b fix/bug-description`
2. Write failing test first (TDD)
3. Implement fix
4. Verify test passes
5. Run full test suite

## 📚 Documentation Standards

### Module Documentation
```rust
/// Brief description of the module.
///
/// # Examples
///
/// ```rust
/// let result = process_target("192.168.1.1").await?;
/// assert!(!result.is_empty());
/// ```
///
/// # Security Notes
///
/// This function performs network operations and should be used with caution.
pub mod security_scanner {
    // Implementation
}
```

### Function Documentation
```rust
/// Scans a target for security vulnerabilities.
///
/// # Arguments
///
/// * `target` - The IP address or hostname to scan
/// * `intensity` - Scan intensity level (0-10)
///
/// # Returns
///
/// * `Ok(Vec<Vulnerability>)` - List of found vulnerabilities
/// * `Err(ScanError)` - Scan failed
///
/// # Examples
///
/// ```
/// let vulnerabilities = scan_target("192.168.1.1", 5).await?;
/// ```
pub async fn scan_target(target: &str, intensity: u8) -> Result<Vec<Vulnerability>> {
    // Implementation
}
```

## 🚨 Important Notes

- **Legal compliance**: Only for authorized security testing
- **No hardcoded secrets**: Use environment variables
- **Input validation**: Always validate user inputs
- **Error handling**: Never panic, always return Result
- **Security first**: Follow security best practices
- **Performance**: Optimize for production use
- **Maintainability**: Write clear, readable code

## 🎯 Quick Reference

```bash
# Development cycle
cargo check          # Fast compilation check
cargo test           # Run tests
cargo clippy         # Lint code
cargo fmt            # Format code
cargo build --release # Production build

# Common patterns
Result<T, E>         # Error handling
async fn             # Async functions
#[tokio::main]       # Async main
use anyhow::Result   # Error handling
```