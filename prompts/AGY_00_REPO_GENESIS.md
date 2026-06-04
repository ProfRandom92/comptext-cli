AGY TASK: Phase 0 comptext-cli repo genesis

Read first:
- AGENTS.md
- .agent/skills/00_project_system.md
- .agent/skills/01_repo_genesis.md

Goal:
Create and validate the initial Rust/Cargo repository scaffold for `comptext`, a provider-agnostic terminal context client.

Allowed files:
- README.md
- LICENSE
- Cargo.toml
- comptext.example.toml
- src/main.rs
- src/cli.rs
- tests/**
- docs/**
- .agent/skills/**
- .agents/skills/**
- .github/workflows/ci.yml
- prompts/**
- mcp/**

Forbidden:
- real provider calls
- network calls
- reading API keys
- printing environment variables
- generated output commits
- git commit
- git push
- production/enterprise/compliance/certification/fully-autonomous claims

Implement or verify:
- binary named `ctxt`
- `ctxt --help`
- `ctxt doctor`
- `ctxt providers list`
- `ctxt version`
- docs for architecture, context pack contract, provider adapters, security model, roadmap, agent operating model, MCP/provider boundary
- CI workflow that runs fmt, check, test, clippy

Validation:
- cargo fmt --all --check
- cargo check
- cargo test
- cargo clippy -- -D warnings
- cargo run --bin ctxt -- --help
- cargo run --bin ctxt -- doctor
- cargo run --bin ctxt -- providers list
- cargo run --bin ctxt -- version

Return:
PHASE: Phase 0 comptext-cli repo genesis
STATUS: success | blocked
FILES_CHANGED:
COMMANDS_RUN:
TESTS:
OUTPUT:
RISKS:
NEXT:
