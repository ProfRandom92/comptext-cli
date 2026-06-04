# Provider Adapters

Providers are transport/model backends. They are not trusted execution authorities.

## Planned providers

| Provider | Network | Auth | Purpose |
|---|---:|---|---|
| `dummy` | no | none | offline tests |
| `ollama-local` | explicit | none | local Ollama API at `http://localhost:11434` |
| `ollama-cloud-via-local` | explicit | local Ollama sign-in | cloud models routed through local Ollama using `*-cloud` models |
| `ollama-cloud-direct` | explicit | `OLLAMA_API_KEY` | direct hosted API at `https://ollama.com` |
| `openai-compatible` | explicit | configured env var | compatible APIs, including local compatibility layers |
| `future-openai` | explicit | TBD | future dedicated adapter |
| `future-gemini` | explicit | TBD | future dedicated adapter |
| `custom` | explicit | configured | user-defined adapters |

## Rules

- `ask --dry-run` must be available before real provider calls.
- Provider network requires explicit command intent.
- Provider responses are untrusted.
- API keys must never be printed, logged, packed, proposed, snapshotted, or committed.
