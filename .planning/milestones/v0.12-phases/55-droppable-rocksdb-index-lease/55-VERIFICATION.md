---
independent: true
verifier_model: cursor-grok-4.6-xhigh
status: passed
score: 6/6
exit: 0
commands:
  - cargo tree -p kutha-runtime --offline
  - cargo test --workspace --offline
  - cargo test -p kutha-runtime --offline --features rocks-index --test m002_rock01
  - cargo test -p kutha-runtime --offline --features rocks-index-spike --test m002_rock00
  - uv run pytest scripts/tests/test_kutha_gov.py -k Freeze
  - uv run kutha-gov --check freeze-rocks selftest
  - uv run kutha-gov precommit
read_only_inputs:
  - crates/kutha-runtime/src/rocks_index.rs
  - crates/kutha-runtime/tests/m002_rock01.rs
  - crates/kutha-runtime/tests/m002_rock00.rs
  - crates/kutha-runtime/Cargo.toml
  - crates/kutha-runtime/src/lib.rs
  - crates/kutha-runtime/src/admission.rs
  - crates/kutha-runtime/src/pipeline.rs
  - scripts/kutha_gov/kinds.py
  - scripts/tests/test_kutha_gov.py
  - .kutha/dictionaries/checks.yaml
  - docs/ADR/ADR-010-event-log-runtime-quantum.md
  - docs/ADR/ADR-012-snapshots-tiers-vacuum.md
  - docs/architecture/m002-cloud-kernel-io-intake.md
  - docs/architecture/benchmark-baseline.md
  - docs/process/gsd-auto-contract.md
  - CHANGELOG.md
---
CBM `root-kutha-graph` ready; new rocks files not_tracked (rg arbiter). Default target dir. Worktree: main only.

| claim | evidence | verdict |
|-------|----------|---------|
| 1 freeze | pytest Freeze 11/11: optional crate passes on STATE copies M002/M018/None; non-optional and root plain fail; neo4j/graphiti/falkordb/hnswlib fail. selftest freeze-rocks OK mutations=2. Always-pass needles → crate test fails. | pass |
| 2 default | tree: no rocksdb/librocksdb-sys. workspace 299 passed / 0 failed (rock00/rock01 0 tests). rocks-index 11/11; spike 3/3 names unchanged vs 0245bbd. | pass |
| 3 index=log | 11 oracles + probes. Winner = last Assert in log (Retract/Correct do not clear). Retract then same key = DeliveryKeyConflict. Duplicate key last-wins only if admission is bypassed. | pass |
| 4 droppable | delete Rocks dir → rebuild same answers. Scan: e:/d:/m: only, values 16/16/4/88, no JSON/terms. catch_up n=3, 4 new keys, prefix e:/d:/m:v unchanged. | pass |
| 5 scope | crates/ = Cargo.toml feature, lib.rs cfg, rocks_index.rs, tests. Persister/segment/store/open/fold/pipeline untouched; no unwrap/expect. ADR-010/012 Proposed, not wired; intake 410ms build / 12.8µs lookup. | pass |
| 6 bench | ingest/as_of/open paths unchanged; −17.8%/−8.7% host noise; not rerun. | pass |

Staleness: open_or_rebuild checks m:tip (sealed-hex+count+last id) and tip-frame CRC/id. Caught: Persister append; tampered m:tip; poisoned tip locator. Not caught: non-tip poison; same last+count+sealed-hex with swapped equal-size middles (get_event InvalidData; last still Ok).
Freeze forms: caught `rocksdb =` plain, inline non-optional, `[dev-dependencies]`/`[build-dependencies]` assignment, root `rocksdb =`. Uncaught: `[dependencies.rocksdb]`, root `[workspace.dependencies.rocksdb]`, `rocksdb = "0.22" # optional = true`, same-line other `optional = true`. False-positive: multiline `rocksdb = {`, `optional=true` without spaces.
Kills a–l: all KILLED (payload; skip CRC/id; skip m:tip; catch_up from 0; first d: winner; index uncommitted; build via jsonl; rocks_index in Persister; drop optional unless; allow root; drop migrate reject; offset+1). Survivors: none.
Gaps: line `unless_line_contains` bypassable as above; tip-only rebuild residual; STATE thaw map still says M002-only freeze. precommit verification-independent OK after this file ≤60 lines.
