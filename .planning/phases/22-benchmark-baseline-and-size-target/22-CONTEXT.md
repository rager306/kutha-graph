# Phase 22 Context: Benchmark baseline and size target

**Milestone:** v0.06 / harness M013 S01
**Requirements:** BASE-01, BASE-02, BASE-03
**Depends on:** nothing
**Source of gaps:** `docs/architecture/semantic-gap-review.md` addendum, R2 (and the 2026-09-30 measured baselines: one N for persist/open)
**Run rules:** `docs/process/gsd-auto-contract.md` (lease first, budget, independent verify)

## Locked decisions

- **D-01:** The harness is `crates/kutha-runtime/benches/baseline.rs` with `[[bench]] harness = false` and std only (`Instant`, `/proc/self/status`). No criterion, no new dependency, no `Cargo.lock` change beyond the bench entry.
- **D-02:** Three sizes N, 2N, 4N, default N = 10,000 asserts (a full run stays under about two minutes); `KUTHA_BENCH_N` overrides. persist and open are measured at all three sizes (the 2026-09-30 numbers were one N).
- **D-03:** Operations: keyed ingest per op, plain ingest per op, `as_of`, `claim_supported_at`, late `Retract`, `persist`, `open`, `replay_check`, plus log events per assert and bytes per fact (RSS delta on Linux, `n/a` elsewhere).
- **D-04:** Output is sorted `key value unit` lines so two runs diff cleanly. Absolute times are noisy; the note compares ratios across N, not wall times across hosts.
- **D-05:** The harness validates answers (live count, fingerprint equality after open). A run whose answers are wrong exits non-zero. A bench that does not check results is not a baseline.
- **D-06:** Target N and memory budget are proposed in `docs/architecture/benchmark-baseline.md` (dated note, not an ADR) as measured bytes per fact times a stated target N; start from 1,000,000 asserts and justify or change it. The controller approves it at verification.
- **D-07:** No product behavior change. Only the bench file, its `Cargo.toml` entry, and the note change.

## Spike anchors (re-locate by symbol; lines as of 2026-10-01)

- `crates/kutha-runtime/src/quantum.rs` — `emit`, `replay_check`, `Runtime::new`, hot-read counters pattern used by `tests/m012a_hot_indexes.rs`
- `crates/kutha-runtime/src/store.rs` — `persist`, `open`
- `crates/kutha-runtime/src/fold.rs` — `as_of`, `claim_supported_at`
- Prior measurement definition: N≈121k log events for ≈40k asserts (log ≈ 3× asserts)

## Tool contract

CBM only (no product edit): `list_projects`, `search_graph` / `get_code_snippet` for the public API the bench drives; `check_index_coverage` on `quantum.rs`, `store.rs`, `fold.rs`. Subagent prompts name `codebase-memory-mcp`.

## Verifier kill-tests (verifier reads requirements and public API only)

- Two runs produce the same key set; diff shows only value changes.
- Doubling N changes per-op ingest cost by a ratio the note reports; `as_of` ratio is reported.
- Corrupt an answer path (for example, drop one assert in the harness copy): the run exits non-zero.
- `git diff --stat` touches only the bench file, its `Cargo.toml` entry, and the note.

## Non-goals

Optimization, Rocks, a benchmark framework, a CI performance gate.

## Harness

Active Milestone M013, Active Slice **S01** while this phase runs (controller leases it; GSD does not edit `.kutha/STATE.md`).
