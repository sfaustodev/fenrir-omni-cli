# COTOA Plan – Fenrir CLI Integration

## Goals
- Deliver a focused Fenrir CLI that can plug other CLIs (Grok/Droid, Gemini, etc.) without brittle shell glue.
- Keep Argo-friendly builds (no heavy artefacts in git) and centralized API-key handling.

## Immediate Next Moves
- Align Grok/Droid CLI with `KAT_KEY` (fallbacks allowed) across Rust + scripts.
- Keep build clean for Argo (`target/`, `.grok/`, logs ignored); prune local artefacts before releases.
- Map CLI touchpoints needed for a user-facing Fenrir interface (commands, flags, status output).

## Async Tasks
- Parse `.sh` and `.md` in this repo (`setup_trinity.sh`, `test_grok_api.sh`, `TRINITY_IA_DOCUMENTATION.md`, `docs/*`) to extract safe Rust-hardcoded defaults (env var names, status messages, health checks).
- Sketch how COTOA (Chain-of-Thoughts-to-Action) orchestration should expose hooks for external CLIs (input parsing, task queue, consensus, guardrails).
- Identify any hardcoded assumptions (model names, base URLs) that should be config-driven.

## Rust Refactor TODOs
- Centralize API-key resolution (KAT_KEY-first) in a small helper to remove duplication between coordinators/clients.
- Add lightweight tests around key resolution and Grok client config (model/base URL parsing).
- Separate orchestrator concerns (parsing vs execution vs IO) to make the custom CLI integration simpler.
- Tighten error messages for missing keys or offline modes, and surface a single diagnostics command.
