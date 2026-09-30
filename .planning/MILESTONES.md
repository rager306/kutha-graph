# Milestones

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
