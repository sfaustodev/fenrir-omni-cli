# 🔴 FENRIR 3.0 - QUICKSTART GUIDE

## 🚀 Installation & Setup

```bash
cd /Users/peluche/Fenrir
cargo build --release
```

## 🎯 Basic Usage

### Interactive Mode
```bash
./target/release/fenrir
```

### One-Liner Commands
```bash
./target/release/fenrir "scan 127.0.0.1"
./target/release/fenrir "list files"
```

## 🔥 NEW: Security Commands

### **BITE (MORDER)** - Penetration Testing
```bash
# In interactive mode
bite <target> [options]
morder <alvo> [opções]

# Examples
bite 192.168.1.100
bite example.com --aggressive
morder 10.0.0.1 --godmode --exploit
```

**Intensity Levels**:
- `--passive` (default): Recon only
- `--cautious`: Light scanning
- `--aggressive`: Full pentest
- `--godmode`: Maximum intensity

### **SCAN** - Security Assessment
```bash
# In interactive mode
scan <target> [options]

# Examples
scan 192.168.1.100
scan example.com --comprehensive --deep
scan target.com --stealth
```

**Scan Types**:
- `--quick` (default): Fast scan
- `--comprehensive`: Full assessment
- `--stealth`: Avoid detection
- `--compliance`: Compliance-focused

### **TOOLS** - Check Available Kali Tools
```bash
tools
# or
kali
```

## 🤖 AI Integration

- **Gemini**: Translation layer (Portuguese → Commands)
- **Grok**: General tasks (guarded)
- **Venice Red Team**: Aggressive pentesting (unguarded)
- **GLM 4.7 (You)**: Orchestrator

## 🎮 Interactive Commands

```bash
godmode              # Activate GOD MODE
status               # System status
starship             # Show Starship config
tools / kali         # List Kali tools
sair / exit / quit  # Exit
```

## 📊 Examples

### Bug Bounty Recon
```bash
scan hackerone.com --comprehensive
bite hackerone.com --cautious
```

### Authenticated Pentest
```bash
scan 192.168.100.50 --deep
bite 192.168.100.50 --aggressive --exploit
```

### Reverse Engineering
```bash
bite malware.bin --godmode
```

## ⚠️ Legal

**Authorized use only!**
- Bug bounty programs
- Written authorization required
- Testing without permission is illegal

## 📚 Full Documentation

See `/docs/KALI_TOOLS_INTEGRATION.md`

---

**🐺 FENRIR 3.0 - KALI TOOLS RESTORED!**
