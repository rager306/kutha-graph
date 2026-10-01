---
schema_version: 1
open_count: 9
waived_count: 0
fixed_count: 0
total_count: 9
last_updated: 2026-10-01T03:34:35.212Z
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
| 7 | 04 | deviation | CHANGELOG.md |  | Wave-close ci auto-mode includes uncommitted .kutha/STATE.md; CHANGELOG kept in worktree set for docs-coupling | open |  | 2026-09-29T15:18:13.481Z |  |
| 8 | 11 | deviation | scripts/kutha_gov/kinds.py |  | Derived cite mutation token changed from __cite_selftest__ to WRONG so the alphanumeric capture actually HIGH | open |  | 2026-09-30T10:25:59.675Z |  |
| 9 | 20 | deviation | crates/kutha-runtime/tests/m011_e2e_fixture.rs | 181 | Leased helper binds Action at u64::MAX because admission ingest is after t1 | open |  | 2026-10-01T03:34:35.212Z |  |

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
  },
  {
    "id": 7,
    "kind": "deviation",
    "phase": "04",
    "file": "CHANGELOG.md",
    "line": null,
    "description": "Wave-close ci auto-mode includes uncommitted .kutha/STATE.md; CHANGELOG kept in worktree set for docs-coupling",
    "status": "open",
    "reason": "",
    "recorded_at": "2026-09-29T15:18:13.481Z",
    "resolved_at": null,
    "milestone": "v0.02"
  },
  {
    "id": 8,
    "kind": "deviation",
    "phase": "11",
    "file": "scripts/kutha_gov/kinds.py",
    "line": null,
    "description": "Derived cite mutation token changed from __cite_selftest__ to WRONG so the alphanumeric capture actually HIGH",
    "status": "open",
    "reason": "",
    "recorded_at": "2026-09-30T10:25:59.675Z",
    "resolved_at": null,
    "milestone": "v0.03"
  },
  {
    "id": 9,
    "kind": "deviation",
    "phase": "20",
    "file": "crates/kutha-runtime/tests/m011_e2e_fixture.rs",
    "line": 181,
    "description": "Leased helper binds Action at u64::MAX because admission ingest is after t1",
    "status": "open",
    "reason": "",
    "recorded_at": "2026-10-01T03:34:35.212Z",
    "resolved_at": null,
    "milestone": "v0.05"
  }
]
````
