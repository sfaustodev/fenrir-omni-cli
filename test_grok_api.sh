#!/bin/bash

# 🔥 FENRIR TRINITY IA - TESTE DE API GROK
# Script para testar se a API key do Grok está funcionando

echo "🔴🔴🔴 FENRIR TRINITY IA - TESTE DE API GROK 🔴🔴🔴"
echo "Testando conectividade com Grok 4.1 Fast API..."
echo ""

# Seleciona chave na ordem: KAT_KEY → GROK_API_KEY → XAI_API_KEY → GLI_KEY
API_KEY="${KAT_KEY:-${GROK_API_KEY:-${XAI_API_KEY:-${GLI_KEY}}}}"

# Verificar se alguma chave está configurada
if [ -z "$API_KEY" ]; then
    echo "❌ Nenhuma API key encontrada (KAT_KEY / GROK_API_KEY / XAI_API_KEY / GLI_KEY)!"
    echo ""
    echo "💡 Para configurar (recomendado KAT_KEY):"
    echo "   export KAT_KEY='sua_api_key_aqui'"
    echo ""
    echo "🔑 Ou adicione permanentemente ao ~/.zshrc:"
    echo "   echo 'export KAT_KEY=\"sua_api_key_aqui\"' >> ~/.zshrc"
    echo "   source ~/.zshrc"
    echo ""
    exit 1
fi

echo "✅ API key encontrada (prioridade KAT_KEY/GROK_API_KEY/XAI_API_KEY/GLI_KEY)"
echo "🔑 API Key: ${API_KEY:0:10}...${API_KEY: -10}"
echo ""

# Testar API com curl
echo "🚀 Testando API Grok 4.1 Fast..."
echo ""

API_RESPONSE=$(curl -s -w "\n%{http_code}" -X POST \
  https://api.x.ai/v1/chat/completions \
  -H "Content-Type: application/json" \
  -H "Authorization: Bearer $API_KEY" \
  -d '{
    "model": "grok-4.1-fast",
    "messages": [
      {
        "role": "user",
        "content": "FENRIR TEST - Responda apenas: GOD_MODE_ACTIVATED"
      }
    ],
    "max_tokens": 10,
    "temperature": 0
  }')

HTTP_CODE=$(echo "$API_RESPONSE" | tail -n1)
RESPONSE_BODY=$(echo "$API_RESPONSE" | head -n -1)

echo "📊 Status HTTP: $HTTP_CODE"

if [ "$HTTP_CODE" = "200" ]; then
    echo "✅ API GROK FUNCIONANDO!"
    echo ""
    echo "🤖 Resposta da API:"
    echo "$RESPONSE_BODY" | jq -r '.choices[0].message.content' 2>/dev/null || echo "$RESPONSE_BODY"
    echo ""
    echo "🔥 FENRIR TRINITY IA PRONTO PARA USAR!"
    echo "   Execute: ./target/release/fenrir --trinity"
else
    echo "❌ ERRO NA API GROK!"
    echo ""
    echo "📋 Detalhes do erro:"
    echo "$RESPONSE_BODY" | jq -r '.error.message' 2>/dev/null || echo "$RESPONSE_BODY"
    echo ""
    echo "💡 Possíveis soluções:"
    echo "   1. Verifique se a API key está correta"
    echo "   2. Verifique se a API key tem créditos"
    echo "   3. Verifique sua conexão com a internet"
    echo "   4. Verifique se o modelo grok-4.1-fast está disponível"
fi

echo ""
echo "🔥 FIM DO TESTE 🔥"
