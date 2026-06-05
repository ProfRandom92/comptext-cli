# Local Skill Bundle Registry

This document serves as the local registry index for all authorized skills in the CompText workspace. 

> [!IMPORTANT]
> - **Local Scope**: This registry operates strictly as a local repository index. It is **not** a remote, distributed, or cloud-based plugin marketplace.
> - **Integrity Hashes**: The SHA-256 checksums recorded below are strictly for local change-detection and workspace configuration verification. They do **not** represent cryptographic security proofs or remote package validation.

---

## Skill Index

### 1. `ctxt-antigravity-governance`
- **Path**: [.agent/skills/ctxt-antigravity-governance/SKILL.md](file:///.agent/skills/ctxt-antigravity-governance/SKILL.md)
- **Description**: Defines token-efficient governance and operating guidelines for agent runs.
- **Intended Use**: Auditing agent runs, defining permissions baseline, and selecting model effort.
- **Allowed Scope**: Reading `AGENTS.md`, `PROJEKT.md`, token docs.
- **Forbidden Scope**: Reading `.env`, printing env, live network/provider calls, out-of-bounds writes.
- **Validation Commands**: `cargo test`, `git diff --exit-code`
- **Local SHA-256 Checksum**: `3BD2A34E74EF24DA0EE871691AFF7528B655D738C3687EBAAA41D8A10CA575EB`

### 2. `ctxt-security-review`
- **Path**: [.agent/skills/ctxt-security-review/SKILL.md](file:///.agent/skills/ctxt-security-review/SKILL.md)
- **Description**: Audits repository modifications against safety boundaries, credential safety, and claims hygiene.
- **Intended Use**: Running security audits on code changes and checking documentation claims before commits.
- **Allowed Scope**: Scanning repository workspace, adjusting `.gitignore` rules, verifying offline providers.
- **Forbidden Scope**: Reading `.env`, printing env, live network/provider calls, out-of-bounds writes.
- **Validation Commands**: `cargo test`
- **Local SHA-256 Checksum**: `0B5E7A631E8CCED2E058870112B8569D2E01FD373E13CE89F448BB2BDA5D4EEC`

### 3. `ctxt-ci-diagnoser`
- **Path**: [.agent/skills/ctxt-ci-diagnoser/SKILL.md](file:///.agent/skills/ctxt-ci-diagnoser/SKILL.md)
- **Description**: Analyzes Cargo compilation failures, clippy warnings, or test logs to suggest precise repairs.
- **Intended Use**: Locating and fixing local Cargo build errors, Clippy lint issues, or failing unit/smoke tests.
- **Allowed Scope**: Modifying Rust source files (`src/**`) and test files (`tests/**`) to fix compilation or lint bugs.
- **Forbidden Scope**: Suppressing compiler errors, adding new dependencies, live network requests.
- **Validation Commands**: `cargo check`, `cargo test`, `cargo clippy -- -D warnings`
- **Local SHA-256 Checksum**: `157A487EF1A349F25A3D37AA73EF52CC0EC3D59375073886673D6AAA8D479E6D`

### 4. `ctxt-docs-consistency-checker`
- **Path**: [.agent/skills/ctxt-docs-consistency-checker/SKILL.md](file:///.agent/skills/ctxt-docs-consistency-checker/SKILL.md)
- **Description**: Audits markdown documentation for broken links, file presence, and claims hygiene.
- **Intended Use**: Verification of markdown integrity, readme references, and documentation locations.
- **Allowed Scope**: Modifying markdown documents in `docs/`, `reports/`, and repository root.
- **Forbidden Scope**: Creating/modifying Rust code, adding forbidden readiness/compatibility claims.
- **Validation Commands**: `git status --porcelain`
- **Local SHA-256 Checksum**: `0E19E1D5443C75704E657CDBCEE882436F34F02F73F1A3551C37EF8D786B1CE2`

### 5. `ctxt-proposal-auditor`
- **Path**: [.agent/skills/ctxt-proposal-auditor/SKILL.md](file:///.agent/skills/ctxt-proposal-auditor/SKILL.md)
- **Description**: Reviews proposal JSON files to verify schema shapes and target write path safety.
- **Intended Use**: Reviewing proposal JSON files under `proposals/` before executing apply operations.
- **Allowed Scope**: Reading proposal files under `proposals/` and checking write allowed boundaries.
- **Forbidden Scope**: Executing manual patch applications outside of the `ctxt apply` command.
- **Validation Commands**: `cargo run --bin ctxt -- validate`
- **Local SHA-256 Checksum**: `9879D22861B7F4A5438BAF3BBA45C65888F7F42A618C1B6E76AE1A9626246FBF`

### 6. `ctxt-release-packaging`
- **Path**: [.agent/skills/ctxt-release-packaging/SKILL.md](file:///.agent/skills/ctxt-release-packaging/SKILL.md)
- **Description**: Audits release checklist, compiles release target binary, and verifies git hygiene.
- **Intended Use**: Checking release readiness checklists, updating version metadata, and compiling release target binary.
- **Allowed Scope**: Compiling release build (`cargo build --release`), updating version metadata in Cargo.toml, editing changelogs.
- **Forbidden Scope**: Committing temporary `.comptext/` cache directories, force pushes.
- **Validation Commands**: `cargo build --release`, `git diff --exit-code`
- **Local SHA-256 Checksum**: `7C5E20A372E08CB9BF7C38BD3EF0B8A8998298C896FB77FA4EB34D0C7DA76CCA`

### 7. `ctxt-phase-13-skill-bundle-registry`
- **Path**: [.agent/skills/ctxt-phase-13-skill-bundle-registry/SKILL.md](file:///.agent/skills/ctxt-phase-13-skill-bundle-registry/SKILL.md)
- **Description**: Manages the local Skill Bundle Registry files, templates, and integrity check metadata.
- **Intended Use**: Registering skill capsules, normalizing directory structures, and updating change-detection hashes.
- **Allowed Scope**: Staging and editing `.agent/skills/REGISTRY.md` or `.agent/skills/registry.json`.
- **Forbidden Scope**: Implementing remote network registries, distributed downloads, or cryptographic proof logic.
- **Validation Commands**: `cargo test`, `git status`
- **Local SHA-256 Checksum**: `7B2F81C2AF5A97B2E0FA8A1243FABA9F605985A2F35E675F4C429DDBAE426B21`
