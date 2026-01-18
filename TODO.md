# FENRIR Enhancement Plan v2.0 - Smart Attack Orchestration

## Phase 1: Interactive Mode Rebuild ✅
- [x] Rebuild interactive mode from scratch with proper input handling
- [x] Implement subject, keywords, context input paradigm
- [x] Add clarification prompts when input is incomplete
- [x] Display keywords and triggered attack sequences on screen
- [x] Add double-check confirmation with user before execution
- [x] Support edit mode for correcting interpretations

## Phase 2: NLP Integration ✅
- [x] Integrate ZAI_API_KEY for NLP interpretation of inputs
- [x] Add language detection and translation via GEMINI_API_KEY
- [x] Implement fallback to Grok API if ZAI fails
- [x] Parse natural language into subject, keywords, context
- [x] Generate smart attack sequences from keywords

## Phase 3: Smart Attack Sequences ✅
- [x] Implement keyword-triggered automation sequences
- [x] PASSWORD: cewl, crunch, hydra, hashcat, john, patator, medusa, ncrack
- [x] SCAN: nmap, masscan, rustscan, nikto, nuclei
- [x] RECON: theHarvester, amass, subfinder, dnsrecon, whois
- [x] SOCIAL: sherlock, maltego, recon-ng, spiderfoot, holehe
- [x] WEB: nikto, gobuster, ffuf, sqlmap, wpscan, xsstrike
- [x] WIRELESS: aircrack-ng, wifite, reaver, bully, kismet
- [x] OAUTH: burpsuite, evilginx2, modlishka, mitmproxy
- [x] DATABASE: sqlmap, odat, mssqlclient, mongoaudit
- [x] FORENSIC: autopsy, volatility, binwalk, foremost, bulk_extractor
- [x] EXPLOIT: metasploit, searchsploit, msfvenom
- [x] PRIVESC: linpeas, winpeas, pspy, linux-exploit-suggester
- [x] SHELL: netcat, socat, pwncat
- [x] SNIFF: wireshark, tcpdump, ettercap, bettercap, responder

## Phase 4: Stealth-First Approach ✅
- [x] Implement automatic stealth scan before any attack
- [x] Add stealth arguments for each tool (low rate, evasion, quiet)
- [x] Implement fallback to aggressive mode if stealth fails
- [x] Add aggressive arguments for each tool (high rate, verbose, thorough)
- [x] Log stealth/aggressive mode decisions

## Phase 5: Async Execution Enhancement ✅
- [x] Implement async batch attacks with tokio
- [x] Add separate thread for sequential attacks
- [x] Implement memory limits: 666MB per async thread
- [x] Implement memory limits: 2GB for sequential thread
- [x] Add semaphore for max concurrent tasks (10)
- [x] Execute async and sequential attacks in parallel

## Phase 6: Performance Optimization ✅
- [x] Use futures::join_all for parallel async execution
- [x] Implement memory monitoring per thread
- [x] Add task spawning with spawn_blocking for CPU-intensive ops
- [x] Optimize tool availability checks

## Phase 7: User Experience ✅
- [x] Add beautiful ASCII banner
- [x] Implement help command with examples
- [x] Add keywords command to list all available sequences
- [x] Show triggered sequences before execution
- [x] Add edit mode for correcting interpretations
- [x] Implement graceful exit handling

## Phase 8: Testing & Verification 🔄
- [ ] Build project with cargo build
- [ ] Test interactive mode with various inputs
- [ ] Test NLP interpretation with English inputs
- [ ] Test NLP interpretation with non-English inputs
- [ ] Test stealth scan execution
- [ ] Test aggressive scan fallback
- [ ] Test async attack sequences
- [ ] Test sequential attack sequences
- [ ] Verify memory limits are enforced
- [ ] Test all keyword triggers

## Summary

### New Features in v2.0:
1. **Complete Interactive Mode Rebuild** - New paradigm with subject, keywords, context
2. **ZAI NLP Integration** - Natural language to attack sequence translation
3. **13 Smart Attack Sequences** - One keyword triggers multiple related tools
4. **Stealth-First Approach** - Automatic stealth scan before any attack
5. **Async/Sequential Execution** - Parallel async (666MB) + sequential (2GB) threads
6. **Memory Limits** - Enforced per-thread memory limits for performance
7. **Double-Check Confirmation** - User confirms interpretation before execution
8. **Edit Mode** - Correct interpretations without retyping

### Attack Flow:
```
User Input → Translation (if needed) → ZAI Interpretation → 
User Confirmation → Stealth Scan → [Aggressive Fallback] → 
Async Attacks (parallel) + Sequential Attacks (one-by-one)
```

### Memory Architecture:
- Async threads: Max 666MB each, up to 10 concurrent
- Sequential thread: Max 2GB dedicated
- Both execute simultaneously for maximum performance

### Available Keywords:
| Keyword | Description | Mode | Memory |
|---------|-------------|------|--------|
| password | Password cracking | Async | 666MB |
| scan | Network scanning | Async | 666MB |
| recon | Reconnaissance | Async | 666MB |
| social | Social engineering | Sequential | 2GB |
| web | Web app testing | Async | 666MB |
| wireless | Wireless attacks | Sequential | 2GB |
| oauth | OAuth2 testing | Sequential | 2GB |
| database | Database exploitation | Async | 666MB |
| forensic | Digital forensics | Sequential | 2GB |
| exploit | Exploitation | Sequential | 2GB |
| privesc | Privilege escalation | Sequential | 2GB |
| shell | Reverse shells | Sequential | 2GB |
| sniff | Network sniffing | Sequential | 2GB |
