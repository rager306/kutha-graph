# Post-M002 ADR semantics audit (HEAD 36fedb9)

Date: 2026-10-04. Read-only. CBM `root-kutha-graph` (indexed 2026-10-04T14:36:49Z; cited paths `metadata_match`). GitNexus `kutha-graph` (same HEAD). Status of every honeycomb cell remains **Proposed**. This note is not a promotion packet.

## 1. ADR claims vs shipped code

**HIGH — Physical SoT.** D1 (ADR-000:106) still draws `Storage: RocksDB / object store + WAL`. D010-1 (ADR-010:70–72) names “the append-only event log” as sole SoT. D010-4 (ADR-010:92–95) names RocksDB `WALRecoveryMode` as the crash cousin; Clarification 2026-10-03 (ADR-010:117; ADR-000:184) still says `events.jsonl` is SoT + covering `KUTHAWAL`. At HEAD, `store::persist_until` (`store.rs`) writes jsonl+WAL only when `writes_legacy_layout`; else `segment::persist_log` (`segment.rs`) rewrites `KUTHASEG` + `manifest.json`. `open` prefers `segment::load_events` when a manifest exists (`store.rs` load path); jsonl is an unread export after `migrate`. Semantic log = event sequence; **physical** SoT for a new/migrated dir is the manifest+segments. Decision bodies were not rewritten; 2026-10-04 pointers say “does not rewrite D1.”

**HIGH — D6 runtime stack.** ADR-000:176 locks `Rust + Tokio + RocksDB (event store)`. Workspace `Cargo.toml` members are `kutha-common` + `kutha-runtime` only; **no `tokio` crate**. Rocks is optional `rocksdb` behind `rocks-index` / `rocks-index-spike` (`kutha-runtime/Cargo.toml`); GitNexus: `SegmentIndex::open_or_rebuild` callers are `m002_rock01.rs` tests, not `store::open` / `Persister`. ADR-000:184 “RocksDB is NOT a dependency” is also false after Phase 51.

**HIGH — Stale Evidence vs later Evidence (same files).** ADR-010:13 and ADR-012:13 still state SNAP-01 (`File::create` sidecars) and “each `persist` rewrites the whole log” as **current** defects. Later blocks close SNAP-01 (Phase 50) and O(N) on the **append** path (`Persister`, `persist_append.rs`). Full `persist` still O(N): `persist_until` → `encoded_log` → `persist_log` resplits every chunk. ADR-010:29 (Phase 54-01) still says `persist`/`Persister` write jsonl+WAL; 54-02/code contradict for segmented dirs. ADR-014:17 still says COMMIT-01 “remains S05”; ADR-010:25 / ADR-014:25 record option A shipped.

**MED — Commit marker decided in Evidence, still Open in Decision.** ADR-010:111–113 and ADR-014:96–98 leave F11 (framing marker vs `Op::QuantumOutcome`) **open**. Code: `commit::committed_prefix` + `op_commit_class` / `class_with_dict` (`commit.rs`): NeedsOutcome = product `Assert`/`Retract`/`Correct`/`CorrectInterval`/`Behavior`; SelfCommit = `Define`/`TtAnchor`/`StoreBlob`/`JustificationCite`/`QuantumOutcome` and `kutha:*` Assert. `store::open` trims before hydrate/`replay_check`. Residual (ADR-010:25, ADR-014:25) is real: Typed/Dictionary groups have no outcome → crash can expose a partial `Define`-level group. “B/C wait for S06” (ADR-010:25) is stale: S06 shipped segments, not commit frames.

**MED — Hash chain vs D014-3 anchor.** `verify_sealed` (`segment.rs`) checks header `expected_prev`, trailer SHA-256, manifest fields. `persist_log` seeds prev as `[0u8;32]` (H0). ADR-010:37 correctly says a rewrite of every file including the manifest recomputes hashes; **no external ADS/epoch tip**. D014-3 (ADR-014:62) still offers optional Merkle/chain anchor and still names “Rocks WAL” as durability cousin.

**MED — Multi-writer.** ADR-000 D8 (198) mitigates single-writer with “RocksDB/SQLite WAL, batching.” ADR-010:37 residual: **no multi-writer lock**. CBM `search_code` over `crates/kutha-runtime/src`: no flock/`try_lock`. Two `Persister`s on one dir are undefined.

**LOW — README vs crate.** `docs/ADR/README.md` Process still says “Rocks crate … remain unaccepted” while an optional Rocks **index** crate feature exists (not SoT; not wired).

**Not contradictions (aligned):** snapshots remain leases (D012-1; `replay_check` + snapshot `log_offset` vs committed prefix). Rocks payloads stay out of `rocks_index.rs`. Sidecar outcomes are leases of log-native `Op::QuantumOutcome` (D014 F1 target).

## 2. Temporal model — decided / open / no oracle

| Noun | Status | Evidence |
|------|--------|----------|
| Core algebra VT×TT | **Decided** (D013-1, D4) | `Fact` stamps; `live_at(tt,vt)`; `as_of(vt)` = `live_at(u64::MAX, vt)` (`fold_query.rs`) |
| Knowledge-cut `as_known_on` | **Decided** as TT-only | Greatest `Op::TtAnchor` with `wall≤D` then `known_at(tt)`; `Fact::is_known_at` ignores VT (`fold.rs`) |
| Clock types / TT↔wall | **Decided** as newtypes + anchors | `kutha-common` `Vt`/`Tt`/`WallTime`; `Op`/`ingested_at` still carry `u64` (ADR-000:8–13) |
| `as_of` vs derivation eligibility | **Open** (ASOF-ELIG-01) | `as_of` is SUPPORT-LIVE (ADR-013:153). `derivation_eligible_at_walk` (`provenance.rs`) is a separate API; tests use `TransactionTime::MAX` (`m012_multi_hop.rs`). No oracle that `as_of` **must** hide ineligible derived rows |
| Horn at TT=MAX | **Open** (HORN-KC-01) | `horn_fires` → `search_prems(..., u64::MAX)`; `live_horns` uses `is_live_at(u64::MAX, vt)` (`eval.rs`). Char test `known04_horn_fires_when_premise_hidden_on_early_cut`; **no option chosen**. `as_known_on` never re-derives |
| Correct / CorrectInterval re-derive | **Open** (HORN-CORR-01) | `rule_follow_ons` matches Assert/Behavior/Retract only; `_ => vec![]` |
| Retract/Correct miss | **Open** (F2 fail-closed) | `GraphFold::apply` silent no-op if `event_id` missing or already invalidated (`fold.rs`) |
| C01 sunset / `tempus regit actum` | **Open** (FIXG-01) | Two axes + `as_known_on` **not** sufficient (`fixg01_c01_sunset_inexpressible_as_horn`). Options a/b/c unchosen. Blocks legal pack, not M002 |
| Derived event identity | **Decided UUID v7, not content-hash** | D2 / D011-1; `Event::new` → `Uuid::now_v7()` (`event.rs`); `head_event` uses `Event::new`. Replay must not require byte-identical derived ids (ADR-060:78). **No oracle** for deterministic derived `EventId` |
| Claim / support / delivery | **Decided in 011; spike partial** | `claim.unwrap_or(event.id)` on Assert; Behavior `claim_id = event.id`; `claim_supported_at` (`fold_query.rs`). Oracles exist (`claim_id_distinct_from_support_slot`). How-polynomials (D011-3) **unshipped** |
| Legal AS OF (force vs publication vs decision) | **Mapped, not implemented** | ADR-090 TR-01: five law-nexus clocks are **typed stamps**, not core axes. TR-02 forbids substituting `source_publication` for `legal_act_effect`. FF5 (`ff5_legal_pit.rs`) compares two **VT** windows at TT=MAX — not known-on, not force vs publication. **No oracle** for TR-02 |

## 3. Honeycomb discipline

No ADR `## Status` is **Accepted**. Dated Evidence “SNAP-01 closed / option A / SEG-01” plus honeycomb `capability: named` on 011/013 can be misread as Accepted **behaviour**; `honeycomb.yaml` still has ADR-010/012/014 `capability: none` despite M002 oracles (`m002_snap01`/`append01`/`commit01`/`seg01`/`rock01`) — index lag, not silent promotion.

Cells with **no falsifiable product-cell test** (map-only / frozen / empty `evidence:`): ADR-020 hexagon/`app-shell` (workspace is two crates; D020 non-goal “shipping”); ADR-021 `PackInstall` ops; ADR-030 max-convolution; ADR-050 six dictionaries (frozen; spike = five `kutha:` facts); ADR-070 Cypher/GPML (`evidence: []`); ADR-090 pack (`evidence: []`); D012 vacuum+legal-hold; D014-1 cryptographic ADS receipts; D060 `Unavailable`.

**Overlap / tension (not 011-vs-050 collapse):** D011-2 / D050-3 fence intern map vs control dictionaries — held. Residual: five reserved facts ≠ six kinds (ADR-000:155; ADR-050:9). **012 / 010 / 060 / 014** duplicate persist/WAL/sidecar stories with **internally inconsistent dated blocks**. **000 D6 Rocks event-store vs 010 ROCK-01 droppable locator.** **013 `as_of` vs 070 Cypher AS OF vs 090 five stamps** — 070:42–44 already names three fold cuts; Cypher skin frozen.

## 4. Minimal documentation corrections (do not apply)

| Sev | ADR | Section | Edit idea |
|-----|-----|---------|-----------|
| HIGH | 000 | D1 diagram; D6 table; Clarification 2026-10-03:184 | Replace Rocks/Tokio/jsonl-as-physical-SoT with: semantic log = events; physical encodings = legacy jsonl+`KUTHAWAL` **or** `manifest.json`+`KUTHASEG`; Rocks = optional unwired locator; drop Tokio until a runtime exists |
| HIGH | 010 | D010-4 + Open Q 2026-10-01 | Rewrite D010-4 to SEG-01 + legacy D-04; mark F11 **option A** (`Op::QuantumOutcome` + `committed_prefix`); move B/C to an explicit residual |
| HIGH | 010, 012, 014, 060 | Evidence 2026-10-03 (pre-Phase-50/52/53) | Mark those blocks **historical**; one “current encoding” subsection pointing at HEAD symbols |
| MED | 000 | D8; Exit checklist :350 | D8: single-writer is **unenforced**; segmentation addresses replay tax, not Rocks WAL. Strike “JSONL SoT; Rocks WAL crash cousin” as current |
| MED | 014 | D014-3; Open Q F11 | Cousin = `KUTHAWAL` lease **or** segment `sync_data`; optional external anchor **absent**; close F11 as A |
| MED | 012 | D012-5 last sentence | Remove “does not authorize … M002” (M002 closed) |
| MED | README | Process ¶ “Rocks crate remain unaccepted” | Distinguish unwired `rocks-index` spike vs unaccepted Rocks **event store** |
| LOW | honeycomb.yaml | ADR-010/012 | Either list M002 oracles under `evidence` or state “durability spike, cell still Proposed” |
| LOW | 010:29 | Phase 54-01 Evidence | Delete “persist still writes jsonl+WAL” for segmented writers |
