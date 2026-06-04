# Skill 02 — Context Pack

Purpose: implement deterministic Context Pack generation.

Commands:
- `ctxt context inspect`
- `ctxt context pack --task "..."`
- `ctxt ask --dry-run "..."`

Output:
- `.comptext/context_pack.latest.json`
- `.comptext/model_request.latest.json`

Rules:
- stable JSON key ordering
- no timestamps unless explicitly normalized
- no hostnames/process IDs/absolute local paths
- no secrets
- generated outputs excluded by default
