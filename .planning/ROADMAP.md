# Roadmap: Kutha

## Overview

GSD overlay tracks the steel thread already in crates. Harness delivery stays in `.kutha/STATE.md` / `.kutha/ROADMAP.md`.

v0.01–v0.08 are shipped. **v0.09** (harness M016) closes test-audit T2–T6 then F9/F16 executable rules + golden fixture. Honeycomb stays Proposed. Run: `docs/process/gsd-auto-contract.md`.

## Id map

See `docs/process/gsd-auto-contract.md` § Id map. Do not copy the table onto this ROADMAP.

## Milestones

- ✅ **v0.01–v0.08** — shipped — see `./milestones/`
- 🚧 **v0.09 Test hardening + executable rules** — Phases 36–40 (harness M016)

## Phases

- [x] **Phase 36: Test hardening T2–T6** — trybuild mix, cite tt/vt, known-on Eager/Lazy/fork, snapshot+tail anchors, replay applies TtAnchor
- [ ] **Phase 37: Rule AST and hash** — canonical encoding; `rule_version` hash; fail-closed unknown bodies
- [ ] **Phase 38: Rule evaluator and support counting** — cascade → rules; retract maintains supports
- [ ] **Phase 39: Policy evaluation and execution replay** — pinned policy; replay re-derives
- [ ] **Phase 40: Legal golden fixture and verdict** — transitional rule + as_known_on; ADR-013 dated note

## Phase Details

### Phase 36: Test hardening T2–T6

**Goal**: Close audit holes T2–T6 without new product features beyond replay/anchor correctness required by TH-04/05.
**Requirements**: TH-01..TH-05
**Success Criteria**:
  1. trybuild E0308 for clock mix (Tt/Wall → Vt).
  2. Swapped cite tt/vt fails a named oracle.
  3. Eager==Lazy `as_known_on`; fork_at + persist/open agree.
  4. Mid-cut snapshot + non-empty tail recovers known-on; anchors in replay path.
  5. `GraphFold::replay` applies TtAnchor.

**Plans**: 2 max
- [x] 36-01-PLAN.md
- [x] 36-02-PLAN.md

### Phase 37: Rule AST and hash

**Goal**: RULE-01. Canonical `RuleAst` encoding; `rule_version` is body hash; fail-closed unknown bodies.
**Requirements**: RULE-01
**Success Criteria**:
  1. Named/Horn canonical tagged bytes round-trip; JSON/trailing/raw utf8 fail `from_canonical`.
  2. `rule_definition_hash(ident)` equals Named body hash; `emit_rule` stores those bytes.
  3. Non-canonical register is `UnknownRuleVersion`; M012/FF5/FF6 oracles stay green.

**Plans**: 2 max
- [x] 37-01-PLAN.md
- [x] 37-02-PLAN.md

### Phase 38: Rule evaluator and support counting

**Goal**: RULE-02. **Plans**: 2 max

### Phase 39: Policy evaluation and execution replay

**Goal**: RULE-03, RULE-04. **Plans**: 2 max

### Phase 40: Legal golden fixture and verdict

**Goal**: FIXG-01. **Plans**: 1–2 max

## Progress

| Milestone | Phases | Status |
|-----------|--------|--------|
| v0.01–v0.08 | 1–35 | shipped |
| v0.09 | 36–40 | active (M016) |

**Next:** Independent verify Phase 37 (D-05). Do not start 38 until M016 S03 is leased.
