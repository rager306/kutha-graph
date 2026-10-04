# Post-M002 roadmap / process audit (2026-10-04)

Read-only. HEAD `36fedb9` / tag `v0.12`. CBM `root-kutha-graph` (`list_projects`; coverage `metadata_match` on cited crate+doc paths). GitNexus `kutha-graph` (`list_repos`; lastCommit `36fedb9`).

## 1. Remaining dependency graph

Closed: M001, M010, M011, M012a, M012, M013–M017, **M002**. Paused: **M018 S02–S04**. Unleased later list (`.kutha/ROADMAP.md` 290–311): M003 Cypher → M004 science → pack/Cui (after two verticals) → M005 HNSW → security → ADR-092.

```text
M018 S02 (owner) → S03/S04 (eng)
M003 (lease) ─┬→ security milestone
M004 (lease) ─┴→ pack lifecycle
M005 (lease)     retrieve-not-truth
SIX-01 / ADR-090 / Consensus 103+ / ADR-092 = owner gates
IO-01 / VAC-01 / CLOCK-01 / API-01 / PERF-02 = candidates, not next
```

| Blocked by owner | Blocked by engineering (needs a lease, not a semantic verdict) |
|------------------|----------------------------------------------------------------|
| M018 S02: C01, HORN-KC-01, ASOF-ELIG-01 | M003 parser; M005 ANN; wire `SegmentIndex` into `Runtime` |
| SIX-01, legal pack, Consensus 103+, ADR-092 | IO-01 soak; VAC-01; CLOCK-01; API-01; PERF-02 |
| SoT wording ack (`events.jsonl` = export) | O(N) `open` hydrate; `rocks_spike` still jsonl |

**Wedge (legal AS OF):** still justified as *next truth work*. FF5 already proves `as_of(2015)≠as_of(2021)`. M018 is the real wedge gate (C01 sunset, Horn vs `as_known_on`, `as_of` vs eligibility). M003 is a query skin over an already-correct cut — not required for legal truth. M002 jumping ahead of M018 already happened; **do not jump again to M003**. M005 remains retrieve-not-truth.

**Obsolete / unreachable as *current* claims after M002:** “M002 frozen”; blanket “RocksDB crate frozen”; “events.jsonl is physical SoT” for empty/new dirs; “Rocks WAL shipped”; “RocksDB is NOT a dependency”. Not obsolete: logical event log = SoT (D1); harness `.kutha/events.jsonl` (H0); legacy jsonl+WAL until `migrate`; `rocks_spike` jsonl offsets; full `persist()` still rewrites.

## 2. Stale or contradictory claims

**HIGH — living docs still freeze M002 / deny Rocks / name jsonl as current physical SoT**

| File:line | Claim | Truth |
|-----------|-------|-------|
| `docs/architecture/m002-cloud-kernel-io-intake.md:3` | “M002 remains **frozen** until STATE leases it” | M002 CLOSED; IO-01 is leftover intake |
| `docs/architecture/p0-spike-inventory.md:13` | “Not in spike: RocksDB crate” | Optional `rocksdb` + `rocks-index` (`Cargo.toml:9–21`); `SegmentIndex` exists |
| `.planning/PROJECT.md:121` | “P0 spike has no Tokio/Rocks in workspace deps” | Same optional dep; no Tokio (STACK.md:27 still OK) |
| `.planning/codebase/STRUCTURE.md:232` | “RocksDB crate … keep out of `crates/`” | Mapped 2026-09-29; crate is in |
| `docs/ADR/ADR-000-…md:184` | “RocksDB is NOT a dependency”; jsonl+WAL as implemented spike | L185 pointer to SEG-01; dep is optional. Sentence 184 is false |
| `docs/ADR/ADR-010-…md:13` | “jsonl is SoT”; “each persist rewrites”; “M002 … planned” | Dated 2026-10-03; later L27–41 supersede. First-read is current-false |
| `docs/ADR/ADR-010-…md:29` | “`persist` / `Persister` still write jsonl+WAL” | L31–33: empty/segmented dirs write `KUTHASEG`+manifest |
| `docs/ADR/ADR-012-…md:21` | “jsonl remains SoT” (Phase 51) | L39–45: jsonl is unread export after migrate |
| `.planning/REQUIREMENTS.md:10` | DOC-06: living “jsonl SoT with covering WAL” | v0.10-era Phase 45 req; file is still the live REQUIREMENTS |
| `.planning/REQUIREMENTS.md:38` | WRITE-01 “WAL vs JSONL preference” | Segmented persist prefers manifest+active segment |
| `.kutha/ROADMAP.md:138,155` | Closed M012/M013: “still frozen … M002” | Historical close text; scans as current freeze |
| `.kutha/dictionaries/honeycomb.yaml:26` | Restage: “persist still jsonl+WAL” | Superseded by L27–28; comment not restaged |

**MED — SoT nouns, persist cost, inventory, L_delivery**

| File:line | Claim | Truth |
|-----------|-------|-------|
| `.kutha/STATE.md:3` | “Product SoT is `crates/kutha-runtime`” | Product SoT is the event log; runtime is the fold/engine. Process SoT `.kutha/events.jsonl` is OK (H0) |
| `.planning/PROJECT.md:37` | Validated: “snapshot + WAL-cousin” only | Also: manifest+`KUTHASEG`, `Persister` segmented append, `migrate`/`export_jsonl` |
| `.planning/codebase/ARCHITECTURE.md:32–40` | WAL cousin as the persist picture | last_mapped `e77132d` / 2026-09-29 |
| `STRATEGY.md:3,89` | `last_updated: 2026-08-17`; “hot path stays RocksDB” | Research lock, not shipped SoT; date stale |
| `semantic-gap-review.md:25,28` | F2/F6 bodies still describe `fact_seq` / `File::create` | Status lines update F6; F2 table body is historical |
| `CHANGELOG.md:11` | “v0.12 is for the controller to create”; “`rocks_spike` still on jsonl”; “O(N) `open`” | Tag exists; last two residuals **true** |
| `.kutha/ROADMAP.md:294–305` | Later list still numbers M002 after M018 | Confuses “next”; M002 is closed |

**L_delivery:** living token is `L_delivery=M002-closed` (`.kutha/STATE.md:15`). GSD STATE/PROJECT/REQUIREMENTS do not mint a competing token. Historical CHANGELOG / closed ROADMAP slices are dated. Governor `cite-lease` / `idle-delivery-closed` (invariants.yaml:87–91, 242–246).

**Phase/slice counts:** v0.12 = 7 phases 49–55 (STATE + gsd-auto-contract:22). v0.11 = 45–48, paused at 46. README layout (README.md:41–60) omits `kutha-tenant` bin and `segment`/`rocks_index` (tenant is a runtime bin, not a third crate — Cargo.toml members = common+runtime).

**Persist O(N):** `persist_until` always `encoded_log` (store.rs:296,704–717) then `persist_log` rewrite-all (segment.rs:591–620) **or** legacy jsonl+WAL replace. **`Persister::append_segmented` is O(batch)** (persist_append.rs:271–308). Residual “O(N) persist per quantum” is **false for append**, **true for full `persist()` / heal**. `open` is O(N) load+hydrate (v0.12 audit; `load_events` 517–544 + `open_hydrated` 604–653). CBM: `SegmentIndex` inbound callers **0**. GitNexus: `SegmentIndex` upstream **0** (not wired). `rocks_spike::build_index` still parses **jsonl lines** (rocks_spike.rs:19–50).

**p0 vs code:** inventory L9 is mostly current (SEG-01). L13 “RocksDB crate” not-in-spike is the live lie. Cypher/HNSW/ABAC/legal pack/six dicts still out.

## 3. Process sprawl

| Tree | Role | Collapse? |
|------|------|-----------|
| `.kutha/STATE.md` + thaw map | Lease | Keep. One SoT for L_* |
| `.kutha/ROADMAP.md` (~318 lines, full closed M001–M017) | Delivery pyramid | **Collapse closed bodies** to one-line + archive pointer; keep M018 + later list |
| `.planning/{STATE,ROADMAP,PROJECT,REQUIREMENTS}` | GSD overlay | Keep thin. PROJECT Validated list is a second inventory |
| `.planning/milestones/v0.0x-*` + `v0.12-phases/` | Falsifiable closeout | **Keep** VERIFICATION/CONTEXT. Do not delete |
| `.planning/phases/45-*` (2 files) | Live leftover of paused v0.11 | Move into `v0.11-*` archive when v0.11 closes |
| `.planning/codebase/*` (2026-09-29) | Stale map | Freeze or one “superseded by p0-inventory + ADR Evidence” banner |
| `CHANGELOG.md` (2300+ lines) | Dated history | Keep. Stop restating full SEG-01 in PROJECT |
| `semantic-gap-review.md` | Accumulating addenda | Freeze register; new residuals → ROADMAP / audit |
| `STRATEGY.md` | Wedge | Keep; refresh date + Rocks sentence |
| `docs/process/*` (4 files) | Contracts | Keep; no merge |

Falsifiability lives in named oracles + independent `NN-VERIFICATION.md`, not in duplicated ROADMAP prose.

## 4. Next 3 steps + minimal doc edits first

1. **Owner: M018 S02** (C01 / Horn-KC / as_of vs eligibility) or an explicit “park M018” note. Rationale: only open semantic gate on the legal wedge. Do not lease M003.
2. **Living-doc SoT pass (docs only, no crate).** Then a new GSD slice if they want engineering.
3. **After S02:** M018 S03 (Horn identity / Correct re-derive) *or* a named residual slice (wire Rocks / O(N) open) — owner pick. Vacuum/IO-01 stay intake.

**Edit first (exact ideas):**
- `m002-cloud-kernel-io-intake.md:3` → Status: M002 CLOSED; IO-01 still intake-only.
- `p0-spike-inventory.md:13` → move Rocks optional index to “In spike (feature-gated, not wired)”.
- `ADR-000:184` → “optional `rocksdb` index lease; jsonl+WAL is legacy.”
- `ADR-010:13` and `:29` → prefix “Historical (pre-SEG-01).”
- `PROJECT.md:121` + `STRUCTURE.md:232` → cite thaw map; Rocks optional allowed.
- `REQUIREMENTS.md:10,38` → mark DOC-06 historical; rewrite WRITE-01 as “intern admit vs segmented persist.”
- `.kutha/ROADMAP.md` later list → “Next lease: M018 S02. M002 closed (see above).” Strike “still frozen … M002” from M012/M013 close paragraphs.
- `STATE.md:3` → “Product SoT = event log (manifest+segments; jsonl export).”

No crate edits. No M003/M005. Honeycomb stays Proposed.
