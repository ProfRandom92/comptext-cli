---
name: ctxt-docs-consistency-checker
description: "Audits markdown documentation for broken links, file presence, and claims hygiene."
summary: "Audits markdown documentation for broken links, file presence, and claims hygiene."
---

# Skill: ctxt-docs-consistency-checker

## Goal
Verify formatting, markdown link integrity, document locations, and claims hygiene across all workspace documentation.

## Read first
- README.md
- docs/ROADMAP.md

## Use when
- Creating or updating files in `docs/` or `reports/`.
- Auditing navigation links in the README.
- Validating that no forbidden readiness or compatibility claims exist.

## Allowed
- Modifying markdown files in `docs/`, `reports/`, and the root directory.
- Relocating misplaced phase status reports to correct folders.

## Forbidden
- Adding claims regarding production readiness, enterprise certification, or guaranteed safety.
- Creating or editing Rust source code.
- Performing live network connections.

## Validation
- `git status --porcelain`

## Return
Standard Phase Return Format.
