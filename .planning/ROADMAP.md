# Roadmap: Kutha

## Overview

GSD overlay tracks the steel thread already in crates — not honeycomb waterfall. Harness delivery stays in `.kutha/STATE.md` / `.kutha/ROADMAP.md`. Do not plan ADR-010–093 as sequential GSD phases.

v0.01–v0.05 are shipped. **v0.05** delivered M012 (dictionaries as facts): allowlist as log facts, rule registry (`rule_version` = hash), admission/policy meta-facts, thin action record, multi-hop derivation eligibility. **v0.06** (harness M013) answers the post-M012 review: baseline and target N, legal fixture reconnaissance, fold indexes, genesis log, dictionary entries as facts, one write path with a public facade, and an independent verification gate. Honeycomb stays Proposed. ADR-050 six dictionaries and M002 Rocks stay frozen until STATE names them. Run protocol: `docs/process/gsd-auto-contract.md`.

## Id map

See `docs/process/gsd-auto-contract.md` § Id map. Do not copy the table onto this ROADMAP.

## Milestones

- ✅ **v0.01 GSD foundation** — Phases 1–3 (shipped 2026-09-29) — [archive](./milestones/v0.01-ROADMAP.md)
- ✅ **v0.02 Semantic core close** — Phases 4–8 (shipped 2026-09-30) — [archive](./milestones/v0.02-ROADMAP.md)
- ✅ **v0.03 Lean context + semantic governor** — Phases 9–11 (shipped 2026-09-30) — [archive](./milestones/v0.03-ROADMAP.md)
- ✅ **v0.04 Single-log SoT + stable references** — Phases 12–16 (shipped 2026-10-01) — [archive](./milestones/v0.04-ROADMAP.md)
- ✅ **v0.05 Dictionaries as facts** — Phases 17–21 (shipped 2026-10-01) — [archive](./milestones/v0.05-ROADMAP.md)
- 🚧 **v0.06 Honest baseline and one write path** — Phases 22–30 (harness M013; Phases 22–28 shipped, 29–30 are the audit tail)

## Phases

**v0.06 phases** (one phase per harness slice S01–S09; the controller leases each slice before the phase runs):

- [x] **Phase 22: Benchmark baseline and size target** — re-runnable bench harness, target N, memory budget
- [x] **Phase 23: Legal fixture reconnaissance** — law-nexus real text mapped against the current model; no product code
- [x] **Phase 24: Fold indexes for ingest** — no log or fact-vector scans on ingest
- [x] **Phase 25: Genesis log** — relation allowance is log events; seeded relations are retractable
- [x] **Phase 26: Dictionary entries as facts** — reserved-namespace facts replace five meta side tables (completed 2026-10-01)
- [x] **Phase 27: One write path and facade** — one pipeline, typed references, public facade with example and integration test (completed 2026-10-02)
- [x] **Phase 28: Independent verification and artifact budget** — governor check, id map, milestone audit
- [ ] **Phase 29: Close-out honesty** — documents and governor follow-ups from the audit and project review; no product code
- [ ] **Phase 30: Write-path integrity hardening** — fail-closed dictionary writes, keyed correct, dual-mode oracle, strong facade test

## Phase Details

### Phase 22: Benchmark baseline and size target

**Goal**: The engine has a declared size target and a re-runnable measured baseline before anything is refactored.
**Depends on**: Nothing
**Requirements**: BASE-01, BASE-02, BASE-03
**Success Criteria** (what must be TRUE):
  1. One command runs ingest, `as_of`, claim support, retract, persist, open, and replay at three sizes and prints bytes per fact.
  2. A dated note declares target N and a memory budget beside the measured numbers.
  3. Two consecutive runs produce output of the same shape that can be diffed.

**Plans**: 2 plans (budget: `docs/process/gsd-auto-contract.md`)

Plans:
- [x] 22-01: bench harness (one size tracer, then N/2N/4N, validation and sabotage run)
- [x] 22-02: dated baseline note with target N and memory budget; path-set check

### Phase 23: Legal fixture reconnaissance

**Goal**: The riskiest assumption (two axes express real norms) is probed on paper with real text before any durability work.
**Depends on**: Nothing (runs independently of Phase 22)
**Requirements**: REC-01, REC-02, REC-03
**Success Criteria** (what must be TRUE):
  1. At least ten law-nexus cases are mapped against the current model, each marked expressible or not with a reason.
  2. The inexpressible set is named and listed as input for M014/M015.
  3. `git diff --stat -- crates/` is empty for the phase.

**Plans**: 1 plan

Plans:
- [x] 23-01: inventory, classify at least ten cases, quote gate, golden-fixture proposal

### Phase 24: Fold indexes for ingest

**Goal**: Ingest, retract, correct, claim admit, and follow-ons cost O(1) in log size.
**Depends on**: Phase 22
**Requirements**: IDX-01, IDX-02, IDX-03
**Success Criteria** (what must be TRUE):
  1. A scaling oracle fails on the pre-change code and passes after.
  2. No whole-log or whole-fact-vector scan remains on the listed paths (`rg` and CBM `trace_path` evidence).
  3. The baseline re-run shows no ingest or `as_of` regression over 10 percent; all named oracles are green.

**Plans**: 2/2 plans complete max

Plans:
- [x] 24-01: RED visit oracle + lease indexes on fold/Runtime
- [x] 24-02: wire D-03 sites, green oracle, bench delta + FoldIndexMode

### Phase 25: Genesis log

**Goal**: The log owns relation admission from its first event; no file is read at admit time.
**Depends on**: Phase 24
**Requirements**: GEN-01, GEN-02, GEN-03
**Success Criteria** (what must be TRUE):
  1. A new log starts with genesis events; opening an existing log never reads the YAML.
  2. Retracting a seeded relation fails admit at the later cut and still admits at the earlier cut.
  3. FF6 and the M012 allowlist oracle are green with unchanged assertions.

**Plans**: 2/2 plans complete 2026-10-01

Plans:
**Wave 1**
- [x] 25-01-PLAN.md — RED retract-seeded relatedTo admit; YAML-missing open

**Wave 2**
- [x] 25-02-PLAN.md — genesis AllowRelation on new log; admit fold-only; persist/open

### Phase 26: Dictionary entries as facts

**Goal**: One substrate: dictionary entries are ordinary bi-temporal facts; the five meta kinds disappear.
**Depends on**: Phase 25
**Requirements**: SUB-01, SUB-02, SUB-03, SUB-04
**Success Criteria** (what must be TRUE):
  1. `rg 'Op::(AllowRelation|RegisterRule|PinPolicy|RecordAdmission|RecordAction)' crates` returns nothing outside a migration note.
  2. Five side tables and five `is_live_at` copies are gone; one `as_of` answers all entries.
  3. Justification and policy references are `EventId` or hash; the RAM buffer is gone.
  4. M012 oracles keep their assertions; the rename table is in the SUMMARY.

**Plans**: 2/2 plans complete

Plans:
**Wave 1**
- [x] 26-01-PLAN.md — RED as_of cannot see admission/policy/allow (side tables)

**Wave 2**
- [x] 26-02-PLAN.md — kutha: facts + StoreBlob; drop five meta Ops; EventId admissions

### Phase 27: One write path and facade

**Goal**: A single propose, admit, commit, receipt pipeline and a public facade used by something other than in-crate tests.
**Depends on**: Phase 26
**Requirements**: PIPE-01, PIPE-02, PIPE-03, PIPE-04
**Success Criteria** (what must be TRUE):
  1. No kind-specific early return or side append remains; all kinds pass through one pipeline.
  2. No file under `crates/kutha-runtime/src` exceeds 800 lines.
  3. One example and one integration test use only the facade and run the statute-shaped AS OF scenario.
  4. The baseline re-run shows no regression over 10 percent.

**Plans**: 2/2 plans complete

Plans:
**Wave 1**
- [x] 27-01-PLAN.md — RED facade statute compile + unknown-relation admit oracle

**Wave 2**
- [x] 27-02-PLAN.md — one pipeline, src split ≤800, facade, bench ≤10%, full ci

### Phase 28: Independent verification and artifact budget

**Goal**: The process stops grading itself: budget and independence are enforced, ids have one map, the milestone is audited.
**Depends on**: Phase 27
**Requirements**: PROC-01, PROC-02, PROC-03, PROC-04
**Success Criteria** (what must be TRUE):
  1. A governor check fails closed when a phase exceeds the artifact budget; its selftest mutation is not vacuous.
  2. Every v0.06 phase has a verification file from a verifier that did not read PLAN or SUMMARY.
  3. The id map exists in one table and is cited by the governor.
  4. `/gsd-audit-milestone` passes before archive.

**Plans**: 2/2 plans complete

Plans:
**Wave 1**
- [x] 28-01-PLAN.md — glob_max_lines, artifact-budget, conform 22–27 VERIFICATION
- [x] 28-02-PLAN.md — id map, milestone audit

### Phase 29: Close-out honesty

**Goal**: The v0.06 bookkeeping, semantic review, ADR evidence, and governor say what the repository actually does.
**Depends on**: Phase 28
**Requirements**: DOC-01, DOC-02, DOC-03, DOC-04, GOV-01, GOV-02, GOV-03, GOV-04
**Success Criteria** (what must be TRUE):
  1. No planning or process document calls M013 delivered work leased-in-progress or cites a stale `L_delivery`; the milestone audit body matches its status and `28-VERIFICATION`.
  2. The semantic-gap review has a dated post-M013 addendum separating closed from open findings, and FF5 scope is stated honestly.
  3. Every touched ADR cell is still Proposed and carries a dated Evidence or Clarification note; D1–D10 are untouched.
  4. A scratch copy with budget below the check count, an empty budget glob, or a VERIFICATION missing `independent: true` makes the governor fail.

**Plans**: 2 plans max

### Phase 30: Write-path integrity hardening

**Goal**: The one write path cannot be bypassed or silently lose identity, in either fold index mode, before typed clocks change the facade.
**Depends on**: Phase 29
**Requirements**: HARD-01, HARD-02, HARD-03, HARD-04
**Success Criteria** (what must be TRUE):
  1. A public write of a `kutha:` relation outside the typed constructors is refused; a RED oracle failed before the change.
  2. Keyed assert, correct, then retry of the same key leaves support count 1.
  3. FF5, retract, correct, and claim support agree under Eager and Lazy.
  4. The facade test asserts live triples at each step, including after persist/open and fork.

**Plans**: 2 plans max

<details>
<summary>✅ v0.01 GSD foundation (Phases 1–3) — SHIPPED 2026-09-29</summary>

- [x] Phase 1: Legal PIT fitness (3/3 plans) — completed 2026-09-29
- [x] Phase 2: Honest harness and freeze (3/3 plans) — completed 2026-09-29
- [x] Phase 3: Lease-gated next slice (3/3 plans) — completed 2026-09-29

Full detail: [milestones/v0.01-ROADMAP.md](./milestones/v0.01-ROADMAP.md)

</details>

<details>
<summary>✅ v0.02 Semantic core close (Phases 4–8) — SHIPPED 2026-09-30</summary>

- [x] Phase 4–8 — see [milestones/v0.02-ROADMAP.md](./milestones/v0.02-ROADMAP.md)

</details>

<details>
<summary>✅ v0.03 Lean context + semantic governor (Phases 9–11) — SHIPPED 2026-09-30</summary>

- [x] Phase 9–11 — see [milestones/v0.03-ROADMAP.md](./milestones/v0.03-ROADMAP.md)

</details>

<details>
<summary>✅ v0.04 Single-log SoT + stable references (Phases 12–16) — SHIPPED 2026-10-01</summary>

- [x] Phase 12: Log-native SoT (3/3 plans) — completed 2026-10-01
- [x] Phase 13: Stable references (3/3 plans) — completed 2026-10-01
- [x] Phase 14: Idempotent ingest (3/3 plans) — completed 2026-10-01
- [x] Phase 15: Verify, persist, and time scale (3/3 plans) — completed 2026-10-01
- [x] Phase 16: Fold-internal hot indexes (3/3 plans) — completed 2026-10-01

Full detail: [milestones/v0.04-ROADMAP.md](./milestones/v0.04-ROADMAP.md) · requirements: [milestones/v0.04-REQUIREMENTS.md](./milestones/v0.04-REQUIREMENTS.md) · phases: [milestones/v0.04-phases/](./milestones/v0.04-phases/)

</details>

<details>
<summary>✅ v0.05 Dictionaries as facts (Phases 17–21) — SHIPPED 2026-10-01</summary>

- [x] Phase 17: Allowlist as log facts (3/3 plans) — completed 2026-10-01
- [x] Phase 18: Rule registry (3/3 plans) — completed 2026-10-01
- [x] Phase 19: Admission and policy meta-facts (3/3 plans) — completed 2026-10-01
- [x] Phase 20: Thin action record (3/3 plans) — completed 2026-10-01
- [x] Phase 21: Multi-hop derivation (3/3 plans) — completed 2026-10-01

Full detail: [milestones/v0.05-ROADMAP.md](./milestones/v0.05-ROADMAP.md) · requirements: [milestones/v0.05-REQUIREMENTS.md](./milestones/v0.05-REQUIREMENTS.md) · phases: [milestones/v0.05-phases/](./milestones/v0.05-phases/)

</details>

## Next

v0.06 is the active GSD milestone (harness M013, leased 2026-10-01, Active Slice None; Phases 22–28 shipped, 29–30 are the audit tail). Run one phase at a time with `/gsd-autonomous --only N` after the controller leases that slice (`docs/process/gsd-auto-contract.md`). Proposed next, each needing its own lease: M014 typed clocks (v0.07), M015 executable rules and the legal golden fixture (v0.08), then M002 durability. Freeze holds for Rocks/Cypher/HNSW/ADR-050 six dictionaries/legal pack until STATE names them.
