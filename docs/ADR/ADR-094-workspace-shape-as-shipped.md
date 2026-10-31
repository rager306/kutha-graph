# ADR-094: Workspace Shape as Shipped

## Status

**Proposed** (Space — records the P0 workspace the tree actually has; does not
accept ADR-020 or ADR-022, and does not replace either)

## Date

2026-10-31

## Honeycomb coordinates

- Axes: **Space** (primary)
- Depends on: ADR-000 (D6), ADR-020, ADR-022
- Anticipates: the first vertical pack (ADR-090 / ADR-093)

## Context

ADR-020 D020-1 names the target shape `app-shell` + `common` + `modules/<slice>`
with `domain/` / `internal/` / `adapters/` and Ports in `lib.rs`. ADR-022
D022-1 names a slice as its own crate with a hexagon inside. Neither is what
HEAD contains.

HEAD (`Cargo.toml:3`) is a two-member workspace:

- `crates/kutha-common` — 6 modules, 2,160 lines, 48 re-exports. Pure encoding:
  `Event`, `EventId`, `Op`, interned terms, rule AST. Dependencies are `uuid`,
  `sha2`, `serde`, `serde_json`; no `std::fs`, no `std::net`, no `std::process`.
- `crates/kutha-runtime` — 32 declared modules (29 private, `pub mod store`,
  2 feature-gated), 13,928 lines under `src/` (≈13,178 engine plus one 750-line
  `#[cfg(test)]` module) and 46,087 lines of tests/benches, 69 re-exports.
  This crate holds the kernel (`log`, `fold`, `quantum`, `pipeline`, `store`,
  `segment`, `commit`, `snapshot`) and every capability that would otherwise be
  a slice (pack lifecycle, enrichment, parser, path policy, harness tenant).
- Two binaries inside the runtime package: `kutha-migrate` and `kutha-tenant`.

ADR-020 already allows this as a P0 variant ("P0 may be `common` + one runtime
crate", ADR-020:66; Open Question 1, ADR-020:83) but never records that the
variant is what shipped. This cell records it so the map and the tree agree, and
so the next agent does not re-derive the gap as a defect.

## Decision

### D094-1. The P0 workspace is `common` + one runtime crate

Compile-time Space is preserved **at crate granularity**: `kutha-runtime`
depends on `kutha-common` and nothing depends on `kutha-runtime`; `kutha-common`
stays I/O-free, so the pure kernel types cannot absorb storage or network code.
Vertical slices are **modules inside `kutha-runtime`**, not crates. This is the
narrowest shape that still has one real dependency edge and one enforced purity
boundary; it is not a claim that per-slice isolation exists.

### D094-2. What this shape does not give us

1. **No per-slice crate boundary.** A future pack crate would be the compiler
   barrier ADR-020 and ADR-022 rely on. Today a capability module can call
   `crate::fold` directly; only the log and `kutha-common` are structural.
2. **No private-by-default enforcement.** 69 names leave `kutha-runtime`, plus
   the whole `store` module, including `#[doc(hidden)]` failpoint and counter
   functions. "Only Ports leave the crate" is a convention over a curated list,
   not a gate. `#[doc(hidden)]` hides documentation, not imports.
3. **No composition-root crate.** Two binaries in the runtime package consume
   the same wide library API; an optional `rocksdb` adapter is wired by Cargo
   feature inside the engine crate (permitted by the freeze rule, not by
   D020-2 law 3).
4. **Slice communication is by direct module call plus the log**, not by Port
   traits. The only traits are `Materializer` and `VocabBinder`.

D094-1 and D094-2 together mean: the ADR-020 / ADR-022 target stays a target.
This cell does not degrade those cells and does not make their laws satisfied.

### D094-3. Trigger for revisiting

Split the workspace per ADR-020 / ADR-022 — `app-shell` + per-slice crates —
when **either** happens:

1. **A second vertical pack** is leased (a legal or science slice with its own
   dictionary facets, relation behaviors, materializers, and Ports): a pack that
   cannot call `crate::fold::*` must live in its own crate, or isolation is a
   fiction.
2. **A second binary needs isolation** from the engine API — i.e. a binary whose
   correctness argument depends on *not* being able to call `store::*` or
   `fold::*` — or the first **non-optional** adapter must be swappable by a pack.

Until one of those is true, growing modules inside `kutha-runtime` is the
cheaper and honest choice. At that trigger, the split is a lease-sized change,
not a rewrite: the module seams already exist.

**Hard separations:**

```
Two crates at HEAD        ≠  ADR-020 D020-1 satisfied
Module inside runtime     ≠  Vertical slice crate
`#[doc(hidden)]`          ≠  Private
Curated re-export list    ≠  Port-only export gate
```

## Consequences

### Positive

- The map stops implying that per-slice isolation, private-by-default, and a
  composition-root `app-shell` exist.
- The next pack lease has a written, testable trigger instead of an implicit
  "someday".
- No refactor is implied by this cell.

### Negative / risks

- A two-crate workspace makes it cheap to keep adding capability modules and
  expensive to notice when isolation was actually needed. The trigger in D094-3
  is the only thing preventing drift.
- `kutha-runtime`'s wide export surface is now an explicitly recorded property,
  so later shrinking it is a visible, reviewable change rather than noise.

### Non-goals

- Not accepting ADR-020 or ADR-022.
- Not authorizing a crate split, a workspace reshuffle, or a new binary.
- Not defining tenant isolation (ADR-080) or pack install (ADR-021).

## Alternatives Considered

| Alternative | Why not now |
|-------------|-------------|
| Split into `app-shell` + slice crates immediately | No second slice exists; ADR-022:57 says the conventions are slogans until one does |
| Leave the tree undocumented and keep ADR-020 as the only statement | The map then contradicts HEAD, and each audit rediscovers it (2026-10-04, 2026-10-08, this one) |
| Treat "two crates" as D020-1 conformance | False: no `app-shell`, no slice crates, no Port gate |

## Related Decisions

- ADR-020 — workspace target shape and isolation laws
- ADR-022 — slice conventions (unimplemented, `delivery: map-only`)
- ADR-011 — `kutha-common` is the shared encoding kernel
