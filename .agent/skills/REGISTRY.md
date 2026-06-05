# Local Skill Bundle Registry

This document serves as the local registry index for all authorized skills in the CompText workspace. 

> [!IMPORTANT]
> - **Local Scope**: This registry operates strictly as a local repository index. It is **not** a remote, distributed, or cloud-based plugin marketplace.
> - **Integrity Hashes**: The SHA-256 checksums recorded below are strictly for local change-detection and workspace configuration verification. They do **not** represent unsupported assurance claims.

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
- **Local SHA-256 Checksum**: `61E8929013C1FBF9DE29E94E8671F9F1B824F7F4D040753AF93ECA5A5C0C6032`

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
- **Forbidden Scope**: No Git tag creation, release publishing, package publishing, committing temporary `.comptext/` cache directories, or force pushes.
- **Validation Commands**: `cargo build --release`, `git diff --exit-code`
- **Local SHA-256 Checksum**: `7C5E20A372E08CB9BF7C38BD3EF0B8A8998298C896FB77FA4EB34D0C7DA76CCA`

### 7. `ctxt-phase-13-skill-bundle-registry`
- **Path**: [.agent/skills/ctxt-phase-13-skill-bundle-registry/SKILL.md](file:///.agent/skills/ctxt-phase-13-skill-bundle-registry/SKILL.md)
- **Description**: Manages the local Skill Bundle Registry files, templates, and integrity check metadata.
- **Intended Use**: Registering skill capsules, normalizing directory structures, and updating change-detection hashes.
- **Allowed Scope**: Staging and editing `.agent/skills/REGISTRY.md` or `.agent/skills/registry.json`.
- **Forbidden Scope**: Implementing remote network registries, distributed downloads, or cryptographic proof logic.
- **Validation Commands**: `cargo test`, `git status`
- **Local SHA-256 Checksum**: `9EE4E5A999D563B95F6ACE081D41D3FA78FA33616FD219EE64E7381CFCC7DAD1`

### 8. `ctxt-phase-14-hook-permission-integration`
- **Path**: [.agent/skills/ctxt-phase-14-hook-permission-integration/SKILL.md](file:///.agent/skills/ctxt-phase-14-hook-permission-integration/SKILL.md)
- **Description**: Documents and integrates inert specifications for hooks and permissions governance.
- **Intended Use**: Documenting hook lifecycle targets, template schemas for permissions, and verification of implemented behavior.
- **Allowed Scope**: Creating/modifying documentation files under `docs/` and inert template files.
- **Forbidden Scope**: Modifying Rust codebase, active hook scripting/enforcement, and enabling provider network socket connectivity.
- **Validation Commands**: `cargo test`
- **Local SHA-256 Checksum**: `234A19F1E9E728412D5E0C1714D2A94F886509E2D47B54FC402EB84A1FD69A6D`

### 9. `ctxt-phase-15-cryptographic-provenance`
- **Path**: [.agent/skills/ctxt-phase-15-cryptographic-provenance/SKILL.md](file:///.agent/skills/ctxt-phase-15-cryptographic-provenance/SKILL.md)
- **Description**: Adds local provenance manifest verification and generation workflows.
- **Intended Use**: Verifying integrity of local context packs, proposal patches, and benchmarks.
- **Allowed Scope**: Modifying Rust CLI modules and unit tests, writing test verification files.
- **Forbidden Scope**: Implementing remote network registries, blockchain, consensus protocols, or active hooks.
- **Validation Commands**: `cargo test`
- **Local SHA-256 Checksum**: `68D210DCAF7E7A95F65AC9EE5179FD60212D63CD9B85A92F24A9D4267B64B329`
