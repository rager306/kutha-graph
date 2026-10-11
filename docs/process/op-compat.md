# Op wire compatibility (G4 / CE #14)

Process note, not an ADR and not honeycomb Acceptance. Controller policy for
persisted product logs after the M013 dictionary-as-facts rewrite.

## Decision (2026-10-10)

**Policy: documented break; serde backfill deferred.**

| Vintage | Wire shape | `store::open` / deserialize |
|---------|------------|-----------------------------|
| Pre-dictionary (main-era / pre–Phase 26) | `Op::AllowRelation`, `RegisterRule`, `PinPolicy`, `RecordAdmission`, `RecordAction` | **Fail open** (`InvalidData` / serde error). No silent skip. |
| Current writers | `Op::Assert` on reserved `kutha:allows` / `kutha:rule` / `kutha:policy` / `kutha:admission` / `kutha:action` (+ `StoreBlob` for bodies) | Supported |

Reasons for deferring backfill in this slice:

1. Legacy variants carry string names / definition bodies and `ValidTime` clocks; current `Assert` needs `TermId` intern slots that exist only after dictionary bind.
2. A wrong TermId remap would launder dictionary truth. Remap belongs in a leased backfill with fixtures, not a silent serde alias.
3. CE #14 minimum path is to **record the break**; this file plus CHANGELOG and the `m050_op_compat` oracle are that record.

## Follow-on (not leased here)

Named future slice (controller): serde/open migration that maps the five legacy variants onto dictionary Asserts (and blobs where needed) with a format marker or explicit migrate step. Until leased, operators must rebuild or discard main-era dirs.

## Related G4 residuals

Pin scheme rotation (#24) is recorded in `docs/process/pin-compat.md` (M051).
Clock `Into` facade (#26) and other contract items stay on the G4 queue.
Honeycomb cells stay **Proposed**.
