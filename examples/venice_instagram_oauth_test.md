# Venice AI - Instagram OAuth Testing Guide

## Setup

1. **Ensure your environment variables are set:**
   ```bash
   # From your .zshrc
   export VENICE_API_KEY="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"
   export VENICE_BASE_URL="https://api.venice.ai/api/v1"
   export OPENAI_BASE_URL="https://openrouter.ai/api/v1"
   ```

2. **Reload your shell:**
   ```bash
   source ~/.zshrc
   ```

## CLI Usage

### BLACKBOX MODE - Stealth Red Team Operations

**Test OAuth flow:**
```bash
# Test Instagram OAuth with stealth mode
./target/release/fenrir blackbox oauth "https://www.instagram.com/oauth/authorize"

# With additional details
./target/release/fenrir blackbox oauth \
  "https://api.instagram.com/oauth/authorize" \
  --details "Instagram uses OAuth 2.0 for third-party app authorization"
```

**Generate payloads:**
```bash
# Generate OAuth payloads
./target/release/fenrir blackbox payloads "oauth"

# Generate redirect_uri payloads
./target/release/fenrir blackbox payloads "redirect_uri"
```

**Get security strategy:**
```bash
./target/release/fenrir blackbox strategy
```

**Analyze findings:**
```bash
./target/release/fenrir blackbox analyze \
  "Found potential open redirect via redirect_uri parameter"
```

**Test connection:**
```bash
./target/release/fenrir blackbox test
```

### OPENCODE MODE - Verbose Security Testing

**Test OAuth flow (verbose):**
```bash
# Test Instagram OAuth with detailed output
./target/release/fenrir opencode oauth "https://www.instagram.com/oauth/authorize"

# With additional details
./target/release/fenrir opencode oauth \
  "https://api.instagram.com/oauth/authorize" \
  --details "Full OAuth 2.0 authorization code flow"
```

**Generate payloads (with explanations):**
```bash
./target/release/fenrir opencode payloads "oauth"
./target/release/fenrir opencode payloads "csrf_state"
```

**Get comprehensive strategy:**
```bash
./target/release/fenrir opencode strategy
```

**Analyze finding (detailed):**
```bash
./target/release/fenrir opencode analyze \
  "Missing state parameter in Instagram OAuth flow"
```

**Generate professional report:**
```bash
./target/release/fenrir opencode report \
  "Open redirect via redirect_uri" \
  "Missing CSRF protection" \
  "Authorization code in URL fragment"
```

**Test connection:**
```bash
./target/release/fenrir opencode test
```

**Show configuration:**
```bash
./target/release/fenrir opencode config
```

## Example Workflows

### 1. OAuth Flow Analysis for Instagram

**Step 1: Reconnaissance (Blackbox mode)**
```bash
# Get testing strategy
./target/release/fenrir blackbox strategy

# Generate OAuth payloads
./target/release/fenrir blackbox payloads "oauth"
```

**Step 2: Deep Analysis (Opencode mode)**
```bash
# Detailed OAuth analysis
./target/release/fenrir opencode oauth \
  "https://www.instagram.com/oauth/authorize?client_id=...&redirect_uri=..."
```

**Step 3: Findings Analysis**
```bash
# Analyze your findings
./target/release/fenrir opencode analyze \
  "Instagram allows redirect_uri manipulation to external domains"
```

**Step 4: Report Generation**
```bash
# Generate professional report
./target/release/fenrir opencode report \
  "OAuth Open Redirect via redirect_uri" \
  "Missing state parameter allows CSRF" \
  "Authorization code leakage in browser history"
```

### 2. Payload Generation for Testing

```bash
# Generate comprehensive payloads
./target/release/fenrir opencode payloads "oauth_redirect"

# Test with specific target
./target/release/fenrir blackbox payloads "instagram_oauth"
```

## Targeted Instagram OAuth Testing

### Common OAuth Vulnerabilities to Test:

1. **Open Redirect via redirect_uri**
   ```bash
   ./target/release/fenrir opencode oauth \
     "https://www.instagram.com/oauth/authorize" \
     --details "Testing redirect_uri manipulation for open redirect"
   ```

2. **Missing/Weak State Parameter (CSRF)**
   ```bash
   ./target/release/fenrir opencode oauth \
     "https://api.instagram.com/oauth/authorize" \
     --details "Checking for CSRF protection via state parameter"
   ```

3. **Authorization Code Leakage**
   ```bash
   ./target/release/fenrir opencode analyze \
     "Authorization code exposed in URL fragment or Referer header"
   ```

4. **Token Exposure**
   ```bash
   ./target/release/fenrir opencode analyze \
     "Access tokens exposed in browser storage or logs"
   ```

## Integration with Bug Bounty Workflow

### Before Submitting to Instagram Bug Bounty:

1. **Validate your findings with AI:**
   ```bash
   ./target/release/fenrir opencode analyze \
     "Your detailed finding description here"
   ```

2. **Generate professional report:**
   ```bash
   ./target/release/fenrir opencode report \
     "Finding 1 description" \
     "Finding 2 description" \
     "Finding 3 description"
   ```

3. **Get impact assessment:**
   ```bash
   ./target/release/fenrir opencode analyze \
     "What's the business impact of this OAuth vulnerability?"
   ```

## Tips for Best Results

1. **Use Blackbox mode for:**
   - Quick reconnaissance
   - Initial vulnerability assessment
   - Stealth testing
   - Minimal logging

2. **Use Opencode mode for:**
   - Detailed analysis
   - Report writing
   - Educational purposes
   - Full documentation

3. **Be specific in your descriptions:**
   ```bash
   # Good
   ./target/release/fenrir opencode oauth "https://api.instagram.com/oauth/authorize" \
     --details "OAuth 2.0 Authorization Code flow, client_id visible, redirect_uri accepts external domains"

   # Less detailed
   ./target/release/fenrir opencode oauth "https://instagram.com/oauth"
   ```

4. **Chain commands for comprehensive testing:**
   ```bash
   # Full workflow
   ./target/release/fenrir blackbox strategy
   ./target/release/fenrir blackbox payloads "oauth"
   ./target/release/fenrir opencode oauth "https://www.instagram.com/oauth/authorize"
   ./target/release/fenrir opencode analyze "Your findings"
   ./target/release/fenrir opencode report "Finding 1" "Finding 2"
   ```

## Important Notes

- Only test targets within authorized bug bounty programs
- Follow Instagram's bug bounty rules of engagement
- Never test outside the defined scope
- Report vulnerabilities responsibly through proper channels
- Use Venice AI to assist, not replace, manual testing
- Always verify AI findings with manual testing
- Document all your testing activities

## API Configuration

The Venice AI integration reads from:

- `VENICE_API_KEY`: Your OpenRouter/Venice API key
- `VENICE_BASE_URL`: API base URL (default: https://api.venice.ai/api/v1)
- `OPENAI_BASE_URL`: Alternative OpenRouter endpoint

Your current configuration from .zshrc:
```bash
VENICE_API_KEY="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"
OPENAI_BASE_URL="https://openrouter.ai/api/v1"
```

This configuration uses OpenRouter to access Venice's uncensored AI models.

## Troubleshooting

**If API connection fails:**
```bash
# Test the connection
./target/release/fenrir opencode test

# Show current configuration
./target/release/fenrir opencode config

# Verify environment variables
echo $VENICE_API_KEY
echo $OPENAI_BASE_URL
```

**If you get authorization errors:**
- Verify your API key is correct
- Check you have credits on OpenRouter
- Ensure the endpoint URL is correct

**If responses are too short or too long:**
- Blackbox mode: Concise, focused responses (max 2048 tokens)
- Opencode mode: Detailed, comprehensive responses (max 4096 tokens)

## Happy Bug Hunting! 🐺

Remember: With great power comes great responsibility. Use these tools ethically and only for authorized testing!
