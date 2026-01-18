# ⚡ VENICE AI - QUICK START GUIDE

## 🚀 Instant Testing

### Test Your Instagram OAuth NOW:

```bash
# 1. Build Fenrir
cargo build --release

# 2. Test Venice AI connection
./target/release/fenrir opencode test

# 3. Test Instagram OAuth (verbose mode)
./target/release/fenrir opencode oauth \
  "https://www.instagram.com/oauth/authorize"

# 4. Generate OAuth payloads
./target/release/fenrir opencode payloads "oauth"

# 5. Get security testing strategy
./target/release/fenrir opencode strategy
```

## 🎯 BLACKBOX vs OPENCODE

### BLACKBOX (Stealth Mode)
```bash
# Quick, concise responses
# Minimal logging
# For rapid testing

./target/release/fenrir blackbox oauth "https://instagram.com/oauth"
./target/release/fenrir blackbox payloads "oauth"
./target/release/fenrir blackbox analyze "Open redirect found"
```

### OPENCODE (Verbose Mode)
```bash
# Detailed, comprehensive responses
# Educational explanations
# For deep analysis and reporting

./target/release/fenrir opencode oauth "https://instagram.com/oauth"
./target/release/fenrir opencode payloads "oauth"
./target/release/fenrir opencode analyze "Open redirect found"
```

## 🔥 INSTAGRAM OAUTH TESTING

### Quick OAuth Analysis:
```bash
./target/release/fenrir opencode oauth \
  "https://www.instagram.com/oauth/authorize?client_id=YOUR_CLIENT_ID&redirect_uri=YOUR_REDIRECT_URI" \
  --details "Testing Instagram OAuth 2.0 flow for vulnerabilities"
```

### Generate Test Payloads:
```bash
# OAuth-specific payloads
./target/release/fenrir opencode payloads "oauth"

# Redirect URI testing
./target/release/fenrir opencode payloads "redirect_uri"

# CSRF testing
./target/release/fenrir opencode payloads "csrf"
```

### Analyze Your Findings:
```bash
./target/release/fenrir opencode analyze \
  "Instagram allows redirect_uri to be manipulated to evil.com"
```

### Generate Professional Report:
```bash
./target/release/fenrir opencode report \
  "OAuth Open Redirect via redirect_uri" \
  "Missing state parameter allows CSRF attacks" \
  "Authorization code leakage in browser history"
```

## 📊 Complete Workflow

```bash
# Step 1: Get strategy
./target/release/fenrir opencode strategy

# Step 2: Generate payloads
./target/release/fenrir opencode payloads "oauth"

# Step 3: Analyze OAuth flow
./target/release/fenrir opencode oauth "https://instagram.com/oauth/authorize"

# Step 4: Analyze your findings
./target/release/fenrir opencode analyze "Your finding here"

# Step 5: Generate report
./target/release/fenrir opencode report "Finding 1" "Finding 2"
```

## 🔧 Configuration Check

```bash
# Check your Venice AI configuration
./target/release/fenrir opencode config

# Test API connection
./target/release/fenrir opencode test
```

## 🎓 Common OAuth Vulnerabilities to Test

### 1. Open Redirect
```bash
./target/release/fenrir opencode analyze \
  "redirect_uri parameter accepts external domains like evil.com"
```

### 2. CSRF (Missing State)
```bash
./target/release/fenrir opencode analyze \
  "OAuth flow missing state parameter for CSRF protection"
```

### 3. Code Leakage
```bash
./target/release/fenrir opencode analyze \
  "Authorization code exposed in URL fragment or Referer header"
```

### 4. HTTP Usage
```bash
./target/release/fenrir opencode analyze \
  "OAuth endpoints using HTTP instead of HTTPS"
```

## 💡 Tips

- Use **opencode** for learning and detailed analysis
- Use **blackbox** for quick reconnaissance
- Be **specific** in your descriptions for better results
- Always **verify** AI findings with manual testing
- Follow Instagram's **bug bounty rules**

## ⚠️ Important

- Only test **authorized targets**
- Follow **rules of engagement**
- Report **responsibly**
- Use **ethically**

Your Venice API Key from .zshrc:
```
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
```

## 🎉 You're Ready!

Start testing Instagram OAuth now:

```bash
./target/release/fenrir opencode oauth "https://www.instagram.com/oauth/authorize"
```

Good luck with your bug bounty! 🍀
