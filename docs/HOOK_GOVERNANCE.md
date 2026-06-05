# Hook Governance Model

Hooks are policy-interceptor targets designed to enforce strict safety boundaries before, during, and after agent runs. This document defines the target architecture for interceptor hooks within the CompText workspace.

---

## 1. Interception Points

The target architecture defines four critical interceptor locations:

```text
[ SessionStart Hook ]
         │
         ▼
[ PreToolUse Hook ] ────( Violations? )────► [ FAIL CLOSED / HALT ]
         │
         ▼
    ( Tool Run )
         │
         ▼
[ PostToolUse Hook ] ───( Redact Secrets )──► [ Filtered Context ]
         │
         ▼
[ PostPhase Hook ] ─────( Cargo Validation )──► [ Git Commit Pipeline ]
```

1. **SessionStart**:
   - **Trigger**: Executed when a new agent session or subagent run is initiated.
   - **Verification**: Parses workspace config, verifies CLI version, and checks that `AGENTS.md` and `PROJEKT.md` match remote origin main branches.
2. **PreToolUse**:
   - **Trigger**: Executed immediately before any tool (e.g. file read, file write, shell command execution) is run.
   - **Verification**: Evaluates inputs against active policy rules. Fails closed and blocks execution if a violation is detected.
3. **PostToolUse**:
   - **Trigger**: Executed immediately after a tool finishes running, before returning the output to the agent's context.
   - **Verification**: Filters and redacts high-entropy secrets, passwords, or credentials from command output and file read buffers.
4. **PostPhase**:
   - **Trigger**: Executed when an agent signals completion of a roadmap phase.
   - **Verification**: Runs the **Global Validation Suite** and checks git status to ensure the working tree remains clean before triggering the git push progression pipeline.

---

## 2. Policy Enforcements

The hook governance architecture must actively enforce the following rules:

- **Block `.env` and Secret Reads**: PreToolUse hooks block attempts to read `.env`, `.env.*`, keyfiles (`*.key`, `*.pem`, `*.p12`, `*.pfx`), or private keys.
- **Block Environment Variable Printing**: Blocks executing commands like `env`, `printenv`, or `Get-ChildItem Env:` to prevent leakages of system configuration credentials.
- **Block Network and Provider Calls**: Intercepts socket calls or provider invocations unless the active phase config explicitly permits network access.
- **Block Out-of-Bounds Writes**: Restricts file modifications to paths inside the repository root. Rejects edits targeting directories outside the workspace.
- **Block Broad Repository Rereads**: Limits tool executions that read the entire codebase recursively unless justified by a phase transition.
- **Require Proposal Before Apply**: Enforces that source code modification is only done via the `ctxt apply` flow referencing a verified JSON proposal from `proposals/`.
- **Require Local Validation**: Blocks marking a phase as complete until all commands in the validation suite pass successfully.
