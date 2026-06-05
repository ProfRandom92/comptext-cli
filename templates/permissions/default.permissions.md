# Inert Runtime Permissions Template

This template specifies target runtime permission configurations. It is inert and not enforced by the CLI runtime.

- **Status**: Inert Design Target
- **Enforcement Layer**: Host Environment
- **Permissions Baseline**:
  - **Read**: Repository workspace only (default deny system directories).
  - **Write**: Allowed paths within the codebase only.
  - **Network**: Block remote sockets by default.
  - **Provider**: Restrict calls to mock/local adapters.
