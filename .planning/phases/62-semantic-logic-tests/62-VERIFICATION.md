---
independent: true
verifier_model: cursor-grok-4.6-xhigh
status: passed-with-findings
score: 11/11
exit: 0
commands:
  - {cmd: "git log da655d8..HEAD", exit: 0}
  - {cmd: "cargo fmt --all --check", exit: 0}
  - {cmd: "cargo clippy --workspace --all-targets --offline", exit: 0}
  - {cmd: "cargo test --workspace --offline", exit: 0}
  - {cmd: "uv run kutha-gov precommit --check test-matrix", exit: 0}
  - {cmd: "uv run kutha-gov precommit", exit: 0}
  - {cmd: "uv run pytest", exit: 0}
read_only_inputs:
  - crates/kutha-runtime/tests/common/sem.rs
  - crates/kutha-runtime/tests/m019_sem_diff.rs
  - crates/kutha-runtime/tests/m019_sem_model.rs
  - crates/kutha-runtime/tests/m019_sem_meta.rs
  - crates/kutha-runtime/src/fold.rs
  - crates/kutha-runtime/src/hydrate.rs
  - crates/kutha-runtime/src/csr.rs
  - crates/kutha-runtime/src/segment.rs
  - crates/kutha-common/src/intern.rs
  - docs/architecture/test-matrix.yaml
  - docs/architecture/test-matrix.md
  - docs/ADR/ADR-013-bitemporal-facts-invalidation.md
  - docs/ADR/ADR-041-csr-graphblas-hot-path.md
  - docs/ADR/ADR-060-strict-replay.md
  - CHANGELOG.md
---
# 62-VERIFICATION — M019-S07 semantic/logic oracles (independent)

Verifier judged from `git log da655d8..HEAD` (HEAD `d64dab3`), tests, matrix, ADRs, CHANGELOG, and scratch-copy runs. **Did not read** Phase 62 CONTEXT/PLAN/SUMMARY.

**CBM:** `list_projects` → `kutha-graph` (generation `2026-10-02T11:34:54Z`). `check_index_coverage` on cited paths: new tests `not_tracked`; `intern.rs` and the three ADRs `metadata_changed`. `index_repository` was not run. Graph hits used: `GraphFold.live_at`, `TypedCsrLease.from_fold`, `GraphFold.push_fact` / `replay`, `store::encoded_log`, `TermDictionary`. Snippets were stale vs disk; claims below are from source.

**Scratch:** mutations under `tmp/p62v-mut` (`CARGO_TARGET_DIR=tmp/p62v-mut-target`); clippy at `da655d8` under `tmp/p62v-da655d8-target`. Working tree not mutated except this file.

---

## Numbered checks

### 1. Seeded LCG, no new dependency, no wall-clock/sleep/network oracle — **PASS** (note)

- `git diff da655d8..HEAD -- Cargo.toml Cargo.lock crates/*/Cargo.toml` is empty.
- LCG is `wrapping_mul(1664525).wrapping_add(1013904223)` in `crates/kutha-runtime/tests/common/sem.rs` (`Lcg`), cousin of the m017 helper. No proptest/bolero/rand.
- No `thread::sleep`, no `std::net`, no `Instant` in the new tests.
- `SystemTime::now()` is used only to uniquify `tmp/p62-sem/` persist dirs (pid+nanos). It is not an assertion clock. Low note, not a fail.

### 2. ≥64 default seeds, `KUTHA_SEEDS` raises, failures print seed/case/shrunk list — **PASS-WITH-FINDINGS**

- `DEFAULT_SEEDS = 64`; `BASE_SEEDS` has 64 entries; `seed_count()` is `.max(DEFAULT_SEEDS)` so the env cannot lower the floor.
- Documented in module headers, CHANGELOG 2026-10-05 Product entry, and `assert_seed_budget`. Not named in `test-matrix.md`.
- Scratch `KUTHA_SEEDS=256` coverage dump: `n=256` (raises the mix/CSR/model families).
- **Finding:** `persist_seed_count() = seed_count().min(12)` — persist/tamper/export use **12** seeds. `KUTHA_SEEDS=256` does **not** raise those families.
- **Finding:** cascade and Horn-correct characterization are fixtures, not 64-seed loops.
- Mutation `claimed-csr-relation0` stderr (scratch): `SEM fail seed=1 case=11 n=32 ops=Assert,Assert,...`. Seed and case are printed; the list is **op names**, and CSR/`live_at(mid tt)` paths pass the **full** log into `fail_seed` (no shrink). Shrink **does** fire on the as_of model path: `own-bt-retract-noop` printed `shrunk ops=Assert,Retract` / `n=2`. Cascade failures use `assert_eq!` (no seed/case). Replay-idempotence claimed kill panics `live replay_check seed=1: ReplayDivergenceError` (seed, no shrink).

### 3. Independent ADR-013 list-scan model, ≤200 lines, can disagree — **PASS-WITH-FINDINGS**

- Model file: `crates/kutha-runtime/tests/m019_sem_model.rs`. `Naive` / `Row` / `intersect` / `is_live` implement half-open VT, losers stay (`invalidated_at`), list scan — matches ADR-013 D013-1/D013-2 and the 2026-09-13 `as_of(vt) = live_at(MAX, vt)` note. No `GraphFold::apply`, no `Fact::is_live_at`.
- Imports of production: `kutha_runtime::{FoldIndexMode, Runtime}` and `kutha_common::{Event, EventId, Op, TermId}` for ingest + dictionary `kutha:` filter. No `fold.rs` internals.
- It **can** disagree: `own-bt-retract-noop` and `own-bt-ignore-valid-to` kill the oracle (model still retracts / still closes intervals).
- **Finding:** Row+Naive+`intersect` is **208 physical lines** (17–224), **198** non-blank non-comment. Letter of “≤200 lines” misses by comments/`allow` noise.

### 4. No owner-gated decision closed; honeycomb stays Proposed — **PASS**

Matrix after the wave (python count of `test-matrix.yaml`): killed 55, characterization 6, gap 0, blocked 6, spec-only 4, gate 1, total 72.

Still **blocked** (kill_mutation `none`): `SEM-c01-decision` (C01 / FIXG-01), `SEM-horn-kc` (HORN-KC-01), `SEM-asof-elig` (ASOF-ELIG-01), `SEM-det02-ids` (DET-02), `SEM-tr02-legal` (TR-02), `SEM-horn-correct` (HORN-CORR-01). Characterization pins: `SEM-c01-inexpressible`, `SEM-horn-tt-max`, `SEM-horn-correct-char` (HORN-CORR-01 current behaviour, no option chosen).

ADR-013/041/060 status lines remain `**Proposed**`. honeycomb `map: Proposed` for those cells; restage comment only. `git diff da655d8..HEAD -- docs/ADR` does not change Status.

### 5. Existing named oracles not weakened — **PASS**

Changed test paths in `da655d8..HEAD`: `tests/common/mod.rs` (`pub mod sem` only), new `tests/common/sem.rs`, `m019_sem_{diff,model,meta}.rs`. Empty diff on `ff5_legal_pit.rs`, `ff6_allowlist.rs`, `m002_*`, `m010*`–`m017*`, `m019_dur*`, `m019_rocks*`. New `bitemporal_model_agrees_on_ff5_statute` is additive.

### 6. Matrix honesty, kill_mutation, counts, governor green — **PASS**

- New rows: seven `killed` with non-`none` `kill_mutation`; `SEM-horn-correct-char` is `characterization` with a named kill. No `gap` rows left.
- `test-matrix.md` counts match yaml (55/6/0/6/4/1 = 72).
- `uv run kutha-gov precommit --check test-matrix` → `OK test-matrix high=0 low=0`. Full precommit EXIT 0 (trajectory LOW=1, pre-existing).

Claimed `kill_mutation` text vs this verifier’s re-runs: all eight claimed kills **killed** (table below). Persist rebuild oracles on the CSR row are also hit by CSR-cut helpers (`assert_views_stable`), so `TypedCsrLease relation=0` is an honest kill for that row.

### 7. Claimed mutations re-run; two own per family — **PASS-WITH-FINDINGS**

All claimed kills reproduced in `tmp/p62v-mut`. Own survivors are weak-oracle notes, not a fail of the claimed list.

### 8. Runtime ≤30s isolated; 20× flake; `KUTHA_SEEDS=256` — **PASS-WITH-FINDINGS**

Isolated `cargo test -p kutha-runtime --offline --test <bin> -- --test-threads=1` (test time, this host, 2026-10-05; machine also had a long samyama-graph `--release` build):

| Binary | Test time | Wall (`/usr/bin/time`) |
|--------|-----------|------------------------|
| `m019_sem_diff` | 17.42 s | 23.41 s (includes ~5 s compile) |
| `m019_sem_model` | 0.34 s | 3.73 s |
| `m019_sem_meta` | 20.03 s | 175 s (**2m34s compile** under host load) |

Sum of isolated **test** times with `--test-threads=1`: **37.79 s** (over a 30 s sequential reading). Default cargo threads, one `cargo test --test m019_sem_diff --test m019_sem_model --test m019_sem_meta`: iter 1 was **12.72 + 14.17 + 0.14 ≈ 27 s**. Per-binary max under default threads is ~20 s.

- 20× default-seed of those three binaries: all green (`FLAKE20_FAIL=0`).
- `KUTHA_SEEDS=256` once: EXIT 0, wall 17.27 s (persist still capped at 12; extra seeds are cheap mix/CSR/model loops). Confirmed `n=256` on the scratch coverage test.

### 9. Gates — **PASS**

| Gate | Result |
|------|--------|
| `cargo fmt --all --check` | EXIT 0 |
| `cargo clippy --workspace --all-targets --offline` | HEAD EXIT 0, **61** compiler warnings (json). `da655d8` worktree json **63** (extra `len_without_is_empty` on `intern.rs`). Runtime **src** warning codes identical (45). |
| `clippy -p kutha-runtime --lib -- -D warnings` | HEAD: **22** errors, `could not compile ... due to 22 previous errors`. `da655d8` json: **22** in `crates/kutha-runtime/src/` plus 1 `kutha-common` `len_without_is_empty` (so `-D warnings` at `da655d8` dies on common before naming the 22). HEAD adds **none** to the runtime lib set. Claim of “22 pre-existing kutha-runtime lib lints” **holds**. |
| `clippy -p kutha-common --lib -- -D warnings` | `da655d8` EXIT 101 (`len_without_is_empty`); HEAD EXIT 0 (`is_empty` added). |
| `cargo test --workspace --offline` | EXIT 0 |
| rocks (`fsm.yaml` `observe_rocks`) | EXIT 0; required names present (`index_answers_equal_the_log`, `rocks_roundtrips_event_id_to_offset`, `verify_full_rejects_poisoned_middle_locator`, `middle_locator_poison_survives_open_or_rebuild`, `open_or_rebuild_rebuilds_when_sealed_list_binding_differs`) |
| `uv run kutha-gov py` | EXIT 0 `PYTHON_TOOLING_GREEN` |
| `uv run kutha-gov precommit` | EXIT 0 |
| `uv run kutha-gov ci` | EXIT 0; harness 0 HIGH, 1 LOW, 65 checks |
| `uv run kutha-gov selftest` | EXIT 0 |
| `uv run pytest` | **181 passed** in 163 s |

### 10. `TermDictionary::is_empty` trivial; bench gate not required — **PASS**

Diff is four lines: `pub fn is_empty(&self) -> bool { self.to_str.is_empty() }` next to public `len`. No product callers of this method. Clippy `len_without_is_empty` only. Not a fold/CSR/hot-path change.

### 11. CHANGELOG + ADR Evidence — **PASS**

- CHANGELOG H2 `## 2026-10-05 — Product: M019 S07 semantic and logic oracles` with Product + Trajectory; legend needles intact; no SemVer/`[Unreleased]`; `changelog-planes` + `docs-coupling` green. Plane heading `### Product` (bullets) matches neighbouring 2026-10-05 entries more than a nested `### Added`.
- ADR-013/041/060 Evidence 2026-10-05: seeded oracles named; **Proposed**; still open Horn@TT=`MAX` (HORN-KC-01), `as_of` vs eligibility (ASOF-ELIG-01), C01. Trajectory bullet also lists DET-02, TR-02, HORN-CORR-01. Accurate.

---

## Mutation table

Scratch `tmp/p62v-mut`, `CARGO_TARGET_DIR=/root/kutha-graph/tmp/p62v-mut-target`. `killed` = oracle test EXIT ≠ 0.

| Kind | Family | Mutation | Oracle filter | Result |
|------|--------|----------|---------------|--------|
| claimed | SEM-csr-fold-diff | `TypedCsrLease::from_fold` `relation=0` | `csr_fold_diff_behavior_free_agrees` | **killed** |
| claimed | SEM-bitemporal-model | `Fact::is_live_at` skips `ingested_at > tt` | `bitemporal_model_matches_fold_live_at_and_as_of` | **killed** |
| claimed | SEM-cascade-order | Lazy `rule_follow_ons` `.reverse()` before commit | `cascade_order_eager_lazy_inverse_knows` | **killed** |
| claimed | SEM-order-independence | `push_fact` invalidates all prior live facts (global LWW-last) | `order_independence_disjoint_subjects` | **killed** |
| claimed | SEM-replay-idempotence | `GraphFold::replay` skips Correct / CorrectInterval | `replay_idempotence_fresh_runtime_twice` | **killed** |
| claimed | SEM-persist-roundtrip-random | `encoded_log` pops last event | `persist_roundtrip_random_layouts` | **killed** |
| claimed | SEM-replay-tamper-random | Assert `valid_from` hardcoded `2010` | `replay_tamper_flip_swap_drop_never_silently_equal` | **killed** |
| claimed | SEM-horn-correct-char | `rule_follow_ons` Horn-fires on Correct | `horn_correct_char_derived_survives_premise_correct` | **killed** |
| own | SEM-csr-fold-diff | untyped `CsrLease::from_fold` ignores `vt` (`MAX`) | `csr_fold_diff_behavior_free_agrees` | **killed** |
| own | SEM-csr-fold-diff | `persist_log` drops last event of each segment chunk | `csr_fold_diff_persist_rebuild_layouts` | **killed** |
| own | SEM-csr-fold-diff (extra) | `as_known_on` uses first matching anchor (`next` not `next_back`) | `csr_fold_diff_behavior_free_agrees` | **killed** |
| own | SEM-bitemporal-model | `is_live_at` ignores `valid_to` | `bitemporal_model_matches_fold_live_at_and_as_of` | **killed** |
| own | SEM-bitemporal-model | `Retract` is a no-op in `apply` | same | **killed** (`shrunk ops=Assert,Retract`) |
| own | SEM-cascade-order | Lazy `follow.pop()` | cascade fixture | **killed** |
| own | SEM-cascade-order | Eager reverse follow-ons | cascade fixture | **killed** |
| own | SEM-order-independence | keep-first live fact | `order_independence_disjoint_subjects` | **killed** |
| own | SEM-order-independence | Assert `valid_from = facts.len()` | same | **killed** |
| own | SEM-replay-idempotence | `replay` applies each event twice | `replay_idempotence_fresh_runtime_twice` | **killed** |
| own | SEM-replay-idempotence | `from_dict_and_events` drops last event when folding | same | **survived** |
| own | SEM-persist-roundtrip-random | `export_jsonl` swaps first/last event | `persist_roundtrip_random_layouts` | **killed** |
| own | SEM-persist-roundtrip-random | `encoded_log` swaps last two events | same | **killed** |
| own | SEM-replay-tamper-random | `Retract` no-op in `apply` | tamper test | **killed** (swap silent-equal) |
| own | SEM-replay-tamper-random | sealed frame CRC check disabled | tamper test | **survived** |
| own | SEM-horn-correct-char | `retract_derived` on Correct | horn char | **killed** |
| own | SEM-horn-correct-char | Correct skips `invalidate_fact_at` (still pushes replacement) | horn char | **survived** |

**Survivors (weak oracles):**

1. **Hydrate drop-last** — last log event is typically `QuantumOutcome` / `Define` (facts fingerprint unchanged). `replay_idempotence` compares stamps / `replay_check`, not “every event is load-bearing.”
2. **Skip sealed CRC** — this oracle tampers in-memory events / jsonl, not middle-segment bytes (those kills live under DUR/M002 rocks/segment tests).
3. **Correct skip-invalidate** — characterization only asserts “inverse stays live and no extra `inverse_knows`.” Leaving the old fact live as well still satisfies that pin.

---

## Generator coverage (instrumented, scratch, default 64 seeds)

`ingest_mix` / `N_OPS=8`. Cuts: `vt_cuts = [2009, 2012, 2017, 2025]`. “Inside” = strictly between min and max product `valid_from`/`valid_to` on that seed.

| Style | Retract | Correct | CorrectInterval | Non-empty `Behavior` (Horn/inverse) | VT cut inside | TT mid inside |
|-------|---------|---------|-----------------|-------------------------------------|---------------|---------------|
| BehaviorFree | 49/64 (77%) | 47/64 (73%) | 38/64 (59%) | **0/64** | 93/256 (36%) | 61/64 (95%) |
| WithHorn | 49/64 | 47/64 | 38/64 | 50/64 (78%) | 93/256 | 56/64 (88%) |

`KUTHA_SEEDS=256`: Retract 173/256, Correct 166/256, CorrectInterval 162/256, WithHorn Behavior 210/256, VT inside 386/1024 (38%).

**Vacuity:** `2009` is before mix `valid_from` (≥2010) — as_of/live_at at that cut is empty vs empty. Persist/tamper/CSR persist layouts use **BehaviorFree** so they never see derived Horn facts. Mix is not Retract/Correct-free (majority of seeds have both).

---

## Findings by severity

**High:** none.

**Medium**

- Persist/tamper seed cap `min(12)`; `KUTHA_SEEDS` does not raise those families.
- Not every failure path shrinks (CSR, mid-TT model, persist, `replay_check`, cascade `assert_eq`).
- Isolated `--test-threads=1` sum of the three binaries is 37.8 s; default-thread combined run is ~27 s.

**Low**

- Reference model 208 physical lines vs ≤200.
- `SystemTime` in persist dir names.
- One of four VT cuts always outside the generated range; BehaviorFree persist path has 0% Horn derived facts.
- Three own mutations survived (QO tail, segment CRC, Correct-without-invalidate vs characterization pin).
- CHANGELOG uses `### Product` bullets rather than nested `Added` (governor still green).

---

## Verdict

**PASS-WITH-FINDINGS**

Claimed S07 oracles exist, kill the mutations they name, leave owner-gated IDs blocked, keep honeycomb **Proposed**, and pass fmt/clippy-comparison/workspace/rocks/governor/pytest. Findings are seed-cap, shrink gaps, runtime-budget reading, model line count, and three weak own-mutation survivors — not a false `killed` row or a closed C01/HORN-CORR-01 decision.
