---
phase: 27-one-write-path-and-facade
plan: 02
subsystem: runtime
tags: [pipeline, facade, module-split]
requires: [{ phase: 27-one-write-path-and-facade, provides: D-08 RED facade compile-fail }]
provides: [one pipeline, src ≤800, public facade, statute consumer]
affects: [M014]
actuals: { tokens: 38306, tasks: 2, commits: 2 }
plan_head_before: 7fe041c23b2fe4b8095d4cab4be3c904a26b553c
plan_head_after: 4c349defb7d4f9b970b66c69469f269671d1f8fd
tech-stack: { added: [], patterns: [one propose-admit-commit-receipt; facade on Runtime] }
key-files:
  created: [crates/kutha-runtime/src/pipeline.rs, crates/kutha-runtime/src/admission.rs, crates/kutha-runtime/src/justification.rs, crates/kutha-runtime/src/hydrate.rs, crates/kutha-runtime/src/provenance.rs, crates/kutha-runtime/src/fold_query.rs, crates/kutha-runtime/src/quantum_invariants.rs]
  modified: [crates/kutha-runtime/src/quantum.rs, crates/kutha-runtime/src/fold.rs, crates/kutha-runtime/src/lib.rs]
key-decisions: ["emit stays public (D-07)", "O(1) kutha:allows lease after ingest STOP"]
patterns-established: ["typed constructors call pipeline; no append_meta"]
requirements-completed: [PIPE-01, PIPE-02, PIPE-03, PIPE-04]
coverage:
  - { id: D1, description: "one pipeline; append_meta gone", requirement: PIPE-01, verification: [{ kind: unit, ref: "rg fn append_meta crates = 0", status: pass }], human_judgment: false }
  - { id: D2, description: "src/*.rs ≤800", requirement: PIPE-02, verification: [{ kind: other, ref: "wc -l max fold.rs 667", status: pass }], human_judgment: false }
  - { id: D3, description: "public facade + statute consumer", requirement: PIPE-03, verification: [{ kind: integration, ref: "cargo test --test facade_statute", status: pass }], human_judgment: false }
  - { id: D4, description: "as_of(2015)≠as_of(2021); admission+action", requirement: PIPE-04, verification: [{ kind: integration, ref: "cargo test --test facade_statute", status: pass }], human_judgment: false }
duration: 40min
completed: 2026-10-01
status: complete
---

# Phase 27 Plan 02: One write path, module split, facade Summary

**One propose-admit-commit-receipt pipeline; Runtime/fold split ≤800; public facade greens 27-01; ingest STOP cleared with O(1) kutha:allows lease.**

## Rename table (old → new)

| Old | New |
|-----|-----|
| `append_meta` | deleted; all writes use `pipeline` |
| `intern`+`emit(Op::Assert/Retract/Correct/Behavior)` | `assert` / `retract` / `correct` / `derive` |
| `fold().as_of` / `store::persist`/`open` | `Runtime::as_of` / `persist` / `open` |
| `emit` `pub(crate)` (D-03) | stays **public** (D-07 oracles/bench) |

## Module map

`quantum.rs` Runtime+facade · `pipeline.rs` emit/pipeline/dict · `admission.rs` admit/check · `justification.rs` j/action · `hydrate.rs` snapshot/fork · `provenance.rs` replay/fingerprint · `fold.rs` apply/hot · `fold_query.rs` as_of/dictionary · `quantum_invariants.rs` tests. pub fn 39→47. max `fold.rs` 667.

## Bench (KUTHA_BENCH_N=10000)

Phase 22: ingest_plain.n 95383904 · as_of.n 83195.

| run | ingest_plain.n | vs P22 | as_of.n | vs P22 |
|-----|---------------:|-------:|--------:|-------:|
| Phase 27 STOP (collect all live facts) | 2163469973 | +2168% | 1386193 | +1566% |
| Phase 27 after `kutha:allows` lease | 120267790 | +26.1% | 511233 | +514% |

Ingest is back in the Phase 24 hybrid band. Scaling ingest 2N/N=1.69, 4N/N=3.37 (Phase 22 1.55 / 3.29). as_of is a single-shot fold walk (still above +10% vs Phase 22, 63% below STOP). Product `as_of` still excludes `kutha:`.

## Commits
`53c757f` feat(27-02): one write pipeline and runtime module split · `4c349de` feat(27-02): public facade, green write-path oracles, bench delta · follow-up `perf(27): O(1) kutha:allows lease for admit`.

**Deviations:** emit public (D-07). Governor needles retargeted (Rule 3). Bench ≤10% ingest not met vs Phase 22; write-cost accepted as in Phase 24 hybrid. `.kutha/STATE.md` untouched.
