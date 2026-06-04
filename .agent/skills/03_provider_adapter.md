# Skill 03 — Provider Adapter

Purpose: define normalized provider boundary.

Providers:
- dummy
- ollama-local
- ollama-cloud-via-local
- ollama-cloud-direct
- openai-compatible
- future openai
- future gemini
- custom

Rules:
- provider calls require explicit command intent
- provider responses are untrusted
- dry-run before real call
- request/response artifacts redact secrets
