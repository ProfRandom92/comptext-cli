AGY TASK: Phase 3 proposal and apply gate

Read first:
- AGENTS.md
- docs/SECURITY_MODEL.md
- docs/ROADMAP.md
- .agent/skills/05_proposal_apply_gate.md

Goal:
Implement proposal generation and a fail-closed apply gate for explicitly selected proposals.

Allowed files:
- src/**
- tests/**
- docs/**
- comptext.example.toml

Forbidden:
- provider calls unless dummy only
- network calls
- secrets access
- writing outside allowed paths
- git commit/push

Implement:
- `ctxt propose --provider dummy "..."`
- proposal JSON schema
- `ctxt apply proposals/...` dry apply or minimal deterministic file operation support
- `ctxt validate`
- validation after apply
- fail-closed policy mismatch behavior

Validation:
- cargo fmt --all --check
- cargo check
- cargo test
- cargo clippy -- -D warnings

Return standard schema.
