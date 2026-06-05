---
name: ctxt-antigravity-governance
summary: "Defines token-efficient governance and operating guidelines for agent runs."
---

# Skill: ctxt-antigravity-governance

## Goal
Enforce token-efficient context parsing, hook governance, permissions checks, and subagent boundaries during Antigravity agent execution.

## Read first
- AGENTS.md
- PROJEKT.md
- docs/TOKEN_ECONOMY.md
- docs/HOOK_GOVERNANCE.md
- docs/SUBAGENT_GOVERNANCE.md

## Use when
- Auditing agent task runs.
- Defining permissions profiles or subagent boundaries.
- Refining token usage and model effort guidelines.

## Forbidden
- Reading `.env` or credentials files.
- Dumping environment variables.
- Performing live network calls without explicit phase authorization.
- Writing outside the repository root.

## Validation
- `cargo test`
- `git diff --exit-code`

## Return
Standard Phase Return Format.
