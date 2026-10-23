---
phase: 231-hop-workload
independent: true
status: pass-with-findings
verifier_model: Cursor Grok 4.6
commands:
  - cmd: cargo test -p kutha-runtime --test b5_hop_workload_kill --test b5_hop_prepared_kill --offline
    exit: 0
  - cmd: cargo test --workspace --offline --no-fail-fast
    exit: 0
  - cmd: uv run kutha-gov ci --jobs 1
    exit: 0
read_only_inputs:
  - crates/kutha-runtime/src/query_ir.rs
  - crates/kutha-runtime/src/lib.rs
  - crates/kutha-runtime/src/csr.rs
  - crates/kutha-runtime/src/store.rs
  - crates/kutha-runtime/benches/hop_k.rs
  - crates/kutha-runtime/tests/m019_dur_a.rs
  - docs/ADR/ADR-040-materialization-plugin-protocol.md
  - docs/ADR/ADR-041-csr-graphblas-hot-path.md
---

# Phase 231 — independent verification

**First round (41295c2): PASS-WITH-FINDINGS.** Scan/Hops vs `live_at` oracle, clocks, fail-closed intern, Horn/CI. MED: `execute(Hops)` rebuilt CSR per call (`hop_k` n×CSR). LOW: set vs typed multiplicity.

## Re-run after fix round (325459f)

**Verdict: PASS-WITH-FINDINGS.** Old kill file untouched, 14/14 green. New `b5_hop_prepared_kill.rs` 8/8: Prepared frozen after Assert/Retract; live `execute` sees the new fold; Scan snapshot = `execute` at the cut; `run_from` = per-start `execute` = `live_at` BFS; unknown/`TermId::MAX`/post-prepare intern fail closed; interned-no-edges → empty; prepared `limit` is a sorted prefix; `RelsShape` at `prepare` not `run`. No `crates/*/src` edit. No bench run.

### Findings

1. **LOW** `m019_dur_a`: WouldBlock oracles still assert kind + `kutha.lock` name. File-local `Mutex` + `PersistScratch` are test-only; `store.rs` still per-dir `try_lock`. Mutex does not hide a production race (two writers on one dir still RED). It does not serialize other files' `set_test_segment_limits` (process-global test hook, pre-existing).
2. Prior MED n×CSR rebuild is addressed by `prepare` + `run_from` (not re-measured here).

## Commands

- `cargo test -p kutha-runtime --test b5_hop_workload_kill --test b5_hop_prepared_kill --offline` **0** (14+8)
- `cargo test --workspace --offline --no-fail-fast` **0** (`m019_dur_a` did not flake)
- `uv run kutha-gov ci --jobs 1` **0**, `run_high=0`
