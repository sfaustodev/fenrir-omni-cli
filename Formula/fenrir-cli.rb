class FenrirCli < Formula
  desc "🐺 FENRIR v1.6.66 - Smart Attack Orchestration Platform with Stealth-First Approach"
  homepage "https://github.com/peluche/fenrir"
  url "https://github.com/peluche/fenrir/archive/refs/tags/v1.6.66.tar.gz"
  sha256 "PLACEHOLDER_SHA256" # Will be updated with actual checksum after release
  license "MIT"
  head "https://github.com/peluche/fenrir.git", branch: "main"

  depends_on "rust" => :build

  def install
    system "cargo", "install", "--path", "src/fenrir", "--root", prefix, "--bin", "fenrir"
  end

  def caveats
    <<~EOS
      🐺 FENRIR v1.6.66 - Smart Attack Orchestration Platform

      NEW IN v1.6.66:
        ✅ Complete Interactive Mode Rebuild
        ✅ 13 Smart Attack Sequences (one keyword triggers all related tools)
        ✅ Stealth-First Approach (automatic stealth scan before attacks)
        ✅ Async + Sequential Execution (666MB/thread async, 2GB sequential)
        ✅ ZAI NLP Integration for natural language commands
        ✅ Double-check confirmation before execution

      AVAILABLE KEYWORDS:
        password, scan, recon, social, web, wireless, oauth,
        database, forensic, exploit, privesc, shell, sniff

      To get started:
        1. Set up your API keys in ~/.fenrir/.env
        2. Run: fenrir
        3. Type 'help' for usage or 'keywords' to see all attack sequences

      Required API keys (set in ~/.fenrir/.env):
        - ZAI_API_KEY: Main orchestrator AI (Venice AI)
        - GEMINI_API_KEY: Translation for non-English input
        - GROK_API_KEY or XAI_API_KEY: Fallback AI

      Example .env file:
        ZAI_API_KEY=your_venice_ai_key
        GEMINI_API_KEY=your_gemini_key
        GROK_API_KEY=your_grok_key

      For more information: https://github.com/peluche/fenrir
    EOS
  end

  test do
    assert_predicate bin/"fenrir", :exist?
    system bin/"fenrir", "--version"
  end
end
