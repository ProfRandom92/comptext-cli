# Phase Report Template

Use this template to document the successful completion or blocking of each development phase in the `reports/` folder.

```markdown
# CompText CLI — Phase <Number> Status Report

## Standard Return Schema
PHASE: <Phase Number and Title>
STATUS: <success | blocked>
FILES_CHANGED:
- <file_path_1>
- <file_path_2>
COMMANDS_RUN:
- `<command_1>`
- `<command_2>`
VALIDATION:
- <cargo validation checks summary>
ARTIFACTS:
- <generated Cache/JSON packs>
GIT:
- <git commit hash and push confirmation>
NETWORK:
- <network status during phase (e.g. offline-only | local-only | allowed-external)>
SECRETS:
- <secrets status (e.g. verified-redacted | none-encountered)>
POLICY_DECISIONS:
- <policy gates matched or skipped>
RISKS:
- <any identified risks or assumptions>
NEXT:
- <planned actions or next queue item>

---

## Detailed Notes & Output Samples
- <Include any sample execution outputs or additional development details here>
```
