---
independent: true
verifier_model: composer-2.5-fast
status: passed
score: 3/3
commands:
  - cmd: "KUTHA_BENCH_N=20 cargo bench -p kutha-runtime --offline --bench baseline 2>/dev/null (run 1, awk NF==3 keys)"
    exit: 0
  - cmd: "KUTHA_BENCH_N=20 cargo bench -p kutha-runtime --offline --bench baseline 2>/dev/null (run 2, key set diff)"
    exit: 0
  - cmd: "KUTHA_BENCH_SABOTAGE=drop_one KUTHA_BENCH_N=20 cargo bench -p kutha-runtime --offline --bench baseline 2>/dev/null"
    exit: nonzero
  - cmd: "git diff --name-only a85cf5ab89514aaad04ed598d4c82692370ccf94..HEAD"
    exit: 0
read_only_inputs:
  - .planning/REQUIREMENTS.md
  - crates/kutha-runtime/benches/baseline.rs
  - crates/kutha-runtime/Cargo.toml
  - docs/architecture/benchmark-baseline.md
---

# Phase 22: Benchmark baseline — independent verification

**Requirements:** BASE-01, BASE-02, BASE-03 (no PLAN/SUMMARY read).

## Kill-tests

| Check | Result |
|-------|--------|
| Stable metric keys (N=20, two runs) | 27 keys each; identical sorted key set |
| `KUTHA_BENCH_SABOTAGE=drop_one` | Process exit code 1 |
| `benchmark-baseline.md` content | ingest_plain & as_of 2N/N and 4N/N ratios; persist/open n/2n/4n; bytes_per_fact; target N; memory budget; `cargo bench` command present |
| `Cargo.toml` | `[[bench]] name=baseline`, `harness=false`; five workspace deps only (kutha-common, sha2, serde, serde_json, crc32fast) |
| Scope since `a85cf5ab` | Only `benches/baseline.rs`, `Cargo.toml`, `benchmark-baseline.md`, `CHANGELOG.md`, `.planning/phases/22-*` — no `crates/kutha-runtime/src/*` |

## Requirement mapping

- **BASE-01:** `baseline.rs` (std-only, `harness=false`) exercises ingest plain/keyed, `as_of`, claim support, retract, persist, open, replay at n/2n/4n; emits `bytes_per_fact`.
- **BASE-02:** Dated note declares target N `1000000`, memory budget `13674000000`, measured table beside targets.
- **BASE-03:** Documented offline bench command; stdout lines are `key value unit` (27 metrics + `validated yes`).

## Target N / memory budget

Pair is **acceptable** for this dated note: budget equals measured `bytes_per_fact` (13674 B) × target N (1e6); methodology stated (Linux VmRSS delta at default N). Extrapolation is explicit, not an optimization claim.

## CBM

`list_projects`: kutha-graph indexed. `check_index_coverage` on cited paths: no recorded issue; freshness `not_tracked` / `metadata_changed` — evidence from source reads and executed benches, not graph completeness.

## Gaps

None.
