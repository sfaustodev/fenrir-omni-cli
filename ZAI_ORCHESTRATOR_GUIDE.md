# 🧠 ZAI - FENRIR ORCHESTRATOR GUIDE

## Overview

**Zai** is the main brain of Fenrir - the orchestrator that makes strategic decisions and delegates tasks to specialized AI providers.

## Architecture

```
┌─────────────────────────────────────────────────────────────┐
│                    ZAI ORCHESTRATOR                         │
│              (Fenrir's Main Brain)                          │
│                                                             │
│  • Strategic Decision Making                               │
│  • Task Delegation                                         │
│  • Multi-AI Coordination                                   │
└─────────────────────────────────────────────────────────────┘
                            │
                            ├──────────────┬──────────────┬──────────────┐
                            ▼              ▼              ▼              ▼
                    ┌──────────────┐ ┌──────────────┐ ┌──────────────┐ ┌──────────────┐
                    │   GEMINI     │ │   BLACKBOX   │ │   VENICE     │ │   FUTURE     │
                    │ (Translator) │ │ (General)    │ │ (Red Team)   │ │   AIs...     │
                    └──────────────┘ └──────────────┘ └──────────────┘ └──────────────┘
```

## AI Providers

### 1. **Zai - Fenrir Orchestrator** 🧠
- **Role**: Main brain and strategic decision maker
- **API**: Blackbox AI (blackboxai-pro model)
- **Capabilities**:
  - Strategic planning
  - Task delegation
  - Multi-AI coordination
  - High-level security analysis
- **Command**: `zai "your prompt"`
- **Environment Variable**: `ZAI_API_KEY`

### 2. **Blackbox (Negão)** 🤖
- **Role**: General task handler (replaces Grok)
- **API**: Blackbox AI (blackboxai-pro model)
- **Capabilities**:
  - Command translation
  - General security tasks
  - Code analysis
  - Tool execution
- **Command**: `negao "your prompt"` or `blackbox "your prompt"`
- **Environment Variable**: `BLACKBOX_API_KEY`

### 3. **Gemini** 🌐
- **Role**: Translation layer
- **API**: Google Gemini
- **Capabilities**:
  - Natural language to command translation
  - Portuguese/English support
- **Environment Variable**: `GEMINI_API_KEY`

### 4. **Venice** 🔴
- **Role**: Red team operations (unguarded)
- **API**: Venice AI
- **Capabilities**:
  - Aggressive penetration testing
  - No safety filters
  - Exploit development
- **Environment Variable**: `VENICE_API_KEY`

## Setup Instructions

### 1. Copy Environment Template
```bash
cp .env.template .env
```

### 2. Add Your API Keys

Edit `.env` and add your keys:
##GET FROM .ENV OR .zshrc 
```bash
# Main orchestrator (required) 
ZAI_API_KEY=your_zai_api_key_here

# General tasks (required)
BLACKBOX_API_KEY=your_blackbox_api_key_here

# Translation (optional)
GEMINI_API_KEY=your_gemini_api_key_here

# Red team (optional)
VENICE_API_KEY=your_venice_api_key_here
```

### 3. Get API Keys

- **Blackbox AI**: https://www.blackbox.ai/
  - Used for both Zai and Blackbox providers
  - Sign up and get your API key
  
- **Google Gemini**: https://makersuite.google.com/app/apikey
  - Optional for translation layer
  
- **Venice AI**: https://venice.ai/
  - Optional for aggressive red team operations

### 4. Build and Run

```bash
cargo build --release
./target/release/fenrir
```

## Usage Examples

### Strategic Planning with Zai
```bash
🐺 fenrir> zai "analyze the security posture of a web application"
🧠 ZAI ORCHESTRATOR (FENRIR BRAIN):
[Strategic analysis and recommendations...]
```

### General Tasks with Negão (Blackbox)
```bash
🐺 fenrir> negao "explain SQL injection vulnerabilities"
🤖 NEGAO (BLACKBOX) RESPONSE:
[Detailed explanation...]
```

### Security Scanning
```bash
🐺 fenrir> scan example.com --comprehensive
✅ SCAN COMPLETE
🎯 Target: example.com
🔍 Open Ports: 3
🛡️  Risk Score: 45/100
```

### Penetration Testing
```bash
🐺 fenrir> bite example.com --aggressive
✅ BITE COMPLETE - FENRIR HAS DEVOURED THE TARGET
🎯 Success: true
🔍 Findings: 12
💥 Vulnerabilities: 3
```

### Natural Language Commands
```bash
🐺 fenrir> listar arquivos
🤖 AI understood: List all files
🔧 Executing: ls -la

🐺 fenrir> onde estou
🤖 AI understood: Print working directory
🔧 Executing: pwd
```

## Command Reference

### Direct AI Queries
- `zai "prompt"` - Query Zai Orchestrator (strategic decisions)
- `negao "prompt"` - Query Blackbox AI (general tasks)
- `blackbox "prompt"` - Alternative to negao

### Security Operations
- `scan <target>` - Security assessment
- `scan <target> --comprehensive` - Deep security scan
- `bite <target>` - Penetration testing
- `bite <target> --aggressive` - Aggressive pentest

### Git Automation
- `gita tudo` - Git add, commit, push
- `gita ai` - Git add, commit (no push)

### System
- `exit` / `quit` / `sair` - Exit Fenrir

## Decision Flow

```
User Input
    │
    ▼
┌─────────────────┐
│ Natural Language│
│   Detection     │
└─────────────────┘
    │
    ▼
┌─────────────────┐      ┌──────────────┐
│ Command Parser  │─────▶│ Direct Exec  │
└─────────────────┘      └──────────────┘
    │
    ▼
┌─────────────────┐
│ AI Translation  │ (Blackbox)
└─────────────────┘
    │
    ▼
┌─────────────────┐
│ Execute Command │
└─────────────────┘
```

## Why Zai as Orchestrator?

1. **Strategic Intelligence**: Makes high-level decisions about task delegation
2. **Multi-AI Coordination**: Coordinates multiple specialized AIs
3. **Context Awareness**: Understands the full security context
4. **Adaptive**: Can choose the best AI for each specific task
5. **Scalable**: Easy to add new AI providers in the future

## Blackbox vs Grok

| Feature | Grok (Old) | Blackbox (New) |
|---------|-----------|----------------|
| API | X.AI | Blackbox AI |
| Model | grok-3 | blackboxai-pro |
| Speed | Fast | Very Fast |
| Context | 4K tokens | 8K tokens |
| Cost | Higher | Lower |
| Availability | Limited | High |

## Security Considerations

- **API Keys**: Never commit `.env` to git
- **Rate Limits**: Be aware of API rate limits
- **Authorization**: Only test systems you own or have permission to test
- **Logging**: All AI calls are logged for audit purposes
- **Guard Rails**: Blackbox has guard rails, Venice does not

## Troubleshooting

### API Key Not Found
```bash
⚠️  ZAI_API_KEY not found
⚠️  BLACKBOX_API_KEY not found
```
**Solution**: Add keys to `.env` file

### API Error 401
```bash
❌ API error (401): Unauthorized
```
**Solution**: Check if your API key is valid

### API Error 429
```bash
❌ API error (429): Rate limit exceeded
```
**Solution**: Wait a few minutes and try again

### Connection Timeout
```bash
❌ Zai API call failed: connection timeout
```
**Solution**: Check your internet connection

## Future Enhancements

- [ ] Add more AI providers (Claude, GPT-4, etc.)
- [ ] Implement AI voting system for critical decisions
- [ ] Add AI performance metrics and comparison
- [ ] Implement cost tracking per AI provider
- [ ] Add AI fallback mechanisms
- [ ] Implement parallel AI queries for speed

## Contributing

To add a new AI provider:

1. Add enum variant to `AIProvider` in `fenrir_ai_layer.rs`
2. Implement `call_your_ai()` function
3. Add routing in `call_ai()` function
4. Update documentation

## Support

- **Email**: sfaustodev@gmail.com
- **Issues**: GitHub Issues
- **Documentation**: `/docs/`

---

**🐺 FENRIR 4.0 - Powered by Zai Orchestrator**

*"The wolf that devours security vulnerabilities with AI intelligence"*
