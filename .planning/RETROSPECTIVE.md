# Project Retrospective

*A living document updated after each milestone. Lessons feed forward into future planning.*

## Milestone: v0.02 — Semantic core close

**Shipped:** 2026-09-30
**Phases:** 5 | **Plans:** 11 | **Tasks:** 32

### What Was Built
- S04 `CorrectInterval` residuals + CORR-02 whole-version Correct unchanged; GATE-01 partial-correction needles
- S05 `quantum_outcomes.jsonl` Zero/Partial/Full + explicit Resume after crash prefix
- S06 `TypedCsrLease` labels + support multiplicity; untyped CSR + FF5 remain
- S07 `provenance_fingerprint` for caused_by / rule_version without state-fingerprint drift
- S08 `justifications.jsonl` + `conflict_report_at` + three FIX e2e oracles; Active Slice cleared

### What Worked
- One GSD phase per harness slice with GATE-02 lease discipline before execute
- Compose-don't-rewrite for S08 (fold/CSR/provenance untouched)
- Honeycomb evidence append while map stays Proposed (GATE-03)

### What Was Inefficient
- Verification digests went stale across Phases 4–8 after later edits; manager blocked verified_closeout
- No formal `/gsd-audit-milestone` before archive (override_closeout under `--auto`)
- CBM index lagged new symbols during Phase 8 verify (source Read fallback)

### Patterns Established
- Tracer product plan then GATE-01 dictionary registration wave
- Lease close only after verify: Active Slice None + `L_delivery=M011-S0N-done`
- docs-coupling: Process CHANGELOG with dictionary registration in the same commit family

### Key Lessons
1. Re-run phase verification before milestone complete if covered sources changed after VERIFICATION.md
2. Formal milestone audit is still worth a cheap pass even when all REQUIREMENTS are Complete
3. Closing GSD v0.02 archives the overlay — harness freeze and honeycomb Proposed still bind delivery

### Cost Observations
- Model mix: adaptive GSD profile (not metered here)
- Sessions: ~1.5 calendar days for Phases 4–8
- Notable: short plans; cargo named tests + `kutha-gov ci` dominated wall clock

---

## Milestone: v0.01 — GSD foundation

**Shipped:** 2026-09-29
**Phases:** 3 | **Plans:** 9 | **Sessions:** 1 day overlay

### What Was Built
- FIT-01…05 verification overlay: cargo hard gate + twelve-row evidence map; REQUIREMENTS batched after VERIFICATION green
- GOV/PLANE/FREEZE/MAP probes with `kutha-gov ci` honesty (green≠Accepted≠L_capability)
- GOV-03/NEXT lease-gate negative proof under Active Slice None (no crate slice, no legal pack)

### What Worked
- Verification-first phases over greenfield delivery — fitness already in crates
- Three-source REQUIREMENTS batching (VERIFICATION → SUMMARY → checkboxes) prevented premature `[x]`
- Cite-only harness lease kept GSD STATE from overwriting `.kutha/STATE.md`

### What Was Inefficient
- Phase placeholders (TBD) in ROADMAP/STATE until later waves created noise for verifiers
- NEXT-02 `[x]` while M011 open is correct D-L5 but easy to misread at milestone close
- Milestone version renamed mid-flight (audit as v1.0 → ship as v0.01)

### Patterns Established
- Tracer → paint probes → batch REQUIREMENTS after VERIFICATION passed
- Phase 2+ governor cycle (D-G1…G3) + D-10 Trajectory in every SUMMARY
- Phase 3 verification-only close when Active Slice is None (D-L1…D-L6)

### Key Lessons
1. Closing a GSD overlay does not close harness M011 — say so in ROADMAP/MILESTONES every time
2. Negative proof is a valid phase goal; do not hold Phase 3 open waiting for a lease
3. Annotated git tags for GSD milestones conflict with Kutha Out of Scope — require explicit override

### Cost Observations
- Model mix: adaptive GSD profile (not metered here)
- Sessions: single calendar day for Phases 1–3
- Notable: plans were short (verification/docs); cargo+ci gates dominated wall clock

---

## Cross-Milestone Trends

### Process Evolution

| Milestone | Sessions | Phases | Key Change |
|-----------|----------|--------|------------|
| v0.01 | 1 day | 3 | First GSD overlay; verification-first + lease-gate negative proof |
| v0.02 | ~1.5 days | 5 | Lease-gated product slices S04–S08; compose e2e fixture; override closeout |

### Cumulative Quality

| Milestone | Fitness gates | Integration | Audit |
|-----------|---------------|-------------|-------|
| v0.01 | 12/12 FIT fns + ci 0 HIGH | 20/20 WIRED | passed 15/15 |
| v0.02 | CORR/OUT/CSR/PROV/FIX named tests + ci 0 HIGH | GATE-01 needles per slice | skipped (override) |

### Top Lessons (Verified Across Milestones)

1. GSD STATE ≠ harness STATE (plane split)
2. Batch REQUIREMENTS only after VERIFICATION passed
3. Refresh verification digests before milestone complete; formal audit still useful
4. Freeze + honeycomb Proposed survive green governor and GSD archive
