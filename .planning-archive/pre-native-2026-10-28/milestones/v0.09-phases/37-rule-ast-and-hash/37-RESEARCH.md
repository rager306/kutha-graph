# Phase 37 Research: Rule AST and hash

**Date:** 2026-10-02. **CBM:** `root-kutha-graph` ready, indexed 2026-10-02T09:06:03Z. `event.rs` / `admission.rs` / `fold_query.rs` / `quantum.rs` freshness `metadata_changed` — source + `rg` arbiter (D-06). GitNexus unused (outside M013). `search_graph` name `RegisterRule` = 0 (Op removed M013); BM25 hits `emit_rule`, `rule_definition_hash`, `m012_rule_registry`.

<user_constraints>
## Locked decisions (CONTEXT, verbatim intent)

- **D-01:** Build on existing RegisterRule / definition-hash pin (`m012_rule_registry`). Canonical body encoding is explicit (struct or bytes), not free-string `rule_version`.
- **D-02:** Unknown or non-canonical bodies fail closed at admit (extend existing mismatch path).
- **D-03:** Do not delete `inverse_knows` cascade yet (Phase 38). Do not implement full evaluator.
- **D-04:** Evaluator (38); policy replay (39); golden fixture (40); M002; honeycomb Accepted.
- **D-05:** Verifier ≠ executor. Do not write `37-VERIFICATION.md`.
- **D-06:** CBM graph-first; GitNexus secondary; `rg` arbiter.
</user_constraints>

## Current surface (rg + snippets)

| Piece | Evidence |
|-------|----------|
| Body | `emit_rule` hashes UTF-8 `definition: String`, `StoreBlob` + `kutha:rule` Assert (`pipeline.rs` ~331). `Op::RegisterRule` is gone (M013). |
| Hash | `rule_definition_hash` = SHA-256(`kutha-rule-def` \|\| utf8). Pin is 64-char lowercase hex. |
| Pin | `Behavior.rule_version` is still `String`. `gate_user_op` requires `rule_hash_live_at`. Free-string pins (`abc`, `""`) already fail closed. |
| Admit blob | `blob_digest_matches` accepts raw SHA-256 **or** utf8 `rule_definition_hash(s)` **or** policy hash. Any non-empty utf8 is a registerable rule body. |
| Oracles | `m012_rule_registry` five tests; FF5/FF6 have **no** `emit_rule`. `derive_pq` / `other-def` are identifier-shaped names. |
| Residual | `follow_ons` still mints `inverse_knows` with empty pin (`pipeline.rs` ~454). |

F9 hole: bodies are opaque strings; hash is of those strings, not of a canonical AST.

## Design lock (this phase)

1. **`RuleAst` in `kutha-common`** (new `rule.rs`): `Named { name }` and `Horn { head, premises }`. Positive only. Bounds: ident `[A-Za-z][A-Za-z0-9_-]{0,63}`; Horn premises 1..=8; vars 0..=15. No JSON as SoT.
2. **Canonical bytes:** tag `0x01` Named / `0x02` Horn; `u16` LE length prefixes; exact consume (`from_canonical` rejects trailing/unknown tag). `encode(parse(encode(x))) == encode(x)`.
3. **`rule_version` IS `rule_body_hash(canonical)`** = SHA-256(`kutha-rule-def` \|\| canonical bytes) hex. `rule_definition_hash(&str)` becomes hash of `Named` canonical encoding of that ident so `rule_definition_hash("derive_pq")` call sites stay (rename-only oracles).
4. **`emit_rule(String)`** accepts only valid idents, stores **tagged** bytes (not raw utf8). `emit_rule_ast(&RuleAst)` registers Horn. Non-ident / JSON / trailing junk → `UnknownRuleVersion`, log unchanged.
5. **`blob_digest_matches`:** rule-domain match only if `from_canonical(bytes)` succeeds and digest equals `rule_body_hash(bytes)`. Policy utf8 path unchanged. Raw SHA-256 path unchanged.
6. **Non-goals:** no evaluator, no cascade delete, no `.kutha/STATE.md`, no `kutha-gov ci`, no `37-VERIFICATION.md`.

**Primary recommendation:** Two plans — AST+canonical+hash in `kutha-common`; then admit/emit wiring + `m016` fail-closed tests with M012/FF5/FF6 green.
