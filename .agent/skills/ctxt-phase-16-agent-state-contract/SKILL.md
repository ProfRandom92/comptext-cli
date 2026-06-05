---
name: ctxt-phase-16-agent-state-contract
description: "Adds local agent state capturing, verification, and reporting commands."
summary: "Defines agent-state contract skeleton for CompText."
---

# Skill: ctxt-phase-16-agent-state-contract

## Goal
Establish a skeleton for agent-state tracking using bounded evidence JSON files.

## Read first
- AGENTS.md
- PROJEKT.md
- docs/AGENT_STATE_CONTRACT.md
- docs/EVIDENCE_CONTROL_PLANE.md

## Use when
- Capturing local agent state.
- Verifying agent-state JSON files against actual files.
- Summarizing evidence status reports.

## Allowed
- Modifying Rust CLI files (`src/cli.rs`) and adding markdown docs.
- Writing test verification artifacts.

## Forbidden
- Performing live network calls or using cloud APIs.
- Creating remote registries.
- Using active hooks.
- Making unsupported assurance claims.

## Validation
- `cargo test`
