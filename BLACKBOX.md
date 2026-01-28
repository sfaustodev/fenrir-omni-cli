# FENRIR - Smart Attack Orchestration Platform

## Project Overview

**Fenrir v1.6.66** is a comprehensive cybersecurity and penetration testing platform built in Rust, featuring stealth-first approaches and AI-powered orchestration. The project serves as a unified command-line interface for ethical hacking, bug bounty hunting, malware detection, and security research.

### Key Features
- **Multi-AI Orchestration**: Integrates with Gemini, Claude, Grok, Venice, and other AI providers
- **Natural Language Interface**: Interactive mode accepts natural language commands for security operations
- **Comprehensive Tool Suite**: Integrates 100+ Kali Linux tools with intelligent orchestration
- **Bug Bounty Automation**: OAuth analysis, subdomain enumeration, parameter fuzzing, Burp Suite integration
- **Malware Detection**: Dedicated malware hunter with advanced scanning capabilities
- **Ethical Framework**: Protocol 0 for handling ethical dilemmas in AI-assisted security testing
- **Plugin System**: Extensible architecture for custom security tools

## Architecture

### Workspace Structure
```
Fenrir/
├── Cargo.toml (workspace)
├── src/
│   ├── fenrir/          # Main CLI application
│   │   ├── Cargo.toml
│   │   ├── main.rs      # Interactive mode entry point
│   │   └── [modules]/   # Core functionality modules
│   └── bin/
│       └── fenrir_malware_hunter.rs  # Dedicated malware scanner
```

### Core Modules
- **fenrir_ai_layer**: Multi-provider AI abstraction with security isolation
- **fenrir_orchestrator**: Coordinates AI operations and task execution
- **kali_tools_comprehensive**: 100+ Kali Linux tools with async orchestration
- **bugbounty_tools**: OAuth analysis, subdomain enumeration, Burp integration
- **ethical_protocol**: Framework for ethical decision-making
- **interactive**: Natural language processing and fuzzy matching
- **batch_executor**: Parallel/sequential tool execution with progress tracking
- **secrets**: Secure credential management with keyring fallback
- **sandbox**: Resource limits and landlock security
- **health**: Monitoring and metrics collection
- **plugins**: Dynamic plugin loading system

## Building and Running

### Prerequisites
- Rust 1.70+ with Cargo
- Optional: Kali Linux tools for full functionality
- Optional: Ghostty terminal for enhanced interface

### Build Commands
```bash
# Development build
cargo build

# Optimized release build
cargo build --release

# Run main interactive mode
cargo run

# Run malware hunter
cargo run --bin fenrir_malware_hunter

# Run with specific arguments
cargo run -- [natural language command]
```

### Environment Setup
Create a `.env` file with API keys:
```env
GEMINI_API_KEY=your_gemini_key
BLACKBOX_API_KEY=your_blackbox_key
GROK_API_KEY=your_grok_key
VENICE_API_KEY=your_venice_key
```

## Development Conventions

### Code Style
- **Rust Edition**: 2021
- **Formatting**: Standard Rust formatting (`cargo fmt`)
- **Linting**: Clippy enabled with workspace dependencies
- **Async Runtime**: Tokio for all asynchronous operations
- **Error Handling**: `anyhow` for simplified error propagation
- **Serialization**: Serde with JSON for data exchange

### Architecture Patterns
- **Modular Design**: Clear separation of concerns with focused modules
- **Async-First**: All I/O operations are asynchronous
- **Plugin Architecture**: Extensible system for custom tools
- **Circuit Breakers**: Fault tolerance for external API calls
- **Resource Limits**: Sandboxing and memory constraints
- **Decision Logging**: Tracks user choices for context awareness

### Security Practices
- **Input Validation**: All user inputs validated and sanitized
- **Sandbox Execution**: Tools run with resource limits and isolation
- **Ethical Checks**: Protocol 0 for ethical dilemma handling
- **Secret Management**: Secure storage with keyring/OS integration
- **Audit Logging**: Comprehensive logging of all operations

## Usage Examples

### Interactive Mode
```bash
cargo run
🐺 fenrir> scan this website for vulnerabilities
🐺 fenrir> find oauth2 vulnerabilities in facebook
🐺 fenrir> perform phishing simulation
```

### CLI Mode
```bash
# Bug bounty reconnaissance
cargo run -- "recon example.com"

# OAuth analysis
cargo run -- "analyze oauth flow for https://example.com"

# Malware scanning
cargo run --bin fenrir_malware_hunter
```

### API Usage
```rust
use fenrir_ai_layer::call_ai;
use fenrir_orchestrator::FenrirOrchestrator;

// Initialize orchestrator
let orchestrator = FenrirOrchestrator::new();

// Process natural language command
let result = orchestrator.process_input("scan target.com for sql injection".to_string()).await;
```

## Testing

### Unit Tests
```bash
cargo test
```

### Integration Tests
```bash
cargo test --test integration
```

### Health Checks
```bash
# Health endpoint (when running)
curl http://localhost:3000/health

# Metrics endpoint
curl http://localhost:3000/metrics
```

## Deployment

### Release Profile
The project uses optimized release settings:
- **Optimization Level**: 3
- **LTO**: Link-time optimization enabled
- **Codegen Units**: 1 for maximum optimization
- **Panic**: Abort on panic for smaller binaries
- **Strip**: Debug symbols removed

### Distribution
```bash
# Build optimized binary
cargo build --release

# Binary location
./target/release/fenrir
```

## Contributing

### Development Workflow
1. **Fork and Clone**: Create a fork and clone locally
2. **Branch**: Create feature branches from main
3. **Code**: Follow Rust conventions and project patterns
4. **Test**: Add tests for new functionality
5. **Commit**: Use conventional commit messages
6. **PR**: Submit pull request with description

### Code Review Process
- **Automated Checks**: CI runs formatting, linting, and tests
- **Security Review**: All changes reviewed for security implications
- **Ethical Review**: New features evaluated against ethical guidelines
- **Performance**: Changes benchmarked for performance impact

## Security Considerations

### Ethical Use
- **Authorized Testing Only**: Only test systems you own or have explicit permission to test
- **Bug Bounty Programs**: All testing must be within authorized bug bounty scopes
- **Legal Compliance**: Follow all applicable laws and regulations
- **Responsible Disclosure**: Report vulnerabilities through proper channels

### Built-in Safeguards
- **Ethical Protocol 0**: AI-assisted ethical decision framework
- **Confirmation Prompts**: User confirmation required for destructive actions
- **Sandbox Limits**: Resource constraints prevent system damage
- **Audit Logging**: All actions logged for accountability

## Directory Structure Details

### Source Code (`src/fenrir/`)
- **Main Application**: Interactive CLI with natural language processing
- **46 Modules**: Comprehensive functionality coverage
- **AI Integration**: Multi-provider orchestration
- **Tool Integration**: Kali Linux and security tools
- **Security Features**: Sandboxing, encryption, monitoring

### Binaries (`src/bin/`)
- **Malware Hunter**: Specialized malware detection and removal
- **Standalone Tools**: Independent utilities for specific tasks

### Supporting Directories
- **docs/**: Documentation and guides
- **examples/**: Usage examples and demonstrations
- **fenrir_logs/**: Application logging
- **Formula/**: Homebrew formula for macOS installation
- **Ai/**: AI-related configurations and data
- **.claude/**: Claude AI integration files

## Performance Characteristics

### Resource Usage
- **Memory**: Configurable limits (default: 666MB async, 2048MB sequential)
- **CPU**: Parallel execution with concurrency controls
- **Network**: Connection pooling and timeout management
- **Storage**: Efficient logging and temporary file management

### Scalability
- **Concurrent Tasks**: Up to 10 parallel async operations
- **Batch Processing**: Sequential and parallel execution modes
- **Resource Monitoring**: Built-in health checks and metrics
- **Circuit Breakers**: Fault tolerance for external services

This platform represents a sophisticated approach to ethical cybersecurity tooling, combining AI intelligence with comprehensive security testing capabilities while maintaining strong ethical and safety boundaries.