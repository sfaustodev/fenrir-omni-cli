# Fenrir Smart API Integration Guide

## 🚀 Overview

Fenrir now uses **Smart API Integration** that combines:
1. **HTTP requests via bash functions** (primary method)
2. **Direct HTTP calls** (fallback method)
3. **Intelligent CLI detection** and cooperation
4. **Smart routing** based on prompt type

## 🔥 Features

### 1. Bash HTTP Functions (Primary)

Fenrir automatically detects and uses bash API functions from your `~/.zshrc`:
- `gemini` - Google Gemini 2.5 Pro
- `grok` / `xai` - X.AI Grok-2
- `zai` - ZAI GLM 4.7 (default)
- `qwen` - Alibaba Qwen Max

**Benefits:**
- ✅ No need to manage API keys in Rust code
- ✅ Centralized configuration in `.zshrc`
- ✅ Easy to test from command line
- ✅ Automatic JSON parsing via python3

### 2. CLI Mode Detection

Fenrir detects when running inside another CLI (like Claude Code):
```rust
let cli_mode = std::env::var("FENRIR_CLI_MODE").is_ok()
    || std::env::var("CLAUDE_CODE_DESKTOP_PARENT_PROCESS_ID").is_ok();
```

When in CLI mode, Fenrir:
- Notifies the parent CLI about API calls
- Cooperates intelligently with command execution
- Provides verbose feedback for debugging

### 3. Smart Routing

The `smart_call()` function routes prompts to the best API:

```rust
pub fn smart_call(&self, prompt: &str) -> Result<String> {
    if prompt.contains("code") || prompt.contains("debug") {
        // Code tasks → ZAI (GLM 4.7)
        self.call_zai(prompt)
    } else if prompt.contains("fast") || prompt.contains("quick") {
        // Fast tasks → Gemini
        self.call_gemini(prompt)
    } else if prompt.contains("creative") || prompt.contains("write") {
        // Creative tasks → Grok
        self.call_grok(prompt)
    } else {
        // Default → ZAI
        self.call_zai(prompt)
    }
}
```

## 💻 Usage

### Interactive Mode Commands

```bash
fenrir
```

Then use any of these commands:

```bash
# Smart routing (automatic API selection)
ai Explain how recursion works

# Direct API calls
gemini What is Rust?
grok Write a poem about coding
zai Debug this function: fn main() {}
qwen Explain machine learning

# Direct aliases
ask same as 'ai'
xai same as 'grok'
```

### Examples

```bash
# Start Fenrir
./fenrir

# In interactive mode:
fenrir> ai Write a Python web scraper
✅ AI Response: [response from best API]

fenrir> gemini Explain quantum computing
✅ Gemini Response: [response from Gemini]

fenrir> zai Help me optimize this code
✅ ZAI Response: [response from GLM 4.7]

fenrir> grok Write a creative story
✅ Grok Response: [response from Grok]

fenrir> sair
🐺 FENRIR encerrando. Até a próxima!
```

## 🔧 Configuration

### Environment Variables

Fenrir respects these environment variables:

```bash
# CLI mode (auto-detected, but can be forced)
export FENRIR_CLI_MODE=1

# API Keys (used for direct HTTP fallback)
export GEMINI_API_KEY="your-key"
export GROK_API_KEY="your-key"
export XAI_API_KEY="your-key"
export ZAI_API_KEY="your-key"
export QWEN3_API_KEY="your-key"
```

### Bash API Functions

Make sure these are in your `~/.zshrc`:

```bash
# From ~/API_USAGE_GUIDE.md
gemini() { ... }
grok() { ... }
xai() { ... }
zai() { ... }
qwen() { ... }
```

Fenrir will automatically detect and use them!

## 🎯 Intelligent Cooperation

### When Running Inside Claude Code

If Fenrir detects it's running inside Claude Code:
1. It uses bash API functions by default
2. Notifies Claude about API calls
3. Cooperates with Claude's command execution
4. Falls back to direct HTTP if bash functions unavailable

### Example Workflow

```bash
# Inside Claude Code
cd ~/Fenrir
./fenrir

# Fenrir detects Claude Code CLI
# Uses bash API functions automatically
# Routes intelligently based on prompt type

fenrir> ai Create a REST API
🔥 Routing to ZAI (GLM 4.7) for code task
✅ AI Response: [generated code]
```

## 📊 Architecture

```
┌─────────────────────────────────────┐
│         Fenrir CLI                  │
│   (Minimalistic FENRIR banner)      │
└──────────────┬──────────────────────┘
               │
               ├─► Smart API Client
               │    │
               │    ├─► Bash Function Detection
               │    ├─► CLI Mode Detection
               │    └─► Smart Routing
               │
               ├─► Primary: Bash HTTP Functions
               │    ├─► gemini "prompt"
               │    ├─► grok "prompt"
               │    ├─► zai "prompt"
               │    └─► qwen "prompt"
               │
               └─► Fallback: Direct HTTP Calls
                    ├─► reqwest::Client
                    └─► API endpoints
```

## 🔍 Troubleshooting

### Bash Functions Not Found

```bash
# Test if bash functions are available
type gemini grok zai qwen

# If not found, reload .zshrc
source ~/.zshrc
```

### API Keys Not Set

```bash
# Check environment variables
echo $GEMINI_API_KEY
echo $GROK_API_KEY
echo $ZAI_API_KEY

# For direct HTTP fallback, keys must be set
```

### Force Direct HTTP Mode

If bash functions fail, Fenrir automatically falls back to direct HTTP calls using `reqwest` client.

## 📝 Minimalistic Banner

The new banner is clean and simple:

```
██   ██ ██    ██ ██████  ███████ ██████
 ██ ██  ██    ██ ██   ██ ██      ██   ██
  ███   ██    ██ ██████  █████   ██████
 ██ ██  ██    ██ ██      ██      ██   ██
██   ██  ██████  ██      ███████ ██   ██
```

No extra text, no complex ASCII art - just **FENRIR** big and bold!

## 🚀 Performance

- **Bash functions**: Fast, uses curl + python3 for parsing
- **Direct HTTP**: Slightly faster, no subprocess overhead
- **Smart routing**: Chooses best API for task type
- **CLI cooperation**: Intelligent mode switching

## 📚 Related Documentation

- `~/API_USAGE_GUIDE.md` - Complete API function guide
- `~/API_GUIDE.md` - Quick reference card
- `~/.zshrc` - Bash API function definitions
- `~/Fenrir/src/fenrir/src/smart_api.rs` - Implementation

## 🎉 Summary

Fenrir now:
1. ✅ Uses bash HTTP functions by default
2. ✅ Detects CLI mode automatically
3. ✅ Routes intelligently based on prompt type
4. ✅ Falls back to direct HTTP when needed
5. ✅ Cooperates with parent CLIs intelligently
6. ✅ Shows minimalistic banner
7. ✅ Centralized configuration in .zshrc

**One binary, multiple APIs, intelligent routing, minimalistic design!**
