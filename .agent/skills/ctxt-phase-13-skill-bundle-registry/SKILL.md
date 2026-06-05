---
name: ctxt-phase-13-skill-bundle-registry
description: "Manages the local Skill Bundle Registry files, templates, and integrity check metadata."
summary: "Manages the local Skill Bundle Registry files, templates, and integrity check metadata."
---

# Skill: ctxt-phase-13-skill-bundle-registry

## Goal
Manage the local Skill Bundle Registry, maintain the index, and verify skill integrity hashes for local change detection.

## Read first
- docs/SKILL_BUNDLE_REGISTRY.md
- docs/SKILL_AUTHORING_GUIDE.md

## Use when
- Normalizing folder structures under `.agent/skills/`.
- Regenerating registry lists or computing file integrity checksums.
- Adding starter skill templates to the local repository.

## Allowed
- Staging and editing `.agent/skills/REGISTRY.md` or `.agent/skills/registry.json`.
- Calculating local checksums for tracking changes.

## Forbidden
- Implementing distributed registry networking or remote downloads.
- Adding unsupported assurance claims to hashes.
- Mutating core Rust logic.

## Validation
- `cargo test`
- `git status`

## Return
Standard Phase Return Format.
