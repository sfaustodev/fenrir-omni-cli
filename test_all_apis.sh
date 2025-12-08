#!/bin/bash

echo "=== TESTE COMPLETO DE TODAS AS APIs ==="
echo "Data: $(date)"
echo "==================================="
echo

# Cores para output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Função para testar API
test_api() {
    local name=$1
    local url=$2
    local model=$3
    local api_key_var=$4
    local data=$5

    echo -e "${BLUE}Testando $name${NC}"
    echo "URL: $url"
    echo "Model: $model"
    echo "API Key: ${!api_key_var:0:20}..."

    response=$(curl -s -X POST "$url" \
        -H "Content-Type: application/json" \
        -H "Authorization: Bearer ${!api_key_var}" \
        -d "$data" 2>&1)

    if echo "$response" | jq -e '.error' >/dev/null 2>&1; then
        echo -e "${RED}❌ FALHOU: $(echo "$response" | jq -r '.error.message // .error // "Erro desconhecido"')${NC}"
    elif echo "$response" | jq -e '.choices[0].message.content // .candidates[0].content.parts[0].text' >/dev/null 2>&1; then
        local content=$(echo "$response" | jq -r '.choices[0].message.content // .candidates[0].content.parts[0].text' | head -c 100)
        echo -e "${GREEN}✅ SUCESSO: $content...${NC}"
    elif echo "$response" | jq -e '.' >/dev/null 2>&1; then
        echo -e "${YELLOW}⚠️ RESPOSTA INESPERADA:${NC}"
        echo "$response" | head -5
    else
        echo -e "${RED}❌ ERRO DE COMUNICAÇÃO:${NC}"
        echo "$response" | head -3
    fi
    echo "----------------------------------------"
    echo
}

# 1. Testar ZAI (GLM-4.6)
echo -e "\n${YELLOW}=== ZHIPU AI (GLM-4.6) ===${NC}"
test_api "Zhipu AI GLM-4.6" \
    "https://api.z.ai/v1/chat/completions" \
    "glm-4.6" \
    "ZAI_API_KEY" \
    '{
        "model": "glm-4.6",
        "messages": [{"role": "user", "content": "Responda apenas com OK"}],
        "max_tokens": 10
    }'

# 2. Testar QWEN3
echo -e "\n${YELLOW}=== QWEN3 (OpenRouter) ===${NC}"
test_api "Qwen3 Coder" \
    "https://openrouter.ai/api/v1/chat/completions" \
    "qwen/qwen3-coder:free" \
    "QWEN3_API_KEY" \
    '{
        "model": "qwen/qwen3-coder:free",
        "messages": [{"role": "user", "content": "Respond with just OK"}],
        "max_tokens": 10
    }'

# 3. Testar API_KEY (OpenAI)
echo -e "\n${YELLOW}=== OpenAI ===${NC}"
test_api "OpenAI" \
    "https://api.openai.com/v1/chat/completions" \
    "gpt-3.5-turbo" \
    "api_key" \
    '{
        "model": "gpt-3.5-turbo",
        "messages": [{"role": "user", "content": "Respond with just OK"}],
        "max_tokens": 10
    }'

# 4. Testar GROK_API_KEY
echo -e "\n${YELLOW}=== GROK (X.AI) ===${NC}"
test_api "Grok" \
    "https://api.x.ai/v1/chat/completions" \
    "grok-beta" \
    "GROK_API_KEY" \
    '{
        "model": "grok-beta",
        "messages": [{"role": "user", "content": "Respond with just OK"}],
        "max_tokens": 10
    }'

# 5. Testar GEMINI_API_KEY
echo -e "\n${YELLOW}=== GEMINI ===${NC}"
test_api "Gemini" \
    "https://generativelanguage.googleapis.com/v1beta/models/gemini-1.5-flash:generateContent" \
    "gemini-1.5-flash" \
    "GEMINI_API_KEY" \
    '{
        "contents": [{"parts":[{"text":"Respond with just OK"}]}],
        "generationConfig": {"maxOutputTokens": 10}
    }'

# 6. Testar ANTHROPIC_AUTH_TOKEN
echo -e "\n${YELLOW}=== ANTHROPIC ===${NC}"
test_api "Anthropic" \
    "https://api.anthropic.com/v1/messages" \
    "claude-3-haiku-20240307" \
    "ANTHROPIC_AUTH_TOKEN" \
    '{
        "model": "claude-3-haiku-20240307",
        "max_tokens": 10,
        "messages": [{"role": "user", "content": "Respond with just OK"}]
    }'

echo -e "\n${YELLOW}=== RESUMO DAS VARIÁVEIS DE AMBIENTE ===${NC}"
env | grep -E "(API_KEY|AUTH_TOKEN)" | sort