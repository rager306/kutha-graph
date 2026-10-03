# S06 sealed segments (SEG-01) — design

**Date:** 2026-10-04. **Status:** research (not an ADR). **Lease:** M002 S06 unleased; GSD Phase 54. **Evidence:** CBM `root-kutha-graph` listed; `persist_append.rs`/`commit.rs` `not_tracked`, ADRs `missing` — claims from source + `rg`. Cousins: SQLite WAL mxFrame; Rocks `kTolerateCorruptedTail` vs `kAbsoluteConsistency`; Fjall Start/End; redb 1PC+C root flip; SlateDB all-or-none seq.

**Recommend:** `KUTHASEG` v1 = existing WAL frame (`len_le32 + crc32 + JSON`) + seal-trailer frame; SHA-256 per-frame chain via in-tree `sha2` (no blake3, no new crate). SoT = `manifest.json` + named sealed files + named active. Dual-read legacy `events.jsonl`+`events.wal`. Writer default after 54b: segments. Convert with explicit `store::migrate`. jsonl = on-demand export, never the append/checkpoint hot path. Split 54a/54b/54c.

## 1. Format

Reuse `wal::frame_bytes` / `write_frames` / CRC-of-payload. **New magic** `KUTHASEG` (8) + `u32` v1 + 32-byte previous-segment chain tip — do not bump `KUTHAWAL` v1 (`unsupported WAL version` would trip legacy `load_events_d04`). Header **44 bytes**, then event frames, then optional trailer. The 32-byte field is H0 = `[0u8;32]` only for the first segment; segment *k* stores the final chain tip of segment *k-1*.

**Trailer** (sealed only): one frame, payload JSON `{"k":"seal","n":N,"first":EventId,"last":EventId,"bytes":u64,"chain":"<64 hex>"}`. Not an `Event` (parse as trailer, not `recover_events`). Trailer CRC’d; **not** in the chain (avoids hashing the hash). Extra bytes after trailer → `InvalidData`.

**Chain:** `H0 = [0u8;32]` starts **only the first segment**. Each event frame `F` (on-disk `len||crc||payload`, not payload alone): `H = SHA256(H || F)`. Segment *k* begins hashing from the tip of *k-1* (copied into the 44-byte header). Tip hex in trailer + manifest. Rollover must **not** reset the hasher to H0. Per-frame CRC does **not** catch swap/reorder of valid frames. Open verifies header prev == expected tip, then recomputes the chain from that prev. One SHA-256 of the whole file would catch intra-file swap too; SEG-01 names a **chain**, and the hash rides the same open scan as serde. `sha2` is already a workspace dep (`kutha-runtime`/`kutha-common` `provenance`/`event`). `blake3` is absent. Do not add a crate. CRC-only “chain” is insufficient.

This does **not** protect an attacker who rewrites every segment **and** the manifest **and** recomputes all hashes. The manifest tip is not externally anchored.

**Rollover** (first hit; test seam thread-local like `LOG_FILE_SYNC_COUNT`): `SEGMENT_MAX_BYTES = 64 MiB`, `SEGMENT_MAX_EVENTS = 65536`. Independent of `CHECKPOINT_EVERY` (4096).

**Names:** `manifest.json`; sealed/active `seg-{seq:08}.log` (seq from 1). No `events.wal` after migrate.

**Manifest** (JSON, `replace_via_tmp` + `sync_dir`): `{"v":1,"sealed":[{name,first,last,count,bytes,chain}],"active":"seg-…log"}`. `bytes` = sealed file length including header+frames+trailer. Concatenate `sealed` order then `active`. Extra `seg-*.log` not named: ignore (crash leftover). `replace_via_tmp` already `sync_all`s the tmp file.

**Seal I/O (redb-like root flip):** (1) append trailer + `sync_data` on current file; (2) create next `seg-N+1` with header only + `sync_data`; (3) atomic manifest listing old as sealed + new as active + dir fsync. Crash before (3): old manifest still names the file **active** — open scans it; a present trailer ends the event stream (not a torn tail). Next append rolls before writing. Crash after (2) before (3): ignore the unnamed next file.

## 2. Recovery

Detect: `manifest.json` present → segmented path; **never read** `events.jsonl`/`events.wal`. Else `load_events_d04`. Then existing `committed_prefix` → hydrate/`replay_check`.

**Sealed:** full verify while loading (must deserialize anyway). Check magic/v1, header prev vs previous tip, trailer last, `count`/`first`/`last`/`bytes`/`chain` vs manifest, chain over frames starting from that prev, adjacent segment names contiguous (then active continues the sequence). **Lazy skip of sealed bodies is forbidden** (SEG-01 tamper would stay silent). Cost: SHA-256 per frame on the scan already paid for JSON; `open.n` is hydrate/replay (~2.7s), not hashing.

**Active:** WAL analog — drop torn last frame (short read / CRC fail / non-Event JSON that is not a seal). Header truncated → `InvalidData`. Seal trailer on a still-named-active file: keep events before it.

| Failure | Result |
|---------|--------|
| Swap two sealed files (meta retargeted) | `InvalidData` (`chain link`) |
| Replace a sealed file with another store's sealed file | `InvalidData` (`chain link`) |
| Delete a middle sealed file and fix names/counts | `InvalidData` (`chain link`) |
| Reorder manifest sealed entries | `InvalidData` (`not contiguous`) |
| Active file from another store | `InvalidData` (`chain link`) |
| Truncated sealed (no trailer, `bytes` mismatch, chain break) | `InvalidData` (not torn-drop) |
| Manifest vs file (name missing, first/last/count/bytes/chain) | `InvalidData` |
| Missing named **active** | `InvalidData` |
| Mixed dir: manifest + disagreeing jsonl | Manifest wins; jsonl unread |
| Torn **active** tail | **Dropped** |
| Uncommitted suffix | **Dropped** (`committed_prefix`, COMMIT-01 A) |
| Disk `Err` other than `InvalidData` | Propagate (SNAP-01) |

## 3. Persister / guard / checkpoint / commit

One log file = the **active** segment. `append`: frames then **one** `sync_data` (`note_log_file_sync`). No jsonl write on the hot path (today jsonl is unsynced; WAL is the commit; IO-01: do not add a second drain).

**O(1) tail guard:** only active `file_len` + last `EventId` + `last_frame_len` (`wal::last_event_id_at_end`). Sealed files are immutable; not re-checked on append. `attach` on a segmented dir must not require jsonl (`persist_append.rs` today `metadata(EVENTS_JSONL)`).

**Checkpoint / `close`:** `snapshot.json` + `terms`/`quantum_outcomes`/`justifications` unchanged (`replace_via_tmp`). Do **not** regenerate jsonl here (that reintroduces O(N) persist). Full `persist` may rewrite sealed files from `encoded_log` (test/heal path) then write a new active+manifest.

**`committed_prefix`:** after concat, before hydrate. Uncommitted suffix lives in the active tail. No commit frame (S05 option A; ADR-010 Phase 53). Seal trailer is **physical**, not a quantum marker.

## 4. Migration / jsonl export / docs

**Readers:** both layouts. **`store::migrate(dir)`:** load via D-04, write `seg-*`+manifest, verify open ids, leave jsonl as export (or delete after verify — OWNER). **Not** on first `persist` (would flip every test dir and `m012a_verify_persist::persist_replaces_events_jsonl_atomically`). **Writer default after 54b:** empty/`migrate`d dirs write segments; legacy dirs keep jsonl+WAL until migrate. **Export:** `store::export_jsonl` (on demand; optional `close` flag). Never consulted when a manifest exists.

**Must change** (jsonl-as-SoT / covering-WAL D-04). `AGENTS.md` says **event log** = SoT (keep) and `.kutha/events.jsonl` is **harness** process SoT (do not touch). Product file SoT today:

- ADR-000 Clarification 2026-10-03; ADR-010 Evidence 2026-10-03 + D010 store subsection; ADR-012 Evidence 2026-10-03 + Phase 51 “jsonl remains SoT”; ADR-010/012 Phase 52 append dual-write
- `store.rs` `load_events_d04` comment; `rocks_spike.rs` EventId→jsonl offset (S07)
- `.kutha/ROADMAP.md` M002 thaw (“keeps events.jsonl as SoT”) + S06 success line; `.planning/REQUIREMENTS.md` DOC-06; `docs/architecture/p0-spike-inventory.md`
- Governor: `checks.yaml` `m012a-s04-verify-persist`; `bridges.yaml` `B-m012a-s04` (atomic jsonl rename — keep as **legacy** oracle)
- Tests: `m017_replay_integrity.rs` (jsonl fallback); `m002_append01.rs` / `m002_snap01.rs` / `m017_verify_killtests.rs` jsonl tails; `m012a_verify_persist.rs`
- Dated history only (do not rewrite as living SoT): CHANGELOG, `.planning/milestones/v0.10-phases/42-*`, Phase 45 CONTEXT/VERIFICATION, prior-art 2026-10-03 notes

Honeycomb stays **Proposed**. New ADR-010/012 Evidence dated 2026-10-04; do not silently rewrite D1–D10 / D010-*.

## 5. Sub-slices (Phase 54 too big)

| Slice | Scope | Oracles (`m002_seg01.rs` + keep m002/m017) | Kill-mutations | Effort | Bench |
|-------|--------|--------------------------------------------|----------------|--------|-------|
| **54a** format+manifest+open | `segment.rs`; `open` branches on manifest; persist still legacy | round-trip tiny segmented dir; missing sealed/active `InvalidData`; truncated sealed `InvalidData`; torn active drop; jsonl ignored when manifest exists; legacy `m017` jsonl fallback still green | skip manifest detect (read jsonl); torn-drop a truncated **sealed**; open without trailer on sealed | M | `open.n` +SHA256 on fixture (expect ≪10%); ingest/`as_of` untouched |
| **54b** Persister+rollover | attach/append/checkpoint on active only; one `sync_data`; rollover+manifest flip; `persist` writes segments on new dirs | prefix sealed bytes unchanged; one log sync/quantum; crash-matrix A or B or `InvalidData`; rollover: sealed immutable, new active; `open_for_append` heal | still write jsonl on append; two `sync_data`; skip dir fsync on manifest; rollover without trailer | H | `persist_append.n` should **drop** (no jsonl); cap +10% vs Phase 52/53; ingest/`as_of` untouched |
| **54c** chain+migrate+docs | tamper oracles; `store::migrate`; export; ADR Evidence; ROADMAP/REQUIREMENTS DOC-06; governor claim text | flip/swap/reorder/delete sealed → `InvalidData`; mutate jsonl with manifest → open Ok; migrate ids equal D-04; export not used on open | skip chain verify; CRC-only; auto-convert inside `persist`; read jsonl when manifest exists | M | `open.n` ≤10% vs 54b; no ingest/`as_of` |

FF5 / M010–M017 assertions unchanged. `m002_commit01` still after D-04-or-segment load.

## 6. OWNER vs controller

**OWNER:** (1) Demote product `events.jsonl` from SoT (contradicts ADR-000/010/012 Evidence, ROADMAP thaw, DOC-06) while D1 “event log = SoT” stands — dated Evidence only, honeycomb Proposed. (2) Explicit `migrate` vs silent convert on `persist`. (3) After S06, is jsonl still dual-written (no) or export-only (yes). (4) New hash crate: **no**; `sha2` only. (5) Mixed manifest+jsonl: manifest-wins vs fail-closed. (6) `rocks_spike` offsets stay jsonl until S07.

**Controller:** magic `KUTHASEG`, names, 64 MiB / 65536, per-frame SHA-256, torn-tail **active only**, one `sync_data`, checkpoint sidecars only, 54a→54b→54c, ignore unnamed leftover segs, no commit-frame revival.
