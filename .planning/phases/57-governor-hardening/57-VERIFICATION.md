---
independent: true
verifier_model: cursor-grok-4.6-xhigh
status: passed
score: 19/19
exit: 0
commands:
  - {cmd: "uv run kutha-gov py", exit: 0}
  - {cmd: "uv run kutha-gov --root tmp/v57-work --check freeze-rocks json", exit: 1}
  - {cmd: "uv run kutha-gov --root tmp/v57-work --check freeze-cypher-deps json", exit: 1}
  - {cmd: "uv run kutha-gov --root tmp/v57-work --check freeze-hnsw-dep json", exit: 1}
  - {cmd: "uv run kutha-gov --root tmp/v57-work --check honeycomb-ledger json", exit: 1}
  - {cmd: "uv run kutha-gov --root tmp/v57-work --check observe-required-fn json", exit: 1}
  - {cmd: "uv run kutha-gov --root tmp/v57-work --check id-map-cite json", exit: 1}
  - {cmd: "uv run kutha-gov --root tmp/v57-work --check artifact-budget json", exit: 1}
  - {cmd: "uv run kutha-gov --root tmp/v57-work --check trajectory json", exit: 1}
  - {cmd: "uv run kutha-gov selftest", exit: 0, wall_sec: 27.96}
  - {cmd: "uv run kutha-gov ci", exit: 0, wall_sec: 56.56}
  - {cmd: "uv run pytest scripts/tests -q", exit: 0, wall_sec: 111.76}
read_only_inputs:
  - scripts/kutha_gov/kinds.py
  - scripts/kutha_gov/selftest.py
  - scripts/kutha_gov/observe.py
  - scripts/kutha_gov/fsm.py
  - scripts/kutha_gov/config.py
  - scripts/tests/test_kutha_gov.py
  - scripts/tests/test_fsm.py
  - .kutha/dictionaries/checks.yaml
  - .kutha/dictionaries/fsm.yaml
  - .kutha/dictionaries/honeycomb.yaml
  - .kutha/META.md
  - .kutha/STATE.md
  - docs/process/gsd-auto-contract.md
  - docs/process/governor-intake.md
  - crates/kutha-runtime/Cargo.toml
  - Cargo.toml
  - crates/kutha-runtime/tests/m002_rock01.rs
  - crates/kutha-runtime/tests/m002_rock00.rs
  - .env.example
---

HEAD `7e27fbe`. CBM `root-kutha-graph` indexed 2026-10-04T14:36:49Z; cited harness paths `metadata_changed` — source/`rg` arbiter. Phase 57 CONTEXT/PLAN/SUMMARY not read. Scratch copies under `tmp/` deleted after.

GOV-04: `cargo_dep_constraint` (`tomllib`) walks dependencies/dev/build, workspace.dependencies, target.*, table form, `package=` rename. Scratch HIGH `frozen-dep` (or `cargo-toml` on garbage): (a) `[dependencies.rocksdb]` (b) `rocksdb="0.22"` (c) `rocksdb = "0.22" # optional = true` (d) `foo = { package = "rocksdb" }` (e) root `[workspace.dependencies]` (f) `[target.'cfg(unix)'.dependencies]` (g) keys `neo4j`/`hnswlib` (h) unparsable fail-closed. (i) comment-only `# rocksdb = "0.22"` exit 0.

GOV-05: ADR-010/012 evidence resolves (`rust_test_asserts`, category `map`). `observe_cargo.required` lists M002 headlines (persist/sidecar/append/payload). `observe_rocks` runs `m002_rock01`/`m002_rock00` (`timeout_sec` 420; not optional; skip only `--skip-cargo` or prior HIGH). Live `ci` observed `index_answers_equal_the_log` and `rocks_roundtrips_event_id_to_offset`. SUMMARY `max_files_per_dir: 1` — extra SUMMARY HIGH `max-files`; CONTEXT with zero SUMMARY/PLAN HIGH `min_hits`. `id-map-cite` requires `v0.12`/`v0.13` (token delete HIGH).

GOV-06: `kutha-gov selftest` 64/64: 58 derived-by-own-mutation, 4 explicit-mutation, 2 explicit-skip (`docs-coupling`, `verification-independent`), 0 file-exists-only, 0 unproven. (n) `map: Accepted` HIGH `yaml-map-vocab`/`yaml-map-embed`; trajectory pointer HIGH `phantom`. Cache: `_RUST_FN_INDEX` keyed by resolved root+stamps; one process two roots: original clean, deleted-fn copy HIGH, 2 keys.

Live `ci` green (0 HIGH, 1 LOW; `observe_cargo=28011ms` `observe_rocks=2818ms` warm). pytest 159 passed, 9 subtests, 110.47s. Surviving named kills: none.
