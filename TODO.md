# Fenrir CLI Evolution - TODO List

## Phase 1: Core Infrastructure (Priority 1)
- [ ] Create `plan.rs` module for natural language interpretation
- [ ] Create `safe_check.rs` module for risk pattern scanning
- [ ] Create `explain.rs` module for AI-powered explanations
- [ ] Create `util_integrations.rs` for CLI tool wrappers
- [ ] Create `decision_history.rs` for tracking user decisions

## Phase 2: CLI Enhancement (Priority 1)
- [ ] Update `cli.rs` to add new subcommands: plan, safe, util, explain, monitor
- [ ] Update `main.rs` for enhanced interactive mode with history
- [ ] Update `mod.rs` to declare new modules

## Phase 3: Feature Implementation (Priority 2)
- [ ] Enhance `disk_cleanup.rs` with duplicate detection by hash
- [ ] Implement CLI utility integrations (fzf, ripgrep, fd, bat, taskwarrior, denet)
- [ ] Add AI explanation features before risky actions
- [ ] Implement decision history tracking

## Phase 4: Testing & Documentation (Priority 3)
- [ ] Test all new features
- [ ] Update Cargo.toml for new dependencies
- [ ] Update README.md with new features
- [ ] Create integration tests

## Phase 5: Advanced Features (Priority 4)
- [ ] Plugin manager for easy module installation
- [ ] Local AI integration (Grok 4.7)
- [ ] External AI CLI connectors
- [ ] Advanced monitoring features
