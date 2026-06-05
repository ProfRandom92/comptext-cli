---
name: ctxt-proposal-auditor
description: "Reviews proposal JSON files to verify schema shapes and target write path safety."
summary: "Reviews proposal JSON files to verify schema shapes and target write path safety."
---

# Skill: ctxt-proposal-auditor

## Goal
Audit the structure of generated JSON proposals and verify that target paths comply with apply-gate write boundaries.

## Read first
- docs/APPLY_GATE.md
- docs/ARTIFACT_POLICY.md

## Use when
- Reviewing a proposal JSON file before executing `ctxt apply`.
- Verifying the list of affected files in the proposal schema.
- Confirming rollback strategy viability.

## Allowed
- Reading and verifying proposal files under `proposals/`.
- Checking that target apply paths lie entirely inside allowed write boundaries.

## Forbidden
- Applying patch changes manually to source code outside the `ctxt apply` flow.
- Modifying target file permissions.

## Validation
- `cargo run --bin ctxt -- validate`

## Return
Standard Phase Return Format.
