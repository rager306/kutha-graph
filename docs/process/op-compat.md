# Op wire compatibility (G4 / CE #14 / CE #1)

Process note, not an ADR and not honeycomb Acceptance. Controller policy for
persisted product logs after the M013 dictionary-as-facts rewrite.

## Decision (2026-10-12)

**Policy: documented break; open fail-open; explicit migrate for AllowRelation, RegisterRule, PinPolicy, RecordAdmission, and RecordAction (serde aliases forbidden).**

| Vintage | Wire shape | `store::open` / deserialize | Explicit migrate |
|---------|------------|-----------------------------|------------------|
| Pre-dictionary (main-era / pre–Phase 26) | `Op::AllowRelation` | **Fail open** (`InvalidData` / serde error). No silent skip. | **Leased (M062-S01):** `remap_legacy_ops` → `kutha:allows` Assert via caller intern |
| Pre-dictionary | `Op::RegisterRule` | **Fail open** | **Leased (M063-S01):** `remap_legacy_ops` → `StoreBlob` + `kutha:rule` Assert (`RuleAst::named`) |
| Pre-dictionary | `Op::PinPolicy` | **Fail open** | **Leased (M064-S01):** `pin_policy_to_ops` → `StoreBlob` + `kutha:policy` Assert (`policy_version_hash`) |
| Pre-dictionary | `Op::RecordAdmission` | **Fail open** | **Leased (M065-S01):** `record_admission_to_assert` → `kutha:admission` Assert (mirrors `record_justification`) |
| Pre-dictionary | `Op::RecordAction` | **Fail open** | **Leased (M066-S01):** `record_action_to_assert` → `kutha:action` Assert (mirrors `record_action`) |
| Current writers | `Op::Assert` on reserved `kutha:allows` / `kutha:rule` / `kutha:policy` / `kutha:admission` / `kutha:action` (+ `StoreBlob` for bodies) | Supported | n/a |
| Current writers (2026-10-22 / M154) | `Op::Checkpoint` | **Fail open** (unknown serde variant). No aliases; no `remap_legacy_ops` entry; no new `op-wire.v1` meaning. | n/a — do **not** run `migrate_op_wire` on a checkpointed store (it demotes segments) |

Reasons serde aliases stay forbidden:

1. Legacy variants carry string names / definition bodies and `ValidTime` clocks; current `Assert` needs `TermId` intern slots that exist only after dictionary bind.
2. A wrong TermId remap would launder dictionary truth. Remap belongs in an explicit migrate with caller-supplied intern, not a silent serde alias.
3. CE #14 minimum path recorded the break; M062–M066 lease explicit remaps.

## Directory rewrite (M067 / M068 / M069)

**Shipped:** `store::migrate_op_wire` rewrites a store directory:

- **jsonl** (M067): line-wise Values via `rewrite_event_json_line`
- **WAL-only** (M068): frame payloads via `wal::recover_json_values`, then the
  same remap path
- **segmented** (M069): sealed+active KUTHASEG payloads via
  `segment::recover_json_values`, then the same remap path; demotes to legacy
  jsonl (drops segments + `manifest.json`)

Materializes `events.jsonl`, drops the prior WAL / segmented layout and stale
`snapshot.json`, and stamps `op-wire.v1`. Idempotent when the marker exists.
Open stays fail-open until the operator runs this step (`recover_events` /
sealed `scan_segment` return `InvalidData` on CRC-good frames that are valid
JSON but not a modern `Event` — no silent drop of legacy meta-ops).

Do **not** run `migrate_op_wire` (or `migrate_compat` / `kutha-migrate`) on a
store that already carries `Op::Checkpoint`: the rewrite demotes sealed
segments to legacy jsonl and would drop the receipt layout. `Op::Checkpoint` is
a modern serde variant; old readers fail open. No serde aliases.

**Shipped (M076):** `store::migrate_compat` runs `migrate_op_wire` then
`migrate_pins` (reuse; no new remap rules) so meta-op rewrite precedes pin
remap. Stamps `op-wire.v1` and both pin markers when needed; idempotent when
all already exist. See `docs/process/pin-compat.md`.

**Shipped (M078):** operator binary `kutha-migrate` (`crates/kutha-runtime/src/bin/kutha-migrate.rs`)
calls `store::migrate_compat` on `<store-dir>` or `KUTHA_MIGRATE_DIR`. Prints a
`migrate:` report line; exit 2 on missing dir / bad args. No new remap logic.

**Shipped (M080/M081):** prefer the observe-built path
`$CARGO_TARGET_DIR/debug/kutha-migrate <store-dir>` (default
`target/debug/kutha-migrate`) — not a second `cargo run` compile.

## Related G4 residuals

Pin scheme rotation (#24) is recorded in `docs/process/pin-compat.md` (M051);
explicit Behavior pin-wire migrate is M071 (`migrate_pin_wire` + `pin-wire.v1`).
Clock-safe facade shipped as M025 (trybuild `ui_clock_facade`) — **not** CE
whole-repo finding #26 (that finding was delivery-key same-relation; closed
M056-S03). Op-wire directory layouts shipped M067–M069; demote orphan
`seg-*.log` cleanup is M070; compat sequence facade is M076. See
`docs/process/ce-review-regression-map.md`. Honeycomb cells stay **Proposed**.
