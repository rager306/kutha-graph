# Rule pin scheme compatibility (G4 / CE #24)

Process note, not an ADR and not honeycomb Acceptance. Controller policy for
`rule_definition_hash` after the RULE-01 Named AST pin (M016).

## Decision (2026-10-11)

**Policy: documented rotation to pin scheme v1; no silent remap of legacy UTF-8 pins.**

| Vintage | Ident input (e.g. `derive_pq`) | Non-ident free string |
|---------|--------------------------------|------------------------|
| Pre–RULE-01 / main-era | SHA-256(`kutha-rule-def` ‖ UTF-8 bytes) | Same domain+UTF-8 |
| Pin scheme **v1** (current) | SHA-256(`kutha-rule-def` ‖ canonical `RuleAst::Named` bytes) | Domain+UTF-8 (fail-closed for admit) |

Reasons:

1. Ident pins that equal raw UTF-8 domain digests invalidate the Named AST contract and let lazy encoders pass.
2. Remapping stored main-era pins onto v1 would launder Behavior `rule_version` without an audited migrate. Operators rebuild or discard caches that still hold UTF-8 pins — until an **explicit** directory migrate runs (M071).
3. CE #24 minimum path is to **version/document the rotation** and lock **hardcoded stability vectors** so Hyrum drift fails closed in CI.

## Stability vectors (pin scheme v1)

Hardcoded in `crates/kutha-common/tests/m051_pin_compat.rs`. Do not recompute in the oracle body beyond calling `rule_definition_hash` / `rule_body_hash`.

| Input | Kind | Expected pin (hex) |
|-------|------|--------------------|
| `derive_pq` | ident → Named AST | `2dca8c1394bfd203753efd7c991030c23719b41b285aad78ef061103b316596d` |
| `derive_pq` | legacy UTF-8 (must **not** equal v1) | `7ce5d5ad5ca8d5148305d84d3eee3f1356540ad44da56fac9fdde748603436db` |
| `not a rule ident!!!` | non-ident → domain+UTF-8 | `ac715049d46ceaeae121295921ba657df849db9fee9fdb1203201e7b7ba7a689` |

## Directory rewrite (M071 / M072)

**Shipped (M071):** `store::migrate_pin_wire` rewrites a store directory (jsonl /
WAL-only / segmented sources, same recovery paths as Op-wire). Requires modern
`Event` JSON (run `migrate_op_wire` first when meta-op envelopes remain). Remaps
`Op::Behavior` `rule_version` only when `name` is a rule ident and the pin
equals the legacy domain+UTF-8 digest; stamps `pin-wire.v1`. Idempotent when
the marker exists.

**Shipped (M072):** `store::migrate_pin_cite` remaps `Op::JustificationCite`
`rule_version` when the pin equals a legacy UTF-8 ident digest for a rule name
**evidenced in the same log** (`Behavior.name` or rule-ident `Define`) via
`remap_events_pins`; stamps `pin-cite.v1`. Cites with no evidenced ident stay
unchanged (no hardcoded name list). Idempotent when the marker exists. After
`pin-cite.v1` is stamped, evidence added later never remaps orphans on rerun;
operators must delete pin-cite.v1 and re-run `migrate_pin_cite` (or
`migrate_pins` / `migrate_compat`) to rematerialize.

**Shipped (M075):** `store::migrate_pins` runs `migrate_pin_wire` then
`migrate_pin_cite` (reuse; no new remap rules) so operators cannot skip the
Behavior step before cite remap. Both markers stamped when needed; idempotent
when both already exist.

**Shipped (M076):** `store::migrate_compat` runs `migrate_op_wire` then
`migrate_pins` so meta-op envelopes are rewritten before pin remap (reuse; no
new remap rules). Prefer this when a directory may still hold removed meta-ops.

**Shipped (M078):** `kutha-migrate` is the operator CLI for that sequence
(`kutha-migrate <store-dir>` or `KUTHA_MIGRATE_DIR`).

**Shipped (M080/M081):** prefer the observe-built path
`$CARGO_TARGET_DIR/debug/kutha-migrate <store-dir>` (default
`target/debug/kutha-migrate`) — not a second `cargo run` compile.

Serde still accepts legacy pin strings on open; admit stays fail-closed for
UnknownRuleVersion until pins match scheme v1 or these migrates run. Helpers
live in `kutha_common::pin_migrate` — not a silent change to
`rule_definition_hash`.

## Related G4 residuals

Clock-safe facade shipped as M025 — **not** CE whole-repo finding #26
(delivery-key same-relation; closed M056-S03). Op wire break and layout migrate
remain in `docs/process/op-compat.md`. See
`docs/process/ce-review-regression-map.md`. Honeycomb cells stay **Proposed**.
