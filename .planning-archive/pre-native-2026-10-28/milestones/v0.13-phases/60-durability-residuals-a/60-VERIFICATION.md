---
independent: true
verifier_model: cursor-grok-4.6-xhigh
status: passed
score: 13/13
exit: 0
commands:
  - {cmd: "git diff 642ddd3..HEAD -- crates/kutha-runtime/tests", exit: 0}
  - {cmd: "cargo fmt --check --all", exit: 0}
  - {cmd: "cargo clippy --workspace --all-targets --offline", exit: 0}
  - {cmd: "cargo test --workspace --offline", exit: 0}
  - {cmd: "cargo test -p kutha-runtime --features rocks-index,rocks-index-spike --test m002_rock01 --test m002_rock00 --offline", exit: 0}
  - {cmd: "uv run kutha-gov py", exit: 0}
  - {cmd: "uv run kutha-gov --check test-matrix json", exit: 0}
  - {cmd: "uv run kutha-gov precommit", exit: 0}
  - {cmd: "uv run kutha-gov ci", exit: 0}
  - {cmd: "uv run pytest scripts/tests -q", exit: 0}
  - {cmd: "scratch layered/old-yaml/poison mutations (fresh CARGO_TARGET_DIR)", exit: 0}
read_only_inputs:
  - crates/kutha-runtime/src/store.rs
  - crates/kutha-runtime/src/persist_append.rs
  - crates/kutha-runtime/src/segment.rs
  - crates/kutha-runtime/tests/m019_dur_a.rs
  - crates/kutha-runtime/tests/m002_commit01.rs
  - crates/kutha-runtime/tests/m002_append01.rs
  - docs/architecture/test-matrix.yaml
  - docs/ADR/ADR-010-event-log-runtime-quantum.md
  - docs/ADR/ADR-014-cascade-budgets-quantum-receipts.md
---
HEAD `7b43beb` (36a0d30 + docs) vs `642ddd3`. CBM `root-kutha-graph` ready but cited paths `metadata_changed` / `m019_dur_a.rs` `not_tracked` (no reindex). `rg` arbiter. Scratch: `tmp/v60b-src` + `CARGO_TARGET_DIR=tmp/v60b-mut-target`; restored and deleted.

(a) `DUR-sealed-trailer-lie`: yaml layered kill (skip sealed trailer + do not propagate `scan.chain` + skip sealed and `read_active` prev) **fails** `sealed_trailer_bytes_lie_fails_closed`; old yaml `return scan+expected_prev` (plus ahead/replay skip) **survives** via active chain-link. `DUR-wrong-role-active`: sequence-only skip **survives**; yaml layered skip **kills**. Wording honest.

(b) `git diff 642ddd3..HEAD -- crates/kutha-runtime/tests`: only `m019_dur_a.rs` +76 (new poison oracle + `listing_bytes`); m002 heal tests untouched (`m002_commit01` 17/17 including `open_for_append_heals_then_appends` / `heal_crash_matrix_reopens_to_committed_prefix`). `Persister`: fail-closed `InvalidData` append poisons + unlocks; later append/checkpoint/close return `InvalidData` containing `poisoned` and write nothing; heal is a new `open_for_append`. No-poison mutation **fails** that oracle (retry append `Ok`). Failpoints stay `ErrorKind::Other` and do not poison (`append_crash_matrix_reopens_to_prefix` + `m002_seg01` 61 + heal crash matrix). Real I/O not remapped: `wal_read_io_error_not_masked`. Residual: Other/failpoint still unlocks without poison (`write_lock=None`).

(c) ADR-010/014 Evidence and Current encoding (2026-10-05, Proposed) list: no fsync of the `kutha.lock` directory entry; advisory local lock only (NFS / another host not covered); writes that bypass `Persister` / `store::*` are not covered; `kutha-tenant` is covered only through `store::persist`.

(d) fmt 0; clippy 0 (54 warning lines, isolated target); workspace 318 passed / 0 failed (m019 19/19, m002_append01 19, m002_commit01 17, m002_snap01 10); rocks 3+11; py 0; test-matrix HIGH=0; precommit 0; ci 0 (0 HIGH, 1 LOW, 65 checks, selftest 65/65); pytest 181 passed, 9 subtests, 135.52s. YAML 67 = md (killed 46).

Residuals: lock dirent; NFS; bypass writes; tenant persist-only; failpoint unlock; `DUR-export-race` still `gap`.
