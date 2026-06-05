# Inert Hook Policy Template

This template specifies target policy interception configurations. It is inert and not executed by the CLI runtime.

- **Status**: Inert Design Target
- **Enforcement Layer**: Orchestrator Runtime
- **Rules (Target-Only / Not Active)**:
  - PreToolUse: Target-only blocking of `.env` file reading.
  - PostToolUse: Target-only filtering of high-entropy patterns.
  - PostPhase: Target-only execution of `cargo test` suite.

- **Metadata**:
  - Requires host/orchestrator integration.
  - Not executed by Rust CLI.
  - No secrets.
  - No network by default.
