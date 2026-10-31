# Test-health audit: environment dependence, leftovers, and the dogfood/tmp interaction

Task G, 2026-10-31. Read-only audit of `/root/kutha-graph` (branch `work/main-line`).

- Contract followed: no tracked file edited, no commit, no `graph index`, no workspace-wide `cargo` run. Only the report and `tmp/task-g-test-health/` (gitignored scratch) were created.
- **Commit note:** the task said HEAD `2680cd9`; the checkout is at **`8acee31`** (branch `work/main-line`, 2026-10-31 17:19 +0700). `2680cd9` is an ancestor (`docs(agents): record the open-design provider`).
- No lease exists (`Active Milestone=None`). Nothing below needs a product change; every recommendation is test-side or harness-check-side and would need its own lease before landing.
- Targeted Rust test runs used the **already-built** test binary `target/debug/deps/m019_dur_a-d8c468ca3f7fb299` (mtime 2026-10-31 14:14). No `crates/*.rs` file is newer than it, so it is current with the sources; this avoids a rebuild and is equivalent to `cargo test --test m019_dur_a`.

## 1. The `m019_dur_a` lock flake: not reproduced

### Runs

| # | command | exit | result | log |
|---|---------|------|--------|-----|
| 1 | `./target/debug/deps/m019_dur_a-d8c468ca3f7fb299` (default 12 threads) | **0** | 19 passed, 0 failed, 1.06 s | `tmp/task-g-test-health/run-default.log` |
| 2 | `... --test-threads=1` | **0** | 19 passed, 0 failed, 2.00 s | `tmp/task-g-test-health/run-serial.log` |
| 3 | stress: 24 runs, 6 concurrent, default threads | **0** (all 24) | 8.1 s wall total, 0 failures | `tmp/task-g-test-health/stress.py` |

So on an idle 12-core host the binary is green both in parallel and serially, and 24 concurrent whole-binary runs did not produce a failure. **The flake did not reproduce.** It is a recorded, low-frequency event:

- `CHANGELOG.md:280` - one full workspace run failed `jsonl_inode_changes_on_persist_rename` once with `WouldBlock` on `kutha.lock` (1 of 4 full runs); not reproduced in 3 isolated, 6 whole-binary and 3 further full runs.
- `CHANGELOG.md:744` - earlier `persist_migrate_race_serialized_or_fails_closed` WouldBlock; "tests now use `PersistScratch` ... a file-local `Mutex` serializes lock oracles".
- `.planning/milestones/v1.75-phases/269-meta-surface-threat-audit-and-adversarial-tests/269-VERIFICATION.md:36` - "One parallel `m019_dur_a` lock flake reproduced only under concurrent workspace runs and passes serially."
- `.compound-engineering/artifacts/research/2026-10-08-governor-gsd-hardening-notes.md:8` - hypothesised "leftover `kutha.lock` / orphaned hold-lock children from parallel runs"; serial and clean-`tmp` runs were green.

### Timing / lock dependence found in source

1. **The lock is a non-blocking `flock`.** `store.rs:64` `acquire_store_write_lock` (`File::try_lock`, `WouldBlock` mapped to `ErrorKind::WouldBlock` with a message naming `kutha.lock`, `store.rs:37`). Every writer entry point takes it once: `persist` -> `persist_until` (`store.rs:155`, `:840`), `migrate_until` (`:722`), `export_jsonl` (`:813`), `checkpoint_sealed_prefix` (`:178`), `migrate_pin_wire` (`:408`), `migrate_pin_cite` (`:502`), `open_for_append_until` (`:1154`). I found **no nesting** that would take the lock twice on one path: `open_for_append_until` reuses the held `File` via `Persister::attach_holding(..., Some(lock))` (`store.rs:1166`), and `persist_unlocked` is the lock-free body. A regression there would be deterministic, not a flake.
2. **Scratch dirs are unique per process**, so a WouldBlock inside a test's own dir cannot come from a sibling test: `common/mod.rs:20-30` names the dir `{label}-{nanos}-{pid}` under repo `tmp/`, and `m019_dur_a.rs:61-67` adds a per-file `AtomicU64` sequence. Two concurrent *cargo* runs have different pids, so name collision is not the plausible route either.
3. **`exclusive_lock()` cannot help the concurrent-run scenario.** `m019_dur_a.rs:18-25` is a process-local `Mutex`. Its doc comment says it serialises tests that set "process-global segment limits", but the segment limits are **thread-local** (`segment.rs:39-56`, `effective_max_bytes` at `:57`). The comment is stale, and the mutex is irrelevant across processes - exactly the setting in which the flake was seen. Note also that `jsonl_inode_changes_on_persist_rename` (`m019_dur_a.rs:231`) never takes the mutex.
4. **The binary spawns itself.** `dead_process_stale_lock_does_not_block` (`m019_dur_a.rs:427`) re-executes `current_exe` (`:449`) and spins until a ready file appears (`:456-461`, no timeout) before `child.kill()` (`:463`). Two timing hazards follow: an `ETXTBSY`-class spawn failure if another cargo process rewrites the same `target/` binary during a concurrent build, and - if the *parent* test binary is SIGKILLed (cargo timeout, operator `kill`) - the child survives as an orphan parked in `thread::park()` while holding `flock` in its scratch dir. That orphan is the residual hazard named in the 2026-10-08 notes. I confirmed the corresponding homemade variant: three SIGKILLed binary runs left five scratch dirs behind (sect. 2).
5. **Concrete cross-run dir sharing exists elsewhere in the suite**, and it does produce `WouldBlock`/`ENOENT` under concurrent runs: `m178_enrichment.rs:199,223`, `m179_gate_gc.rs:102,127` and `m185_enrichment_admission.rs:242` use **fixed relative** paths (`tmp/m178-enrichment-persist`, `tmp/m179-gate-gc`, `tmp/m185-checkpoint-source`, ...) and `std::fs::remove_dir_all` them at test start and end. A second cargo process on the same checkout therefore deletes the peer's store mid-write. This is the same failure class the changelog recorded for the old fixed `tmp/p32-02-store` (`CHANGELOG.md:1438`). It cannot explain the one recorded `m019_dur_a` failure (different directories), but it is a real, reproducible-by-inspection parallel-workspace lock flake that should not be left in place.
6. **Host pressure is real:** root fs is ext4 at 95% used (14.7 GiB free of 242 GiB) and the governance `disk-free` check requires `min_free_gib: 2` (`.kutha/dictionaries/checks.yaml:1491-1495`); `CHANGELOG`/handoff m148 record `observe_cargo` exit-101 flakes under disk pressure.

**What would settle it:** capture the failing run's scratch dir name and whether the process survived. The cheapest instrument is inside `assert_writer_lock_error` (`m019_dur_a.rs:109`): on failure, print the dir, `ls -la` of the dir and any `kutha.lock` holder. Without that, any "fix" is guesswork.

## 2. Leftovers under the repo's `tmp/`

### Who roots temp dirs in the repo

| helper | file:line | root | cleanup |
|---|---|---|---|
| `PersistScratch` (Rust, used by 19 test files) | `crates/kutha-runtime/tests/common/mod.rs:20-30` | `CARGO_MANIFEST_DIR/../..//tmp` | `Drop` (`:54-57`) - unwinds on panic, **not** on SIGKILL/SIGTERM |
| local `tmp_dir` helpers | `m002_snap01.rs:22-23`, `m017_replay_integrity.rs:23-24` (label+nanos, **no pid**), `m029_rollover_heal.rs:29-34`, `m031_wal_tail_heal.rs:17-22`, `m052_lock_before_heal.rs:31-36`, `m054_id_structure.rs:16-21`, `m054_scaling_bench.rs:36-50`, `m056_persist_heal.rs:31-36` | repo `tmp/` | `remove_dir_all` at start; most have no `Drop` -> left on kill |
| fixed relative dirs | `m178_enrichment.rs:199,223`, `m179_gate_gc.rs:102,127`, `m185_enrichment_admission.rs:242` | `./tmp/<fixed name>` (CWD-relative!) | `remove_dir_all` at start and end |
| pytest global tempdir | `scripts/tests/conftest.py:8-10` (`tempfile.tempdir = <repo>/tmp`) | repo `tmp/` for **all** pytest `tempfile` use | none by default |
| governor selftest | `scripts/kutha_gov/selftest.py:161,178,195` (`dir=scratch`, `SCRATCH_DIR = "tmp"` at `:79`) | repo `tmp/` | `TemporaryDirectory` / `rmtree` in `finally` (killed run leaks) |
| system-temp tests | `m012a_log_native.rs`, `m012a_hot_indexes.rs`, `m068`, `m075`, `m078`, `m091`, `m092`, `m106`, `wal.rs:262`, `quantum_invariants.rs:236` | `std::env::temp_dir()` | leftovers land outside the repo (not a dogfood hazard) |

### Evidence: a passing test still leaves directories

- `tmp/` before: 392 entries (543 MB). After two whole-binary runs: no new entries (Rust `Drop` works on success).
- `uv run pytest -q scripts/tests/test_m112_selftest_substring.py` -> **2 passed in 0.07 s**, and **2 new dirs**: `tmp/kutha-m112-eu8poh9i`, `tmp/kutha-m112-uhokwv1n`. `test_m112_selftest_substring.py:16` calls `tempfile.mkdtemp(prefix="kutha-m112-")` with no cleanup.
- SIGKILL check: 3 runs of the `m019_dur_a` binary killed at 250 ms (`timeout -s KILL`, exit 137) left 5 dirs (`p60-m019-final-sync-fp-8-...`, `p60-m019-panic-lock-11-...`, `p60-m019-pre-rename-7-...`, ...). Confirmed: **the RAII helper cleans on unwind, not on kill.**
- Pre-existing leftovers measured in `tmp/` (before my runs): 35 `kutha-m1*` (m112/m113/m114 pytest helpers, oldest 2026-10-28), 8 `kutha-fold-*` + 8 `kutha-fsm-fold-*` (`test_fsm.py:468,494`, no cleanup), 7 `kutha-jobs-mini-*` (atexit cleanup, so these were killed runs), 1 `kutha-ci-stub-*`, 34 `b3-kill*` dirs (Rust checkpoint kill tests; the *test* was killed) - plus ~250 agent-written logs.
- I removed only the 7 directories my own experiments created.

### The 2026-10-31 incident, reproduced end-to-end

`tmp/kutha-selftest-*/tree/kutha-gov-selftest.egg-info` is created on purpose by the selftest mutation for `glob_none` (`scripts/kutha_gov/kinds.py:2070-2077`, target `kutha-gov-selftest.egg-info/PKG-INFO`) inside a throw-away copy of the tree. A killed pytest run leaves the copy behind; the live `dogfood` check then sees it.

Reproduced (probe dir created, then removed):

- `uv run kutha-gov json --check dogfood` with no probe: `"passed": true`, `high: 0`.
- With `tmp/kutha-selftest-probe/tree/kutha-gov-selftest.egg-info/PKG-INFO`: `"passed": false`, `high: 1`, finding `[high] tmp/kutha-selftest-probe/tree/kutha-gov-selftest.egg-info/PKG-INFO (dogfood/build-artifact) setuptools egg-info must not be in the tree`.
- `uv run pytest -q scripts/tests/test_kutha_gov.py::HarnessTests::test_no_high_on_live_tree` with the same probe: **FAILED** at `scripts/tests/test_kutha_gov.py:130` with `dogfood: [Finding(... build-artifact ... 'tmp/task-g-test-health/probe/tree/kutha-gov-selftest.egg-info/PKG-INFO')]` (whole sweep, 0.94 s; nothing else failed, so `dogfood` was the only affected check). The clean-tree run of this node was not repeated; the check-level baseline on a clean tree is green (`kutha-gov json --check dogfood`, above).

The second harness test is identified from source; its empirical confirmation was **not** obtained. `scripts/tests/test_fsm.py:41` `test_ci_quantum_scheduling_reaches_ok_with_skips` asserts `assertEqual(0, outcome.high)` at `test_fsm.py:77`, and `run_quantum` sums every check's HIGH count into `outcome.high` (`scripts/kutha_gov/fsm.py:310`), `dogfood` included. No budget truncation can hide the check: the FSM budget is 105 (`.kutha/dictionaries/fsm.yaml:8`) and the dictionary declares 105 checks (`rg -c '^  - id: ' .kutha/dictionaries/checks.yaml` -> 105), and the test also asserts `outcome.skipped == 0` (`test_fsm.py:84`). I started that pytest run twice with a probe egg-info present and terminated both after >350 s and >220 s of ~90% CPU each without a result; both runs' temp copies were removed afterwards (0 `*.egg-info` under the root, verified).

## 3. The `dogfood` check: what it scans and why `tmp/` is in scope

- Check id `dogfood`, `.kutha/dictionaries/checks.yaml:180-229`. It is mostly a docs ladder (H0-H5 needles in `docs/process/kutha-harness.md`), plus `file_absent` guards; the relevant step is `:225-229`:
  - kind `glob_none`, `glob: "**/*.egg-info/*"`, `category: build-artifact`, no `severity:` key -> default **HIGH** (`kinds.py:101-103`).
- Step kind `_kind_glob_none` (`scripts/kutha_gov/kinds.py:462-474`): `hits = sorted(ctx.root.glob(pattern))`, one HIGH `Finding` per non-empty hit list, file = first hit path. `ctx.root` is the repo root (`protocol.py:59-66`). `pathlib` glob has **no gitignore awareness and no skip list**, and `**` descends into `tmp/`, `target/`, `.git/` and `.venv/`. Measured cost of the bare glob over the current tree: **0.41 s**, 0 hits.
- So the check's real scope is "the whole working directory, ignored or not". Its intent (`message: "setuptools egg-info must not be in the tree"`) is *build artifacts in the source tree*.
- The harness itself already treats `tmp/` as out of scope elsewhere: `copy_harness_tree` ignores `SCRATCH_DIR` at the copy root (`scripts/kutha_gov/selftest.py:115`, `SCRATCH_DIR = "tmp"` at `:79`), and `SKIP_DIR_NAMES` (`:64-76`) drops `target`, `.venv`, `.git`, caches. The live `glob_none` does not. **That inconsistency is the defect**, not the tests alone.

**Is skipping gitignored `tmp/` safe for its purpose?** Yes. `tmp/` is declared scratch (AGENTS.md), is gitignored (`/tmp/` and `crates/**/tmp/`, `.gitignore:25,30`), and is excluded from every harness copy the selftest mutates. An egg-info there cannot be committed and cannot reach a published artifact; the assertion it protects ("no setuptools build output in the source tree") keeps its full force on the tracked tree.

**Options and risk**

| option | effect | risk | who |
|---|---|---|---|
| A. skip the declared scratch root in `glob_none` | leftovers stop turning the live tree red; scope matches `copy_harness_tree` | a real egg-info placed under `tmp/` is no longer reported - harmless by the scratch contract, but it hides a *deliberate* misuse of `tmp/`; second hard-coded copy of the scratch name unless it imports `SCRATCH_DIR`/`SKIP_DIR_NAMES`; kernel change to `kinds.py` (needs `governor-intake` row + a selftest mutation, per AGENTS.md) | harness check |
| B. test cleanup only | fewer leftovers | cannot cover SIGKILL or an operator `kill` - the incident *was* a killed run; couples the live-tree gate to test hygiene; 250+ agent-written files under `tmp/` remain in scope by design | test |
| C. both (recommended) | leftover-resistant gate *and* less garbage | one kernel change plus small test edits; the intake row is unavoidable | harness check + test |
| D. downgrade the step to `severity: low` | cheap | loses fail-closed behaviour for the real target (egg-info in `crates/` or docs) - not recommended | none |

With C, the failure a person sees is again the intended one (an artifact in the source tree), and a killed run no longer poisons the next `ci` with a path under `tmp/kutha-selftest-*/tree/`.

## 4. Catalogue of environment-dependent tests

Every row was read in source; the two `scripts/tests` rows marked *measured* were also observed running.

| test (file::fn, line) | dependence | current failure mode | recommended owner / fix | severity |
|---|---|---|---|---|
| `tests/m019_dur_a.rs::jsonl_inode_changes_on_persist_rename` (:231) | exclusive `flock` on a unique scratch dir; needs no foreign holder | `WouldBlock` panic inside `store::persist` (recorded 1 in 4 workspace runs, `CHANGELOG.md:280`); not reproduced in 26 completed runs today (2 formal + 24 stress) | test: log the dir + lock holder in `assert_writer_lock_error` (:109) so the next occurrence is diagnosable; do not "fix" blind | medium |
| `tests/m019_dur_a.rs::dead_process_stale_lock_does_not_block` (:427) | re-execs its own binary (`:449`); spawn/kill scheduling; no timeout on the ready-spin (`:456-461`) | hangs forever if the child never writes the ready file; `ETXTBSY`-class spawn panic if `target/` is being rebuilt concurrently; orphan child holding `flock` if the parent is killed | test: bound the spin, kill+`wait` on panic (guard), and reap on parent death | medium |
| `tests/m019_dur_a.rs::{two_writer_attach_fails_closed (:387), second_writer_names_kutha_lock (:401), reader_open_while_writer_holds_lock (:414), panic_releases_writer_lock (:470), lock_file_ignored_by_dir_listings (:484), persist_migrate_race_serialized_or_fails_closed (:593), persister_is_poisoned... (:629)}` | advisory `flock` semantics of the local fs (ext4 here); the in-process `Mutex` only serialises the file group | flock differs on NFS/other hosts (`ENOLCK`, no cooperation); mutex cannot help cross-process runs - the very scenario of the flake | none (correct oracle); fix only the stale comment at :18 | low |
| `tests/m019_dur_b.rs::{export_jsonl_while_persister_holds_lock_fails_closed (:74), export_jsonl_after_writer_drops_equals_log (:93), export_jsonl_takes_lock_before_reading (:113)}` | same `flock` dependence | same as above | none | low |
| `tests/m030_writer_lock.rs::{failpoint_append_keeps_lock_and_writer_usable (:63), failpoint_then_checkpoint_still_holds_lock (:109)}` | `flock` + injected failpoint ordering | WouldBlock assertions invert only if the lock is dropped early (deterministic) | none | low |
| `tests/m052_lock_before_heal.rs::{losing_attach_while_writer_holds_lock_does_not_heal (:142), attach_heals_interrupted_seal_after_lock_released (:172)}` | `flock` + thread-local tiny segment limits (`tiny_limits`, `m052_lock_before_heal.rs:19`) | WouldBlock oracle | none | low |
| `tests/m135_migrate_pins_unlock_window.rs::{migrate_pins_and_compat_do_not_take_parent_lock (:140), migrate_pins_gap_blocks_peer_cite_then_converges (:153)}` | holds the parent lock in-process and asserts the callee does not re-take it | would fail if a nested acquire were introduced (deterministic guard) | none | low |
| `tests/m182_meta_threats.rs::replay_meta_is_not_quadratic` (:31-58) | **absolute wall clock**: `elapsed < 3 s` for 4000 `PackInstall` replays | fails on a loaded/oversubscribed host or slow disk; a real quadratic regression shows as ~2 s pre-fix, so the margin is thin | test: replace with an instrumented/structural oracle (the m184 pattern, `m184_admission_index.rs:171,253`) or gate behind an env flag | medium |
| `tests/m183_digest_falsifiers.rs::manifest_replay_scales_linearly_with_packs` (:160-207) | **wall-clock ratio**: `t_large < t_small * 10 + 50 ms` | if `t_small` is tiny (fast host), the 10x bound is tight and noise-prone; on a loaded host a scheduling stall can exceed +50 ms | test: same as above - counter-based or `#[ignore]`-style bench | medium |
| `tests/m178_enrichment.rs::{ledger_survives_persist_open (:195), ledger_survives_checkpoint_sealed_prefix (:210)}` | **fixed relative dir** `tmp/m178-enrichment-*`, CWD-relative | concurrent cargo runs delete each other's store -> `WouldBlock`/ENOENT; leftover under `tmp/` if killed; stray `tmp/` in the crate dir if run from `crates/kutha-runtime` | test: use `common::PersistScratch` or label+pid+nanos | high |
| `tests/m179_gate_gc.rs::{toy_pack_installs_queries_retires_and_replays_without_core_edits (:69), pack_retire_then_replay_is_parity_equal_to_reopen (:119)}` | **fixed relative dirs** `tmp/m179-gate-gc`, `tmp/m179-gate-gc-replay` | same | test: same | high |
| `tests/m185_enrichment_admission.rs::checkpoint_tail_proposal_cannot_cite_a_dropped_source` (:227, dir at :242) | **fixed relative dir** `tmp/m185-checkpoint-source` | same | test: same | high |
| `tests/m002_snap01.rs` (`:22-23`), `tests/m017_replay_integrity.rs` (`:23-24`) | scratch name is `{label}-{nanos}` **without pid** | two processes with the same nanosecond stamp collide (unlikely); weaker than the `m019` pattern | test: add pid | low |
| `tests/m078_migrate_cli.rs` (`:77`), `tests/m093_migrate_cli_contract.rs` (`:6`), `tests/b1_checkpoint_kill3.rs` (`:731`), `tests/b3_checkpoint_cut_kill.rs` (`:951`) | need the built `kutha-migrate` binary (`CARGO_BIN_EXE_kutha-migrate`) | fails if the binary is absent/stale (e.g. a filter that builds tests only) | none (cargo builds bins for these targets) | low |
| `tests/ui_clock_facade.rs` and `crates/kutha-common/tests/ui_clock_mix.rs` | **trybuild**: shells out to cargo/rustc and diffs `.stderr` text | toolchain wording change or a busy/disk-full host fails them; slowest part of a serial run | none for now; note in `CHANGELOG`/docs that they need a working cargo at test time | low |
| `tests/m019_dur_a.rs` `seg_*` counters (`:247`, `:264`) | **thread-local** instrumentation counters (`store.rs:38-60`, `segment.rs:39-56`) | immune to parallel tests by construction (checked: `Cell` per thread); no risk | none | none |
| `tests/m017_ingest_scaling.rs::extra_relatedto_horn_visits_constant_in_n` (:30), `tests/m136_retract_n_scaling.rs::retract_n_derived_examines_grow_near_linear_not_quadratic` (:49) | **counters**, not clocks | load-independent | none | none |
| `scripts/tests/test_fsm.py::test_ci_quantum_scheduling_reaches_ok_with_skips` (:41) | runs **every live check** (budget 105 = 105 checks, no truncation) plus the live selftest copy; CPU-bound - two concurrent runs each held ~90% CPU for >350 s / >220 s without finishing | any live-tree HIGH fails it (`:77`), so a leftover under `tmp/` fails it; also the biggest time sink in the pytest suite | test/harness: fix `glob_none` scope (sect. 3); consider `skip_cargo`-style scoping for the copy | high (as a gate) |
| `scripts/tests/test_kutha_gov.py::test_no_high_on_live_tree` (:126) | runs every live check (measured: full sweep in 0.94 s; red as soon as one stray file exists) | one stray artifact anywhere under the root fails it; reproduced | harness check (sect. 3); no test change needed | high |
| `scripts/tests/test_selftest.py::test_live_repo_selftest_is_green` (:123) | copies the live tree (minus `target`, `.tmp`, `.venv`, `.planning/milestones`) and mutates it | slow under load; fails if a *copied* file trips a baseline | none | low |
| `scripts/tests/test_m114_cargo_observe_lock.py` (`kutha-m114-*`) | `fcntl.flock`, a stub `cargo`, `sleep 0.25` ordering, global `os.environ` writes (`CARGO_TARGET_DIR`, `KUTHA_GOV_CARGO_LOCK_WAIT_SEC`) | timing-sensitive on a loaded host ("holder never acquired the cargo lock"); leaves `kutha-m114-*` dirs under `tmp/` | test: cleanup + a longer ready-loop budget | medium |
| `scripts/tests/test_m112_selftest_substring.py:16`, `test_m113_selftest_mutation_static.py:18`, `test_fsm.py:468,494` | `mkdtemp` under repo `tmp/` (conftest redirect) with **no cleanup** | green run already leaves 2+ dirs; killed runs leave more | test: `TemporaryDirectory` / `atexit` | medium (feeds sect. 3) |
| network | none found: `rg 'TcpListener|reqwest|hyper|127.0.0.1|localhost'` over `crates/*/tests` and `scripts/tests` returns nothing | - | none | none |
| governor, not tests: `disk-free` (`checks.yaml:1491-1495`, `min_free_gib: 2`) and `observe_cargo` (`fsm.yaml:153`, `timeout_sec: 180`, overridable via `KUTHA_GOV_CARGO_TIMEOUT_SEC`) | host disk (95% used here) and wall clock | ENOSPC / timeout -> HIGH; `handoffs/m148-closed-gsd-auto.md:16` records exit-101 flakes under disk pressure, serial `--jobs 1` as the honest closeout | harness check: already documented; keep serial closeouts | medium |

## What to change (proposals only - no edits made)

1. **`glob_none` scope (harness check; needs a lease, a `governor-intake` row and a selftest mutation).** Give the step kind an optional `exclude:` list, or import `SCRATCH_DIR`/`SKIP_DIR_NAMES` from a single module and skip the declared scratch root by default (`kinds.py:462-474`), so the live check matches `copy_harness_tree` (`selftest.py:64-79,115`). Keep severity HIGH. Extend the existing synthetic `no-egg` case (`scripts/tests/test_selftest.py:242`) with "a file under `tmp/` does not trip; one under `crates/` does".
2. **Fixed scratch dirs (test).** Convert `m178_enrichment.rs:199,223`, `m179_gate_gc.rs:102,127`, `m185_enrichment_admission.rs:242` to `common::PersistScratch` or `{label}-{nanos}-{pid}`; this removes the only *confirmed* cross-run store-sharing mechanism and the CWD dependence.
3. **pytest temp hygiene (test).** `test_m112_selftest_substring.py:16`, `test_m113_selftest_mutation_static.py:18`, `test_fsm.py:468,494`: use `TemporaryDirectory`/`atexit.rmtree`; consider one helper in `scripts/tests/conftest.py` that registers cleanup for every dir created under `_SCRATCH` (`conftest.py:8-10`). This reduces, but does not eliminate, the dogfood interaction.
4. **`m019_dur_a` diagnostics (test).** Extend `assert_writer_lock_error` (`m019_dur_a.rs:109`) to print the store dir, its listing and the lock holder; add a timeout + panic reaping to the self-exec in `dead_process_stale_lock_does_not_block` (`:449-463`). Fix the stale "process-global segment limits" comment (`:18`) - the limits are thread-local (`segment.rs:39-56`).
5. **Wall-clock oracles (test).** `m182_meta_threats.rs::replay_meta_is_not_quadratic` (3 s absolute) and `m183_digest_falsifiers.rs::manifest_replay_scales_linearly_with_packs` (ratio + 50 ms) should become counter/structural oracles, as `m184_admission_index.rs:171,253` already documents ("a wall-clock ratio test was removed because it flaked").
6. **Small hardening.** Add `pid` to `m002_snap01.rs:22` and `m017_replay_integrity.rs:23` scratch names; add cleanup to `test_m114_cargo_observe_lock.py` and a longer ready-loop budget.
7. **No change** for the counter-based scaling tests (`m017_ingest_scaling`, `m136_retract_n_scaling`) or the thread-local instrumentation counters - they are load-independent by construction.

## Open items

- The `m019_dur_a` root cause is still unproven; the recorded failure names `jsonl_inode_changes_on_persist_rename`, whose directory is unique per process, so no source-level mechanism I could find explains it. One failing run with the diagnostics from item 4 is worth more than any further blind re-runs.
- The second live-tree pytest (`test_fsm.py:41`) never completed: two concurrent runs of that single node were still CPU-bound (~90% CPU each) after 350 s and 220 s and were terminated. Its failure under a probe egg-info is therefore established from source (`fsm.py:310` + `test_fsm.py:77` + budget 105 = 105 checks), not from a finished run. The cost itself is a finding: one node of the pytest suite runs the whole live sweep *and* a live selftest, spending minutes of CPU and a tree copy, which is also what a killed run leaves behind.
