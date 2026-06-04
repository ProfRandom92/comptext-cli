AGY TASK: Phase 1 deterministic Context Pack contract

Read first:
- AGENTS.md
- docs/CONTEXT_PACK_CONTRACT.md
- docs/SECURITY_MODEL.md
- .agent/skills/02_context_pack.md

Goal:
Implement deterministic context inspection, context pack generation, and ask dry-run artifacts.

Allowed files:
- src/**
- tests/**
- docs/CONTEXT_PACK_CONTRACT.md
- docs/SECURITY_MODEL.md
- comptext.example.toml

Forbidden:
- real provider calls
- network calls
- reading API keys
- secrets in output
- git commit/push

Implement commands:
- ctxt context inspect
- ctxt context pack --task "..."
- ctxt ask --dry-run "..."

Validation:
- cargo fmt --all --check
- cargo check
- cargo test
- cargo clippy -- -D warnings

Return standard schema.
