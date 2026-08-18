# Legal PIT fixture (FF5)

Three interned facts, statute-shaped. Not a corpus. Not Cypher.

| Term | Role |
|------|------|
| `art-12-speed` | subject (article) |
| `inForceAs` | relation |
| `50-kmh` | object, in force `[2010, 2020)` |
| `30-kmh` | object, in force `[2020, ∞)` |

Cuts: `as_of(2015)` ≠ `as_of(2021)`. Cosine is not the oracle. CSR lease at the same cuts must disagree the same way.

See `crates/kutha-runtime/tests/ff5_legal_pit.rs`.
