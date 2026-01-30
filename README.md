# 🐺 FENRIR - Smart Attack Orchestration Platform

**Version:** 1.6.66
**Architecture:** Rust-based Security Intelligence & Automation Platform

---

# ⚠️  CRITICAL ETHICAL & LEGAL DISCLAIMER

## 🚨 READ THIS BEFORE USING FENRIR

**FENRIR IS A POWERFUL SECURITY TOOL THAT MUST BE USED RESPONSIBLY AND ETHICALLY.**

### 🛡️ ACCEPTABLE USE ONLY

**YOU MAY ONLY USE FENRIR FOR:**
- ✅ Testing your **own** systems and infrastructure
- ✅ Systems you **own** or have **explicit written permission** to test
- ✅ **Authorized** penetration testing engagements
- ✅ **Bug bounty programs** with clearly defined scope
- ✅ **Educational purposes** with safe, authorized targets
- ✅ **Cybersecurity research** with proper ethical oversight
- ✅ **Ethical hacking** within legal boundaries

### 🚫 PROHIBITED USE (STRICTLY FORBIDDEN)

**YOU MUST NOT USE FENRIR FOR:**
- ❌ Testing systems **without written permission**
- ❌ Targeting **individuals' personal accounts** (email, social media, etc.)
- ❌ **Unauthorized access** to any computer system
- ❌ **Harassment**, stalking, or threatening any person or organization
- ❌ **Illegal activities** of any kind
- ❌ Violating **terms of service** of any platform
- ❌ **Data theft** or exfiltration without authorization
- ❌ Any activity that violates **local, state, national, or international laws**

### ⚖️ LEGAL CONSEQUENCES

**Unauthorized use of security tools like FENRIR can result in:**
- **Criminal charges** under laws such as:
  - Computer Fraud and Abuse Act (CFAA) - USA
  - Computer Misuse Act - UK
  - GDPR - European Union
  - Similar laws in most countries
- **Civil lawsuits** from targets
- **Permanent criminal records**
- **Imprisonment** and **heavy fines**
- **Ban from platforms** and services

### 📋 RESPONSIBLE USE CHECKLIST

**Before running FENRIR, ensure you:**
- [ ] Have **written permission** (contract/email) to test the target
- [ ] Understand the **exact scope** of authorization
- [ ] Are aware of all **applicable laws** in your jurisdiction
- [ ] Have **proper documentation** of your authorization
- [ ] Will follow **responsible disclosure** practices
- [ ] Are using it for **legitimate security purposes**

### 🎯 ETHICAL GUIDELINES

1. **Respect Privacy**: Never target individuals without explicit consent
2. **Obtain Permission**: Always get written authorization before testing
3. **Stay in Scope**: Only test what you've been authorized to test
4. **Report Responsibly**: Disclose vulnerabilities through proper channels
5. **Use for Good**: Contribute to security, don't harm others

### 📜 LIABILITY

**By using FENRIR, you agree that:**
- You are **solely responsible** for your use of this tool
- The developers and contributors are **NOT liable** for any misuse
- You understand the **legal implications** of unauthorized testing
- You will use this tool **ethically and legally**

---

# 🚀 QUICK START GUIDE

## Step 1: Prerequisites

```bash
# Required:
- Rust 1.70+ (curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh)
- Git (for cloning)
- macOS, Linux, or Windows (WSL2 recommended)
```

## Step 2: Installation

```bash
# Clone the repository
git clone https://github.com/sfaustodev/fenrir-omni-cli.git
cd fenrir-omni-cli

# Build in release mode (optimized, faster binary)
cargo build --release

# Binary location:
# macOS/Linux: target/release/fenrir
# Windows: target/release/fenrir.exe

# Optional: Add to PATH (for easy access)
# Temporary (current session only):
export PATH="$PWD/target/release:$PATH"

# Permanent (add to ~/.zshrc or ~/.bashrc):
echo 'export PATH="$PATH:/Users/peluche/Fenrir/target/release"' >> ~/.zshrc
source ~/.zshrc
```

## Step 3: Install Kali Tools (Optional but Recommended)

```bash
# Automatic installation of 200+ security tools
./target/release/fenrir install-tools all

# Manual installation script
chmod +x install_all_kali_tools.sh
./install_all_kali_tools.sh
```

**Tools installed:**
- **Recon**: nmap, gobuster, ffuf, nuclei, rustscan
- **Web**: nikto, sqlmap, whatweb
- **Password**: hydra, john, hashcat
- **Wireless**: aircrack-ng, reaver, wifite
- **Forensics**: binwalk, autopsy, foremost
- **200+ more tools**

## Step 4: Configure API Keys

**Create a `.env` file in the project root:**

```bash
cat > .env << 'EOF'
# ============================================================================
# AI PROVIDERS (Required for intelligence features)
# ============================================================================

ZAI_API_KEY=your_zai_api_key_here
BLACKBOX_API_KEY=your_blackbox_api_key_here
GEMINI_API_KEY=your_gemini_api_key_here
XAI_API_KEY=your_grok_api_key_here
VENICE_API_KEY=your_venice_api_key_here

# ============================================================================
# OPTIONAL: Intelligence & OSINT Services
# ============================================================================

# Have I Been Pwned - Email breach checking
HIBP_API_KEY=your_hibp_api_key_here

# AbuseIPDB - IP reputation checking
ABUSEIPDB_KEY=your_abuseipdb_key_here

# GitHub - Increased rate limits
GITHUB_TOKEN=your_github_token_here
EOF
```

**How to get API keys:**
- **ZAI**: zai.com
- **BLACKBOX**: blackbox.ai
- **GEMINI**: makersuite.google.com
- **GROK/XAI**: x.ai
- **VENICE**: venice.ai
- **HIBP**: haveibeenpwned.com/API (free tier)
- **AbuseIPDB**: abuseipdb.com (free tier)
- **GitHub**: github.com/settings/tokens

---

# 💻 USAGE EXAMPLES

## 1. Intel Mode (Passive Reconnaissance)

**For gathering OSINT information on your own targets:**

```bash
# Quick scan
./target/release/fenrir intel scan your-domain.com

# Full analysis
./target/release/fenrir intel analyze your-domain.com

# Test all modules
./target/release/fenrir intel test target@example.com

# Analyze your own email for breaches
./target/release/fenrir intel scan your-email@gmail.com
```

**What it does:**
- OSINT gathering (domain info, email breaches, IP data)
- CSI threat analysis
- IOC extraction
- Risk scoring (0-100)
- Report generation

## 2. Interactive Mode (Active Testing)

**For authorized security testing:**

```bash
./target/release/fenrir interactive
```

**Then follow the prompts:**

```
🐺 fenrir> your-domain.com scan web

# FENRIR will:
# 1. Detect target type (DOMAIN)
# 2. Select appropriate tools (nmap, gobuster, nikto, etc.)
# 3. Ask: Stealth or Aggressive mode?
#    [1] STEALTH    - Quiet, slow, avoids detection
#    [2] AGGRESSIVE - Fast, loud, thorough
# 4. Execute tools and show live results
# 5. Display what was found or what failed
```

**Available keywords:**
- `scan` - Basic scanning
- `web` - Web application testing
- `password` - Password attacks
- `recon` - Deep reconnaissance
- `forensic` - Forensic analysis

## 3. Kali Tools (Manual Execution)

**Use installed tools directly:**

```bash
# Port scanning (requires sudo for stealth scan)
sudo nmap -sS -sV -sC your-domain.com

# Web directory brute forcing
gobuster dir -u https://your-domain.com -w /usr/share/wordlists/dirb/common.txt

# Vulnerability scanning
nuclei -u https://your-domain.com

# Fast port scanning
rustscan -a your-domain.com -- -sV

# SQL injection testing
sqlmap -u "https://your-domain.com/page?id=1" --batch

# And 200+ more tools...
```

## 4. Status Check

```bash
# Check what's installed and working
./target/release/fenrir status

# Show version
./target/release/fenrir --version

# Show help
./target/release/fenrir --help
```

---

# 🧪 TESTING

## Automated Test Suite

```bash
# Run comprehensive tests (uses safe targets)
./test_fenrir_tools.sh

# Test results saved to:
# /tmp/fenrir_tests/
```

**See `TESTING_GUIDE.md` for detailed testing instructions.**

---

# 📚 CORE FEATURES

## 1. OSINT Engine
Real-time intelligence gathering:
- IP Geolocation & ASN lookup
- Subdomain enumeration
- Email breach checking (Have I Been Pwned)
- IP reputation (AbuseIPDB)
- WHOIS queries
- Certificate transparency (crt.sh)
- GitHub intelligence
- Username searches
- 47+ findings typical, 96.9% confidence

## 2. CSI Analyzer
Threat intelligence and IOC extraction:
- Automatic IOC extraction (IPs, domains, emails, hashes, MACs, CVEs, URLs)
- Threat scoring with weighted algorithms
- Risk assessment (impact, likelihood)
- Pattern recognition (recon, exploitation, exfiltration)
- Correlation analysis
- Actionable recommendations

## 3. Forensics Engine
Digital forensics capabilities:
- File metadata extraction
- Cryptographic hashes (MD5, SHA1, SHA256, SHA512)
- Timeline generation
- Process analysis
- Suspicious indicator detection
- Artifact classification

## 4. Intel Dashboard
Interactive terminal UI:
- 4 view modes (OSINT, CSI/Threat, Forensics, Summary)
- Keyboard navigation (↑/↓, PgUp/PgDn)
- Color-coded severity (Critical=Red, High=DarkRed, Medium=Yellow, Low=Blue)
- Scrollable data panels
- Real-time updates

## 5. Intel Workflow
Automation pipelines:
- Predefined templates (Quick Scan, Full Analysis, Forensics)
- Custom workflows with dependencies
- Multi-format exports (JSON, CSV, Text, HTML)
- Comprehensive reporting
- Error handling & recovery

## 6. Intel Mode
Central orchestrator:
- Auto target type detection (Email, Domain, IP, Username)
- Multi-phase processing (OSINT → CSI → Forensics)
- Risk scoring (0-100 scale)
- Report management
- Export capabilities

## 7. Interactive Attack Execution
NEW in v1.6.66:
- Real tool execution (not just checks)
- Stealth/aggressive mode selection
- Live result display
- Target type auto-detection
- Tool output capture
- Success/failure reporting

---

# 🛠️ KALI TOOLS INTEGRATION

**200+ security tools organized by category:**

### Reconnaissance (50+ tools)
- nmap, gobuster, ffuf, nuclei, rustscan, masscan
- subfinder, httpx, assetfinder, amass
- whois, dig, nslookup, dnsrecon
- And 40+ more...

### Web Application Testing (40+ tools)
- nikto, sqlmap, whatweb, dirb, dirbuster
- burpsuite, zap, ffuf, gobuster
- And 35+ more...

### Password Attacks (30+ tools)
- hydra, john, hashcat, medusa
- patator, crowbar, crunch
- And 25+ more...

### Wireless Testing (20+ tools)
- aircrack-ng, reaver, wifite, bully
- cowpatty, pyrit, kismet
- And 15+ more...

### Forensics (30+ tools)
- binwalk, autopsy, foremost, scalpel
- volatility, sleuthkit, bulk_extractor
- And 25+ more...

### And many more categories...
- Exploitation, Sniffing, Social Engineering, Phishing, etc.

---

# 📊 PERFORMANCE

- **Binary Size:** ~4.0 MB (release build)
- **Memory Usage:** ~50-100 MB typical
- **Startup Time:** <1 second
- **Compilation Time:** ~40 seconds (release mode)
- **Tool Detection:** Instant (using `which` command)

---

# 🐛 TROUBLESHOOTING

## Tool Not Found

```bash
# Check if tool is installed
which nmap gobuster ffuf nuclei

# Install missing tools
./target/release/fenrir install-tools all

# Or manually:
brew install nmap gobuster
go install github.com/ffuf/ffuf@latest
go install github.com/projectdiscovery/nuclei/v3/cmd/nuclei@latest
```

## API Key Not Loaded

```bash
# Check if .env file exists
ls -la .env

# Test API key loading
./target/release/fenrir status

# Reload environment variables
source ~/.zshrc  # or source ~/.bashrc
```

## Permission Denied

```bash
# Some tools require sudo
sudo nmap -sS target

# Or run fenrir with sudo
sudo ./target/release/fenrir interactive
```

---

# 📖 DOCUMENTATION

- **TESTING_GUIDE.md** - Comprehensive testing instructions
- **API_GUIDE.md** - API usage and integration
- **BLACKBOX.md** - Blackbox AI integration
- Each module has inline documentation (~4,750 lines total)

---

# 🎯 ROADMAP

### Phase 1 ✅ COMPLETE
- ✅ OSINT/CSI/Forensics infrastructure
- ✅ Dashboard and visualization
- ✅ Workflow automation
- ✅ Central orchestration
- ✅ Kali tools integration (200+ tools)
- ✅ Interactive mode with real execution
- ✅ Stealth/aggressive modes
- ✅ macOS compatibility

### Future Phases
- AI Threat Intelligence Network
- Quantum Cryptography modules
- Predictive Analytics
- Swarm Security orchestration
- Cloud platform integration

---

# 🤝 CONTRIBUTING

**Contributions welcome!** Please:
1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Add tests if applicable
5. Submit a pull request

**Remember:**
- Only ethical security features
- Proper documentation required
- Test with safe targets only
- Follow responsible disclosure

---

# 📜 LICENSE

This tool is provided for educational and authorized security testing purposes only. Use responsibly and ethically.

**Users are solely responsible for ensuring their use complies with all applicable laws and regulations.**

---

<div align="center">

# 📧 CONTACT & SERVICES

## **sfaustodev@gmail.com**

### **Para Investigação Cybernetica, Bug Bounty e demais serviços de Hacking Etico**

</div>

<div align="center">

**Professional Ethical Hacking & Cybersecurity Services**

- 🔍 **Investigação Cybernetica** - Cyber Investigation
- 🐛 **Bug Bounty Programs** - Vulnerability Discovery
- 🛡️ **Hacking Etico** - Ethical Hacking & Penetration Testing
- 🎯 **Security Consulting** - Security Architecture & Strategy
- 📊 **Threat Intelligence** - Cyber Threat Analysis
- 🔒 **Digital Forensics** - Incident Response & Forensics

</div>

---

<div align="center">

### **🌐 Available Worldwide**

### **🇧🇷 Brazil | 🇺🇸 USA | 🇪🇺 Europe | Global Remote**

</div>

---

**Built with ❤️ and Rust - Fast, Safe, and Concurrent**

**Version 1.6.66 | Last Updated: 2026-01-30**
