---
schema_version: 1
open_count: 1
waived_count: 0
fixed_count: 0
total_count: 1
last_updated: 2026-09-29T07:26:58.548Z
---

# Broken Windows Ledger

> Cross-phase defect register. With `workflow.windows_enforce` enabled, `/gsd-ship` blocks while `open_count > 0`.
> Waive with `gsd-tools windows waive <id> "<reason>"` (reason required).
> Mark fixed with `gsd-tools windows fixed <id>`.

| id | phase | kind | file | line | description | status | reason | recorded_at | resolved_at |
|----|-------|------|------|------|-------------|--------|--------|-------------|-------------|
| 1 | 01 | deviation | .planning/STATE.md |  | state.update-progress wrote percent 0; corrected to 67% in closeout | open |  | 2026-09-29T07:26:58.548Z |  |

````json
[
  {
    "id": 1,
    "kind": "deviation",
    "phase": "01",
    "file": ".planning/STATE.md",
    "line": null,
    "description": "state.update-progress wrote percent 0; corrected to 67% in closeout",
    "status": "open",
    "reason": "",
    "recorded_at": "2026-09-29T07:26:58.548Z",
    "resolved_at": null,
    "milestone": null
  }
]
````
