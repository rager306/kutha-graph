# Requirements: Kutha

**Defined:** 2026-10-01
**Core Value:** Legal PIT stays falsifiable; governor CI stays honest; one Active Slice — not honeycomb delivery.
**Milestone:** v0.06 Honest baseline and one write path (harness lease M013)

## v0.06 Requirements

Closes F10, F11, F12, F14, F15 and R2, R4, R5, R6 from `docs/architecture/semantic-gap-review.md` (addendum 2026-10-01). Refactors keep every named oracle (FF5, FF6, M010, M011, M012a, M012) green with unchanged assertions. Cells stay Proposed. Not rules (M015), not typed clocks (M014), not M002. Execution rules: `docs/process/gsd-auto-contract.md`.

### Benchmark baseline (R2)

- [ ] **BASE-01**: A bench harness with no new dependencies measures ingest, `as_of`, claim support, retract, persist, open, and replay at three log sizes, and reports bytes per fact
- [ ] **BASE-02**: A target N and a memory budget are declared in a dated note (not an ADR), with the measured numbers beside them
- [ ] **BASE-03**: One documented command re-runs the harness; its output has a stable, diffable shape

### Legal fixture reconnaissance (R1)

- [ ] **REC-01**: A note maps at least ten real-text cases from law-nexus (FZ-44 snippets, consultant sources with editions, the synthetic sunset clause) against the current two-axis model
- [ ] **REC-02**: Each case is marked expressible or not with the reason; the inexpressible set is named and carried into M014/M015 as input, not solved here
- [ ] **REC-03**: No file under `crates/` changes in this phase

### Fold indexes (F15)

- [x] **IDX-01**: Keyed Assert retry, claim admit, retract, correct, and follow-ons do no whole-log or whole-fact-vector scan
- [x] **IDX-02**: A scaling oracle fails on the pre-change code and passes after (per-op cost does not grow with N across three sizes)
- [x] **IDX-03**: The baseline is re-run and ingest and `as_of` regress by no more than 10 percent

### Genesis log (F10)

- [x] **GEN-01**: A new log begins with genesis events that carry the initial relation allowance; the YAML file is read only when a new log is created
- [x] **GEN-02**: `admit` consults the fold cut only; no file or environment path is read at admit time
- [x] **GEN-03**: Retracting a seeded relation makes admit fail closed for it at the later cut and still admits it at the earlier cut; FF6 stays green

### Dictionary entries as facts (F11, F14)

- [x] **SUB-01**: Relation, rule, policy, admission, and action records are facts in a reserved namespace; rule and policy bodies are content-addressed blobs referenced by hash
- [x] **SUB-02**: The five meta `Op` variants, their five side-table structs, and the five copies of `is_live_at` are removed; one `as_of` and one Retract/Correct path serve all of them
- [x] **SUB-03**: Justification and policy references are `EventId` or content hash; the RAM justification buffer is gone
- [x] **SUB-04**: The M012 oracles keep their assertions; every mechanical rename is listed in an old-to-new table in the phase SUMMARY

### One write path and facade (F11, F12)

- [x] **PIPE-01**: Every record kind enters the log through one propose, admit, commit, receipt pipeline; no kind-specific early return or side append remains
- [x] **PIPE-02**: The runtime is split by concern so no source file under `crates/kutha-runtime/src` exceeds 800 lines
- [x] **PIPE-03**: A public facade exposes assert, retract, correct, derive, admit, record action, `as_of`, and open/persist
- [x] **PIPE-04**: One example and one integration test use only the facade and run the statute-shaped AS OF scenario end to end

### Process gates (R4, R5, R6)

- [x] **PROC-01**: A governor check enforces the planning-artifact budget (`docs/process/gsd-auto-contract.md`) and fails closed on excess
- [x] **PROC-02**: Every v0.06 phase has a `NN-VERIFICATION.md` written by a verifier that did not read the PLAN or SUMMARY, with kill-tests and exit statuses
- [x] **PROC-03**: One id map table (GSD version, phases, harness milestone and slice, ADR) exists and the governor cites it
- [x] **PROC-04**: The milestone audit runs before archive (`/gsd-audit-milestone`), not skipped by override

### Close-out honesty (audit tail, Phase 29)

Source: `.planning/v0.06-MILESTONE-AUDIT.md` and the 2026-10-02 project review (roadmap, ADR, code, tests, harness). Harness plane and documents only; no `crates/` change.

- [ ] **DOC-01**: v0.06 bookkeeping is consistent: `.kutha/ROADMAP.md`, `.planning/ROADMAP.md`, `PROJECT.md`, `STATE.md`, the id map, REQUIREMENTS checkboxes (BASE, REC with evidence), and the milestone audit body agree with `.kutha/STATE.md`; M013 stays leased until S09 closes
- [ ] **DOC-02**: `docs/architecture/semantic-gap-review.md` has a dated post-M013 addendum: F10, F11, F12, F14, F15 closed with their oracles; F1 encoding, F7, F8 remainder, F9, F13, F16 open; FF5 is stated as two valid-time windows at current knowledge, not “as known on D”
- [ ] **DOC-03**: Proposed ADR cells (000, 010, 011, 012, 013, 014, 040, 041, 050, 060, 061) carry a dated Evidence or Clarification note that matches the code; `honeycomb.yaml` evidence names point at the real tests; `docs/ADR/README.md` has one status-vocabulary note; no cell is promoted and D1–D10 are not rewritten
- [ ] **DOC-04**: Process docs match behavior: `ci` includes selftest and `--skip-cargo` does not skip it; `glob_max_lines` zero-match rule; `codex-subagents.md` and `governor-intake.md` examples are current

- [ ] **GOV-01**: The FSM fails closed when the check budget truncates any check, and a test pins budget at or above the check count
- [ ] **GOV-02**: `glob_max_lines` fails closed when a scoped glob matches nothing; its selftest mutation is not vacuous
- [ ] **GOV-03**: The independence check covers every `NN-VERIFICATION.md` from phase 22 (including the current phase) and verifies that listed `read_only_inputs` exist and are not plan or summary artifacts; model-family independence stays documented as honor-system
- [ ] **GOV-04**: `observe_cargo.required` names the M013 oracles (facade, write path, genesis, dictionary facts, fold indexes, fold index mode); `deferred` invariants whose `until` milestone has shipped are restaged without losing their bridge or test evidence

### Write-path integrity (audit tail, Phase 30)

Source: code and test review 2026-10-02. Named oracles keep their assertions; no typed clocks (M014), no rules (M015), no durability (M002).

- [ ] **HARD-01**: A public write of a reserved `kutha:` relation outside the typed constructors (genesis allow, rule, policy, admission, action) is refused; a RED oracle failed before the change
- [ ] **HARD-02**: `correct` and `correct_interval` preserve `delivery_key`; a keyed assert, correct, then retry of the same key leaves support count 1
- [ ] **HARD-03**: One oracle runs FF5, retract, correct, and claim support under both `FoldIndexMode::Eager` and `Lazy` with identical results
- [ ] **HARD-04**: The facade integration test asserts the exact live triples after assert, correct, retract, persist/open, and `fork_at`

## Future Requirements (not v0.06)

### M014 — typed clocks and as-known-on (v0.07)

- **CLK-01**: Valid, transaction, and wall clocks are distinct types; a silent mix does not compile
- **CLK-02**: `tt_anchor` facts map transaction time to wall time; `as_known_on(date)` is an anchor lookup

### M015 — executable rules and legal golden fixture (v0.08)

- **RUL-01**: A canonical positive rule AST is hashed into `rule_version`
- **RUL-02**: Rule bodies are evaluated by the leapfrog join over the CSR lease with support counting
- **RUL-03**: Admission runs the pinned policy body; execution replay re-derives and compares
- **LEG-01**: A real-text fixture with one transitional rule and an as-known-on query is green, or the failing case is recorded and M002 is re-planned

### M002 — durability (after M015)

- **DUR-02**: Segmented log, hash chain, atomic manifest, commit-marker framing; Rocks for indexes only

## Out of Scope

| Feature | Reason |
|---------|--------|
| New product features in phases 24–27 | Refactor slices; capability comes in M014/M015 |
| Rule language, evaluator, typed clocks | M014/M015 need their own lease |
| RocksDB / segmented WAL protocol | M002 |
| Cypher / GPML / HNSW | Freeze |
| ADR-050 six dictionaries, legal pack, full ADR-090 | Freeze; recon reads law-nexus text as test data only |
| Legacy-format compatibility for meta-op rows | Crates are `0.0.0` and unpublished; recorded as a decision in phase 26 CONTEXT |
| Promoting honeycomb to Accepted | L_map stays Proposed |

## Traceability

| Requirement | Phase | Status |
|-------------|-------|--------|
| BASE-01, BASE-02, BASE-03 | Phase 22 | Pending |
| REC-01, REC-02, REC-03 | Phase 23 | Pending |
| IDX-01, IDX-02, IDX-03 | Phase 24 | complete |
| GEN-01, GEN-02, GEN-03 | Phase 25 | complete |
| SUB-01, SUB-02, SUB-03, SUB-04 | Phase 26 | complete |
| PIPE-01, PIPE-02, PIPE-03, PIPE-04 | Phase 27 | complete |
| PROC-01, PROC-02, PROC-03, PROC-04 | Phase 28 | complete |
| DOC-01, DOC-02, DOC-03, DOC-04, GOV-01, GOV-02, GOV-03, GOV-04 | Phase 29 | Pending |
| HARD-01, HARD-02, HARD-03, HARD-04 | Phase 30 | Pending |

**Coverage:** v0.06 requirements: 36 total; mapped to phases: 36; unmapped: 0. BASE and REC rows flip to complete in Phase 29 (DOC-01).

---
*Requirements defined: 2026-10-01*
