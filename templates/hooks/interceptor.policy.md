# Inert Hook Policy Template

This template specifies target policy interception configurations. It is inert and not executed by the CLI runtime.

- **Status**: Inert Design Target
- **Enforcement Layer**: Orchestrator Runtime
- **Rules**:
  - PreToolUse: Block `.env` file reading.
  - PostToolUse: Filter high-entropy patterns.
  - PostPhase: Execute `cargo test` suite.
