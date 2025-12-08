#!/bin/bash

echo "=== Testando GEMINI_API_KEY com Gemini 3.0 Pro ==="

# Testar com o modelo correto
echo "Testando com gemini-3.0-flash-preview..."
response=$(curl -s -X POST "https://generativelanguage.googleapis.com/v1beta/models/gemini-3.0-flash-preview:generateContent?key=${GEMINI_API_KEY}" \
  -H "Content-Type: application/json" \
  -d '{
    "contents": [{"parts":[{"text":"API test, respond with OK"}]}],
    "generationConfig": {"maxOutputTokens": 10}
  }')

echo "Resposta: $response" | jq -r '.candidates[0].content.parts[0].text // .error.message // "Error processing response"'

echo -e "\nTestando com gemini-3.0-pro-preview..."
response=$(curl -s -X POST "https://generativelanguage.googleapis.com/v1beta/models/gemini-3.0-pro-preview:generateContent?key=${GEMINI_API_KEY}" \
  -H "Content-Type: application/json" \
  -d '{
    "contents": [{"parts":[{"text":"API test, respond with OK"}]}],
    "generationConfig": {"maxOutputTokens": 10}
  }')

echo "Resposta: $response" | jq -r '.candidates[0].content.parts[0].text // .error.message // "Error processing response"'