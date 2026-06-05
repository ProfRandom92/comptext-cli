---
name: ctxt-ci-diagnoser
description: "Analyzes Cargo compilation failures, clippy warnings, or test logs to suggest precise repairs."
summary: "Analyzes Cargo compilation failures, clippy warnings, or test logs to suggest precise repairs."
---

# Skill: ctxt-ci-diagnoser

## Goal
Diagnose and repair Cargo compilation errors, clippy warnings, or test suite failures in the local workspace.

## Read first
- PROJEKT.md
- Cargo.toml

## Use when
- Local `cargo check`, `cargo build`, or `cargo test` commands fail.
- Clippy output contains warnings.
- The CI clean tree check fails.

## Allowed
- Modifying Rust source files (`src/**`) and test files (`tests/**`) to fix lint, compiler, or test bugs.
- Running compilation and lint verification suites in the local workspace.

## Forbidden
- Bypassing compiler warnings or suppressing clippy errors without resolving them.
- Adding new Rust dependencies without prior approval.
- Performing live network requests to fetch external crates during diagnosing.

## Validation
- `cargo check`
- `cargo test`
- `cargo clippy -- -D warnings`

## Return
Standard Phase Return Format.
