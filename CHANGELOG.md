# Changelog

All notable changes to the **CompText CLI** (`ctxt`) project will be documented in this file.

## [0.1.0-mvp] - 2026-06-05

This is the initial MVP release of the CompText CLI, establishing the core local-first context packing, proposal generation, sandboxed apply gates, and provider config layer.

### Added
- **Core CLI Shell**: Initial bootstrap commands (`help`, `version`, `doctor`, `providers list`).
- **Context Harvester**: Structured analysis of active codebase files to build deterministic Context Packs (`context inspect`, `context pack`).
- **Provider Boundary**: Normalized request metadata generation supporting offline dry-runs and dummy providers.
- **Ollama Adapter**: Localhost adapter support with fail-closed network boundaries.
- **OpenAI-Compatible Adapter**: Schema-checked request translation for OpenAI-compatible local APIs.
- **Proposal Generation**: Writes reviewable proposals to `proposals/` prior to applying updates (`propose`).
- **Apply Gate**: Sandboxed file patch apply system with path-traversal check, folder write-permissions validation, and confirmation prompt.
- **Config Layer**: Dynamic TOML profile configuration support switching default providers and policies (`--config`).
- **Validate & Benchmark**: Deterministic benchmarking for offline provider profile performance tracking (`validate`, `benchmark`).
- **Safety Boundaries**: Active sensitive file exclusion (e.g. `.env`, `.key`, `.pem`, `id_rsa`) to prevent secret packing.
