class FenrirCli < Formula
  desc "🐺 FENRIR v1.6.66 - Smart Attack Orchestration Platform with Kali Tools Integration"
  homepage "https://github.com/sfaustodev/fenrir-omni-cli"
  url "https://github.com/sfaustodev/fenrir-omni-cli/archive/refs/tags/v1.6.66.tar.gz"
  sha256 "PLACEHOLDER_SHA256" # Will be updated with actual checksum after release
  license "MIT"
  head "https://github.com/sfaustodev/fenrir-omni-cli.git", branch: "master"

  depends_on "rust" => :build

  def install
    system "cargo", "install", "--path", ".", "--root", prefix, "--bin", "fenrir"
  end

  def caveats
    <<~EOS
      ╔═══════════════════════════════════════════════════════════════════════════╗
      ║  🐺 FENRIR v1.6.66 - Smart Attack Orchestration Platform                    ║
      ║  Cyber Security Intelligence & Automation Tool                             ║
      ╚═══════════════════════════════════════════════════════════════════════════╝

      ⚠️  CRITICAL ETHICAL DISCLAIMER

      FENRIR is a powerful security tool that MUST be used responsibly and ethically.

      ACCEPTABLE USE ONLY:
        ✅ Your own systems and infrastructure
        ✅ Systems with explicit written permission
        ✅ Authorized penetration testing
        ✅ Bug bounty programs with clear scope
        ✅ Educational purposes with safe targets
        ✅ Ethical hacking within legal boundaries

      PROHIBITED USE (STRICTLY FORBIDDEN):
        ❌ Testing systems without written permission
        ❌ Targeting individuals' personal accounts
        ❌ Unauthorized access to any computer system
        ❌ Harassment, stalking, or threatening
        ❌ Illegal activities of any kind

      Unauthorized use can result in CRIMINAL CHARGES under:
        - Computer Fraud and Abuse Act (CFAA) - USA
        - Computer Misuse Act - UK
        - GDPR - European Union
        - Similar laws in most countries

      ─────────────────────────────────────────────────────────────────────────────

      🚀 QUICK START

      1. Set up API keys (create .env file in installation directory):

         ZAI_API_KEY=your_zai_api_key_here
         BLACKBOX_API_KEY=your_blackbox_api_key_here
         GEMINI_API_KEY=your_gemini_api_key_here
         XAI_API_KEY=your_grok_api_key_here
         VENICE_API_KEY=your_venice_api_key_here

         Optional:
         HIBP_API_KEY=your_hibp_api_key_here
         ABUSEIPDB_KEY=your_abuseipdb_key_here
         GITHUB_TOKEN=your_github_token_here

      2. Install Kali Tools (optional but recommended):

         fenrir install-tools all

         This installs 200+ security tools:
         - Recon: nmap, gobuster, ffuf, nuclei, rustscan
         - Web: nikto, sqlmap, whatweb
         - Password: hydra, john, hashcat
         - And 200+ more tools

      3. Use FENRIR:

         # Intel Mode (Passive OSINT)
         fenrir intel scan your-domain.com
         fenrir intel analyze target@example.com

         # Interactive Mode (Active Testing)
         fenrir interactive
         # Then type: your-domain.com scan web
         # Select: Stealth or Aggressive mode

         # Status Check
         fenrir status

      ─────────────────────────────────────────────────────────────────────────────

      📚 KEY FEATURES

      NEW IN v1.6.66:
        ✅ Complete Interactive Mode Rebuild
        ✅ Real Tool Execution (not just checks)
        ✅ Stealth/Aggressive Mode Selection
        ✅ Live Result Display
        ✅ Target Type Auto-Detection
        ✅ 200+ Kali Tools Integration
        ✅ macOS Compatibility

      CORE CAPABILITIES:
        • OSINT Engine - 47+ findings, 96.9% confidence
        • CSI Analyzer - IOC extraction & threat scoring
        • Forensics Engine - Digital artifact analysis
        • Intel Dashboard - Interactive TUI
        • Intel Workflow - Automation pipelines
        • Attack Execution - Real tool orchestration

      ─────────────────────────────────────────────────────────────────────────────

      📖 DOCUMENTATION

      • README.md - Complete user guide
      • TESTING_GUIDE.md - Testing instructions
      • API_GUIDE.md - API integration
      • BLACKBOX.md - AI integration

      ─────────────────────────────────────────────────────────────────────────────

      📧 CONTACT & SERVICES

      sfaustodev@gmail.com

      Para Investigação Cybernetica, Bug Bounty e demais serviços de Hacking Etico

      Professional Ethical Hacking & Cybersecurity Services:
        • Investigação Cybernetica - Cyber Investigation
        • Bug Bounty Programs - Vulnerability Discovery
        • Hacking Etico - Ethical Hacking & Penetration Testing
        • Security Consulting - Security Architecture & Strategy
        • Threat Intelligence - Cyber Threat Analysis
        • Digital Forensics - Incident Response & Forensics

      🌐 Available Worldwide: Brazil 🇧🇷 | USA 🇺🇸 | Europe 🇪🇺 | Global Remote

      ─────────────────────────────────────────────────────────────────────────────

      Use responsibly and ethically. Users are solely responsible for ensuring
      their use complies with all applicable laws and regulations.

      For more information: https://github.com/sfaustodev/fenrir-omni-cli
    EOS
  end

  test do
    assert_predicate bin/"fenrir", :exist?
    system bin/"fenrir", "--version"
  end
end
