---
schema_version: 1
open_count: 6
waived_count: 0
fixed_count: 0
total_count: 6
last_updated: 2026-09-29T15:07:11.391Z
---

# Broken Windows Ledger

> Cross-phase defect register. With `workflow.windows_enforce` enabled, `/gsd-ship` blocks while `open_count > 0`.
> Waive with `gsd-tools windows waive <id> "<reason>"` (reason required).
> Mark fixed with `gsd-tools windows fixed <id>`.

| id | phase | kind | file | line | description | status | reason | recorded_at | resolved_at |
|----|-------|------|------|------|-------------|--------|--------|-------------|-------------|
| 1 | 01 | deviation | .planning/STATE.md |  | state.update-progress wrote percent 0; corrected to 67% in closeout | open |  | 2026-09-29T07:26:58.548Z |  |
| 2 | 01 | deviation | .kutha/STATE.md |  | Plan grep Active Slice: None vs markdown **Active Slice:** None; lease still None | open |  | 2026-09-29T07:32:17.882Z |  |
| 3 | 01 | deviation | .planning/STATE.md |  | SDK state.update-progress wrote 33%; restored 100% after Phase 1 plan closeout | open |  | 2026-09-29T07:32:18.201Z |  |
| 4 | 02 | deviation | .kutha/STATE.md | 8 | Plan verify grep Active Slice: None misses markdown **Active Slice:** None; cite-only, no harness edit | open |  | 2026-09-29T08:23:38.090Z |  |
| 5 | 03 | deviation | .planning/STATE.md |  | Relabeled Current Plan to numeric N so state.advance-plan can parse (Rule 3) | open |  | 2026-09-29T11:06:24.445Z |  |
| 6 | 04 | deviation | crates/kutha-runtime/tests/m011_partial_correction.rs |  | Task 2 tdd=true had no RED: tracer already shipped IntervalPatchRejected; tests committed green | open |  | 2026-09-29T15:07:11.391Z |  |

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
  },
  {
    "id": 2,
    "kind": "deviation",
    "phase": "01",
    "file": ".kutha/STATE.md",
    "line": null,
    "description": "Plan grep Active Slice: None vs markdown **Active Slice:** None; lease still None",
    "status": "open",
    "reason": "",
    "recorded_at": "2026-09-29T07:32:17.882Z",
    "resolved_at": null,
    "milestone": null
  },
  {
    "id": 3,
    "kind": "deviation",
    "phase": "01",
    "file": ".planning/STATE.md",
    "line": null,
    "description": "SDK state.update-progress wrote 33%; restored 100% after Phase 1 plan closeout",
    "status": "open",
    "reason": "",
    "recorded_at": "2026-09-29T07:32:18.201Z",
    "resolved_at": null,
    "milestone": null
  },
  {
    "id": 4,
    "kind": "deviation",
    "phase": "02",
    "file": ".kutha/STATE.md",
    "line": 8,
    "description": "Plan verify grep Active Slice: None misses markdown **Active Slice:** None; cite-only, no harness edit",
    "status": "open",
    "reason": "",
    "recorded_at": "2026-09-29T08:23:38.090Z",
    "resolved_at": null,
    "milestone": null
  },
  {
    "id": 5,
    "kind": "deviation",
    "phase": "03",
    "file": ".planning/STATE.md",
    "line": null,
    "description": "Relabeled Current Plan to numeric N so state.advance-plan can parse (Rule 3)",
    "status": "open",
    "reason": "",
    "recorded_at": "2026-09-29T11:06:24.445Z",
    "resolved_at": null,
    "milestone": null
  },
  {
    "id": 6,
    "kind": "deviation",
    "phase": "04",
    "file": "crates/kutha-runtime/tests/m011_partial_correction.rs",
    "line": null,
    "description": "Task 2 tdd=true had no RED: tracer already shipped IntervalPatchRejected; tests committed green",
    "status": "open",
    "reason": "",
    "recorded_at": "2026-09-29T15:07:11.391Z",
    "resolved_at": null,
    "milestone": "v0.02"
  }
]
````
