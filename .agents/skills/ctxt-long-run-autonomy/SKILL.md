---
name: ctxt-long-run-autonomy
summary: "Defines rules for autonomous execution, state tracking, and git phase progression."
---

# Skill: ctxt-long-run-autonomy

## Purpose
Guide the Antigravity agent in long-running autonomous execution using the PROJEKT.md state tracker, safety constitution rules, and local verification loops.

## When to Use
Trigger this skill when coordinating multi-phase transitions, executing automated validation checks, or staging and pushing changes.

## Allowed Files/Actions
- **Files**: `PROJEKT.md`, `reports/**`, source/tests/docs within phase scope.
- **Actions**: Committing changes, updating tracker files, running validation commands, and triggering git push.

## Forbidden Files/Actions
- **Files**: Committing `.comptext/` artifacts.
- **Actions**: Doing `git push -f` or force push unless explicitly approved, calling cloud providers, or reading unapproved environment files.

## Validation Commands
```bash
cargo fmt --all --check
cargo check
cargo test
cargo clippy -- -D warnings
```

## Stop Conditions
Halt immediately if:
1. Compilation or test checks fail and cannot be fixed cleanly.
2. A Git merge conflict occurs.
3. Credentials or cloud API execution is needed.

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
