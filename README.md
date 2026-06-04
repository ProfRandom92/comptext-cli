# CompText CLI

CompText CLI is an experimental terminal context client for building deterministic, schema-checked Context Packs before interacting with local or cloud model providers.

Core sentence:

> Models are providers. Context is the product.

## Repository role

`ProfRandom92/comptext-cli` owns the product CLI / terminal UX / provider adapter layer:

- deterministic Context Pack generation
- policy gates before provider calls
- dry-run artifacts before network
- proposal mode before apply
- local validation workflow
- provider adapters for dummy, Ollama, OpenAI-compatible APIs, future OpenAI/Gemini/custom providers

It is not a blind autonomous coding agent. Antigravity may plan and implement phase-scoped tasks, but risky actions remain explicit.

## Quickstart

```bash
cargo fmt --all --check
cargo check
cargo test
cargo run --bin ctxt -- --help
cargo run --bin ctxt -- doctor
cargo run --bin ctxt -- providers list
cargo run --bin ctxt -- version
```

## Current commands

```bash
ctxt --help
ctxt doctor
ctxt providers list
ctxt version
ctxt context inspect
ctxt context pack --task "..."
ctxt ask --dry-run "..."
ctxt ask --provider dummy "..."
ctxt propose --provider dummy "..."
ctxt apply proposals/...
ctxt validate
ctxt benchmark
```

## Antigravity entrypoint

Open this repository in Antigravity CLI/IDE. First read:

1. `AGENTS.md`
2. `.agent/skills/00_project_system.md`
3. the phase prompt under `prompts/`

Start with:

```text
prompts/AGY_00_REPO_GENESIS.md
```

Then continue with:

```text
prompts/AGY_01_CONTEXT_PACK.md
```

## Claim policy

Allowed language:

- experimental terminal context client
- local-first
- provider-agnostic
- context-pack driven
- proposal-gated
- model/provider/tool output treated as untrusted
- explicit provider network boundary
- deterministic Context Pack generation
- local validation workflow

Do not claim production-ready, enterprise-ready, compliance-ready, certified, fully autonomous, guaranteed safe, or official SPARK compatibility.
