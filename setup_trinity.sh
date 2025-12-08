#!/bin/bash

# 🔥 FENRIR TRINITY IA - SETUP SCRIPT
# Configuração do ambiente para modo Trinity IA

echo "🔴🔴🔴 FENRIR TRINITY IA - SETUP 🔴🔴🔴"
echo "Configurando ambiente para coordenação Gemini + Claude + Grok"
echo ""

# Verificar se API_KEY/KAT_KEY (ou fallbacks) já está configurada
if [ -n "$API_KEY" ]; then
    echo '✅ $API_KEY detectada (prioritária)'
    echo "🚀 Grok/Droid CLI pronto para uso"
elif [ -n "$KAT_KEY" ]; then
    echo '✅ $KAT_KEY já está configurada'
    echo "🚀 Grok/Droid CLI pronto para uso"
elif [ -n "$QWEN_CODE" ] || [ -n "$QWEN_CODE_KEY" ] || [ -n "$GLM4_6_KEY" ] || \
     [ -n "$GLM_4_6_KEY" ] || [ -n "$GLM_KEY" ] || [ -n "$GLM_API_KEY" ] || \
     [ -n "$GROK_API_KEY" ] || [ -n "$XAI_API_KEY" ] || [ -n "$GLI_KEY" ]; then
    echo '⚠️ $KAT_KEY não encontrada, mas foi detectada uma chave fallback ($QWEN_CODE / $QWEN_CODE_KEY / $GLM4_6_KEY / $GLM_4_6_KEY / $GLM_KEY / $GLM_API_KEY / $GROK_API_KEY / $XAI_API_KEY / $GLI_KEY)'
    echo "   Recomenda-se padronizar em KAT_KEY para o CLI Droid/Grok."
else
    echo '❌ Nenhuma chave encontrada ($API_KEY → $KAT_KEY → $QWEN_CODE → $QWEN_CODE_KEY → $GLM4_6_KEY → $GLM_4_6_KEY → $GLM_KEY → $GLM_API_KEY → $GROK_API_KEY → $XAI_API_KEY → $GLI_KEY)'
    echo ""
    echo "💡 Para configurar a API key (padrão KAT_KEY):"
    echo "   export KAT_KEY='sua_api_key_aqui'"
    echo ""
    echo "🔑 Ou adicione ao seu ~/.zshrc ou ~/.bashrc:"
    echo "   export KAT_KEY='sua_api_key_aqui'"
    echo ""
    echo "⚠️ Execute 'source ~/.zshrc' após configurar"
fi

echo ""
echo "🐺 Modos disponíveis:"
echo "   ./target/release/fenrir              - Modo GOD MODE padrão"
echo "   ./target/release/fenrir --trinity    - Modo Trinity IA (Chain of Thoughts)"
echo ""
echo "🧠 Trinity IA features:"
echo "   ✅ Coordenação Gemini + Claude + Grok"
echo "   ✅ Chain of Thoughts completo"
echo "   ✅ Sistema de consenso entre IAs"
echo "   ✅ Particionamento automático de tarefas"
echo "   ✅ Aprovação final FENRIR GOD MODE"
echo ""

# Testar se o binário existe
if [ -f "./target/release/fenrir" ]; then
    echo "✅ FENRIR Trinity compilado e pronto"
else
    echo "❌ FENRIR não encontrado"
    echo "💡 Execute: cargo build --release"
fi

echo ""
echo "🔥 FENRIR TRINITY IA - PRONTO PARA AÇÃO! 🔥"
