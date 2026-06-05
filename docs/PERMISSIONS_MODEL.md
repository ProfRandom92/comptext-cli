# Permissions Model

CompText utilizes a defense-in-depth permissions model to restrict agent actions at the operating system and execution environment level. 

---

## 1. Defense-in-Depth Doctrine

Permissions do not serve as the primary policy compiler. Instead, they act as low-level guards supporting the Safety Constitution (`AGENTS.md`):

1. **Safety Constitution (`AGENTS.md`)**: The primary rulebook governing logical behavior.
2. **Hook Interceptors (`docs/HOOK_GOVERNANCE.md`)**: Contextual software gates executing within the workspace.
3. **Permissions Model**: Hardware/runtime restrictions enforced by the orchestration host.

If a hook fails or an agent attempts to bypass logical constraints, the permissions model catches the violation and halts the execution thread.

---

## 2. Permission Scopes

The active execution environment is partitioned into narrow permission scopes:

- **Read Access**: Restricted to the repository workspace directory. Reading files under `/etc`, `C:\Windows`, system temp directories, or user home directories (outside the workspace) is denied by default.
- **Write Access**: Restricted to allowed targets within the workspace. Writing to system folders or configuration targets outside the project root is denied.
- **Network Access**: Denied by default. Enforces blockades on remote socket connections.
- **Provider Access**: Denied by default. Enforces blockades on calling external LLM providers or remote endpoints unless the phase explicitly authorizes mock or local integrations.

---

## 3. Explicit Phase Approval

If a task requires a restricted operation (e.g., querying a local Ollama socket), the permissions must be upgraded using explicit, phase-scoped approvals in the task definition. Once the phase transitions, permissions automatically downgrade back to the default-deny baseline.
