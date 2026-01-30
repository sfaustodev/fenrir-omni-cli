#!/bin/bash
# ============================================================================
# FENRIR COMPREHENSIVE TOOL TESTING SCRIPT
# ============================================================================
# WARNING: Only test against targets you OWN or have WRITTEN PERMISSION to test
# Testing real individuals' accounts without authorization is ILLEGAL
#
# Safe test targets:
# - example.com (reserved for documentation/testing)
# - testphp.vulnweb.com (authorized test site)
# - localhost, 127.0.0.1 (your own machine)
# - Your own domains/accounts
# ============================================================================

set -e

# Color codes
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

FENRIR_BIN="./target/release/fenrir"
LOG_DIR="/tmp/fenrir_tests"
TIMESTAMP=$(date +%Y%m%d_%H%M%S)

# Create log directory
mkdir -p "$LOG_DIR"

echo "╔════════════════════════════════════════════════════════════╗"
echo "║     FENRIR TOOL TESTING - COMPREHENSIVE SUITE              ║"
echo "╚════════════════════════════════════════════════════════════╝"
echo ""
echo "⚠️  IMPORTANT:"
echo "   This script uses SAFE test targets (example.com, localhost)"
echo "   Replace with your own AUTHORIZED targets before production use"
echo ""
echo "📁 Logs will be saved to: $LOG_DIR"
echo ""

# ============================================================================
# TEST CONFIGURATION - MODIFY THESE FOR YOUR AUTHORIZED TARGETS
# ============================================================================

# Example safe targets (REPLACE with your authorized targets)
TEST_EMAIL="test@example.com"
TEST_DOMAIN="example.com"
TEST_USERNAME="testuser"
TEST_IP="127.0.0.1"

# If you want to test with your own targets, uncomment and modify:
# TEST_EMAIL="your@email.com"
# TEST_DOMAIN="your-domain.com"
# TEST_USERNAME="your_username"
# TEST_IP="192.168.1.1"

# ============================================================================
# TEST FUNCTIONS
# ============================================================================

test_section() {
    echo -e "\n${BLUE}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
    echo -e "${BLUE}  $1${NC}"
    echo -e "${BLUE}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}\n"
}

test_command() {
    local test_name="$1"
    local command="$2"
    local log_file="$LOG_DIR/test_${TIMESTAMP}_$(echo $test_name | tr ' ' '_').log"

    echo -e "${YELLOW}Testing: $test_name${NC}"
    echo "   Command: $command"
    echo "   Log: $log_file"

    if eval "$command" > "$log_file" 2>&1; then
        echo -e "${GREEN}✅ PASS: $test_name${NC}"
        return 0
    else
        echo -e "${RED}❌ FAIL: $test_name${NC}"
        echo "   Check log: $log_file"
        return 1
    fi
}

# ============================================================================
# TEST SUITE 1: INTEL MODE (OSINT/CSI/FORENSICS)
# ============================================================================

test_section "SUITE 1: INTEL MODE - PASSIVE RECONNAISSANCE"

test_command "Intel Mode - Email Target" \
    "$FENRIR_BIN intel scan $TEST_EMAIL"

test_command "Intel Mode - Domain Target" \
    "$FENRIR_BIN intel scan $TEST_DOMAIN"

test_command "Intel Mode - Full Analysis" \
    "$FENRIR_BIN intel analyze $TEST_DOMAIN"

test_command "Intel Mode - Test All Modules" \
    "$FENRIR_BIN intel test $TEST_EMAIL"

# ============================================================================
# TEST SUITE 2: TOOL VERIFICATION
# ============================================================================

test_section "SUITE 2: KALI TOOLS - VERIFICATION"

echo -e "${YELLOW}Checking tool availability...${NC}\n"

tools=(
    "nmap"
    "gobuster"
    "rustscan"
    "hydra"
    "sqlmap"
    "nikto"
    "masscan"
)

for tool in "${tools[@]}"; do
    if which "$tool" &> /dev/null; then
        version=$($tool --version 2>&1 | head -1 || echo "version unknown")
        echo -e "${GREEN}✅ $tool${NC} - $version"
    else
        echo -e "${RED}❌ $tool${NC} - NOT FOUND"
    fi
done

echo ""
echo -e "${YELLOW}Checking Go tools...${NC}\n"

go_tools=(
    "ffuf"
    "nuclei"
    "subfinder"
    "httpx"
)

for tool in "${go_tools[@]}"; do
    if ~/go/bin/"$tool" --version &> /dev/null || ~/go/bin/"$tool" -version &> /dev/null; then
        echo -e "${GREEN}✅ $tool${NC} - Installed in ~/go/bin/"
    else
        echo -e "${RED}❌ $tool${NC} - NOT FOUND in ~/go/bin/"
    fi
done

# ============================================================================
# TEST SUITE 3: REAL TOOL EXECUTION (SAFE TARGETS)
# ============================================================================

test_section "SUITE 3: ACTIVE TOOL EXECUTION - SAFE TARGETS"

echo -e "${YELLOW}NOTE: These tests use SAFE targets only${NC}"
echo -e "${YELLOW}For real penetration testing, use your AUTHORIZED targets${NC}\n"

# Test nmap on localhost (safe)
test_command "NMAP - Localhost Scan" \
    "nmap -sV -sC localhost"

# Test whois on example.com (safe)
test_command "WHOIS - Example Domain" \
    "whois example.com | head -20"

# Test dig on example.com (safe)
test_command "DIG - DNS Lookup" \
    "dig example.com ANY +short"

# Test gobuster mode (dry run, no actual scan)
echo -e "${YELLOW}⚠️  gobuster requires a running web server${NC}"
echo "   Skipping (would need authorized target with web server)"

# Test nuclei version
test_command "NUCLEI - Version Check" \
    "~/go/bin/nuclei --version"

# Test ffuf version
test_command "FFUF - Version Check" \
    "~/go/bin/ffuf -V"

# ============================================================================
# TEST SUITE 4: INTERACTIVE MODE DEMONSTRATION
# ============================================================================

test_section "SUITE 4: INTERACTIVE MODE - STRUCTURE DEMONSTRATION"

echo -e "${YELLOW}Interactive mode requires manual input${NC}"
echo ""
echo "To test interactive mode with your targets, run:"
echo ""
echo "  $FENRIR_BIN interactive"
echo ""
echo "Then try these commands:"
echo "  🎯 $TEST_EMAIL scan recon"
echo "  🎯 $TEST_DOMAIN scan web"
echo "  🎯 $TEST_IP scan password"
echo ""
echo "When prompted for mode:"
echo "  [1] STEALTH    - Quiet, slow, avoids detection"
echo "  [2] AGGRESSIVE - Fast, loud, thorough detection"
echo ""

# ============================================================================
# SUMMARY
# ============================================================================

test_section "TEST SUMMARY"

echo "✅ Tests completed"
echo "📁 Log directory: $LOG_DIR"
echo ""
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo ""
echo "📋 NEXT STEPS:"
echo ""
echo "1. Review logs in $LOG_DIR"
echo "2. Replace test targets with your AUTHORIZED targets:"
echo "   - Edit TEST_EMAIL, TEST_DOMAIN, TEST_USERNAME, TEST_IP"
echo "3. Re-run script: ./test_fenrir_tools.sh"
echo "4. For interactive mode testing: $FENRIR_BIN interactive"
echo ""
echo "⚠️  REMEMBER:"
echo "   Only test targets you OWN or have WRITTEN PERMISSION to test"
echo "   Unauthorized testing is ILLEGAL"
echo ""
echo "╔════════════════════════════════════════════════════════════╗"
echo "║  For production use with REAL targets:                     ║"
echo "║  1. Ensure you have written authorization                  ║"
echo "║  2. Use proper scope documentation                          ║"
echo "║  3. Follow responsible disclosure practices                ║"
echo "╚════════════════════════════════════════════════════════════╝"
