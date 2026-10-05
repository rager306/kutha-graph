---
independent: true
verifier_model: cursor-grok-4.6-xhigh
status: passed
score: 13/13
exit: 0
commands:
  - {cmd: "uv run kutha-gov py", exit: 0}
  - {cmd: "uv run pytest scripts/tests -q", exit: 0}
  - {cmd: "python -m kutha_gov --jobs 1|2|4|default --root tmp/p58-head precommit", exit: 0}
  - {cmd: "python -m kutha_gov --jobs 1|2|4|default --root tmp/p58-head ci", exit: 0}
  - {cmd: "python -m kutha_gov --root tmp/p58-base ci  # e4b80f8", exit: 0}
  - {cmd: "python -m kutha_gov --jobs 1|default --root tmp/p58-c1 selftest", exit: 0}
  - {cmd: "python -m kutha_gov --jobs 0|abc|-3 precommit", exit: 2}
  - {cmd: "python -m kutha_gov --jobs 4 --root tmp/p58-boom precommit", exit: 1}
  - {cmd: "python -m kutha_gov --jobs 4 --root tmp/p58-kill precommit  # SIGKILL worker", exit: 1}
  - {cmd: "uv run kutha-gov precommit|selftest|ci  # live", exit: 0}
read_only_inputs:
  - scripts/kutha_gov/parallel.py
  - scripts/kutha_gov/selftest.py
  - scripts/kutha_gov/config.py
  - scripts/kutha_gov/__main__.py
  - scripts/kutha_gov/fsm.py
  - scripts/kutha_gov/time_log.py
  - scripts/kutha_gov/kinds.py
  - scripts/kutha_gov/observe.py
  - scripts/tests/test_jobs.py
  - .kutha/dictionaries/fsm.yaml
  - .kutha/dictionaries/checks.yaml
  - docs/process/governor-intake.md
---
PAR-01 independent verify at 7be691e vs lease e4b80f8. CBM `scripts/kutha_gov` stale; source/rg used. Default jobs=min(12,8)=8.

1. Parity (scratch): precommit 1/2/4/default rc=0, findings equal. selftest 1 vs default rc=0, stdout+58/4/2 classes equal. ci 1/2/4/default rc=0; findings, stripped stdout, JSONL (op,subject,relation,object) equal; 8 rows each.
2. Seeded HIGH early (adr-status), LOW (honeycomb-map), HIGH late (version-freeze): jobs 1/4/default rc=1, identical ordered findings.
3. `KUTHA_GOV_TEST_DELAY_CHECK=adr-status=1.5` jobs 1 vs 4: stdout equal; adr-status still first.
4. Injected `RuntimeError` in worker (`parallel.py` adr-status): jobs 4 rc=1, HIGH `adr-status/worker-error`; selftest `WORKER-ERROR adr-status`; no `tmp/kutha-selftest-*` leftover; no hang.
5. SIGKILL one spawn child mid `--jobs 4` precommit: rc=1 in 0.58s, HIGH `BrokenProcessPool` worker-error (not silent; no `pool-fallback` LOW — per-future catch at parallel.py:107). SIGINT on ci: rc=-2, no hang>2min (unwind 21.6s), no orphans.
6. Flag>env>default: `--jobs 1` + `KUTHA_GOV_JOBS=8` rc=0. `0`/`abc`/`-3` flag and env: exit 2, clear stderr.
7. Two concurrent `ci` on tmp/p58-c1 and c2: both rc=0, identical findings+JSONL.
8. Writers only `fsm.py` `append_run`/`append_observe`; workers run `check.run` only. JSONL n=8 for jobs 1 and N.
9. e4b80f8 worktree `ci` vs HEAD `--jobs 1`: findings+stripped stdout+JSONL tuples equal, rc=0.
10. Skip: `SKIP_DIR_NAMES` includes `.compound-engineering` plus D-V1 heavies (.git,target,.venv,…); `.planning/milestones` via `SKIP_PLANNING_CHILDREN`. checks.yaml does not glob those archives. Classification 58 derived / 4 explicit-mutation / 2 explicit-skip at jobs 1 and default.
11. Live: py 0; pytest 171 passed 9 subtests 106.9s 0; precommit 0 (2.1s); selftest jobs1 0 (19.7s) default 0 (16.7s); ci default 0 (18.1s, 0 HIGH 1 LOW).
12. Warm 3-run means (scratch): ci 41.864s vs 18.351s (**-56.2%**, ~-54% reproduces). selftest 21.140 vs 14.063 (-33.5%). precommit 2.389 vs 2.522 (+5.6%, spawn-bound). `--jobs 2` selftest 21.7s vs jobs1 ~21.1s (copytree anomaly). Cold first ci 96.4s vs later ~16–23s.
13. Pool: `spawn` not fork (argv still `--multiprocessing-fork`). Rust cache keyed by resolved root+mtime (`kinds.py`). No check-worker timeout (`fut.result()` unbounded; cargo timeout is `observe-error`). selftest.py:199 bare `except Exception` serializes leftovers with no LOW `pool-fallback`. SIGINT has no pool cancel; workers drain.
