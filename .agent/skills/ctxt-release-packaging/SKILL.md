---
name: ctxt-release-packaging
description: "Audits release checklist, compiles release target binary, and verifies git hygiene."
summary: "Audits release checklist, compiles release target binary, and verifies git hygiene."
---

# Skill: ctxt-release-packaging

## Goal
Verify the release checklist, edit the changelog, build the production release binary, and ensure Git tree cleanliness.

## Read first
- docs/RELEASE_CHECKLIST.md
- CHANGELOG.md

## Use when
- Compiling a stable local release package of the CLI.
- Updating release tags and metadata.
- Validating Git ignore rules before final release commits.

## Allowed
- Compiling the target binary with release optimization (`cargo build --release`).
- Modifying version metadata in `Cargo.toml`.
- Writing release checklists and changelogs.

## Forbidden
- Committing temporary `.comptext/` cache directories.
- Rewriting Git history or performing force pushes.
- Adding unverified or uncompiled files to the release package.

## Validation
- `cargo build --release`
- `git diff --exit-code`

## Return
Standard Phase Return Format.
