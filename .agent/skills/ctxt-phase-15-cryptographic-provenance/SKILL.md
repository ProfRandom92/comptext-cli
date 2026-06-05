---
name: ctxt-phase-15-cryptographic-provenance
description: "Adds local provenance manifest verification and generation workflows."
summary: "Adds SHA-256 provenance checking for local CompText artifacts."
---

# Skill: ctxt-phase-15-cryptographic-provenance

## Goal
Implement a local cryptographic provenance verification and generation mechanism using pure Rust SHA-256 hashing.

## Read first
- AGENTS.md
- PROJEKT.md
- docs/PROVENANCE_MODEL.md
- reports/phase_15_status.md

## Use when
- Verifying the integrity of local artifacts like Context Packs, proposals, and benchmarks.
- Generating provenance manifests with parent links.
- Auditing local file change state checks.

## Allowed
- Modifying Rust CLI modules (`src/cli.rs`, `src/main.rs`) and unit tests.
- Writing test verification artifacts and temporary test files.
- Indexing this skill in the registry.

## Forbidden
- Implementing blockchain, distributed consensus, or remote provenance engines.
- Utilizing external network sockets or third-party web APIs.
- Storing high-entropy secrets in public repositories.

## Validation
- `cargo fmt --all --check`
- `cargo check`
- `cargo test`
- `cargo clippy -- -D warnings`

## Return
Standard Phase Return Format.
