#!/bin/bash

# Unified Setup Script for Both Qwen CLI and Codex CLI
echo "🔧 Setting up both Qwen CLI and Codex CLI with OpenRouter QWEN3-CODER..."

# Check if API key is provided
if [ -z "$1" ]; then
    echo "❌ Please provide your OpenRouter API key"
    echo "Usage: ./setup_unified.sh YOUR_OPENROUTER_API_KEY"
    echo "Get API key from: https://openrouter.ai/keys"
    exit 1
fi

API_KEY="$1"

# Set environment variables for both tools
export OPENAI_API_KEY="$API_KEY"
export OPENAI_BASE_URL="https://openrouter.ai/api/v1"

# Create Codex config directory and file
mkdir -p ~/.codex
cat > ~/.codex/config.toml << EOF
[model_provider]
type = "openai"

[model_provider.openai]
api_key = "$API_KEY"
base_url = "https://openrouter.ai/api/v1"
model = "qwen/qwen3-coder:free"

[features]
web_search = true
EOF

# Add unified configuration to shell profile
echo "" >> ~/.zshrc
echo "# Unified AI CLI Configuration (Qwen + Codex)" >> ~/.zshrc
echo "export OPENAI_API_KEY=\"$API_KEY\"" >> ~/.zshrc
echo "export OPENAI_BASE_URL=\"https://openrouter.ai/api/v1\"" >> ~/.zshrc

# Test both configurations
echo "🧪 Testing Qwen CLI..."
qwen --model "qwen/qwen3-coder:free" "Hello, respond with 'Qwen works!'"

echo "🧪 Testing Codex CLI..."
codex -m "qwen/qwen3-coder:free" "Hello, respond with 'Codex works!'"

echo "✅ Both Qwen CLI and Codex CLI configured successfully!"
echo "📝 Configuration added to ~/.zshrc"
echo "🔄 Restart your terminal or run 'source ~/.zshrc' to apply changes"
echo ""
echo "🎯 Usage:"
echo "  Qwen CLI:  qwen --model \"qwen/qwen3-coder:free\" \"your prompt\""
echo "  Codex CLI: codex -m \"qwen/qwen3-coder:free\" \"your prompt\""