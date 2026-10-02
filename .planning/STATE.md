---
gsd_state_version: "1.0"
milestone: v0.09
milestone_name: Test hardening + executable rules
current_plan: 2
status: phase_complete
stopped_at: Phase 37 verified; M016-S02 closing
last_updated: "2026-10-02T12:00:00.000Z"
state_head: c784361
progress:
  total_phases: 5
  completed_phases: 2
  total_plans: 4
  completed_plans: 4
current_phase: 37
current_phase_name: rule-ast-and-hash
---

# Project State

**Milestone:** v0.09 Test hardening + executable rules (harness M016)
**Phase:** 37 — Rule AST and hash
**Status:** Phase 37 verification complete
**Next:** Lease M016 S03, then Phase 38 (evaluator). Do not edit `.kutha/STATE.md` from GSD executors.

## Current Position

**Current Plan:** 2
**Total Plans in Phase:** 2

## Performance Metrics

| Plan | Duration | Tasks | Files |
|------|----------|-------|-------|
| Phase 36 P01 | 3min | 2 tasks | 6 files |
| Phase 36 P02 | 6min | 2 tasks | 3 files |
| Phase 37 P01 | 1 | 1 tasks | 4 files |
| Phase 37 P02 | 2 | 2 tasks | 4 files |

## Decisions

- [Phase 36]: D-01: dtolnay trybuild 1.x on kutha-common; rustdoc compile_fail kept
- [Phase 36]: D-02: swap oracle is Justification equality; Op tt/vt stay u64
- [Phase 36]: D-03: no fold.rs/hydrate.rs patch; apply already inserts TtAnchor; replay as_known_on bind_reserved
- [Phase 37]: D-01: RuleAst Named+Horn tagged LE bytes; hash is SHA-256(kutha-rule-def || canonical)
- [Phase 37]: D-02: non-canonical bodies fail closed at emit_rule / blob_digest_matches
- [Phase 37]: D-03: no evaluator; inverse_knows cascade stays
- [Phase 37]: D-01: Named tag 0x01 / Horn 0x02; u16 LE lengths; exact-consume from_canonical
- [Phase 37]: rule_definition_hash(ident) = Named canonical hash; blob_digest_matches requires from_canonical

## Session

**Last session:** 2026-10-02T12:00:00.000Z
**Stopped at:** Phase 37 verified; M016-S02 closing
**Resume file:** None
