# Skill 07 — Antigravity Autonomous Builder

Purpose: let Antigravity execute long phase-scoped implementation loops safely.

Loop:
1. read AGENTS.md and relevant skill
2. inspect current repo state
3. create/update minimal files
4. run validation commands
5. fix local deterministic failures
6. return structured result

Allowed autonomy:
- scaffold and edit allowed files
- run local cargo validation
- write context/proposals

Forbidden without explicit user request:
- network/provider calls
- secrets access
- git commit/push
- deployment
- broad unrelated refactors
