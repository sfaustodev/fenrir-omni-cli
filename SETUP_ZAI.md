# 🚀 Quick Setup: Zai Orchestrator

## Step 1: Copy Environment Template

```bash
cp .env.template .env
```

## Step 2: Get Your API Keys

### Blackbox AI (Required for both Zai and Blackbox)
1. Go to https://www.blackbox.ai/
2. Sign up or log in
3. Navigate to API settings
4. Copy your API key

### Optional: Other Providers
- **Gemini**: https://makersuite.google.com/app/apikey
- **Venice**: https://venice.ai/

## Step 3: Edit .env File

Open `.env` and add your keys:

```bash
# Required
ZAI_API_KEY=your_blackbox_api_key_here
BLACKBOX_API_KEY=your_blackbox_api_key_here

# Optional
GEMINI_API_KEY=your_gemini_key_here
VENICE_API_KEY=your_venice_key_here
```

**Note**: You can use the same Blackbox API key for both `ZAI_API_KEY` and `BLACKBOX_API_KEY`.

## Step 4: Build Fenrir

```bash
cargo build --release
```

## Step 5: Run Fenrir

```bash
./target/release/fenrir
```

You should see:
```
✅ ZAI_API_KEY loaded (Fenrir Orchestrator)
✅ BLACKBOX_API_KEY loaded
🐺 FENRIR 4.0 - AI-Powered Command Translation
```

## Step 6: Test Commands

### Test Zai Orchestrator
```bash
🐺 fenrir> zai "what is your purpose?"
```

### Test Negão (Blackbox)
```bash
🐺 fenrir> negao "explain SQL injection"
```

### Test Security Scan
```bash
🐺 fenrir> scan localhost
```

## Troubleshooting

### Missing API Keys
```
⚠️  ZAI_API_KEY not found
```
**Solution**: Make sure you copied `.env.template` to `.env` and added your keys.

### Invalid API Key
```
❌ API error (401): Unauthorized
```
**Solution**: Check that your API key is correct in `.env`.

### Build Errors
```bash
cargo clean
cargo build --release
```

## Quick Reference

### Commands
- `zai "prompt"` - Strategic AI (orchestrator)
- `negao "prompt"` - General AI tasks
- `scan <target>` - Security scan
- `bite <target>` - Penetration test
- `gita tudo` - Git automation
- `exit` - Exit Fenrir

### Files
- `.env` - Your API keys (never commit!)
- `.env.template` - Template with instructions
- `ZAI_ORCHESTRATOR_GUIDE.md` - Full documentation
- `ZAI_IMPLEMENTATION_SUMMARY.md` - Technical details

## Need Help?

📖 **Full Guide**: Read `ZAI_ORCHESTRATOR_GUIDE.md`
📧 **Support**: sfaustodev@gmail.com

---

**🐺 Ready to hunt vulnerabilities with Zai!**
