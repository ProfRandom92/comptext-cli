---
name: ctxt-project-system
summary: "Enforces project build status tracking, roadmap alignment, and local compilation integrity checks."
---

# Skill: ctxt-project-system

## Purpose
Track overall CompText CLI roadmap execution and ensure that local compilations conform to workspace guidelines.

## When to Use
Trigger this skill at the beginning and end of every development phase, during workspace initialization, or when verifying build pipeline readiness.

## Allowed Files/Actions
- **Files**: `Cargo.toml`, `Cargo.lock`, `PROJEKT.md`, `reports/**`.
- **Actions**: Run `cargo check`, `cargo build`, and read project state logs.

## Forbidden Files/Actions
- **Files**: Editing source code (`src/**`) or tests (`tests/**`) outside active task scope.
- **Actions**: Committing changes without verifying successful compilation.

## Validation Commands
```bash
cargo check
cargo build
```

## Stop Conditions
Halt immediately if:
1. Compilation fails and cannot be resolved through minimal dependency updates.
2. The active phase name or tracking metrics in `PROJEKT.md` are out of sync.

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
