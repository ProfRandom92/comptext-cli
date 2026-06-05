# Policy Interceptor Specification

This document provides the inert specification format for policy interceptor hooks. These hooks represent target integration endpoints for orchestrator runtimes and are not actively executed by the local Rust CLI.

---

## 1. Interceptor Lifecycle Targets

```text
[ SessionStart ] ──────► Instantiates the workspace validation profile
       │
       ▼
[ PreToolUse ] ────────► Intercepts and validates tool input parameters
       │
       ▼
  ( Tool Run )
       │
       ▼
[ PostToolUse ] ───────► Inspects and filters output streams
       │
       ▼
[ PostPhase ] ─────────► Evaluates phase completeness and git tree clean status
```

---

## 2. Specification Formats (Inert Targets)

### SessionStart Specification
- **Intent**: Initializes the session state tracking profile.
- **Inert Spec**:
  ```json
  {
    "hook": "SessionStart",
    "status": "target",
    "actions": [
      "verify_local_git_cleanliness",
      "load_active_phase_restrictions"
    ]
  }
  ```

### PreToolUse Specification
- **Intent**: Validates tool invocation arguments before execution.
- **Inert Spec**:
  ```json
  {
    "hook": "PreToolUse",
    "status": "target",
    "restricted_arguments": {
      "command_prefixes": ["env", "printenv"],
      "blocked_paths": [".env", "*.key", "*.pem"]
    }
  }
  ```

### PostToolUse Specification
- **Intent**: Filters and sanitizes outputs before returning them to the model context.
- **Inert Spec**:
  ```json
  {
    "hook": "PostToolUse",
    "status": "target",
    "redaction_patterns": [
      "high_entropy_strings",
      "credential_variables"
    ]
  }
  ```

### PostPhase Specification
- **Intent**: Verifies phase completeness and triggers git progression pipeline.
- **Inert Spec**:
  ```json
  {
    "hook": "PostPhase",
    "status": "target",
    "verification_suite": [
      "cargo fmt --all --check",
      "cargo check",
      "cargo test",
      "cargo clippy -- -D warnings"
    ]
  }
  ```
