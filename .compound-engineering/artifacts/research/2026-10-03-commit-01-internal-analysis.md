# COMMIT-01 internal analysis (M002 S05)

**Status:** decision note (not an ADR rewrite). **Lease:** M002 paused before S05. **CBM:** project `root-kutha-graph`; `persist_append.rs` / `store.rs` / `wal.rs` / `hydrate.rs` `metadata_changed` or `not_tracked` — claims below from source (`rg` arbiter). Intake `m002-cloud-kernel-io-intake.md` is IO-01 (writeback stalls), not this gate.

**Recommendation: A** — keep `Op::QuantumOutcome` as the User-quantum commit marker; on `open`, drop a **typed uncommitted suffix**. Do **not** pick D (write-time env dual-encoding). B/C wait for S06 segments.

## 1. What a quantum is today

One **User** `emit` (`pipeline.rs` `emit` 442–445 → `pipeline` → `pipeline_kind` 185–209 → `commit_user_cascade` 243–276):

1. `reject_forged_meta` / `gate_user_op` / `admit` / `admit_claim` (no log write).
2. `delivery_key_retry` may return `events_in_quantum: 0` with **no append** (`admission.rs` 225–247).
3. Else: `Event::new(op)` then a DFS of `rule_follow_ons` (`eval.rs` 28+): each node `commit_event` (fold + `append_indexed`) then enqueue Horn `Op::Behavior` follow-ons until idle or `max_cascade`.
4. `buffer_receipt` then `persist_outcome_event` (174–178): **one extra** `Op::QuantumOutcome` via `commit_event`. Receipt `event_ids` are payload only; the QO event is **not** in that vec.

Order: payload (Assert/Retract/Correct/Behavior…) then **always last** QO for this path. Not nested: follow-ons share one quantum / one QO. Budget abort still writes QO with `Partial`/`Zero` (`disposition` `quantum.rs` 199–204). `StoreBlob`/`TtAnchor` on User `emit` re-enter `ReceiptMode::Dictionary` (194–196) — **no** QO.

`ReceiptMode` (`pipeline.rs` 11–17): User → QO; Dictionary (`intern` Define, `emit_allow`/`emit_rule*`/`emit_policy`/`emit_tt_anchor`) → no QO; Typed (`record_justification` `justification.rs` 128) → in-memory receipt, log cites/admission, **no** QO. `intern` (`quantum.rs` 335–348) appends `Op::Define` outside the pipeline. Failed `admit`/`gate` leaves the log unchanged for that op; `assert()` may already have interned Defines.

## 2. Crash between payload and outcome

In-memory log is **not** durable. `Persister::append` (`persist_append.rs` 91–147) is the **runtime encoded suffix** (`collect_new` 234–266: new `stable_define`s then graph events after `durable_graph_count`), not “one quantum”. Several `emit`s before one `append` are one batch. A batch **can** contain a partial last quantum: jsonl writes line-by-line (122–125), WAL one frame per event (`wal.rs` `write_frames` 41–45), **one** `sync_data` after all frames (136). Crash after complete payload lines/frames and before the QO line/frame leaves a readable uncommitted prefix. `recover_events` (115–171) drops only a torn last **frame**; `read_jsonl_events` (237–255) drops bytes after the last `\n`. `load_events_d04` (`store.rs` 274–321) then prefers the covering id sequence — so complete payload-without-QO **is visible** today.

| Option | Recovery | Filter home |
|--------|----------|-------------|
| **A** | After D-04 merge, drop typed suffix (see §5). | `store::open` after `load_events_d04` (323), **before** `from_snapshot` / `hydrate_from_log`. Not WAL recover (no quantum). Not hydrate-only (tail would already `fold.apply`). |
| **B** | Commit flag frame + jsonl marker line. | WAL v2 + `read_jsonl_events` (non-`Event` lines break today’s parser). |
| **C** | One CRC envelope per quantum. | `write_frames` / `last_event_id_at_end` (`wal.rs` 49–88); O(1) tail guard uses **last event** `frame_len`. |
| **D** | Writer env; reader both. | All of the above plus `CommitMode::from_env`. |

Heal: in-memory trim without rewriting files leaves `Persister::attach` seeing a dirty tail (`tail_guard` 183–200). Minimal A: trim on `open`; fail-closed `attach` until full `persist` (or rewrite prefix in `open`).

## 3. Interactions

- **replay_check** (`provenance.rs` 19–41): fold fingerprint + exact `TtAnchor` map + Behavior lineage. Uncommitted Asserts would fold; A must trim **before** `open`’s `replay_check` (342).
- **provenance_fingerprint** (76–104): mixes QO/`JustificationCite`/`StoreBlob` digests. Dropping a suffix without QO does not change a sealed provenance; dropping a QO would.
- **Define (Phase 52):** `encoded_log` synthesizes `stable_define` prefix; live Defines stripped (`store.rs` 406–418). `intern_between_appends_survives_open_without_checkpoint` / `intern_only_append_reattach_and_open` (`m002_append01.rs` 566–617) require Define-only suffixes to **survive**. Naive “drop after last QO” kills them.
- **TtAnchor:** Dictionary, no QO. `check_tt_anchors_range` (`provenance.rs` 54–69); SNAP-01 log-ahead vs `snap.next_tt`. Crash-matrix uses `emit_tt_anchor` (`m002_append01.rs` 91–93) — **does not** exercise payload-without-QO.
- **`quantum_outcomes.jsonl`:** lease; SoT is log QO (`store.rs` 13–14). `outcomes_from_events` (`hydrate.rs` 30–56); sidecar prefix-ok (`sidecar_or_stale_prefix` 392–403). Checkpoint refreshes sidecar (`persist_append.rs` 170); between checkpoints sidecar lag is already accepted (ADR-014 Evidence Phase 52).
- **`snapshot.log_offset`:** `graph_len` counts `op_affects_fold` only (`quantum.rs` 223–231, 391–392) — QO/Define/TtAnchor/StoreBlob/Cite excluded. `from_snapshot` (`hydrate.rs` 79–101) tail-folds by that offset. Trim uncommitted **product** ops so offset stays aligned.
- **O(1) tail guard:** last jsonl `EventId` vs WAL last frame of `last_frame_len`. A does not change frame layout. C would.

## 4. Minimal A vs D

**A (ship):** `fn committed_prefix(events: &[Event]) -> Vec<Event>`: walk; last QO commits listed `event_ids` plus the QO event; suffix after last QO: keep **self-commit** ops; cut from first **requires-outcome** op. Self-commit: `Define`, `TtAnchor`, `StoreBlob`, `JustificationCite`, `QuantumOutcome`, dictionary `Assert` (`kutha:*`). Requires-outcome: product `Assert`/`Retract`/`Correct`/`CorrectInterval`/`Behavior`. No QO in the file (genesis) → keep all (all genesis rows are self-commit). Writer unchanged.

**D (reject for S05):** `enum CommitMode { Outcome, Frame, Envelope }` via `apply_dotenv` (`allow.rs` 9–40) like `FoldIndexMode::from_env` (`fold.rs` 189–205) / `KUTHA_MAX_CASCADE` (`quantum.rs` 236–242). Suggested `KUTHA_COMMIT_MODE`, default `outcome`; reader accepts v1 frames + QO trim **and** v2 envelopes. Dual crash matrices; `KUTHA_MAX_CASCADE=0` pitfall class (`m011_quantum_outcome.rs` 8). WAL `VERSION` is `1` (`wal.rs` 9, 139). Leave dual-read to a later **migration** slice if S06 introduces envelopes.

Files (A): `store.rs` (`committed_prefix`, `open`); tests `m002_append01.rs`; Evidence lines ADR-010/014/060/012. Not `pipeline` unless Dictionary holes are closed. D would add `quantum.rs`/`allow.rs`/`.env.example`/`wal.rs`/`persist_append.rs`.

**Oracles (keep + add):** keep `m002_append01` prefix/sync/torn/guard/crash-matrix/intern-only; `m002_snap01` sidecar/tt-ahead; `m017_replay_integrity` WAL-cover/tt-tamper/outcomes-spoof; `m011_quantum_outcome` Zero/Partial/Full + `crash_after_prefix_has_no_terminal_success_until_explicit_resume`; `m012a_log_native` discard-sidecar. **New:** `append_user_quantum_payload_without_qo_not_visible`; `intern_only_suffix_kept_when_no_qo`; `tt_anchor_suffix_kept`; `justification_typed_pair_policy`; failpoint after N−1 frames of Assert+QO. **Kill-mutations:** naive drop-after-last-QO (must fail intern-only); keep product Assert without QO; trim inside `hydrate_from_log` only; env writer Frame / reader Outcome-only; count QO in `log_offset`.

## 5. Risks if payload has no QO

Naive A silently drops: intern-only Defines (Phase 52); genesis `kutha:allows`/`StoreBlob`/rule Asserts (`Runtime::new` 301–308); `TtAnchor`; Typed cite+admission; `emit_rule_ast` blob-then-assert. Retract/correct **User** paths do write QO — safe under typed A. Typed/Dictionary multi-event groups remain a residual hole (no QO by design, `+1/+2` log-growth oracles). B/C without grouping those paths have the same hole. D can hide a bad writer behind “the other mode”.

## 6. Effort and bench

A: **1 slice** after this gate (filter + oracles + Evidence). Optional heal-rewrite: **+0.5**. B/C: **2–3 slices** (WAL v2, jsonl, tail guard, migrate) — overlaps S06. D: **+1 slice** of matrix with no durability gain. Bench: A is an extra open walk (negligible vs replay). QO still ~+1 event per User emit (F11 3× size stays until S06). Do not expect `ingest_plain.n` / `as_of.n` movement if writer unchanged.
