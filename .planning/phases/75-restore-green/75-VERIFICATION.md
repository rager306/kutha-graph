---
phase: 75-restore-green
independent: true
verifier_model: composer-2.5-fast
status: passed
commands:
  - { cmd: "cargo test -p kutha-runtime --offline --test m013_write_path --test m022_keyed_retry --test m019_sem_model --test m019_dur_a -- --test-threads=8", exit: 0 }
  - { cmd: "cargo test --workspace --offline --no-fail-fast", exit: 0 }
read_only_inputs:
  - .planning/phases/75-restore-green/75-CONTEXT.md
  - .planning/REQUIREMENTS.md
  - crates/kutha-runtime/tests/m019_dur_a.rs
  - crates/kutha-runtime/src/admission.rs
  - crates/kutha-runtime/src/fold_query.rs
  - crates/kutha-runtime/tests/m013_write_path.rs
  - crates/kutha-runtime/tests/m022_keyed_retry.rs
  - crates/kutha-runtime/tests/m019_sem_model.rs
  - docs/architecture/test-matrix.yaml
  - CHANGELOG.md
---

## Verdict

**Passed.** Re-verification after reopen: both required commands exited **0**. Prior failure (`m019_dur_a` lock `WouldBlock` under parallel dirs) addressed by `persist_dir` PID + monotonic `AtomicU64` seq (post-failed-VERIFICATION commit).

## Kill-tests

| Suite | Result |
|-------|--------|
| `m013_write_path` (7, keyed-correct pair) | ok |
| `m019_dur_a` (19, `--test-threads=8`) | ok |
| `m019_sem_model` (5, incl. naive CI retract) | ok |
| `m022_keyed_retry` (4, CI lineage oracles) | ok |
| Workspace `--no-fail-fast` | ok |

## Static (rg)

- `persist_dir`: `static N: AtomicU64`, `fetch_add(1, Relaxed)` → path `p60-store-{label}-{pid}-{nanos}-{seq}`.
- `delivery_key_retry` → `claim_id` + `has_live_keyed_support` (`admission.rs` / `fold_query.rs`).
- Naive `Retract` invalidates all live rows for `event_id` (`m019_sem_model.rs`).

## Requirements (GREEN-01..03)

- **GREEN-01:** Keyed-correct pair + `m022_keyed_retry` green; CI retract remint oracle present.
- **GREEN-02:** `naive_retract_clears_all_correct_interval_residuals` + property tests green.
- **GREEN-03:** `m019_dur_a` stable at 8 threads; full workspace run zero failures (single run this pass).

## Test matrix

`SEM-keyed-retry-live` and `SEM-bitemporal-model` oracles include CI/retract keyed tests and `naive_retract_clears_all_correct_interval_residuals` (unchanged; spot-checked via `rg`).

## Note

Did not read `75-PLAN.md` / `75-SUMMARY.md`. Three consecutive workspace runs not re-run (scope: one run per reopen instructions).
