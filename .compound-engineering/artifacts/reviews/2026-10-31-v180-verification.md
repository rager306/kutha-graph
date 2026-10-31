# v1.80 / M187 independent verification — test honesty

Date: 2026-10-31. Verifier: fresh subagent, no part in the implementation.
Commit under test: `6d8dd90` on `work/main-line`. Plans:
`.planning/phases/275-*/275-01-PLAN.md`,
`.planning/phases/276-*/276-01-PLAN.md`, `276-02-PLAN.md`. Requirements:
`.planning/REQUIREMENTS.md` (TEST-275-01..03, TEST-276-01..03).

## Verdict

| Phase | Requirement | Verdict |
|-------|-------------|---------|
| 275 | TEST-275-01, TEST-275-02, TEST-275-03 | `passed` |
| 276 | TEST-276-01 | `passed` |
| 276 | TEST-276-02 | **`gaps_found`** |
| 276 | TEST-276-03 | `passed` |

Phase 275 `passed`. Phase 276 `gaps_found` on TEST-276-02 only.

## Test counts and exits

- Targeted v1.80 set, 11 binaries, exit 0, **74 passed / 0 failed**:
  m167 1, m175 7, m176 5, m177 5, m178 10, m179 2, m180 13, m182 10,
  m183 5, m184 10, m185 6.
- Two concurrent copies of each scratch-affected binary (m178, m179, m185,
  m176, m177, m184): 12 runs, all exit 0.
- `cargo test --workspace`, run 1: exit **101**; 534 passed / 4 failed over the
  88 targets that ran before cargo stopped. All four failures are
  `m019_dur_a` (`WouldBlock: store writer lock held (kutha.lock)`).
- `cargo test --workspace`, rerun: exit **0**; 837 passed / 0 failed over 185
  targets (183 test binaries + 2 doc-test targets).
- `uv run kutha-gov precommit`: exit 1, two HIGH checks (`cite-lease`,
  `state-readme`); every other check `high=0`, `low=0`.

## What was verified, by mutation

Each mutation was applied to a backup copy (`tmp/v180-verify/backups/`), run,
and reverted; every restored file hashes equal to its pre-mutation sha256, and
`git status --porcelain` shows no tracked modification.

| Requirement | Mutation | Expected | Observed |
|---|---|---|---|
| TEST-275-01 | old test body: empty `source_event_ids` | red, wrong reason | exit 101 both tests; error reason `"replay: enrichment source_event_ids is empty"` |
| TEST-275-02 | drop lifecycle state guard | second activate/retire red | 101 / 101; wrong-version stays 0 |
| TEST-275-02 | drop installed-version equality | wrong-version red | 101; the other two stay 0 |
| TEST-275-03 | 2017-03 fixture := 2017-01 (lost) | equality assert red | 101 at line 148 `assert_ne!` |
| TEST-275-03 | 2017-01 fixture gains text | equality assert red | 101 at line 147 `assert_eq!` |
| TEST-275-03 | runtime stores 2017-01 as third edition | AS_OF assert red | 101 at line 134 |
| TEST-276-01 | `PersistScratch` uses fixed `tmp/<label>` | concurrent pair collides | round 3: both exit 101 (persist ENOENT; missing sealed segment) |
| TEST-276-03 | `QueryRejected` mapping := `PolicyDenied` | both new tests red | 101 / 101, `got PolicyDenied { .. }` |
| TEST-276-02 | product replay rebuilt per lifecycle event (O(n^2)) | oracle red | **exit 0, 31.54 s** |
| TEST-276-02 | `from_log` made quadratic | oracle red | **exit 0, 10.77 s** |

## Gap: TEST-276-02

The two oracles assert a counter, but the counter is taken from a standalone
`PackLifecycleTracker::from_log` call and increments once per `replay()`.
`large.replay_calls() == 4 * small.replay_calls()` therefore reduces to
`3200 == 4 x 800` on the input lengths and is true for any one-pass
implementation. The product path (`Runtime::replay_check` ->
`check_meta_replay`) is only guarded by a 60 s wall-clock assertion. The
product-path mutation reverts `check_meta_replay` to the documented pre-fix
O(n^2) shape and the oracle stays green; the second mutation makes `from_log`
itself quadratic and the oracle stays green. An O(n^2) replay regression would therefore pass the "structural" gate.

The checklist item "the two scaling oracles contain no wall-clock assertion"
also fails literally: both retain `start.elapsed() < Duration::from_secs(60)`
(m182:61-64, m183:214-218). m182's guard is allowed by plan 276-01 task 3;
m183's is not named by its plan.

Fix direction (not applied): assert the count on the tracker that
`check_meta_replay` actually uses, and remove or strictly bound the wall-clock
guards.

## Pre-existing workspace flake: m019_dur_a (not a v1.80 defect)

The first workspace run aborted on `m019_dur_a` with four
`WouldBlock: store writer lock held (kutha.lock)` panics
(`jsonl_inode_changes_on_persist_rename`,
`mixed_typed_dict_partial_group_is_visible`,
`migrate_tail_guard_rejects_same_length_last_frame_swap`,
`active_truncate_at_exact_frame_end_keeps_prefix`). The v1.80 diff does not
touch `m019_dur_a`, `store.rs` or `segment.rs`, and every failing test uses a
unique `PersistScratch` directory.

Reproduced outside the workspace run: `cargo test -p kutha-runtime --test
m019_dur_a` failed in 2 of 10 runs (18/19 each time, always
`migrate_tail_guard...` at `store::migrate`), and the named test passes 15/15
when run alone. The flake is already recorded in
`.compound-engineering/artifacts/research/2026-10-31-test-health-audit.md`
(medium severity, "needs instrumentation"). Run 1 also overlapped a
full-filesystem `find` started by the verifier; the 2/10 reproduction ran after
it finished, so load is not the only factor. A green workspace rerun exists
(837 passed / 0 failed), but the close gate can land on the red side until the
flake is instrumented.

## Static checks

- `rg --color=never -n '"tmp/' crates/` -> exit 1 (no match) over 235 `.rs`
  files; no `PathBuf::from("tmp` / `format!("tmp` match either.
- 38 `.join("tmp")` sites, all with a unique suffix; 13 converted call sites use
  `common::PersistScratch`; governor `fixed-tmp-persist-dir` is `OK`.
- The two scaling oracles each contain exactly one `Instant::now` /
  `elapsed()` / `Duration::from_secs(60)` assertion.

## Harness gate

`uv run kutha-gov precommit` exit 1:

- `[HIGH] cite-lease` — README.md cites `M186-S01-closed`, `.kutha/STATE.md`
  leases `M187-S01`.
- `[HIGH] state-readme` — Active Milestone M187 is not present in README.md.

Both mean the README was not updated when the M187 lease opened. `README.md`
is a shared file owned by the parent/integrator, so this was not edited here.
It must be fixed before the close gate (`kutha-gov ci --jobs 1`, `run_high=0`).

## Housekeeping

- Verifier scratch: `tmp/v180-verify/` (mutant backups, logs).
- While removing fixed-directory mutant scratch dirs, an over-broad
  `rm -rf tmp/m178-*` also deleted five pre-existing gitignored scratch logs
  (`tmp/m178-ci.log`, `tmp/m178-final-ci.log`, `tmp/m178-test.log`,
  `tmp/m178-test2.log`, `tmp/m178-test3.log`). No tracked file references them;
  they were one-shot logs, not evidence artifacts. Recorded here for honesty.
- `docs/review/` is untracked and predates this verification; it was not
  created or modified by the verifier.
