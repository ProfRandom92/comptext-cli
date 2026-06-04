---
name: ctxt-provider-boundary
summary: "Defines normalized provider adapters with explicit network/execution boundaries."
---

# Skill: ctxt-provider-boundary

## Purpose
Ensure that LLM/model provider execution flows are normalized, offline-testable by default, and isolated from unexpected network connections.

## When to Use
Use when implementing or updating provider integrations (Dummy, Ollama, OpenAI-compatible), auth variable reading, or query formatting layers.

## Allowed Files/Actions
- **Files**: `src/provider.rs`, `src/cli.rs`, `Cargo.toml`.
- **Actions**: Executing offline tests, initiating HTTP requests targeting explicitly requested local providers (e.g. `http://localhost:11434`), and reading named authorization env vars.

## Forbidden Files/Actions
- **Files**: Writing or hardcoding API keys in source files or configuration files.
- **Actions**: Making unauthorized HTTP requests to external, non-local endpoints during coding/testing phases.

## Validation Commands
```bash
cargo test
cargo run --bin ctxt -- ask --provider dummy "test prompt"
```

## Stop Conditions
Halt immediately if:
1. Provider execution requires connecting to external cloud endpoints without explicit offline mock fallback.
2. Credentials or API keys must be retrieved from unapproved storage.

## Return Format
Standard schema:
```text
PHASE:
STATUS:
FILES_CHANGED:
COMMANDS_RUN:
VALIDATION:
ARTIFACTS:
GIT:
RISKS:
NEXT:
```
