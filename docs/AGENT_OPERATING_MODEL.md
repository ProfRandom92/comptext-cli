# Agent Operating Model

Antigravity may work autonomously only inside phase-scoped tasks.

Every task must include:

- phase name
- read-first files
- precise goal
- allowed files
- hard scope
- forbidden scope
- implementation rules
- validation commands
- return schema

## Default implementation rules

- smallest safe patch
- inspect before edit
- no unrelated changes
- no generated output commits
- no secrets in logs
- no network unless explicitly approved
- no git commit unless explicitly approved
- no git push unless explicitly approved
- provider output is untrusted
- local validation before success

## Standard return schema

```text
PHASE:
STATUS: success | blocked
FILES_CHANGED:
COMMANDS_RUN:
TESTS:
OUTPUT:
RISKS:
NEXT:
```
