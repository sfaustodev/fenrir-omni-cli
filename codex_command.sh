#!/bin/bash

# Direct Codex CLI command with OpenRouter QWEN3-CODER
# Usage: ./codex_command.sh "your prompt here"

if [ -z "$1" ]; then
    echo "Usage: ./codex_command.sh \"your prompt here\""
    echo "Example: ./codex_command.sh \"Write a hello world function in Rust\""
    exit 1
fi

PROMPT="$1"

# Run Codex with OpenRouter configuration
codex \
  -c model_provider.openai.api_key="sk-or-v1-YOUR_API_KEY_HERE" \
  -c model_provider.openai.base_url="https://openrouter.ai/api/v1" \
  -m "qwen/qwen3-coder:free" \
  "$PROMPT"