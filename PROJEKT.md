# PROJEKT.md — CompText CLI Project Tracker

This file serves as the tracking document and roadmap execution record for the **CompText CLI** (`ctxt`) project.

## Project Vision & Architecture
CompText CLI is an experimental terminal context client for building deterministic, schema-checked Context Packs before interacting with local or cloud model providers.
> Models are providers. Context is the product.

### Core Architecture
- **CLI Shell**: Command parser and handler for local, offline-first commands.
- **Context Harvester**: Logic to inspect and aggregate codebase context.
- **Deterministic Context Packs**: Unified payload stored in `.comptext/context_pack.latest.json` with keys sorted recursively and secrets redacted.
- **Provider Adapters**: Pluggable provider system (Dummy, Ollama, OpenAI-compatible, etc.) with explicit network/security boundaries.
- **Proposal/Apply Gate**: Workflows to review planned changes (`propose`) and apply them (`apply`) safely.

---

## Phase Roadmap Status

| Phase | Description | Goal / Scope | Status |
|---|---|---|---|
| **Phase 0** | Repo Genesis & Bootstrap | Scaffold Rust project, basic commands (`help`, `doctor`, `providers list`, `version`), verify CI | **COMPLETE** |
| **Phase 1** | CLI Shell Hardening | Hardening argument parsing, input handling, and errors for the base shell commands | **COMPLETE** |
| **Phase 2** | Context Pack Contract | Implement `ctxt context inspect`, `ctxt context pack --task "..."`, and `ctxt ask --dry-run "..."` | **COMPLETE** |
| **Phase 3** | Provider Adapter Layer | Define provider interface and Dummy offline test provider | *PLANNED* |
| **Phase 4** | Ollama Local Adapter | Support local Ollama integrations with explicit network boundaries | *PLANNED* |
| **Phase 5** | Proposal Apply Gate | Implement proposal files, approval checks, and validation flow | *PLANNED* |

---

## Active Phase Details

### Phase 1: CLI Shell Hardening
- **Objective**: Harden command parsing in `src/cli.rs`.
- **Target commands**:
  - `ctxt --help` (or `-h`)
  - `ctxt doctor`
  - `ctxt version` (or `-V` / `version`)
  - `ctxt providers list`
- **Validation**:
  - `cargo fmt --all --check`
  - `cargo check`
  - `cargo test`
  - `cargo clippy -- -D warnings`

### Phase 2: Context Pack Contract
- **Objective**: Build deterministic JSON Context Packs.
- **Target commands**:
  - `ctxt context inspect`
  - `ctxt context pack --task "<TASK>"`
  - `ctxt ask --dry-run "<PROMPT>"`
- **Artifacts**:
  - `.comptext/context_pack.latest.json`
  - `.comptext/model_request.latest.json`
