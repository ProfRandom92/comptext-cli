AGY TASK: Phase 2 dummy provider pipeline

Read first:
- AGENTS.md
- docs/PROVIDER_ADAPTERS.md
- docs/SECURITY_MODEL.md
- .agent/skills/03_provider_adapter.md

Goal:
Implement the offline dummy provider so the request/response pipeline can be validated without network or model access.

Allowed files:
- src/**
- tests/**
- docs/PROVIDER_ADAPTERS.md
- comptext.example.toml

Forbidden:
- network calls
- API key reads
- real provider calls
- git commit/push

Implement:
- provider trait/interface
- dummy provider adapter
- normalized request/response structs
- `ctxt ask --provider dummy "..."`
- tests proving no network/API key access is needed

Validation:
- cargo fmt --all --check
- cargo check
- cargo test
- cargo clippy -- -D warnings

Return standard schema.
