---
name: ctxt-context-pack
summary: "Manages deterministic, schema-checked workspace context packaging."
---

# Skill: ctxt-context-pack

## Purpose
Collect and compile workspace files deterministically into schema-checked Context Packs, redacting potential secrets.

## When to Use
Use when modifying, expanding, or validating context packaging logic, directory harvesting filters, or dry-run prompt creation commands (`ctxt context inspect`, `ctxt context pack`, `ctxt ask --dry-run`).

## Allowed Files/Actions
- **Files**: `src/cli.rs`, `src/main.rs`, `tests/**`, `.comptext/**`.
- **Actions**: Reading source files, inspecting directory structures, writing temporary serialization output under `.comptext/`.

## Forbidden Files/Actions
- **Files**: Staging or committing generated `.comptext/` artifacts in Git.
- **Actions**: Reading or writing outside the repository root.

## Validation Commands
```bash
cargo test
cargo run --bin ctxt -- context inspect
cargo run --bin ctxt -- context pack --task "test deterministic context"
```

## Stop Conditions
Halt immediately if:
1. Generated JSON output contains unredacted API keys, passwords, or high-entropy credentials.
2. Context collection fails to yield deterministic alphabetical key sorting.

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
