# Skill 05 — Proposal Apply Gate

Purpose: implement proposal/apply/validate workflow.

Commands:
- `ctxt propose --provider ... "..."`
- `ctxt apply proposals/...`
- `ctxt validate`

Rules:
- propose never mutates source files
- apply only touches allowed write paths
- fail closed on policy mismatch
- validation runs after apply
- patches are untrusted until checked
