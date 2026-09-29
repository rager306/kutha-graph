# Project Retrospective

*A living document updated after each milestone. Lessons feed forward into future planning.*

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

### Cumulative Quality

| Milestone | Fitness gates | Integration | Audit |
|-----------|---------------|-------------|-------|
| v0.01 | 12/12 FIT fns + ci 0 HIGH | 20/20 WIRED | passed 15/15 |

### Top Lessons (Verified Across Milestones)

1. GSD STATE ≠ harness STATE (plane split)
2. Batch REQUIREMENTS only after VERIFICATION passed
