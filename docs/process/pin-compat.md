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
2. Remapping stored main-era pins onto v1 would launder Behavior `rule_version` without an audited migrate. Operators rebuild or discard caches that still hold UTF-8 pins.
3. CE #24 minimum path is to **version/document the rotation** and lock **hardcoded stability vectors** so Hyrum drift fails closed in CI.

## Stability vectors (pin scheme v1)

Hardcoded in `crates/kutha-common/tests/m051_pin_compat.rs`. Do not recompute in the oracle body beyond calling `rule_definition_hash` / `rule_body_hash`.

| Input | Kind | Expected pin (hex) |
|-------|------|--------------------|
| `derive_pq` | ident → Named AST | `2dca8c1394bfd203753efd7c991030c23719b41b285aad78ef061103b316596d` |
| `derive_pq` | legacy UTF-8 (must **not** equal v1) | `7ce5d5ad5ca8d5148305d84d3eee3f1356540ad44da56fac9fdde748603436db` |
| `not a rule ident!!!` | non-ident → domain+UTF-8 | `ac715049d46ceaeae121295921ba657df849db9fee9fdb1203201e7b7ba7a689` |

## Follow-on (not leased here)

A migrate that rewrites persisted Behavior `rule_version` from UTF-8 pins to v1 Named pins needs its own lease and fixtures. Until then, out-of-repo caches holding main-era ident pins stay UnknownRuleVersion under v1 admit.

## Related G4 residuals

Clock `Into` facade (#26) and other contract items stay on the G4 queue. Op wire break remains in `docs/process/op-compat.md`. Honeycomb cells stay **Proposed**.
