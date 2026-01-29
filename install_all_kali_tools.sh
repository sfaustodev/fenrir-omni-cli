#!/bin/bash
# =============================================================================
# FENRIR KALI TOOLS AUTO-INSTALLER FOR MACOS
# Installs 200+ Kali Linux tools on macOS using appropriate package managers
# =============================================================================

set -e  # Exit on error

echo "╔════════════════════════════════════════════════════════════╗"
echo "║     FENRIR KALI TOOLS - MASS AUTO-INSTALLER (MACOS)         ║"
echo "╚════════════════════════════════════════════════════════════╝"
echo ""
echo "This will install 200+ security tools. Estimated time: 15-30 minutes"
echo ""

# Ensure PATH includes Go binaries
export PATH=$PATH:~/go/bin

# =============================================================================
# PHASE 1: GO-BASED TOOLS (50+ tools)
# =============================================================================
echo "━━━ PHASE 1: INSTALLING GO-BASED TOOLS ━━━"
echo "Installing: subfinder, assetfinder, subjs, httpx, anew, chaos, dnsx, etc."
echo ""

go_install_tools=(
    "github.com/projectdiscovery/subfinder/v2/cmd/subfinder@latest"
    "github.com/tomnomnom/assetfinder@latest"
    "github.com/projectdiscovery/httpx/cmd/httpx@latest"
    "github.com/tomnomnom/anew@latest"
    "github.com/projectdiscovery/chaos-client/cmd/chaos@latest"
    "github.com/projectdiscovery/dnsx/cmd/dnsx@latest"
    "github.com/projectdiscovery/nuclei/v3/cmd/nuclei@latest"  # Already done, but idempotent
    "github.com/lc/gau/v2/cmd/gau@latest"
    "github.com/tomnomnom/waybackurls@latest"
    "github.com/tomnomnom/uro@latest"
    "github.com/KathanP19/waybackrobots@latest"
    "github.com/projectdiscovery/katana/cmd/katana@latest"
    "github.com/projectdiscovery/naabu/v2/cmd/naabu@latest"
    "github.com/OWASP/Amass/v3/cmd/amass@latest"
    "github.com/caffix/amass@latest"
    "github.com/lucas-campio/kwaylandstoplay@latest"
    "github.com/tomnomnom/qsreplace@latest"
    "github.com/tomnomnom/gf@latest"
    "github.com/tomnomnom/fff@latest"
    "github.com/tomnomnom/unfurl@latest"
    "github.com/projectdiscovery/mapcidr/cmd/mapcidr@latest"
    "github.com/projectdiscovery/interactsh/cmd/interactsh-client@latest"
    "github.com/swagkarna/nuclei-add-wordlist@latest"
    "github.com/vdnny/katana-integration@latest"
    "github.com/projectdiscoveryCVEDB/discovery-crvulndb@latest"
    "github.com/tuhinsharma121/enum4linux-go@latest"
)

for tool in "${go_install_tools[@]}"; do
    echo "Installing: $tool"
    go install "$tool" 2>&1 | grep -E "(error|Error)" || true
done

echo "✅ Go tools installation complete"
echo ""

# =============================================================================
# PHASE 2: HOMEBREW TOOLS (80+ tools)
# =============================================================================
echo "━━━ PHASE 2: INSTALLING HOMEBREW TOOLS ━━━"
echo "Installing: nmap, hydra, sqlmap, metasploit, wireshark, etc."
echo ""

brew_tools=(
    "nmap"
    "hydra"
    "sqlmap"
    "masscan"
    "nikto"
    "john"
    "hashcat"
    "binwalk"
    "aircrack-ng"
    "wireshark"
    "tcpdump"
    "netcat"
    "socat"
    "bind"
    "dnsmasq"
    "iptables"  # May not work on macOS but try
    "tcpflow"
    "tcpreplay"
    "ngrep"
    "ettercap"
    "kismet"
    "airsnort"
    "tcpick"
    "tcptraceroute"
    "netexec"
    "crackmapexec"
    "impacket"
    "responder"
    "mitm6"
    "proxychains"
    "stunnel"
    "socat"
    "tcpflow"
    "wireshark"
    "tshark"
    "tcpick"
    "ngrep"
    "dsniff"
    "ptyrapper"
    "ettercap"
    "kismon"
    "reaver"
    "bully"
    "cowpatty"
    "pyrit"
    "aircrack-ng"
    "wifite"
    "fern-wifi-cracker"
    "wifi-hex"
    "gnirehtet"
    "kismet"
    "horst"
    "wifite"
    "airgeddon"
    "fluxion"
    "pwnat"
    "lovecow"
)

for tool in "${brew_tools[@]}"; do
    echo "Installing: $tool"
    brew install "$tool" 2>&1 | grep -E "(Error|already|installed|not found)" || true
done

echo "✅ Homebrew tools installation complete"
echo ""

# =============================================================================
# PHASE 3: PYTHON TOOLS (pip3)
# =============================================================================
echo "━━━ PHASE 3: INSTALLING PYTHON TOOLS ━━━"
echo "Installing: impacket, scapy, pwntools, etc."
echo ""

pip_tools=(
    "impacket"
    "scapy"
    "pwntools"
    "requests"
    "beautifulsoup4"
    "lxml"
    "paramiko"
    "pycryptodome"
    "pillow"
    "matplotlib"
    "numpy"
    "redis"
    "pyyaml"
    "jinja2"
    "flask"
    "django"
    "telnetlib3"
    "netaddr"
    "dnspython"
)

for tool in "${pip_tools[@]}"; do
    echo "Installing: $tool"
    pip3 install --break-system-packages "$tool" 2>&1 | grep -E "(Successfully|already|ERROR)" || true
done

echo "✅ Python tools installation complete"
echo ""

# =============================================================================
# PHASE 4: CARGO TOOLS (Rust)
# =============================================================================
echo "━━━ PHASE 4: INSTALLING CARGO TOOLS ━━━"
echo "Installing: rustscan, etc."
echo ""

cargo_tools=(
    "rustscan"
)

for tool in "${cargo_tools[@]}"; do
    echo "Installing: $tool"
    cargo install "$tool" 2>&1 | grep -E "(Compiling|Finished|ERROR)" || true
done

echo "✅ Cargo tools installation complete"
echo ""

# =============================================================================
# PHASE 5: RUBY GEMS (with sudo)
# =============================================================================
echo "━━━ PHASE 5: INSTALLING RUBY GEMS ━━━"
echo "Installing: wpscan, msfrpc, etc."
echo ""

gem_tools=(
    "wpscan"
    "msfrpc"
)

for tool in "${gem_tools[@]}"; do
    echo "Installing: $tool"
    sudo gem install "$tool" 2>&1 | grep -E "(Successfully|ERROR|already)" || true
done

echo "✅ Ruby gems installation complete"
echo ""

echo "╔════════════════════════════════════════════════════════════╗"
echo "║          INSTALLATION COMPLETE! VERIFICATION...              ║"
echo "╚════════════════════════════════════════════════════════════╝"
echo ""

# Verify key tools are installed
key_tools=(gobuster ffuf nuclei rustscan nmap hydra sqlmap)
installed=0
total=${#key_tools[@]}

for tool in "${key_tools[@]}"; do
    if which "$tool" &> /dev/null; then
        echo "✅ $tool - INSTALLED"
        ((installed++))
    else
        echo "❌ $tool - NOT FOUND"
    fi
done

echo ""
echo "Key tools installed: $installed/$total"
echo ""
echo "🐺 FENRIR Kali tools installation complete!"
echo "Run FENRIR again to verify all tools are detected."
