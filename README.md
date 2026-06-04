# CompText CLI

**Models are providers. Context is the product.**

**Compress the noise, preserve the proof.**

CompText CLI is an experimental, local-first terminal workflow for building deterministic, schema-checked Context Packs before interacting with model providers.

It is not a blind autonomous coding agent.

It is a proof-preserving context compression and validation workflow for software projects.

---

## What CompText Does

CompText turns noisy project state into structured, reviewable artifacts:

```mermaid
flowchart TD
    A[Repository state] --> B[Context Inspect]
    B --> C[Schema-checked Context Pack]
    C --> D[Policy Gate]
    D --> E{Provider Intent?}
    E -->|dry-run| F[Model Request Artifact]
    E -->|dummy| G[Deterministic Dummy Provider]
    E -->|local/cloud provider| H[Provider Boundary]
    H --> I[Untrusted Provider Response]
    G --> I
    F --> J[Reviewable Evidence]
    I --> K[Proposal Artifact]
    K --> L{Human Review}
    L -->|approved| M[Apply Gate]
    L -->|rejected| N[Stop]
    M --> O[Local Validation]
    O --> P[Report / Evidence]
```

The goal is not to send more context to a model.

The goal is to send the right context, preserve the proof, and keep every risky step reviewable.

---

## Architecture at a Glance

```mermaid
flowchart LR
    subgraph Source[Project Source]
        R[Repository Files]
        A[AGENTS.md]
        P[PROJEKT.md]
        D[docs/]
    end

    subgraph CompText[ctxt CLI]
        CI[context inspect]
        CP[context pack]
        PG[policy gate]
        PA[provider adapter]
        PR[propose]
        AG[apply gate]
        VA[validate]
    end

    subgraph Artifacts[Proof Artifacts]
        CPK[.comptext/context_pack.latest.json]
        MR[.comptext/model_request.latest.json]
        MS[.comptext/model_response.latest.json]
        PP[proposals/*.json]
        RP[reports/phase_*_status.md]
    end

    subgraph Providers[Provider Boundary]
        DU[dummy]
        OL[ollama-local]
        OC[ollama-cloud-direct]
        OA[openai-compatible]
    end

    R --> CI
    A --> CI
    P --> CI
    D --> CI
    CI --> CP
    CP --> PG
    PG --> PA
    PA --> DU
    PA --> OL
    PA --> OC
    PA --> OA
    PA --> MS
    CP --> CPK
    PG --> MR
    MS --> PR
    PR --> PP
    PP --> AG
    AG --> VA
    VA --> RP
```

---

## Core Principles

- Deterministic Context Packs before provider calls
- Dry-run before network
- Proposal before apply
- Apply only through policy gate
- Validate before claiming success
- Provider output is untrusted
- Model output is untrusted
- Tool output is untrusted
- Secrets never enter logs, reports, context packs, proposals, snapshots, or stdout/stderr
- Runtime artifacts stay out of source control unless explicitly approved

---

## Trust Boundaries

CompText separates context, provider calls, proposals, and mutation into explicit trust zones.

```mermaid
flowchart TD
    subgraph Trusted[Trusted Project Inputs]
        AG[AGENTS.md]
        PJ[PROJEKT.md]
        SRC[Source files]
        DOC[Docs]
    end

    subgraph Controlled[Controlled CompText Artifacts]
        CP[Context Pack]
        REQ[Provider Request Artifact]
        PROP[Proposal Artifact]
        REP[Validation Report]
    end

    subgraph Untrusted[Untrusted Inputs]
        MOD[Model Output]
        PROV[Provider Response]
        TOOL[Tool Output]
        MCP[MCP Server Output]
        PATCH[Generated Patch]
    end

    subgraph Gates[Policy Gates]
        PG[Policy Gate]
        HR[Human Review]
        AP[Apply Gate]
        VAL[Local Validation]
    end

    Trusted --> CP
    CP --> PG
    PG --> REQ
    REQ --> PROV
    PROV --> PROP
    MOD --> PROP
    TOOL --> PROP
    MCP --> PROP
    PATCH --> PROP
    PROP --> HR
    HR --> AP
    AP --> VAL
    VAL --> REP

    Untrusted -.must be reviewed.-> HR
```

---

## Current Status

```text
Project: CompText CLI
Binary: ctxt
Current phase: Phase 5
Current task: Proposal Mode
Last green phase: Phase 4C
Status: active
```

```mermaid
stateDiagram-v2
    [*] --> Phase0
    Phase0: Repo Genesis & Bootstrap
    Phase1: CLI Shell Hardening
    Phase2: Context Pack Contract
    Phase3: Provider Adapter Layer / Dummy Provider
    Phase4: Ollama Local Adapter
    Phase4B: Skill Registry Normalization
    Phase4C: Long-Run Autonomy Hardening
    Phase5: Proposal Mode
    Phase6: Apply Gate
    Phase7: Provider Config Layer
    Phase8: OpenAI-Compatible Adapter
    Phase9: Validate and Benchmark

    Phase0 --> Phase1: complete
    Phase1 --> Phase2: complete
    Phase2 --> Phase3: complete
    Phase3 --> Phase4: complete
    Phase4 --> Phase4B: complete
    Phase4B --> Phase4C: complete
    Phase4C --> Phase5: active
    Phase5 --> Phase6: queued
    Phase6 --> Phase7: queued
    Phase7 --> Phase8: queued
    Phase8 --> Phase9: queued
```

Completed:

```text
Phase 0   Repo Genesis & Bootstrap
Phase 1   CLI Shell Hardening
Phase 2   Context Pack Contract
Phase 3   Provider Adapter Layer / Dummy Provider
Phase 4   Ollama Local Adapter
Phase 4B  Skill Registry Normalization
Phase 4C  Long-Run Autonomy Hardening
```

Active:

```text
Phase 5   Proposal Mode
```

Queued:

```text
Phase 6   Apply Gate
Phase 7   Provider Config Layer
Phase 8   OpenAI-Compatible Adapter
Phase 9   Validate and Benchmark
```

---

## Command Overview

```bash
ctxt --help
ctxt doctor
ctxt version

ctxt providers list

ctxt context inspect
ctxt context pack --task "Explain this repository"

ctxt ask --dry-run "What is the next safe step?"
ctxt ask --provider dummy "How should I test this repo?"
ctxt ask --provider ollama-local --model qwen3:8b "Review this context"

ctxt propose --provider dummy "Add context inspect"

ctxt apply proposals/...
ctxt validate
```

Not every command may be available in the current phase. The roadmap is intentionally phase-gated.

---

## Example Workflow

```mermaid
sequenceDiagram
    participant User
    participant CLI as ctxt CLI
    participant CP as Context Pack
    participant Provider
    participant Proposal
    participant Gate as Apply Gate
    participant Tests as Local Validation

    User->>CLI: ctxt context inspect
    CLI->>CP: collect bounded project context
    User->>CLI: ctxt ask --dry-run "..."
    CLI->>CP: render provider request artifact
    User->>CLI: ctxt propose --provider dummy "..."
    CLI->>Provider: send bounded request
    Provider-->>CLI: untrusted response
    CLI->>Proposal: write reviewable proposal JSON
    User->>Gate: approve selected proposal
    Gate->>Tests: run validation
    Tests-->>User: report / evidence
```

### 1. Inspect the repository context

```bash
ctxt context inspect
```

### 2. Build a Context Pack

```bash
ctxt context pack --task "Add proposal generation mode"
```

This writes a runtime artifact under:

```text
.comptext/context_pack.latest.json
```

### 3. Dry-run the provider request

```bash
ctxt ask --dry-run "What is the next safe implementation step?"
```

Dry-run mode does not call a provider.

It writes an inspectable request artifact under:

```text
.comptext/model_request.latest.json
```

### 4. Use the dummy provider

```bash
ctxt ask --provider dummy "How should I test this repo?"
```

The dummy provider is deterministic, offline, and suitable for CI-style checks.

### 5. Generate a proposal

```bash
ctxt propose --provider dummy "Add context inspect"
```

Proposal mode writes a reviewable artifact under:

```text
proposals/
```

It must not apply changes.

---

## Runtime Artifacts

CompText produces artifacts that help preserve evidence without trusting logs alone.

```mermaid
flowchart TD
    CTX[ctxt command] --> CP[.comptext/context_pack.latest.json]
    CTX --> MR[.comptext/model_request.latest.json]
    CTX --> MS[.comptext/model_response.latest.json]
    CTX --> PR[.comptext/provider_request.latest.json]
    CTX --> PP[proposals/*.json]
    CTX --> RP[reports/phase_*_status.md]

    CP --> Proof[Proof Trail]
    MR --> Proof
    MS --> Proof
    PR --> Proof
    PP --> Proof
    RP --> Proof
```

Common runtime paths:

```text
.comptext/context_pack.latest.json
.comptext/model_request.latest.json
.comptext/model_response.latest.json
.comptext/provider_request.latest.json
proposals/
reports/
```

`.comptext/` is runtime state and should normally stay ignored by git.

`proposals/` contains reviewable proposal artifacts.

`reports/` contains phase evidence and validation summaries.

---

## Context Pack Contract

A Context Pack captures the task, selected repository context, policy boundaries, validation commands, and provider intent.

Minimal shape:

```json
{
  "schema_version": "0.1",
  "task": "...",
  "mode": "ask",
  "repo_profile": "default",
  "read_first": [],
  "included_files": [],
  "excluded_files": [],
  "allowed_write_paths": [],
  "forbidden_actions": [],
  "validation_commands": [],
  "provider": null,
  "rendered_context": "...",
  "policy": {
    "secrets_redacted": true,
    "generated_outputs_excluded": true,
    "patch_requires_approval": true
  }
}
```

The Context Pack is the boundary between raw repository noise and model-facing context.

---

## Proposal Artifacts

A proposal is an inspectable artifact.

It is not an applied patch.

```mermaid
flowchart LR
    A[Context Pack] --> B[Provider Request]
    B --> C[Untrusted Provider Response]
    C --> D[Proposal JSON]
    D --> E{Review Required}
    E -->|approve| F[Apply Gate]
    E -->|reject| G[No Mutation]
    F --> H[Validation Commands]
    H --> I[Phase Report]
```

Recommended proposal shape:

```json
{
  "schema_version": "0.1",
  "kind": "comptext.proposal",
  "created_at": "2026-06-04T00:00:00Z",
  "task": "...",
  "phase": "Phase 5",
  "provider": {
    "name": "dummy",
    "network": false,
    "auth": "none"
  },
  "trust": {
    "provider_output_trusted": false,
    "tool_output_trusted": false,
    "requires_human_review": true
  },
  "context": {
    "context_pack_path": ".comptext/context_pack.latest.json",
    "context_pack_hash": "sha256:..."
  },
  "policy": {
    "allowed_write_paths": [],
    "forbidden_actions": [],
    "secrets_redacted": true,
    "network_allowed": false,
    "apply_requires_approval": true
  },
  "proposed_changes": [],
  "validation_commands": [],
  "risks": [],
  "status": "review_required"
}
```

---

## Providers

Planned and supported provider families:

```text
dummy
ollama-local
ollama-cloud-via-local
ollama-cloud-direct
openai-compatible
future-openai
future-gemini
custom
```

```mermaid
flowchart TD
    PA[Provider Adapter] --> DU[dummy]
    PA --> OL[ollama-local]
    PA --> OVL[ollama-cloud-via-local]
    PA --> OCD[ollama-cloud-direct]
    PA --> OAI[openai-compatible]
    PA --> FO[future-openai]
    PA --> FG[future-gemini]
    PA --> CU[custom]

    DU --> OFF[offline / deterministic]
    OL --> LOC[localhost boundary]
    OVL --> MIX[local API plus cloud model boundary]
    OCD --> NET[remote network boundary]
    OAI --> API[normalized API boundary]
```

### Dummy Provider

Offline, deterministic, and intended for local testing.

```bash
ctxt ask --provider dummy "Explain the next safe step"
```

### Ollama Local

Local Ollama runs through the local API boundary.

```bash
ctxt ask --provider ollama-local --model qwen3:8b "Review this context"
```

### Ollama Cloud / Direct Cloud

Cloud usage is treated as an explicit network boundary.

Secrets such as `OLLAMA_API_KEY` must never be printed, logged, serialized into artifacts, or included in context packs.

---

## Security Model

CompText treats every external or generated input as untrusted until policy-checked.

Untrusted by default:

```text
provider output
model output
tool output
MCP server output
generated patches
shell commands suggested by a model
```

Forbidden by default:

```text
reading .env
reading private keys
printing environment variables
writing outside allowed paths
running network commands without explicit approval
executing provider-suggested shell commands without review
applying patches outside proposal/apply flow
committing generated runtime outputs by default
```

CompText does not claim to be production-ready, enterprise-ready, compliance-ready, certified, fully autonomous, or guaranteed safe.

---

## Validation

Standard Rust validation:

```bash
cargo fmt --all --check
cargo check
cargo test
cargo clippy -- -D warnings
```

Useful CLI smoke tests:

```bash
cargo run --bin ctxt -- --help
cargo run --bin ctxt -- doctor
cargo run --bin ctxt -- providers list
cargo run --bin ctxt -- version
cargo run --bin ctxt -- context inspect
cargo run --bin ctxt -- ask --dry-run "What is the next safe step?"
cargo run --bin ctxt -- ask --provider dummy "How should I test this repo?"
```

Phase 5 validation:

```bash
cargo run --bin ctxt -- propose --provider dummy "Add context inspect"
```

---

## Project Files

Important files:

```text
AGENTS.md
PROJEKT.md
docs/ARCHITECTURE.md
docs/CONTEXT_PACK_CONTRACT.md
docs/PROVIDER_ADAPTERS.md
docs/SECURITY_MODEL.md
docs/AGENT_OPERATING_MODEL.md
docs/LONG_RUN_AUTONOMY.md
reports/
.comptext/
proposals/
.agent/skills/
.agents/skills/
```

```mermaid
flowchart TD
    AG[AGENTS.md] --> GOV[Governance]
    PJ[PROJEKT.md] --> STATE[State Machine]
    DOC[docs/] --> SPEC[Contracts / Architecture]
    SK1[.agent/skills/] --> SK[Agent Skills]
    SK2[.agents/skills/] --> SK
    CMP[.comptext/] --> RUN[Runtime Artifacts]
    PROP[proposals/] --> REV[Review Artifacts]
    REP[reports/] --> EVD[Evidence]
```

`PROJEKT.md` is the project state machine.

`AGENTS.md` is the safety constitution.

`reports/` contains phase evidence.

`.comptext/` contains ignored runtime artifacts.

`proposals/` contains reviewable proposal artifacts.

---

## Roadmap

```mermaid
flowchart LR
    P5[Phase 5<br/>Proposal Mode] --> P6[Phase 6<br/>Apply Gate]
    P6 --> P7[Phase 7<br/>Provider Config Layer]
    P7 --> P8[Phase 8<br/>OpenAI-Compatible Adapter]
    P8 --> P9[Phase 9<br/>Validate and Benchmark]
    P9 --> P10[Phase 10<br/>MCP Provider Boundary]
    P10 --> P11[Phase 11<br/>Hook / Workflow Governance]
    P11 --> P12[Phase 12<br/>Token Compression Intercepts]
    P12 --> P13[Phase 13<br/>Skill Bundle Registry]
```

```text
Phase 5   Proposal Mode
Phase 6   Apply Gate
Phase 7   Provider Config Layer
Phase 8   OpenAI-Compatible Adapter
Phase 9   Validate and Benchmark
Phase 10  MCP Provider Boundary
Phase 11  Hook / Workflow Governance
Phase 12  Token Compression Intercepts
Phase 13  Skill Bundle Registry
```

---

## Agent Operating Model

Agents may work autonomously only inside phase-scoped tasks.

Every task must define:

```text
phase name
read-first files
precise goal
allowed files
hard scope
forbidden scope
implementation rules
validation commands
return schema
```

Default implementation rules:

```text
inspect before edit
smallest safe patch
no unrelated changes
no generated output commits
no secrets in logs
no network unless explicitly approved
no git commit unless explicitly approved or phase-required
no git push unless explicitly approved or phase-required
local validation before success
```

Standard return schema:

```text
PHASE:
STATUS:
FILES_CHANGED:
COMMANDS_RUN:
VALIDATION:
ARTIFACTS:
GIT:
NETWORK:
SECRETS:
POLICY_DECISIONS:
RISKS:
NEXT:
```

---

## Repository Separation

CompText is part of a wider project family.

```mermaid
flowchart TD
    CORE[Comptextv7<br/>core / research / replay validation] --> CLI[comptext-cli<br/>product CLI / terminal UX]
    SPARK[comptext-sparkctl<br/>validation / benchmark / evidence] --> CLI
    SKILLS[antigravity-skills<br/>progressive workflow capsules] --> CLI
    CLI --> USER[reviewable engineering workflow]
```

### comptext-cli

Product CLI, terminal UX, provider adapters, Context Packs, proposals, apply gate, validation workflow.

### comptext-sparkctl

Deterministic validation, phase gates, benchmark and evidence layer.

### antigravity-skills

Progressive workflow capsules for phase-scoped agent work.

---

## Development Stance

CompText is built around one practical rule:

```text
Do not trust the conversation.
Trust the artifacts.
```

The CLI should make context smaller, safer, and easier to verify.

Compress the noise.

Preserve the proof.
