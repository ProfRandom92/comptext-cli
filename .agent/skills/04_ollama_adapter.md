# Skill 04 — Ollama Adapter

Purpose: implement Ollama variants safely.

Support:
- local Ollama via `http://localhost:11434`
- cloud models via local Ollama using `*-cloud` models
- direct cloud API via `https://ollama.com`
- direct cloud auth via `OLLAMA_API_KEY`

Rules:
- never log `OLLAMA_API_KEY`
- dry-run before real provider call
- no network unless command explicitly asks for provider call
