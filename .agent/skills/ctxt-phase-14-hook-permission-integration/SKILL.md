---
name: ctxt-phase-14-hook-permission-integration
description: "Documents and integrates inert specifications for hooks and permissions governance."
summary: "Integrates hook and permission models as inert specification artifacts."
---

# Skill: ctxt-phase-14-hook-permission-integration

## Goal
Create documentation, integration specifications, and templates for hook and permission governance without active runtime enforcement.

## Read first
- AGENTS.md
- PROJEKT.md
- docs/HOOK_PERMISSION_INTEGRATION.md
- docs/POLICY_INTERCEPTOR_SPEC.md
- docs/RUNTIME_PERMISSION_TEMPLATE.md

## Use when
- Documenting policies for SessionStart, PreToolUse, PostToolUse, and PostPhase hooks.
- Drafting template specifications for host permissions.
- Verifying the separation between implemented and target behavior.

## Allowed
- Creating and modifying documentation files under `docs/`.
- Creating inert templates under `templates/hooks/` and `templates/permissions/`.
- Modifying `.agent/skills/REGISTRY.md` to index this skill.

## Forbidden
- Modifying Rust codebase or implementing active/executable hooks/scripts.
- Enabling provider network sockets or reading credentials.
- Writing files outside the repository root.

## Validation
- `cargo fmt --all --check`
- `cargo check`
- `cargo test`
- `cargo clippy -- -D warnings`

## Return
Standard Phase Return Format.
