#!/bin/bash

echo "=== Testando APIs disponíveis ==="
echo

# Testar Anthropic Claude
echo "1. Testando ANTHROPIC_AUTH_TOKEN..."
curl -s -X POST https://api.anthropic.com/v1/messages \
  -H "Content-Type: application/json" \
  -H "Authorization: Bearer $ANTHROPIC_AUTH_TOKEN" \
  -H "anthropic-version: 2023-06-01" \
  -d '{
    "model": "claude-3-haiku-20240307",
    "max_tokens": 10,
    "messages": [{"role": "user", "content": "API test"}]
  }' | jq -r '.error // "OK"' 2>/dev/null || echo "Falha na requisição"

# Testar Gemini
echo -e "\n2. Testando GEMINI_API_KEY..."
curl -s -X POST "https://generativelanguage.googleapis.com/v1beta/models/gemini-pro:generateContent?key=$GEMINI_API_KEY" \
  -H "Content-Type: application/json" \
  -d '{
    "contents": [{"parts":[{"text":"API test"}]}],
    "generationConfig": {"maxOutputTokens": 10}
  }' | jq -r '.error // "OK"' 2>/dev/null || echo "Falha na requisição"

# Testar OpenAI
echo -e "\n3. Testando api_key (provavelmente OpenAI)..."
curl -s -X POST https://api.openai.com/v1/chat/completions \
  -H "Content-Type: application/json" \
  -H "Authorization: Bearer $api_key" \
  -d '{
    "model": "gpt-3.5-turbo",
    "max_tokens": 10,
    "messages": [{"role": "user", "content": "API test"}]
  }' | jq -r '.error // "OK"' 2>/dev/null || echo "Falha na requisição"

# Testar Grok
echo -e "\n4. Testando GROK_API_KEY..."
curl -s -X POST https://api.x.ai/v1/chat/completions \
  -H "Content-Type: application/json" \
  -H "Authorization: Bearer $GROK_API_KEY" \
  -d '{
    "model": "grok-beta",
    "max_tokens": 10,
    "messages": [{"role": "user", "content": "API test"}]
  }' | jq -r '.error // "OK"' 2>/dev/null || echo "Falha na requisição"

# Testar XAI (mesma do Grok?)
echo -e "\n5. Testando XAI_API_KEY..."
curl -s -X POST https://api.x.ai/v1/chat/completions \
  -H "Content-Type: application/json" \
  -H "Authorization: Bearer $XAI_API_KEY" \
  -d '{
    "model": "grok-beta",
    "max_tokens": 10,
    "messages": [{"role": "user", "content": "API test"}]
  }' | jq -r '.error // "OK"' 2>/dev/null || echo "Falha na requisição"

echo -e "\n=== Testando modelo GLM (se tiver chave) ==="
# Verificar se tem alguma chave GLM
for var in GLM_API_KEY ZHIPU_API_KEY; do
  if [ -n "${!var}" ]; then
    echo "Testando $var..."
    curl -s -X POST https://open.bigmodel.cn/api/paas/v4/chat/completions \
      -H "Content-Type: application/json" \
      -H "Authorization: Bearer ${!var}" \
      -d '{
        "model": "glm-4",
        "max_tokens": 10,
        "messages": [{"role": "user", "content": "API test"}]
      }' | jq -r '.error // "OK"' 2>/dev/null || echo "Falha na requisição"
  fi
done