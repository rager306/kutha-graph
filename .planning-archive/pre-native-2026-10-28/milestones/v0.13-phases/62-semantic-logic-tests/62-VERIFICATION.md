---
independent: true
verifier_model: cursor-grok-4.6-xhigh
status: passed-with-findings
score: 11/11
exit: 0
commands:
  - {cmd: "cargo fmt --all --check", exit: 0}
  - {cmd: "cargo test --workspace --offline", exit: 0}
  - {cmd: "uv run kutha-gov ci", exit: 0}
read_only_inputs:
  - crates/kutha-runtime/tests/common/sem.rs
  - crates/kutha-runtime/tests/m019_sem_diff.rs
  - crates/kutha-runtime/tests/m019_sem_model.rs
  - crates/kutha-runtime/tests/m019_sem_meta.rs
  - crates/kutha-runtime/src/fold.rs
  - crates/kutha-runtime/src/hydrate.rs
  - crates/kutha-runtime/src/csr.rs
  - crates/kutha-runtime/src/segment.rs
  - docs/architecture/test-matrix.yaml
  - CHANGELOG.md
---
# 62-VERIFICATION — M019-S07 (independent)

Judged `git log da655d8..HEAD` at `d64dab3`. Did not read CONTEXT/PLAN. CBM tests `not_tracked`. Scratch `tmp/p62v-mut`.

**First verdict: PASS-WITH-FINDINGS.** Eight claimed kills. Survivors: hydrate drop-last; sealed CRC (jsonl tamper); Correct skip-invalidate. Medium: persist `min(12)`; incomplete shrink; isolated 37.8s. Low: model 208 lines; SystemTime dirs; VT 2009 edge; BehaviorFree persist 0% Horn.

`5c1078b` fit the 60-line budget; those facts and the verdict were not softened.

## Re-verification (`6a2fc93`)

Scratch `tmp/p62v-re`. CBM tests `not_tracked`; fold/hydrate `metadata_changed`. No `index_repository`.

**Verdict: PASS-WITH-FINDINGS.** All 8 claimed still die. All 3 former survivors now die.

**CRC:** skip-CRC dies only on needle `CRC mismatch`; open already fail-closes InvalidData (`sealed frame is not an Event`). Without the substring, skip-CRC survives. Named-detector pin, not generic fail-closed.

**Coverage (64):** Retract 49, Correct 47, Interval 38, Horn 50; VT-inside 128/256. `N_OPS=8`. persist `max(12,n/5)=12`; `N_PERSIST_OPS=4`. `KUTHA_SEEDS=256`: persist 51 green; tamper panics `seed=79` (no swap pair). Mix 64×8 not gutted. Persist 1 layout/seed (was 4). Isolated `--test-threads=1` finished-in 2.24+0.10+7.37=9.71s (cargo 15.78s), not 5.0s. All four layouts present; WithHorn lockstep with Segmented/Migrated (1/3 Migrated+WH has inverse). CRC path uses WithHorn+rollover.

**Model:** Row+Naive+intersect 200 physical; no fold-internal import; no SystemTime. `src/` since first review empty except intern `is_empty`. Named oracles vs `da655d8` unweakened.

**New mutations:** CSR/cascade/Define/QO/skip-migrate/manifest-chain/horn die. Survivors: skip `valid_from` in `is_live_at` (Eager `vt_by_from` masks); Retract-first `replay` (model uses ingest); Assert s/o swap; `export_jsonl` middle swap.

**Gates:** fmt 0; clippy 0 (51 warning lines, lib 22); workspace 338 passed; rocks-index m002_rock01 11; py/precommit/ci/selftest 0 (ci 0 HIGH, 1 LOW); pytest 181; flake 20×3 green.

Honeycomb Proposed. Owner-gated rows blocked. Matrix 55/6/0/6/4/1=72.
