#!/bin/bash

# Test script for Blackbox CLI fixes
echo "🔧 Testing Blackbox CLI fixes..."

# Check if the main files exist
echo "📁 Checking modified files..."

if [ -f "/Users/peluche/.blackbox-cli-v2/packages/core/dist/src/core/encryptedTurn.js" ]; then
    echo "✅ encryptedTurn.js exists"
    # Check if null check was added
    if grep -q "if (!tools || !Array.isArray(tools))" "/Users/peluche/.blackbox-cli-v2/packages/core/dist/src/core/encryptedTurn.js"; then
        echo "✅ Null check added to createToolsSystemPrompt"
    else
        echo "❌ Null check missing in createToolsSystemPrompt"
    fi
else
    echo "❌ encryptedTurn.js not found"
fi

if [ -f "/Users/peluche/.blackbox-cli-v2/packages/core/dist/src/encrypt/tool-formatter.js" ]; then
    echo "✅ tool-formatter.js exists"
    # Check if debugging was added
    if grep -q "console.log.*DEBUG.*formatToolsForSystemPrompt" "/Users/peluche/.blackbox-cli-v2/packages/core/dist/src/encrypt/tool-formatter.js"; then
        echo "✅ Debug logging added to formatToolsForSystemPrompt"
    else
        echo "❌ Debug logging missing in formatToolsForSystemPrompt"
    fi
else
    echo "❌ tool-formatter.js not found"
fi

if [ -f "/Users/peluche/.blackbox-cli-v2/packages/core/dist/src/tools/tool-registry.js" ]; then
    echo "✅ tool-registry.js exists"
    # Check if debugging was added to initializeWithoutMcp
    if grep -q "console.log.*DEBUG.*initializeWithoutMcp" "/Users/peluche/.blackbox-cli-v2/packages/core/dist/src/tools/tool-registry.js"; then
        echo "✅ Debug logging added to initializeWithoutMcp"
    else
        echo "❌ Debug logging missing in initializeWithoutMcp"
    fi
else
    echo "❌ tool-registry.js not found"
fi

if [ -f "/Users/peluche/.blackbox-cli-v2/packages/core/dist/src/core/encryptedGeminiClientWrapper.js" ]; then
    echo "✅ encryptedGeminiClientWrapper.js exists"
    # Check if debugging was added
    if grep -q "console.log.*DEBUG.*EncryptedGeminiClientWrapper" "/Users/peluche/.blackbox-cli-v2/packages/core/dist/src/core/encryptedGeminiClientWrapper.js"; then
        echo "✅ Debug logging added to EncryptedGeminiClientWrapper"
    else
        echo "❌ Debug logging missing in EncryptedGeminiClientWrapper"
    fi
else
    echo "❌ encryptedGeminiClientWrapper.js not found"
fi

if [ -f "/Users/peluche/.blackbox-cli-v2/packages/core/dist/src/core/encryptedGeminiClientBridge.js" ]; then
    echo "✅ encryptedGeminiClientBridge.js exists"
    # Check if debugging was added
    if grep -q "console.log.*DEBUG.*EncryptedGeminiClientBridge" "/Users/peluche/.blackbox-cli-v2/packages/core/dist/src/core/encryptedGeminiClientBridge.js"; then
        echo "✅ Debug logging added to EncryptedGeminiClientBridge"
    else
        echo "❌ Debug logging missing in EncryptedGeminiClientBridge"
    fi
else
    echo "❌ encryptedGeminiClientBridge.js not found"
fi

echo ""
echo "🎯 Test completed. Run Blackbox CLI to see if the fixes resolve the issue."
echo "🔍 Look for DEBUG and WARNING messages in the console output."