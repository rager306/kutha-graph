# ROCK-02: index staleness beyond the tip

**Date:** 2026-10-05. **Phase:** 61 / M019 S06. **Status:** design note (not an ADR). **Hard stop:** event / snapshot / `KUTHASEG` frame change → stop; do not implement.

**Problem.** `SegmentIndex::open_or_rebuild` keeps an existing Rocks dir when `m:tip` matches the log (last sealed chain hex, committed count, last EventId) and `get_event(tip.last)` succeeds. A poisoned or bit-rotted `e:` locator in the middle therefore survives until a later `get_event` re-reads the frame (`InvalidData`). `catch_up` is specified not to rewrite that prefix (`catch_up_indexes_only_the_tail`). The index is a droppable lease: never SoT, never payloads (Phase 55 `rocks_values_hold_no_payload`).

**Writer today.** `write_meta` stores `m:v` (u32be=1) and `m:tip` (88 bytes). Build/catch_up scan the verified segmented log. Locators are seq+offset+frame_len. No per-sealed binding, no index digest.

## Options

### (a) Per-sealed-segment binding (always-on)

Store each sealed manifest entry (seq + 64-char chain hex) under `m:` and compare the whole sealed list to the current manifest on open. O(#segments), no key scan, no extra log IO beyond the tip path that already calls `located_committed`.

- **Detects.** Any change of a sealed segment's identity in the manifest (middle file replaced, list length/order/chain hex changed) while `m:tip` still matches the last sealed chain + count + last id.
- **Does not detect.** A poisoned `e:` value while the sealed list and tip still match. That is (b) plus existing `get_event`.
- **Format.** Rocks meta only. Bump `m:v`; old dirs destroy+rebuild. Log/segments unchanged.
- **Oracle.** Tamper stored sealed bindings (keep `m:tip`); `open_or_rebuild` rebuilds locators. Kill: skip the sealed-list compare.

### (b) Index digest + `verify_full()` (opt-in)

At build/catch_up, store a digest of every `e:` key||value (Rocks iterator order). `verify_full()` rescans those keys (O(N) keys, no log IO) and errors on mismatch or `e:` count ≠ tip count.

- **Detects.** Locator bit-rot / poison while `m:tip` matches. Cheap vs re-reading frames.
- **Does not detect.** An adversary who rewrites both locators and the digest (still a lease; `get_event` CRC/id check remains). Does not replace (a) for a sealed-file identity change that leaves locators internally consistent.
- **Format.** Extra `m:` digest key; same `m:v` bump as (a).
- **Oracle.** Poison a middle locator; `verify_full` is `InvalidData`; `get_event` stays `InvalidData`. Kill: `verify_full` returns `Ok`.

### (c) RocksDB paranoid checks only

Enable `paranoid_checks` / SST checksums. Detects engine corruption, not a semantically wrong locator written with a valid checksum. Insufficient for ROCK-02. Reject as the always-on check.

## Verdict

**(a) always-on in `open_or_rebuild`; (b) as explicit `verify_full()`.** (c) is not a substitute. Poisoned middle locator + matching tip: `open_or_rebuild` may keep the dir (sealed list still matches); `verify_full` and `get_event` fail closed. Sealed-list mismatch vs manifest: rebuild. `m:v` 1→2 (v1 never read). No log/segment/event format change. Not wired into `Runtime`.
