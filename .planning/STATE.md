---
gsd_state_version: "1.0"
milestone: v0.09
milestone_name: Test hardening + executable rules
current_plan: 2
status: executing
stopped_at: Completed 37-01-PLAN.md
last_updated: "2026-10-02T11:26:48.733Z"
state_head: 598ee7401740b7fe0204bca00ee037c69ce493b7
progress:
  total_phases: 5
  completed_phases: 0
  total_plans: 4
  completed_plans: 3
current_phase: 37
current_phase_name: rule-ast-and-hash
---

# Project State

**Milestone:** v0.09 Test hardening + executable rules (harness M016)
**Phase:** 37 — Rule AST and hash
**Status:** Executing 37-01 (M016 S02 / RULE-01)
**Next:** Finish 37-01 then 37-02; independent verify is a different agent (do not write 37-VERIFICATION)

## Current Position

**Current Plan:** 2
**Total Plans in Phase:** 2

## Performance Metrics

| Plan | Duration | Tasks | Files |
|------|----------|-------|-------|
| Phase 36 P01 | 3min | 2 tasks | 6 files |
| Phase 36 P02 | 6min | 2 tasks | 3 files |
| Phase 37 P01 | 1 | 1 tasks | 4 files |

## Decisions

- [Phase 36]: D-01: dtolnay trybuild 1.x on kutha-common; rustdoc compile_fail kept
- [Phase 36]: D-02: swap oracle is Justification equality; Op tt/vt stay u64
- [Phase 36]: D-03: no fold.rs/hydrate.rs patch; apply already inserts TtAnchor; replay as_known_on bind_reserved
- [Phase 37]: D-01: RuleAst Named+Horn tagged LE bytes; hash is SHA-256(kutha-rule-def || canonical)
- [Phase 37]: D-02: non-canonical bodies fail closed at emit_rule / blob_digest_matches
- [Phase 37]: D-03: no evaluator; inverse_knows cascade stays
- [Phase 37]: D-01: Named tag 0x01 / Horn 0x02; u16 LE lengths; exact-consume from_canonical

## Session

**Last session:** 2026-10-02T11:26:48.673Z
**Stopped at:** Completed 37-01-PLAN.md
**Resume file:** None
