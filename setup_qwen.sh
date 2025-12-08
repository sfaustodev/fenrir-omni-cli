#!/bin/bash

# Qwen CLI Setup Script for OpenRouter
echo "🔧 Setting up Qwen CLI with OpenRouter..."

# Check if API key is provided
if [ -z "$1" ]; then
    echo "❌ Please provide your OpenRouter API key"
    echo "Usage: ./setup_qwen.sh YOUR_OPENROUTER_API_KEY"
    echo "Get API key from: https://openrouter.ai/keys"
    exit 1
fi

API_KEY="$1"

# Set environment variables
export OPENAI_API_KEY="$API_KEY"
export OPENAI_BASE_URL="https://openrouter.ai/api/v1"

# Test the configuration
echo "🧪 Testing Qwen CLI configuration with QWEN3-CODER:FREE..."
qwen --model "qwen/qwen3-coder:free" "Hello, can you respond with 'Configuration successful!'?"

# Add to shell profile for persistence
echo "" >> ~/.zshrc
echo "# Qwen CLI Configuration" >> ~/.zshrc
echo "export OPENAI_API_KEY=\"$API_KEY\"" >> ~/.zshrc
echo "export OPENAI_BASE_URL=\"https://openrouter.ai/api/v1\"" >> ~/.zshrc

echo "✅ Qwen CLI configured successfully!"
echo "📝 Configuration added to ~/.zshrc"
echo "🔄 Restart your terminal or run 'source ~/.zshrc' to apply changes"