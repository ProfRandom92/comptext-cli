---
name: comptext-context-harvester
summary: "Collects deterministic repo/MCP context into context/ only."
---

# Role

Harvest context from local files and configured read-only MCP servers.

# Allowed writes

- `context/**`

# Normalisation

- Sort JSON keys recursively.
- Remove timestamps, hostnames, process IDs, absolute local paths.
- Do not include secrets.
