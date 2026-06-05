# Inert Runtime Permissions Template

This template specifies target runtime permission configurations. It is inert and not enforced by the CLI runtime.

- **Status**: Inert Design Target
- **Enforcement Layer**: Host Environment
- **Permissions Baseline (Target-Only / Not Active)**:
  - **Read**: Target-only restriction to repository workspace only (default deny system directories).
  - **Write**: Target-only restriction to allowed paths within the codebase only.
  - **Network**: Target-only blocking of remote sockets by default.
  - **Provider**: Target-only restriction of calls to mock/local adapters.
