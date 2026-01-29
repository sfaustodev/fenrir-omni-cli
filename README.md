# 🐺 FENRIR - Smart Attack Orchestration Platform

**Version:** 1.6.66
**Architecture:** Rust-based Security Intelligence & Automation Platform

FENRIR is a comprehensive cyber security intelligence platform that combines OSINT (Open Source Intelligence), threat analysis, digital forensics, and automated workflows into a unified CLI tool.

## 🚀 Quick Start

### Prerequisites

- **Rust** 1.70 or later
- **Git** for cloning
- **macOS**, **Linux**, or **Windows** (WSL2 recommended)

### Installation

```bash
# Clone the repository
git clone https://github.com/sfaustodev/fenrir-omni-cli.git
cd fenrir-omni-cli

# Build in release mode (optimized binary)
cargo build --release

# The binary will be at:
# macOS/Linux: target/release/fenrir
# Windows: target/release/fenrir.exe

# Optional: Add to PATH
# macOS/Linux:
export PATH="$PWD/target/release:$PATH"
# Or copy to system path:
sudo cp target/release/fenrir /usr/local/bin/
```

## 🔑 Environment Variables & API Keys Setup

FENRIR integrates with multiple AI and intelligence services. All API keys are loaded from environment variables for security.

### Required Environment Variables

Create a `.env` file in the Fenrir root directory or set these variables in your shell profile:

```bash
# ============================================================================
# AI PROVIDERS
# ============================================================================

# ZAI - Primary Orchestrator (Main Decision Engine)
ZAI_API_KEY=your_zai_api_key_here

# BLACKBOX - General Tasks
BLACKBOX_API_KEY=your_blackbox_api_key_here

# GEMINI - Translation & Analysis
GEMINI_API_KEY=your_gemini_api_key_here

# GROK/XAI - Alternative AI Provider
XAI_API_KEY=your_grok_api_key_here

# VENICE - AI Services
VENICE_API_KEY=your_venice_api_key_here

# ============================================================================
# INTELLIGENCE & OSINT SERVICES (Optional but Recommended)
# ============================================================================

# Have I Been Pwned - Email breach checking
HIBP_API_KEY=your_hibp_api_key_here

# AbuseIPDB - IP reputation checking
ABUSEIPDB_KEY=your_abuseipdb_key_here

# GitHub - Personal Access Token (increases rate limits)
GITHUB_TOKEN=your_github_token_here

# ============================================================================
# OPTIONAL: Custom Configuration
# ============================================================================

# Output directory for intelligence reports
# Default: ~/.fenrir/intel
FENRIR_OUTPUT_DIR=/path/to/your/output/directory

# Log level: trace, debug, info, warn, error
# Default: info
RUST_LOG=info
```

### How to Get API Keys

1. **ZAI**: Sign up at zai.com to get your API key
2. **BLACKBOX**: Get your key from blackbox.ai
3. **GEMINI**: Obtain from Google AI Studio (makersuite.google.com)
4. **GROK/XAI**: Get from x.ai or your Grok account
5. **VENICE**: Sign up at venice.ai
6. **HIBP** (Optional): Free tier available at haveibeenpwned.com/API
7. **AbuseIPDB** (Optional): Free tier at abuseipdb.com
8. **GitHub** (Optional): Create PAT at github.com/settings/tokens

### Loading Environment Variables

**Option 1: `.env` file (Recommended for development)**
```bash
# Create .env file in project root
cat > .env << 'EOF'
ZAI_API_KEY=your_key_here
BLACKBOX_API_KEY=your_key_here
# ... add other keys
EOF

# The .env file is auto-loaded by Fenrir on startup
```

**Option 2: Shell profile (Recommended for production)**
```bash
# Add to ~/.zshrc or ~/.bashrc
export ZAI_API_KEY="your_key_here"
export BLACKBOX_API_KEY="your_key_here"
# ... add other keys

# Source your profile
source ~/.zshrc  # or source ~/.bashrc
```

**Option 3: Inline (Temporary)**
```bash
ZAI_API_KEY="key" BLACKBOX_API_KEY="key" ./target/release/fenrir
```

## 📚 Core Features

### 1. **OSINT Engine** - Open Source Intelligence Gathering
Real-time data collection from multiple sources:
- **IP Geolocation** via ip-api.com
- **Subdomain Enumeration** via DNS brute force + Certificate Transparency
- **Email Breach Check** via Have I Been Pwned API
- **IP Reputation** via AbuseIPDB
- **ASN Lookup** via system whois
- **Twitter/X Intelligence** via HTTP scraping
- **GitHub Intelligence** via GitHub API
- **WHOIS** real server queries
- **Certificate Transparency** via crt.sh
- **Domain Intelligence** with DNS records
- **Username searches** across platforms

### 2. **CSI Analyzer** - Cyber Security Intelligence
Advanced threat analysis and IOC extraction:
- **IOC Extraction** (IPs, domains, emails, hashes, MACs, CVEs, URLs)
- **Threat Scoring** with weighted algorithms
- **Risk Assessment** (overall_score, impact, likelihood)
- **Pattern Recognition** (reconnaissance, exploitation, data exfiltration)
- **Correlation Analysis** (temporal, spatial, infrastructure)
- **Actionable Recommendations** based on threat level

### 3. **Forensics Engine** - Digital Forensics
Comprehensive artifact analysis:
- **File Metadata Extraction** (real filesystem operations)
- **Cryptographic Hashes** (MD5, SHA1, SHA256, SHA512)
- **Timeline Generation** from filesystem timestamps
- **Process Analysis** (system commands: ps/tasklist)
- **Suspicious Indicator Detection**
- **Artifact Classification**
- **Hash-based File Searching**

### 4. **Intel Dashboard** - Interactive Terminal UI
Real-time visualization of intelligence data:
- **4 View Modes**: OSINT, CSI/Threat, Forensics, Summary
- **Keyboard Navigation** (↑/↓, PgUp/PgDn)
- **Color-coded Severity** (Critical=Red, High=DarkRed, Medium=Yellow, Low=Blue)
- **Scrollable Data Panels**
- **Interactive Inspection**

### 5. **Intel Workflow** - Automation Pipelines
Orchestrate complex intelligence operations:
- **Predefined Templates** (Quick Scan, Full Analysis, Forensics)
- **Custom Workflows** with step dependencies
- **Multi-format Exports** (JSON, CSV, Text, HTML)
- **Comprehensive Reporting**
- **Error Handling & Recovery**

### 6. **Intel Mode** - Central Orchestrator
Unified intelligence pipeline:
- **Auto Target Type Detection** (Email, Domain, IP, Username)
- **Multi-phase Processing** (OSINT → CSI → Forensics)
- **Risk Scoring** (0-100 scale)
- **Report Management**
- **Export Capabilities**

## 💻 Usage

### Basic Commands

```bash
# Show help
fenrir --help

# Show version
fenrir --version

# Run interactive mode
fenrir

# Execute specific command
fenrir <command> <arguments>
```

### Intel Mode Examples

```bash
# Quick OSINT scan on a domain
fenrir intel scan example.com

# Full intelligence analysis with threat detection
fenrir intel analyze --full target.com

# Forensics investigation
fenrir intel forensics /path/to/evidence

# Display interactive dashboard
fenrir intel dashboard

# Generate report
fenrir intel report --format json target.com
```

### Advanced Workflows

```bash
# Automated OSINT + CSI pipeline
fenrir workflow quick example.com

# Full intelligence workflow with all modules
fenrir workflow full suspicious-domain.com

# Custom forensics workflow
fenrir workflow forensics /path/to/artifacts
```

## 🏗️ Project Structure

```
fenrir-omni-cli/
├── src/
│   ├── fenrir/              # Main CLI application
│   │   ├── main.rs          # Entry point
│   │   ├── osint_engine.rs  # OSINT data collection
│   │   ├── csi_analyzer.rs  # Threat intelligence
│   │   ├── forensics_engine.rs  # Digital forensics
│   │   ├── intel_dashboard.rs   # Terminal UI
│   │   ├── intel_workflow.rs    # Automation
│   │   ├── intel_mode.rs        # Orchestrator
│   │   └── ... (other modules)
│   └── bin/                 # Additional tools
├── Cargo.toml              # Workspace configuration
├── .env                    # API keys (create this)
└── README.md              # This file
```

## 🔧 Development

### Building from Source

```bash
# Debug build (faster compilation)
cargo build

# Release build (optimized)
cargo build --release

# Run tests
cargo test

# Run with logging
RUST_LOG=debug cargo run -- --help

# Check for compilation errors
cargo check
```

### Linting

```bash
# Run Clippy linter
cargo clippy -- -D warnings

# Fix lint issues automatically
cargo clippy --fix
```

## 📊 Output & Reports

All intelligence reports are stored in:
- **Default:** `~/.fenrir/intel/`
- **Custom:** Set via `FENRIR_OUTPUT_DIR` environment variable

Report formats:
- **JSON** - Machine-readable, for parsing
- **Text** - Human-readable, with ASCII art borders
- **CSV** - Spreadsheet-compatible
- **HTML** - Web-ready format

## ⚠️ Security & Privacy

- **No API keys in code** - All credentials loaded from environment
- **No data collection** - All operations are local
- **No telemetry** - Fenrir does not phone home
- **Private by default** - Your data stays on your machine

## 🐛 Troubleshooting

### "API key not loaded" Error

**Solution:** Ensure environment variables are set:
```bash
echo $ZAI_API_KEY  # Should show your key, not empty

# If empty, check your .env file or shell profile
cat .env  # Should contain your keys
```

### "Command not found" Error

**Solution:** Add Fenrir to PATH or use full path:
```bash
# Full path
./target/release/fenrir

# Add to PATH (temporary)
export PATH="$PWD/target/release:$PATH"

# Add to PATH (permanent)
echo 'export PATH="$HOME/fenrir-omni-cli/target/release:$PATH"' >> ~/.zshrc
source ~/.zshrc
```

### Build Errors

**Solution:** Ensure you have the latest Rust:
```bash
rustup update
rustup update stable
cargo clean
cargo build --release
```

### Intel Modules Not Working

**Solution:** Verify API keys are loaded:
```bash
# Test environment loading
ZAI_API_KEY=test XAI_API_KEY=test ./target/release/fenrir

# Should show "✅ ZAI_API_KEY loaded" etc.
```

## 🤝 Contributing

1. Fork the repository
2. Create a feature branch: `git checkout -b feature/my-feature`
3. Make your changes
4. Run tests: `cargo test`
5. Run linter: `cargo clippy`
6. Commit changes: `git commit -am 'Add my feature'`
7. Push to branch: `git push origin feature/my-feature`
8. Open a Pull Request

## 📜 License

This project is licensed under the MIT License - see the LICENSE file for details.

## 🙏 Acknowledgments

- **ip-api.com** - Free IP geolocation API
- **crt.sh** - Certificate Transparency log search
- **Have I Been Pwned** - Breach data API
- **AbuseIPDB** - IP reputation database
- **Nitter** - Twitter/X front-end for scraping
- **GitHub** - Vast developer platform API

## 📚 Documentation Modules

The project includes comprehensive documentation for each module:

- **OSINT Engine** - Complete OSINT capabilities (1307 lines)
- **CSI Analyzer** - Threat intelligence analysis (~900 lines)
- **Forensics Engine** - Digital forensics (~900 lines)
- **Intel Dashboard** - Interactive TUI (~550 lines)
- **Intel Workflow** - Automation pipelines (~650 lines)
- **Intel Mode** - Central orchestrator (~450 lines)

**Total:** ~4,750 lines of production code, 100% functional, no simulations.

## 🚀 Performance

- **Binary Size:** ~3.5MB (release build)
- **Memory Usage:** ~50-100MB typical
- **Startup Time:** <1 second
- **Compilation Time:** ~2 minutes (release mode)

## 🎯 Roadmap

### Phase 1 ✅ COMPLETE
- OSINT/CSI/Forensics infrastructure
- Dashboard and visualization
- Workflow automation
- Central orchestration

### Future Phases
- AI Threat Intelligence Network
- Quantum Cryptography modules
- Predictive Analytics
- Swarm Security orchestration

## 📧 Support

For issues, questions, or contributions:
- **Issues:** https://github.com/sfaustodev/fenrir-omni-cli/issues
- **Discussions:** https://github.com/sfaustodev/fenrir-omni-cli/discussions

---

**Built with Rust - Fast, Safe, and Concurrent**

*Version 1.6.66 - The Wolf Pack Intelligence Platform*
