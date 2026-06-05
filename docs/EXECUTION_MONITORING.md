# Bounded Execution Monitoring

This document details the local execution-monitoring design and schema rules for CompText CLI.

---

## 1. Local Execution Monitoring Model

CompText observes and logs execution events inside the local workspace to serve as bounded evidence artifacts. All logs are generated and validated offline.

- **Schema Definition**: Execution monitoring logs are stored as JSON files with the latest log at `.comptext/execution_log.latest.json`.
- **Schema Version**: Exactly `0.1`.
- **Deterministic Event Ordering**: Events in the log are deterministically sorted by `timestamp` (chronological order) and then by `label`.

### Supported Event Labels (ExecutionLabel Enum)

- `CommandStarted`: A CLI action or command has commenced.
- `CommandFinished`: A CLI action or command has successfully completed.
- `ValidationPassed`: Local compilation, format, or test validation passed.
- `ValidationFailed`: Local compilation, format, or test validation failed.
- `ReviewGateRequired`: Transition halted awaiting manual gate review.
- `PolicyBlocked`: Action blocked due to security or policy boundary violation.

### Schema Shape

```json
{
  "schema_version": "0.1",
  "events": [
    {
      "label": "CommandStarted",
      "timestamp": "2026-06-05T14:33:23Z"
    },
    {
      "label": "CommandFinished",
      "timestamp": "2026-06-05T14:35:10Z"
    }
  ]
}
```

---

## 2. Path Safety and Validation Rules

- **Local Verification**: The `exec verify <path>` command parses the execution log and verifies that `schema_version` is exactly `"0.1"`, all event labels belong to the standard enum, and all events are deterministically ordered.
- **Path Rejections**: Rejects absolute paths and sensitive paths (such as `.env`, `.netrc`, `.git`, `.ssh`, `.aws`, `.git-credentials`, `.envrc`, private keys) before reading any log.
- **Authoritative Stance**: CompText behaves as the control plane; these logs do not represent active shell triggers, dynamic wrappers, or unsupported assurance claims.
