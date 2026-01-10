# Kali Tools Enhancement TODO

## Phase 1: Expand Tool Database ✅
- [x] Add 100+ more Kali tools to kali_tools_comprehensive.rs
- [x] Add credential hunting tools (hydra, john, hashcat, patator, ncrack, etc.)
- [x] Add wireless tools (aircrack-ng suite, wifite, fluxion, etc.)
- [x] Add social engineering tools (setoolkit, king-phisher, gophish, evilginx2)
- [x] Add forensics tools (scalpel, testdisk, photorec, bulk_extractor, etc.)
- [x] Add reverse engineering tools (ghidra, radare2, objdump, angr, pwntools)
- [x] Add fuzzing tools (honggfuzz, libfuzzer, radamsa, afl)
- [x] Add reporting tools (dradis, faraday, pipal)

## Phase 2: Create Batch Abstractions ✅
- [x] Create src/fenrir/batch_executor.rs
- [x] Implement BatchExecutor struct with parallel/sequential execution
- [x] Add progress tracking and cancellation
- [x] Add dependency resolution for tool chains
- [x] Add resource management (CPU, memory limits)

## Phase 3: Improve Tool Interface
- [ ] Add version detection to KaliTool
- [ ] Add dependency checking
- [ ] Add installation verification
- [ ] Add tool health checks
- [ ] Add configuration validation

## Phase 4: Enhanced Error Handling ✅
- [x] Implement custom error types for tool execution
- [x] Add retry logic with exponential backoff
- [x] Add timeout handling
- [x] Add cleanup on failure
- [x] Add error reporting and logging

## Phase 5: Update Integration ✅
- [x] Modify main.rs to support batch commands
- [x] Add new CLI commands for batch execution (batch recon/vuln/passwd/full)
- [x] Update scan/bite functions to use batch executor
- [x] Add configuration for batch settings

## Phase 6: Testing
- [ ] Add unit tests for batch_executor.rs
- [ ] Add integration tests for tool execution
- [ ] Add mock tools for testing
- [ ] Test parallel execution limits
- [ ] Test error recovery

## Phase 7: Documentation
- [ ] Update docs/KALI_TOOLS_INTEGRATION.md
- [ ] Add batch execution examples
- [ ] Document new tools and categories
- [ ] Add troubleshooting guide
