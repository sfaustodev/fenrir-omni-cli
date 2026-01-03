# 🔧 BLACKBOX CLI FIXES PLAN

## 🎯 Problem Summary
Error: `TypeError: Cannot read properties of undefined (reading 'length')` in `createToolsSystemPrompt` at line 144 of `encryptedTurn.js`

## 🚨 Root Cause
The `tools` parameter is `undefined` when `createToolsSystemPrompt` is called, causing the `.length` access to fail.

## 📋 Implementation Plan

### Phase 1: Critical Fixes (Prevent Crashes)

1. **Fix `createToolsSystemPrompt` method** - Add null checks
2. **Fix `formatToolsForSystemPrompt` function** - Enhanced protection
3. **Fix tool registry initialization** - Prevent race conditions

### Phase 2: Tool Registry Fixes (Resolve Race Conditions)

4. **Modify `initializeWithoutMcp()`** - Preserve core tools
5. **Fix `createToolRegistry()` sequence** - Proper initialization order

### Phase 3: Enhanced Error Handling

6. **Add debugging logs** - Better error tracking
7. **Improve error messages** - Clearer feedback

## 🎯 Expected Outcome

- ✅ No more crashes from undefined tools
- ✅ Proper tool initialization with core tools available
- ✅ Background MCP discovery continues to work
- ✅ Better error messages for debugging
- ✅ Robust null checking throughout the system

## 📝 Files to Modify

1. `/Users/peluche/.blackbox-cli-v2/packages/core/dist/src/core/encryptedTurn.js`
2. `/Users/peluche/.blackbox-cli-v2/packages/core/dist/src/encrypt/tool-formatter.js`
3. `/Users/peluche/.blackbox-cli-v2/packages/core/dist/src/tools/tool-registry.js`
4. `/Users/peluche/.blackbox-cli-v2/packages/core/dist/src/config/config.js`
5. `/Users/peluche/.blackbox-cli-v2/packages/core/dist/src/core/encryptedGeminiClientWrapper.js`

## 🔧 Implementation Priority

1. **🔴 CRITICAL**: Phase 1 fixes (prevent immediate crash)
2. **🟡 HIGH**: Phase 2 fixes (fix race condition)
3. **🟢 MEDIUM**: Phase 3 fixes (improve initialization order)
4. **🔵 LOW**: Phase 4 fixes (add debugging)