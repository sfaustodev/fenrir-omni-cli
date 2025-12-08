#!/bin/bash

# Codex CLI Setup Script for OpenRouter
echo "🔧 Setting up Codex CLI with OpenRouter QWEN3-CODER..."

# Check if API key is provided
if [ -z "$1" ]; then
    echo "❌ Please provide your OpenRouter API key"
    echo "Usage: ./setup_codex.sh YOUR_OPENROUTER_API_KEY"
    echo "Get API key from: https://openrouter.ai/keys"
    exit 1
fi

API_KEY="$1"

# Create Codex config directory
mkdir -p ~/.codex

# Create Codex config file
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

# Set environment variables for backup
export OPENAI_API_KEY="$API_KEY"
export OPENAI_BASE_URL="https://openrouter.ai/api/v1"

# Test the configuration
echo "🧪 Testing Codex CLI configuration..."
codex -m "qwen/qwen3-coder:free" --cd . "Hello, respond with 'Codex configuration successful!'"

# Add to shell profile for persistence
echo "" >> ~/.zshrc
echo "# Codex CLI Configuration" >> ~/.zshrc
echo "export OPENAI_API_KEY=\"$API_KEY\"" >> ~/.zshrc
echo "export OPENAI_BASE_URL=\"https://openrouter.ai/api/v1\"" >> ~/.zshrc

echo "✅ Codex CLI configured successfully!"
echo "📝 Configuration added to ~/.zshrc"
echo "🔄 Restart your terminal or run 'source ~/.zshrc' to apply changes"