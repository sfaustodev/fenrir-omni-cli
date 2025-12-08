#!/bin/bash

# Gemini CLI command (using qwen CLI as base)
# Now configured to use Gemini 3.0 Pro
# Usage: ./qwen_command.sh "your prompt here"

if [ -z "$1" ]; then
    echo "Usage: ./qwen_command.sh \"your prompt here\""
    echo "Example: ./qwen_command.sh \"Write a hello world function in Rust\""
    exit 1
fi

PROMPT="$1"

# Set Gemini API key
export GEMINI_API_KEY="AIzaSyC4sVYrVqlTY8lMPBUf6lOBXJW_Mt6oXAQ"

# Run with Gemini 3.0 Pro configuration
qwen \
  --openai-base-url "https://generativelanguage.googleapis.com/v1beta/models/gemini-3.0-pro:generateContent" \
  --model "gemini-3.0-pro" \
  --openai-api-key "$GEMINI_API_KEY" \
  "$PROMPT"