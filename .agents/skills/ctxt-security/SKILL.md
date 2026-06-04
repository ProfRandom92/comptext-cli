---
name: ctxt-security
summary: "Audits repository modifications against safety boundaries, credential safety, and claims hygiene."
---

# Skill: ctxt-security

## Purpose
Prevent security vulnerabilities, API credential exposure, unapproved network leakage, or exaggerations regarding readiness/safety compliance in project documentation.

## When to Use
Use when auditing codebase safety, writing reports, adjusting `.gitignore` rules, or adding claims in public documents.

## Allowed Files/Actions
- **Files**: `.gitignore`, `docs/SECURITY_MODEL.md`, `reports/**`.
- **Actions**: Performing security dry-runs, verifying secret redaction logic.

## Forbidden Files/Actions
- **Files**: Editing private configuration files (e.g., `.env`), hardcoding credentials.
- **Actions**: Claiming production readiness, certification, full autonomy, or SPARK compatibility.

## Validation Commands
```bash
cargo test
git status --porcelain
```

## Stop Conditions
Halt immediately if:
1. Secrets are discovered in git-tracked or serializable files.
2. A document makes forbidden compliance/readiness assertions.

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
