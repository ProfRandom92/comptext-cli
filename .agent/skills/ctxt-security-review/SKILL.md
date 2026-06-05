---
name: ctxt-security-review
description: "Audits repository modifications against safety boundaries, credential safety, and claims hygiene."
summary: "Audits repository modifications against safety boundaries, credential safety, and claims hygiene."
---

# Skill: ctxt-security-review

## Goal
Audit codebase safety, confirm secret redaction, and ensure no forbidden readiness or compliance claims exist in repository files.

## Read first
- AGENTS.md
- docs/SECURITY_MODEL.md

## Use when
- Performing security checks before a git commit.
- Inspecting files for high-entropy secrets or private keys.
- Reviewing documentation for forbidden readiness or compliance claims.

## Allowed
- Reading and scanning files within the repository workspace.
- Adjusting `.gitignore` files to exclude runtime state.
- Verifying offline status of providers.

## Forbidden
- Reading `.env` or configuration files.
- Printing environment variables.
- Performing live network calls without explicit phase authorization.
- Writing outside the repository root.

## Validation
- `cargo test`

## Return
Standard Phase Return Format.
