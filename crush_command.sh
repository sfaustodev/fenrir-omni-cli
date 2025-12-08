#!/bin/bash

# Crush CLI command - Gemini 3.0 Pro
# Usage: ./crush_command.sh "your prompt here"

if [ -z "$1" ]; then
    echo "Usage: ./crush_command.sh \"your prompt here\""
    echo "Example: ./crush_command.sh \"Analyze this code and optimize it\""
    exit 1
fi

PROMPT="$1"

# Set Gemini API key
export GEMINI_API_KEY="AIzaSyC4sVYrVqlTY8lMPBUf6lOBXJW_Mt6oXAQ"

# Run with Gemini 3.0 Pro configuration
crush \
  --openai-base-url "https://generativelanguage.googleapis.com/v1beta/models/gemini-3.0-pro:generateContent" \
  --model "gemini-3.0-pro" \
  --openai-api-key "$GEMINI_API_KEY" \
  "$PROMPT"