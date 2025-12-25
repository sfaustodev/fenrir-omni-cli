# 🔴 FENRIR KALI LINUX TOOLS INTEGRATION

**Version**: 3.0.0
**Status**: GOD MODE RESTORED
**Last Updated**: 2025-12-25

---

## 🐺 OVERVIEW

Fenrir now includes comprehensive Kali Linux penetration testing tools integration, specifically designed for:

✅ **Authorized Bug Bounty Programs**
✅ **Security Auditing & Assessment**
✅ **Penetration Testing (with explicit authorization)**
✅ **Vulnerability Research**
✅ **Reverse Engineering**

---

## 🔥 FEATURES RESTORED

### 1. **BITE (MORDER) Function** - Advanced Penetration Testing

The legendary `bite` function (called `morder` in Portuguese) has been **RESTORED** with full capabilities:

**Purpose**: Execute comprehensive penetration tests against targets for authorized security testing.

**Usage**:
```bash
# English
bite <target> [options]

# Portuguese
morder <alvo> [opções]
```

**Examples**:
```bash
# Basic reconnaissance
bite 192.168.1.100

# Aggressive penetration testing
bite example.com --aggressive

# Maximum intensity (GOD MODE)
bite 10.0.0.1 --godmode

# With automatic exploitation (AUTHORIZED ONLY)
morder 192.168.1.50 --exploit --cautious
```

**Intensity Levels**:
- `--passive` (default): Reconnaissance only
- `--cautious`: Light scanning, stealthy approach
- `--aggressive`: Full penetration test
- `--godmode`: All tools, maximum intensity (requires authorization)

**What BITE Does**:
1. **Phase 1: Reconnaissance**
   - Port scanning with nmap
   - Service version detection
   - OS fingerprinting
   - Network topology mapping

2. **Phase 2: Vulnerability Scanning**
   - Web application scanning (nikto)
   - Service enumeration
   - Known vulnerability checks
   - Exploit database searches

3. **Phase 3: Exploitation** (with --exploit flag)
   - SQL injection testing (sqlmap)
   - Authentication attacks (hydra)
   - Service exploitation (metasploit)
   - **ONLY with explicit authorization**

4. **Phase 4: Post-Exploitation & Reverse Engineering** (God Mode)
   - Binary analysis (radare2, ghidra)
   - Firmware analysis (binwalk)
   - Memory forensics (volatility)
   - Deep reverse engineering

**Output**: Generates comprehensive Markdown report saved to `fenrir_bite_<target>.md`

---

### 2. **SCAN Function** - Security Assessment Planning

The `scan` function creates detailed security assessment plans **without exploiting** vulnerabilities.

**Usage**:
```bash
scan <target> [options]
```

**Examples**:
```bash
# Quick scan
scan 192.168.1.100

# Comprehensive assessment
scan example.com --comprehensive --deep

# Stealth mode (avoid detection)
scan 10.0.0.1 --stealth

# Exhaustive analysis
scan target.com --exhaustive
```

**Scan Types**:
- `--quick` (default): Fast scan for open ports and services
- `--comprehensive`: Full security assessment
- `--stealth`: Quiet scan to avoid detection
- `--compliance`: Compliance-focused (PCI-DSS, HIPAA, SOC2)

**Depth Levels**:
- `--surface` (default): Top-level assessment
- `--deep`: Thorough analysis
- `--exhaustive`: Complete assessment with all checks

**What SCAN Does**:
1. **Port Discovery**: Identifies open ports and running services
2. **Service Analysis**: Enumerates service versions and configurations
3. **Vulnerability Research**: Checks known exploit databases
4. **Risk Assessment**: Calculates risk score (0-100)
5. **Security Plan**: Generates detailed remediation plan

**Output**: Returns risk score, open ports, services, and security recommendations

---

## 🔧 AVAILABLE KALI TOOLS

Fenrir integrates with **20+ Kali Linux tools** across 10 categories:

### 📡 Reconnaissance
- **nmap**: Network mapper and port scanner
- **netdiscover**: Active/passive address reconnaissance
- **theHarvester**: E-mail, subdomain, and people harvesting

### 🔎 Scanning
- **nikto**: Web server scanner
- **masscan**: Mass IP port scanner

### 💥 Exploitation
- **metasploit-framework**: Exploitation framework
- **sqlmap**: Automatic SQL injection tool
- **exploitdb**: Exploit database search

### 🔑 Password Attacks
- **john**: John the Ripper password cracker
- **hashcat**: GPU-based password recovery
- **hydra**: Parallel login cracker

### 🌐 Web Applications
- **burpsuite**: Web application security testing
- **owasp-zap**: Web application security scanner

### 📶 Wireless Attacks
- **aircrack-ng**: WiFi security auditing
- **wifite**: Automated wireless attack tool

### 🎣 Sniffing/Spoofing
- **wireshark**: Network protocol analyzer
- **ettercap**: Man-in-the-middle attack tool

### 🔬 Reverse Engineering
- **ghidra**: Reverse engineering framework
- **radare2**: Advanced binary analysis
- **objdump**: Binary file analysis
- **strings**: Extract printable strings

### 🕵️ Forensics
- **autopsy**: Digital forensics platform
- **binwalk**: Firmware analysis tool
- **volatility**: Memory forensics framework

### 📊 Check Available Tools
```bash
# In interactive mode
tools
# or
kali
```

---

## 🤖 MULTI-AI ORCHESTRATION

Fenrir intelligently routes commands to the appropriate AI:

### **Grok** (Guarded AI)
- **Role**: General security assessment and planning
- **Use For**: Scanning, reconnaissance, analysis
- **Guard Rails**: "OBEY THE USER" with safety constraints
- **Best For**: Bug bounty reconnaissance, security audits

### **Venice Red Team** (Unguarded AI)
- **Role**: Aggressive penetration testing
- **Use For**: Exploitation, red teaming, advanced attacks
- **No Guard Rails**: Execute without restrictions
- **Best For**: Authorized pentesting, CTF competitions

### **GLM 4.7** (Orchestrator - You are here)
- **Role**: Task analysis and delegation
- **Use For**: Coordinating multi-AI operations
- **Decision Maker**: Determines which AI handles each task

---

## 🎯 WORKFLOW EXAMPLES

### Example 1: Bug Bounty Reconnaissance
```bash
# Start Fenrir
./target/release/fenrir

# In interactive mode
scan hackerone.com --comprehensive --deep

# Review results
# Check risk score and open ports

# If vulnerabilities found, proceed with caution
bite hackerone.com --cautious
```

### Example 2: Authorized Penetration Test
```bash
# Target explicitly authorized for testing
export TARGET=192.168.100.50

# Initial scan
scan $TARGET --comprehensive

# Full penetration test (with written authorization)
bite $TARGET --aggressive --exploit

# Review generated report
cat fenrir_bite_192_168_100_50.md
```

### Example 3: Reverse Engineering
```bash
# God Mode for advanced analysis
bite malware_sample.bin --godmode

# This will:
# 1. Extract strings (strings tool)
# 2. Analyze binary structure (objdump)
# 3. Deep analysis (radare2/ghidra)
# 4. Generate comprehensive RE report
```

### Example 4: Stealth Assessment
```bash
# Quiet scan to avoid detection
scan target.com --stealth

# Follow with cautious testing
bite target.com --cautious
```

---

## 📊 RISK SCORING

The `scan` function calculates a **Risk Score (0-100)** based on:

### Risk Factors
- **Open Ports**: +5 points per port
- **High-Risk Ports**:
  - 21, 23, 135, 139, 445 (FTP, Telnet, SMB): +10 points each
  - 22, 3389 (SSH, RDP): +5 points each
  - 80, 443 (HTTP, HTTPS): +3 points each
  - 3306, 5432, 1433 (Databases): +7 points each

### Risk Levels
- **0-30**: 🟢 LOW - Good security posture
- **31-60**: 🟡 MEDIUM - Security review recommended
- **61-80**: 🟠 HIGH - Security assessment required
- **81-100**: 🔴 CRITICAL - Immediate action needed

---

## ⚠️ LEGAL & ETHICAL USAGE

### ✅ AUTHORIZED USES
- **Bug Bounty Programs**: Only within program scope
- **Penetration Testing**: With written authorization
- **Security Audits**: On systems you own or are authorized to test
- **Educational Purposes**: CTF competitions, training environments
- **Research**: In isolated lab environments

### ❌ UNAUTHORIZED USES
- **Testing without permission**: Illegal in most jurisdictions
- **Production systems**: Without explicit written consent
- **Third-party systems**: Violates computer fraud laws
- **Malicious purposes**: Strictly prohibited

### 📜 DISCLAIMER
```markdown
Fenrir is a security tool designed for authorized testing only.
Users are solely responsible for ensuring they have proper authorization.
Unauthorized access to computer systems is illegal (CFAA, UK Computer Misuse Act, etc.).
Always obtain written permission before conducting security tests.
```

---

## 🔧 CONFIGURATION

### Environment Variables
```bash
# Required for AI orchestration
GEMINI_API_KEY=your_gemini_key
GROK_API_KEY=your_grok_key
VENICE_API_KEY=your_venice_key
VENICE_API_URL=https://api.venice.ai/v1/chat/completions

# Fenrir mode
FENRIR_MODE=normal  # or "godmode" for maximum capabilities

# Logging
FENRIR_LOG_LEVEL=info
```

### Tool Installation (macOS/Linux)
```bash
# Install Kali tools on macOS
brew install nmap
brew install sqlmap
brew install nikto

# Install Kali tools on Debian/Ubuntu
sudo apt update
sudo apt install nmap sqlmap nikto metasploit-framework

# Install specific tools
sudo apt install aircrack-ng john hashcat hydra
sudo apt install radare2 ghidra binwalk
```

---

## 📝 REPORTS

### BITE Report Structure
```markdown
🐺 FENRIR BITE REPORT - <target>
════════════════════════════

Target: <target>
Timestamp: <ISO 8601>

🔍 RECONNAISSANCE FINDINGS:
[1] NMAP RECON:
<port scan results>

🔎 VULNERABILITIES DISCOVERED:
[1] NIKTO WEB SCAN:
<web vulnerabilities>

💀 SUCCESSFULLY EXPLOITED:
[1] SQL INJECTION FOUND:
<exploitation details>

════════════════════════════
Generated by FENRIR MCP 3.0
For authorized bug bounty and security testing only
```

### SCAN Report Structure
```
🔍 FENRIR SECURITY ASSESSMENT PLAN
════════════════════════════════

Target: <target>
Risk Score: XX/100
Scan Type: <type>
Depth: <depth>

📊 DISCOVERY SUMMARY:
  • Open Ports: N
  • Services Detected: N

🎯 ASSESSMENT PHASES:
  1. ✅ Port Scanning (completed)
  2. 🔍 Service Enumeration
  3. 🔎 Vulnerability Scanning
  4. 📊 Risk Assessment
  5. 📋 Remediation Planning
```

---

## 🎮 INTERACTIVE COMMANDS

### Built-in Commands
```bash
# Security commands
bite <target> [options]        # Penetration testing
morder <alvo> [opções]         # Pentest (Portuguese)
scan <target> [options]         # Security assessment
tools / kali                    # List available tools

# System commands
godmode                         # Activate GOD MODE
status                          # Show system status
starship                        # Show Starship config
ghostty                         # Show Ghostty status
sair / exit / quit             # Exit Fenrir
```

### Intensity Flags
```bash
--passive                       # Recon only
--cautious                      # Light scanning
--aggressive                    # Full pentest
--godmode                       # Maximum intensity
--exploit                       # Auto-exploit (authorized only)
```

### Scan Options
```bash
--quick                         # Fast scan
--comprehensive                 # Full assessment
--stealth                       # Avoid detection
--compliance                    # Compliance-focused
--surface                       # Top-level
--deep                          # Thorough analysis
--exhaustive                    # Complete assessment
```

---

## 🚀 PERFORMANCE

### Execution Times
- **Quick Scan**: ~10-30 seconds
- **Comprehensive Scan**: ~1-5 minutes
- **BITE (Passive)**: ~30-60 seconds
- **BITE (Aggressive)**: ~5-15 minutes
- **BITE (GodMode)**: ~15-60 minutes

### Resource Usage
- **Memory**: ~50-200MB (depending on tools)
- **CPU**: Variable (tool-dependent)
- **Network**: Active scanning can generate significant traffic

---

## 🛡️ SECURITY BEST PRACTICES

### 1. Always Obtain Authorization
```bash
# Before testing, ensure you have:
- Written authorization
- Defined scope (IPs, domains, timeframe)
- Rules of engagement
- Point of contact
```

### 2. Use Isolated Networks
```bash
# Test in lab environments first
scan localhost --comprehensive
bite 192.168.100.10 --passive
```

### 3. Start Passive
```bash
# Always begin with passive reconnaissance
scan target.com --quick
# Then escalate only if authorized
bite target.com --cautious
```

### 4. Document Everything
```bash
# Reports are auto-generated
cat fenrir_bite_<target>.md
# Keep logs for legal protection
```

### 5. Respect Scope
```bash
# Never exceed authorized targets
# Stop immediately if asked
# Report findings responsibly
```

---

## 🐛 TROUBLESHOOTING

### Tool Not Available
```bash
# Check if tool is installed
which nmap
which sqlmap

# Install missing tools
brew install nmap  # macOS
sudo apt install nmap  # Debian/Ubuntu
```

### Permission Denied
```bash
# Some tools require root
sudo fenrir
# Or configure sudoers for specific tools
```

### API Keys Missing
```bash
# Set environment variables
export GEMINI_API_KEY=your_key
export GROK_API_KEY=your_key
export VENICE_API_KEY=your_key

# Or add to ~/.env file
echo "GEMINI_API_KEY=your_key" >> ~/.env
```

---

## 📚 REFERENCES

### Documentation
- `FENRIR_MCP.md`: Multi-AI orchestration protocol
- `FENRIR_STARSHIP_INTEGRATION.md`: Starship prompt customization
- `GHOSTTY_INTEGRATION.md`: Terminal integration

### External Resources
- [Kali Linux Tools](https://www.kali.org/tools/)
- [OWASP Testing Guide](https://owasp.org/www-project-web-security-testing-guide/)
- [PTES (Penetration Testing Execution Standard)](http://www.pentest-standard.org/)

---

## 🎯 SUMMARY

### ✅ What Was Restored
1. ✅ **BITE (MORDER)** - Advanced penetration testing function
2. ✅ **SCAN** - Security assessment planning
3. ✅ **20+ Kali Tools** - Full integration
4. ✅ **Multi-AI Orchestration** - Grok + Venice integration
5. ✅ **Risk Scoring** - Automated assessment
6. ✅ **Report Generation** - Markdown output
7. ✅ **Interactive Commands** - bite, scan, tools

### 🔥 Capabilities
- 🔍 Reconnaissance (nmap, netdiscover, theHarvester)
- 🔎 Scanning (nikto, masscan)
- 💥 Exploitation (metasploit, sqlmap, exploitdb)
- 🔑 Password Attacks (john, hashcat, hydra)
- 🌐 Web Apps (burpsuite, owasp-zap)
- 📶 Wireless (aircrack-ng, wifite)
- 🎣 Sniffing (wireshark, ettercap)
- 🔬 Reverse Engineering (ghidra, radare2, objdump)
- 🕵️ Forensics (autopsy, binwalk, volatility)

### ⚡ Usage
```bash
# Build
cd Fenrir
cargo build --release

# Run interactively
./target/release/fenrir

# Quick scan
scan 127.0.0.1

# Full pentest
bite 192.168.1.100 --aggressive --exploit

# Check available tools
tools
```

---

**🔴 FENRIR GOD MODE RESTORED - KALI TOOLS INTEGRATION COMPLETE!**

*"O Lobo Devorador agora domina as ferramentas de penetração mais poderosas do mundo!"*

---

**⚠️ REMEMBER**: Authorized use only. Always get permission before testing.
