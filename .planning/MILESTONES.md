## v0.17 Green HEAD and truth reset (Active: 2026-10-08)

**Phases:** 75–78. Process tag `v0.17`. Restore green (keyed retry Correct lineage; naive Retract-all-live; dur_a parallel scratch), close-gate honesty (`product-close-gate`), living overlay truth, close. Corrects false M022 Phase 72 verified claim.
**Git range:** `v0.16..` (tag pending close)

---

## v0.13 Post-M002 coherence (Shipped: 2026-10-05)

**Phases:** 56–62. Process tag `v0.13` (GSD process tag, not a product release). Docs honesty r3, cargo_dep_constraint, governor SMP, test matrix, durability residuals, DIFF-01/DIFF-02/SEM-02. MEM-01 partial (RSS `bytes_per_fact` not explained to a lower bound).
**Git range:** `v0.12..v0.13`

---

## v0.12 Log durability protocol (Shipped: 2026-10-04)

**Phases:** 49–55. Process tag `v0.12`. SNAP-01, fail-closed WAL, framed append, commit marker option A, sealed segments with a cross-segment SHA-256 chain, droppable Rocks index lease.
**Git range:** `v0.10..v0.12` (v0.11 was not a tag)

---

## v0.10 Docs honesty and replay integrity (Shipped: 2026-10-03)

**Phases:** 41–44. Process tag `v0.10`. Living docs/governor honesty, replay integrity, determinism oracles, M016 ingest regression repaired.
**Git range:** `v0.09..v0.10`

---

## v0.09 Test hardening + executable rules (Shipped: 2026-10-02)

**Phases:** 36–40. Process tag `v0.09`. T2–T6, F9, F16. FIXG-01 fail-closed (C01 sunset is not a Horn).
**Git range:** `v0.08..v0.09`

---

## v0.08 Known-on honesty and M014 residuals (Shipped: 2026-10-02)

**Phases:** 34–35. Process tag `v0.08`. H1/H2 closed; ANK in observe_cargo; living docs aligned.
**Git range:** `v0.07..v0.08`

---

# Milestones

## v0.06 Honest baseline and one write path (Shipped: 2026-10-02)

**Phases completed:** 9 phases, 17 plans, 9 independent VERIFICATIONs
**Closeout type:** override_closeout (residual tech debt acknowledged; all HARD/DOC/GOV requirements satisfied)
**Known verification overrides:** SUMMARY `requirements-completed` gaps on early phases; IDX-03/D-07 bench overrides recorded; Nyquist skipped; CBM coverage freshness. Not product blockers.

**Key accomplishments:**
- Re-runnable bench harness + dated baseline note with target N (Phase 22)
- Legal fixture reconnaissance: ≥10 law-nexus cases mapped; crates untouched (Phase 23)
- Fold indexes: O(1) ingest/retract/correct/claim paths; FoldIndexMode Eager/Lazy (Phase 24)
- Genesis log owns relation allowance; YAML not read at admit (Phase 25)
- Dictionary entries as reserved-namespace facts; one write pipeline + public facade (Phases 26–27)
- Governor artifact budget + verification independence + id-map cite (Phase 28)
- Close-out honesty: documents, ADR evidence notes, budget fail-closed, I-F restage (Phase 29)
- Write-path hardening: User emit refuses reserved kutha: Assert/Correct/CorrectInterval/Behavior; keyed Correct keeps delivery_key; dual-mode oracle; exact facade triples (Phase 30)
- `uv run kutha-gov ci` 0 HIGH (60 checks / budget 64); honeycomb stays Proposed; M013 CLOSED

**Delivered:** M013 S01–S09 under lease discipline. Harness Active Slice cleared (`L_delivery=M013-S09-done`). Archive + GSD process tag `v0.06`. No crate SemVer bump.

**Git range:** `v0.05..v0.06` · Timeline: 2026-10-01 → 2026-10-02

---

## v0.05 Dictionaries as facts (Shipped: 2026-10-01)

**Phases completed:** 5 phases, 15 plans, 34 tasks

**Key accomplishments:**
- `Op::AllowRelation` versioned allowlist facts; admit consults the fold cut; unknown relations still fail closed (FF6)
- `Op::RegisterRule` registry; Behavior `rule_version` equals definition hash; unknown/mismatched/free-string pins fail closed
- `Op::PinPolicy` + `RecordAdmission`; `check_admission` on the product path; admission and policy queryable AS OF a cut
- `Op::RecordAction` binds resolved arguments to admission + policy version; Action + admission survive a later policy pin
- `derivation_eligible_at` walks `caused_by` for a named two-hop hashed fixture; `inverse_knows` remains the residual automatic cascade
- Governor observes ALL/RULE/ADM/ACT/DER named oracles; `uv run kutha-gov ci` is 0 HIGH; honeycomb stays Proposed; M012 CLOSED

---

## v0.04 Single-log SoT + stable references (Shipped: 2026-10-01)

**Phases completed:** 5 phases, 15 plans, 31 tasks

**Key accomplishments:**
- Op::QuantumOutcome fold-noop log records; discarding quantum_outcomes.jsonl still reconstructs Zero/Partial/Full
- Op::JustificationCite and Resume QuantumOutcome Events; justifications.jsonl and quantum_outcomes.jsonl are droppable leases
- provenance_fingerprint mixes log-native outcome and justification Event bytes; governor observes LOG-01..03 with ci HIGH 0
- Retract, Correct, and CorrectInterval target Fact.event_id (minting Event.id); fold-local seq is no longer the operator key
- Justification cites and check_admission key on minting EventId; a fork prefix still admits after the parent retracts that support
- A rebuilt fold that renumbers local seqs still applies the same Retract and JustificationCite Events; governor observes REF-01..03 with ci HIGH 0
- Identical Assert retry under a non-empty delivery key returns the original EventId and does not append; mismatch fails closed as DeliveryKeyConflict
- Claim identity stays distinct from the support EventId; e2e conflict reports partition by stored Fact.polarity without caller TermIds
- Governor observes ING-01..03 named tests; `uv run kutha-gov ci` is 0 HIGH; honeycomb stays Proposed
- open rejects a tampered snapshot via replay_check; persist replaces events.jsonl by same-directory rename; Define ids are name-stable across persist/open
- Fixtures treat 2015/2017/2021 as Gregorian YearCe valid-time; ingested_at is log sequence, not wall-clock
- Governor requires the five DUR/TIME cargo oracles; ci is 0 HIGH; honeycomb stays Proposed
- Fold-internal VT and claim maps so as_of and claim_supported_at skip non-overlapping facts, matching a brute-force live filter
- Untyped CSR lease is built through CsrMaterializer and dropped; from_fold reads the fold live iterator; typed from_fold stays the named spike limit
- Governor observes the three HOT oracles; honeycomb stays Proposed; S06 lease is not cleared

---

## v0.03 Lean context + semantic governor (Shipped: 2026-09-30)

**Phases completed:** 3 phases, 8 plans, 24 tasks

**Key accomplishments:**
- Operator Phase H5 named on STATE, README, ROADMAP dogfood, and existing h4-lease/dogfood needles without thawing freeze or leasing a product slice
- AGENTS.md slimed to 96 durable operating-rule lines; volatile P0/literature/Leapfrog/layout/lease captions relocated with a grep-backed diet ledger
- Proposed semantic-gap review maps F1–F8 to honeycomb verdicts from ADR prose; Later milestones list M012a before M012 with M002 as log durability first
- Dated Proposed clarifications and open questions on verdict-selected honeycomb cells; ADR README points at the semantic-gap review; map stays Proposed
- `kutha-gov selftest` copies the tree once, proves 27 checks by mutation and skip-reasons the other 5, and `ci` labels H5 from `h5_selftest=32/32`
- `rust_test_asserts` proves named `#[test]` bodies assert after stripping comments/strings/chars; every former `fn` needle now uses that kind; honeycomb `capability: named` empty evidence is HIGH
- `cite_equals` / `refs_resolve` / `agents-lean` fail closed on stale lease tokens, dangling ADR/lock ids, and AGENTS.md budget or token regression; ci V=48 runs all 35 checks
- Eight F-gaps ledgered as deferred control-loop claims; Phases 9–11 CHANGELOG is real Process/Trajectory; H5 dogfood is checked without touching crates or `.kutha/STATE.md`

---

## v0.02 Semantic core close (Shipped: 2026-09-30)

**Phases completed:** 5 phases, 11 plans, 32 tasks
**Closeout type:** override_closeout (`--auto`)
**Known verification overrides:** manager reported verification digests stale for Phases 4–8 after later source edits; each phase `*-VERIFICATION.md` still records `status: passed` with named cargo + `kutha-gov ci` evidence. Formal `.planning/v0.02-MILESTONE-AUDIT.md` was not produced before close.

**Key accomplishments:**
- Explicit `Op::CorrectInterval` patch on a wide-VT fact leaves residual `P` at VT 2012 and 2021 and replacement `P-prime` at 2017, sharing `claim_id`, with fail-closed emit and an unchanged whole-version `Correct` fold arm.
- Whole-version `Op::Correct` with a narrower `valid_from`/`valid_to` still invalidates the entire live fact and pushes one replacement — it does not invent VT 2012 or 2021 residuals (CORR-02, D-C1).
- Named CORR-01 and CORR-02 cargo tests are observed by FSM `required`, `m011-partial-correction` needles, and `B-m011-partial-correction`; `uv run kutha-gov ci` is HIGH-free; ADR-013 stays Proposed and the harness lease file is unedited.
- Sidecar `quantum_outcomes.jsonl` makes budgets 0/1/2 durable as Zero/Partial/Full and keeps crash-after-prefix from looking like terminal success until an explicit Resume row.
- Governor observes the two named S05 cargo tests via FSM + `m011-quantum-outcome` / `B-m011-quantum-outcome`; ADR-014 evidence is recorded while the map stays Proposed.
- Parallel `TypedCsrLease` projects one labeled edge per live Fact (labels + support multiplicity) while untyped `CsrLease` object-only dedup and FF5 stay green.
- Governor observes the two named S06 cargo tests via FSM + `m011-typed-csr` / `B-m011-typed-csr`; ADR-040 and ADR-041 evidence is recorded while both maps stay Proposed.
- `Runtime::provenance_fingerprint` hashes Behavior lineage (`id`, `caused_by`, `name`, `rule_version`) so a valid-prior `caused_by` swap or an `r1`→`r2` pin move the digest while `GraphFold::fingerprint` and `replay_check` stay green.
- Governor observes the two named S07 cargo tests via FSM + `m011-provenance` / `B-m011-provenance`; ADR-060 and ADR-011 evidence is recorded while both maps stay Proposed.
- Durable `justifications.jsonl` plus `conflict_report_at` make one S08 fixture distinguish history, evidence, and stale admission at named TT×VT cuts without rewriting fold, eligibility, CSR, or provenance.
- Governor observes the three named S08 cargo tests via FSM + `m011-e2e` / `B-m011-e2e`; ADR-013, ADR-011, ADR-012, and ADR-040 evidence is recorded while all four maps stay Proposed.

**Delivered:** M011 S04–S08 under lease discipline; one e2e fixture separates preserved history, current evidence, and allowed action. Harness Active Slice cleared (`L_delivery=M011-S08-done`). Honeycomb stays Proposed. Freeze holds.

**Git range:** `v0.01..HEAD` (~97 commits; 77 files, +15144/−124) · Timeline: 2026-09-29 → 2026-09-30

---

## v0.01 GSD foundation (Shipped: 2026-09-29)

**Phases completed:** 3 phases, 9 plans, 18 tasks

**Key accomplishments:**
- Workspace cargo offline hard gate green; twelve-row FIT evidence SoT opened with pending cells and VALIDATION Task IDs locked.
- Hard-gate re-run green; twelve-row FIT→fn map in `01-VERIFICATION.md` all pass; SUMMARY trusts that SoT only.
- FIT-01…FIT-05 checked in one D-05 batch after green VERIFICATION; GSD STATE/ROADMAP closed; harness lease untouched.
- Governor ci green (0 HIGH, 0 LOW), cargo smoke green, and Phase 2 evidence SoT opened with pending GOV/PLANE/FREEZE/MAP probe rows plus VALIDATION Task IDs.
- All GOV/PLANE/FREEZE/MAP probe rows in `02-VERIFICATION.md` pass; ci HIGH-free (0 HIGH, 0 LOW); D-10 Trajectory documents green≠Accepted≠L_capability.
- Phase 2 GOV/PLANE/FREEZE/MAP requirements batched after green VERIFICATION and pre-verify ci+cargo; GSD closeout complete; harness lease untouched.
- Governor ci green (0 HIGH, 0 LOW), cargo smoke green, D-L4 Active Slice still None, and Phase 3 evidence SoT opened with pending GOV-03/NEXT-01/NEXT-02 probe rows plus VALIDATION Task IDs.
- All GOV-03 / NEXT-01 / NEXT-02 / D-L4 probe rows in `03-VERIFICATION.md` pass; ci HIGH-free (0 HIGH, 0 LOW); D-10 Trajectory documents green≠Accepted≠L_capability≠lease grant; ROADMAP Overview is verification-only negative proof.
- Phase 3 GOV-03/NEXT requirements batched after green VERIFICATION and pre-verify ci+explain+cargo; GSD closeout complete; harness lease untouched (D-L4 None).

---

## v0.07 — Typed clocks and as-known-on (2026-10-02)

M014 S01–S03 / Phases 31–33. Process tag `v0.07`. F13 closed.
