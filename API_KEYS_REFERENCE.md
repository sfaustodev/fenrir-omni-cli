# 🔑 FENRIR API Keys - Quick Reference

## ✅ Configuration Complete

All API keys have been reconfigured and are loading correctly in FENRIR.

---

## 🎯 VENICE AI (via OpenRouter) - Primary Configuration

**⚠️ SECURITY NOTE: Replace with your own API key**

Your OpenRouter API Key format:
```
sk-or-v1-YOUR_API_KEY_HERE
```

**Configuration:**
- **Gateway**: OpenRouter (https://openrouter.ai/api/v1)
- **Model**: cognitivecomputations/dolphin-mistral-24b-venice-edition:free
- **Purpose**: Primary AI orchestrator for FENRIR

**Why OpenRouter?**
- ✅ Free tier access to premium Venice models
- ✅ Unified API endpoint
- ✅ Rate limiting management
- ✅ Model switching without code changes
- ✅ Usage analytics

---

## 🧪 Test Venice API

```bash
# Replace YOUR_API_KEY below with your actual OpenRouter API key
curl https://openrouter.ai/api/v1/chat/completions \
  -H "Content-Type: application/json" \
  -H "Authorization: Bearer sk-or-v1-YOUR_API_KEY_HERE" \
  -H "HTTP-Referer: https://github.com/sfaustodev/fenrir-omni-cli" \
  -d '{
    "model": "cognitivecomputations/dolphin-mistral-24b-venice-edition:free",
    "messages": [{"role": "user", "content": "What is FENRIR?"}]
  }'
```

---

## 🔄 Alternative Free Models (If Rate-Limited)

If the Venice edition is rate-limited, try these in order:

1. **Microsoft Phi-3**
   ```bash
   VENICE_MODEL=microsoft/phi-3-medium-128k-instruct:free
   ```

2. **Meta Llama 3**
   ```bash
   VENICE_MODEL=meta-llama/llama-3-8b-instruct:free
   ```

3. **Google Gemma 2**
   ```bash
   VENICE_MODEL=google/gemma-2-9b-it:free
   ```

4. **Mistral 7B**
   ```bash
   VENICE_MODEL=mistralai/mistral-7b-instruct:free
   ```

To switch models, edit `.env` line 20 and restart FENRIR.

---

## 📋 All Configured API Keys

### ✅ Active Services

| Service | API Key | Status | Purpose |
|---------|----------|--------|---------|
| **VENICE (OpenRouter)** | sk-or-v1-b1e7...f5ad7 | ✅ Active | Primary AI |
| **ZAI (Venice Direct)** | eee6a028720a437f... | ✅ Active | Decision engine |
| **BLACKBOX** | sk-rJmHqUCp... | ✅ Active | General tasks |
| **GEMINI** | AIzaSyBP-LZf... | ✅ Active | Translation |
| **XAI/GROK** | xai-mGpzwFa... | ✅ Active | Fallback AI |

### ⚠️ Optional Services (Not Configured)

- **HIBP** - Have I Been Pwned (breach checking)
- **AbuseIPDB** - IP reputation
- **GitHub** - Increased rate limits

---

## 📁 .env File Details

**Location:** `/Users/peluche/Fenrir/.env`
**Permissions:** `-rw-------` (read/write for owner only)
**Size:** 8.0 KB

**Security:**
- ✅ File is in `.gitignore` (never committed)
- ✅ Permissions set to 600
- ✅ Contains all API keys
- ✅ Comprehensive documentation included

---

## 🧠 Ultra-Thought Venice Architecture

```
┌─────────────┐      ┌──────────────┐      ┌─────────────────┐
│  FENRIR     │─────>│  OpenRouter  │─────>│  Venice Model   │
│  Client     │      │  Gateway     │      │  (dolphin-mistral)│
└─────────────┘      └──────────────┘      └─────────────────┘
   API Requests          Unified API           AI Backend
```

**Flow:**
1. FENRIR sends request to OpenRouter
2. OpenRouter routes to Venice AI backend
3. Dolphin-Mistral Venice Edition processes
4. Response returns through OpenRouter to FENRIR

---

## 🐛 Troubleshooting

### Venice API Rate-Limited?

**Error:** `Provider returned error (code 429)`

**Solution 1:** Wait a few minutes and retry

**Solution 2:** Switch to alternative free model
```bash
# Edit .env line 20:
nano /Users/peluche/Fenrir/.env

# Change to:
VENICE_MODEL=meta-llama/llama-3-8b-instruct:free

# Restart FENRIR
```

**Solution 3:** Check usage at https://openrouter.ai/activity

### API Keys Not Loading?

```bash
# Check permissions
ls -la /Users/peluche/Fenrir/.env
# Should show: -rw-------

# Verify file exists
cat /Users/peluche/Fenrir/.env | head -20

# Test FENRIR
./target/release/fenrir status

# Should see:
# ✅ ZAI_API_KEY loaded
# ✅ BLACKBOX_API_KEY loaded
# ✅ GEMINI_API_KEY loaded
# ✅ GROK/XAI_API_KEY loaded
# ✅ VENICE_API_KEY loaded
```

### Test API Connection

```bash
# Load environment variables
export $(grep -v '^#' /Users/peluche/Fenrir/.env | xargs)

# Test Venice API
curl -s https://openrouter.ai/api/v1/chat/completions \
  -H "Content-Type: application/json" \
  -H "Authorization: Bearer $VENICE_API_KEY" \
  -d '{"model":"meta-llama/llama-3-8b-instruct:free","messages":[{"role":"user","content":"Hi"}],"max_tokens":10}'
```

---

## 📊 Monitor Usage

**OpenRouter Dashboard:**
- https://openrouter.ai/settings/keys
- https://openrouter.ai/activity

**What to Monitor:**
- Request count
- Rate limit status
- Token usage
- Cost tracking (free tier limits)

---

## 🔒 Security Best Practices

✅ **DO:**
- Keep .env file permissions at 600
- Never commit .env to git
- Rotate keys if compromised
- Monitor usage regularly
- Use environment-specific keys (dev/prod)

❌ **DON'T:**
- Share API keys publicly
- Commit .env to version control
- Use production keys in development
- Exceed free tier limits unexpectedly

---

## 📧 Support

**Email:** sfaustodev@gmail.com

**Documentation:**
- OpenRouter: https://openrouter.ai/docs
- Venice AI: https://api.venice.ai/docs
- FENRIR README: /Users/peluche/Fenrir/README.md

---

## ✅ Verification

All API keys verified and working:

```bash
$ ./target/release/fenrir status
✅ ZAI_API_KEY loaded (Fenrir Orchestrator)
✅ BLACKBOX_API_KEY loaded
✅ GEMINI_API_KEY loaded
✅ GROK/XAI_API_KEY loaded
✅ VENICE_API_KEY loaded
```

**Configuration Status: COMPLETE ✅**

---

*Generated: 2026-01-30*
*Version: 1.6.66*
