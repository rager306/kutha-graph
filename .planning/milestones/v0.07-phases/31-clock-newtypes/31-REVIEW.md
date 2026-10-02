# Phase 31 — Code review (clock newtypes)

**Branch:** `chore/m013-roadmap-reset`  
**Scope:** `e6200ef..HEAD` (`feat(31-01)` … `docs(31-02)`) — `crates/` only  
**CBM:** `detect_changes` (inbound; large working-tree delta — use git range for this review)  
**Focus:** newtype correctness, assertion edits, `as_known_on` / `tt_anchor` scope creep  

## Verdict

**Clean** — no findings at confidence ≥ 80.

## Scope checked

| Area | Result |
|------|--------|
| Newtypes (`ValidTime`, `TransactionTime`, `WallTime`, `Vt`/`Tt`) | Tuple structs, `serde(transparent)`, `From`/`Into` u64, no cross-clock `From`; `compile_fail` doctest passes (`cargo test -p kutha-common --doc`) |
| Query / cut APIs | `impl Into<TransactionTime>` + `impl Into<ValidTime>` with `as_u64()` at boundaries; fold storage and `Op`/`Event`/`Fact` clock fields remain `u64` per D-04 |
| Assertion edits | Only TIME-01 constructor wraps in `m012a_time_scale.rs`; `assert_eq!(y*, …)` bodies unchanged; no FF5/FF6 test file diffs in range |
| Phase 32 / anchors | No `as_known_on` or `tt_anchor` symbols added; `WallTime` doc comment only references Phase 32 |

## Commits reviewed

```
8967861 docs(31): create phase plan
804c8b4 feat(31-01): clock newtypes and compile_fail mix
721b718 feat(31-01): fold query takes distinct clocks via Into
6898e6d feat(31-01): Runtime as_of takes ValidTime via Into
ae0cf41 docs(31-01): complete clock newtypes plan
ae0cf41..828a6c9 feat(31-02): CSR / justification / derivation cuts
820d7ac test(31-02): TIME-01 ValidTime::from bindings
d3d9685 docs(31-02): complete remaining cuts and TIME-01 plan
```

## Severity findings

_None (Critical 90–100, Important 80–89)._

## Notes (below review bar)

- Bare `u64` literals still type-check for both cut parameters (`Into` from u64); cross-clock safety is at typed-value call sites — matches phase plan (D-04 / oracle literals).
- `GraphFold::as_of` still passes `u64::MAX` for transaction-time; behavior unchanged from pre-31.

**Reviewer:** code-reviewer subagent · **Date:** 2026-10-02
