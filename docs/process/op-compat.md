# Op wire compatibility (G4 / CE #14 / CE #1)

Process note, not an ADR and not honeycomb Acceptance. Controller policy for
persisted product logs after the M013 dictionary-as-facts rewrite.

## Decision (2026-10-12)

**Policy: documented break; open fail-open; explicit migrate for AllowRelation and RegisterRule (serde aliases forbidden).**

| Vintage | Wire shape | `store::open` / deserialize | Explicit migrate |
|---------|------------|-----------------------------|------------------|
| Pre-dictionary (main-era / pre–Phase 26) | `Op::AllowRelation` | **Fail open** (`InvalidData` / serde error). No silent skip. | **Leased (M062-S01):** `remap_legacy_ops` → `kutha:allows` Assert via caller intern |
| Pre-dictionary | `Op::RegisterRule` | **Fail open** | **Leased (M063-S01):** `remap_legacy_ops` → `StoreBlob` + `kutha:rule` Assert (`RuleAst::named`) |
| Pre-dictionary | `PinPolicy`, `RecordAdmission`, `RecordAction` | **Fail open** | Deferred (later slices) |
| Current writers | `Op::Assert` on reserved `kutha:allows` / `kutha:rule` / `kutha:policy` / `kutha:admission` / `kutha:action` (+ `StoreBlob` for bodies) | Supported | n/a |

Reasons serde aliases stay forbidden:

1. Legacy variants carry string names / definition bodies and `ValidTime` clocks; current `Assert` needs `TermId` intern slots that exist only after dictionary bind.
2. A wrong TermId remap would launder dictionary truth. Remap belongs in an explicit migrate with caller-supplied intern, not a silent serde alias.
3. CE #14 minimum path recorded the break; M062/M063 lease explicit remaps.

## Follow-on (deferred)

`PinPolicy` / `RecordAdmission` / `RecordAction` (and directory-level rewrite with a format marker) remain unleased. Until those land, operators must rebuild or discard main-era dirs that still contain those variants after AllowRelation/RegisterRule remap.

## Related G4 residuals

Pin scheme rotation (#24) is recorded in `docs/process/pin-compat.md` (M051).
Clock `Into` facade (#26) and other contract items stay on the G4 queue.
Honeycomb cells stay **Proposed**.
