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

## Autonomous State Machine Configuration

### Current State
```text
CURRENT_PHASE: 12
CURRENT_TASK: Antigravity CLI Governance & Token Economy
LAST_GREEN_PHASE: 12
STATUS: complete
```

### Autonomy Contract
- **Allowed Modifications**: May edit source code (`src/**`), tests (`tests/**`), docs (`docs/**`), skills (`.agent/skills/**`, `.agents/skills/**`), prompts (`prompts/**`), and configurations (`Cargo.toml`, `comptext.example.toml`).
- **Allowed Commands**: May run local compilation, lint checks, tests, and formatting validation.
- **Error Remediation**: May automatically modify code to fix local build, format, test, or clippy failures.
- **Phase Transition**: May commit and push changes after all validation passes for a green phase, and automatically proceed to the next phase queue item.

### Forbidden Rules
- **No Private Keys / Secrets**: Forbidden to read or parse `.env`, `.env.*`, `.netrc`, `.git-credentials`, private keys (`*.key`, `*.pem`), or credentials.
- **No Secret Leakage**: Forbidden to print environment variables, dump secrets in stdout/stderr, or write them to logs/reports/artifacts.
- **No Untrusted Provider Action**: Forbidden to execute real cloud API provider calls during coding/validation phases (unless explicitly approved for live integration runs).
- **No Destructive/Out-of-Scope Commands**: Forbidden to run shell operations outside the repo root.
- **No Overwriting Remote History**: Forbidden to run `git push -f` or force push unless explicitly approved.
- **No Compliance Claims**: Forbidden to claim production/enterprise readiness, certified status, full autonomy, or official SPARK compatibility.

### Stop Conditions
The agent must halt execution and yield to the user when:
1. API credentials or auth keys are required to proceed.
2. Real cloud provider execution / live network calls are needed.
3. Git merge conflicts arise that cannot be resolved safely.
4. Validation fails and cannot be resolved with small, safe changes.
5. Codebase requirements or user requests are contradictory.
6. Target files outside the repository root need to be accessed or created.

### Global Validation Suite
The agent must run and satisfy the following validation suite before completing any phase:
```bash
cargo fmt --all --check
cargo check
cargo test
cargo clippy -- -D warnings
```

### Git Progression Rule
Upon achieving green status for any phase, the agent must execute:
```bash
git status
git add .
git commit -m "<phase commit message>"
git push
```

---

## Phase Queue & Roadmap Status

| Phase | Description | Goal / Scope | Status |
|---|---|---|---|
| **Phase 0** | Repo Genesis & Bootstrap | Scaffold Rust project, basic commands (`help`, `doctor`, `providers list`, `version`), verify CI | **COMPLETE** |
| **Phase 1** | CLI Shell Hardening | Hardening argument parsing, input handling, and errors for the base shell commands | **COMPLETE** |
| **Phase 2** | Context Pack Contract | Implement `ctxt context inspect`, `ctxt context pack --task "..."`, and `ctxt ask --dry-run "..."` | **COMPLETE** |
| **Phase 3** | Provider Adapter Layer | Define provider interface and Dummy offline test provider | **COMPLETE** |
| **Phase 4** | Ollama Local Adapter | Support local Ollama integrations with explicit network boundaries | **COMPLETE** |
| **Phase 4B** | Skill Registry Normalization | Normalize the local Antigravity skill structure and crystallize autonomy rules | **COMPLETE** |
| **Phase 4C** | Long-Run Autonomy Hardening | Harden state machine progression rules and git safety boundaries | **COMPLETE** |
| **Phase 5** | Proposal Mode | Implement `ctxt propose` to output changes as structured proposals | **COMPLETE** |
| **Phase 6** | Apply Gate | Implement `ctxt apply` to confirm/apply changes and run verification | **COMPLETE** |
| **Phase 7** | Provider Config Layer | Support dynamic provider profile switching and configurations | **COMPLETE** |
| **Phase 8** | OpenAI-Compatible Adapter | Implement OpenAI adapter skeleton | **COMPLETE** |
| **Phase 9** | Validate and Benchmark | Local validation, dry-runs, and deterministic benchmark flows | **COMPLETE** |
| **Phase 10** | MVP Stabilization & Release Readiness | Audit documentation, verify command flows, safety hygiene checks | **COMPLETE** |
| **Phase 11** | Release Packaging | Package CLI binary, finalize manifests, release artifact generation | **COMPLETE** |
| **Phase 12** | Antigravity CLI Governance & Token Economy | Hook, registry token bindings, runtime stake checks | **COMPLETE** |
| **Phase 13** | Skill Bundle Registry | Distributed skills indexing and integrity hashing | *NEXT* |
| **Phase 14** | Hook/Permission Integration | Hook boundaries, dynamic run approvals | *QUEUED* |
| **Phase 15** | Cryptographic Provenance Engine | Signed evidence trail generation and cryptographic integrity seals | *QUEUED* |

---

## Standard Phase Return Format
All phase transitions must output their status report using the following schema:
```text
PHASE: <Phase Number and Title>
STATUS: <success | blocked>
FILES_CHANGED: <list of changed files>
COMMANDS_RUN: <list of commands executed>
VALIDATION: <validation output summary>
ARTIFACTS: <list of generated artifacts>
GIT: <git commit and push hash/result>
RISKS: <analysis of potential risks>
NEXT: <next action or phase name>
```
