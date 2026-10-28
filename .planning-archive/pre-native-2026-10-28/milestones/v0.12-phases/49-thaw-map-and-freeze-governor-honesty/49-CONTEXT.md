# Phase 49 Context: Thaw map and freeze governor honesty

**Milestone:** v0.12 / harness M002 S01
**Requirements:** THAW-01, GOV-03
**Depends on:** none (M018 S01 shipped; M018 paused at the S02 decision gate)
**Source:** 2026-10-03 M002 re-verification at `f4af544`: provenance audit, governor freeze map, durability design research (CBM + GitNexus, `rg` arbiter)
**Run rules:** `docs/process/gsd-auto-contract.md`

## Locked decisions

- **D-01 (no product code):** nothing under `crates/` changes. The RocksDB dependency stays banned in `Cargo.toml` until Phase 51 (S03); this phase only makes the ban honest.
- **D-02 (freeze was agent-made):** `1ee5942` and `85db1f7` (2026-08-18, Cursor co-authored) created the list and the "until explicit M002 lease" gate; ADR-000 D1–D10 do not impose it (D6 names Rust+RocksDB event store, D8.1 asks for segmentation). Docs say "thawed 2026-10-03 at the owner's request", not "unfrozen by a new decision".
- **D-03 (per-item thaw map):** RocksDB crate: Phase 51; Cypher/GPML: M003; HNSW: M005; ADR-050 six dictionaries: SIX-01; ADR-080/081: security milestone after M003; full ADR-090 ontology: STATE names a legal pack (`I-legal-pack-not-next`); ADR-093: science pack (M004); Consensus Query 103+: lifted only by the owner. `.kutha/STATE.md` § Thaw map is the single table (controller already wrote it); every other file cites it and carries no copy.
- **D-04 (governor, YAML only):** split `freeze` in `.kutha/dictionaries/checks.yaml` into `freeze-rocks` (needle `rocksdb`), `freeze-cypher-deps` (`neo4j`, `graphiti`, `falkordb`), `freeze-hnsw-dep` (`hnswlib`). Each message names its real gate (Phase 51 / M003 / M005); no message claims to read STATE. Drop the `file_contains` "until Active Milestone names M002" step. Scan the workspace root `Cargo.toml` too (second step, same needles). Update `bridges.yaml` (`B-freeze-deps` becomes three rows citing STATE, unique check ids). No new Python class. The `unless:` kernel extension is Phase 51's first task, not this phase.
- **D-05 (selftest honest):** each split check keeps a derivable selftest (append the needle to a Cargo.toml) or a declared mutation; `uv run kutha-gov selftest --check freeze-rocks` etc. must report OK, never VACUOUS/SKIP. `scripts/tests/test_kutha_gov.py` replaces the old `freeze` message tests with ones for the three ids.
- **D-06 (ADR wording):** ADR-000/010/012/014/060 and `docs/process/*.md` that say "frozen M002" or "until M002 lease" become "M002 (thawed 2026-10-03; slice list in `.kutha/ROADMAP.md`)". Decision text is not rewritten; add dated lines. `honeycomb.yaml` gets a restage comment only; `freeze_as` fields and statuses unchanged.
- **D-07:** Verifier ≠ executor. Only the controller edits `.kutha/STATE.md`, `.planning/STATE.md`, `README.md` lease row, `CHANGELOG.md`. Do not write `49-VERIFICATION.md`.

## Owned file groups (disjoint)

- **G (governor):** `.kutha/dictionaries/checks.yaml` (freeze rows and the `honeycomb-ledger` comment only), `.kutha/dictionaries/bridges.yaml`, `scripts/tests/test_kutha_gov.py`, `scripts/tests/test_selftest.py`, `docs/process/governor-intake.md`.
- **D1 (process and planning docs):** `docs/process/kutha-harness.md`, `.planning/PROJECT.md` (not the Current Milestone block), `.planning/codebase/ARCHITECTURE.md`, `.planning/codebase/CONCERNS.md`, `AGENTS.md` (freeze pointer wording only), `docs/process/gsd-auto-contract.md` (line about stopping when a freeze item is thawed).
- **D2 (ADR):** `docs/ADR/ADR-000-*.md`, `ADR-010-*.md`, `ADR-012-*.md`, `ADR-014-*.md`, `ADR-060-*.md`, `ADR-091-*.md` (only if it says Rocks frozen), `.kutha/dictionaries/honeycomb.yaml` (restage comment only).
- **Controller:** STATE files, README lease row, CHANGELOG, `.kutha/ROADMAP.md`, `.planning/{REQUIREMENTS,ROADMAP}.md`.

## Verifier kill-tests

- Add `rocksdb = "0.22"` to `crates/kutha-common/Cargo.toml` and to the root `Cargo.toml` (copy): `uv run kutha-gov ci` fails HIGH on `freeze-rocks` with a message naming Phase 51; same for `neo4j` → `freeze-cypher-deps` naming M003, `hnswlib` → M005. Restore afterwards (work on a copy under `tmp/`).
- `uv run kutha-gov selftest` reports OK for the three new ids and for `honeycomb-ledger`; no check VACUOUS.
- `rg 'until Active Milestone names M002|until explicit M002 lease' --glob '!CHANGELOG.md' --glob '!.planning/milestones/**' --glob '!.compound-engineering/**'` returns nothing.
- `.kutha/STATE.md` § Thaw map lists all eight items; each `freeze_as` string in `honeycomb.yaml` still appears in it.
- `uv run pytest scripts/tests` is green; `git diff --stat -- crates/` is empty; `uv run kutha-gov ci` is green.

## Non-goals

Adding the RocksDB dependency; the `unless:` kernel extension (Phase 51); any M002 product slice; choosing C01; thawing Cypher, HNSW, six dictionaries, ADR-080/081, ADR-090, ADR-093.

## Harness

Active Milestone M002, Active Slice **S01**.
