# Phase 174 Context — duplicate Define keeps first id

**Milestone:** v0.97 (M103-S01)
**Status:** ready (discuss skipped — lease names the slice)

## Decision

`TermDictionary::from_strings` inserts every string into the id map, so a later duplicate overwrites the earlier index. Keep the vector (positions stay stable) and insert into the map only when the name is new (`or_insert`).

## Oracle

`from_strings(["Alice", "Bob", "Alice"])` resolves `Alice` to 0 and `Bob` to 1. `lookup(2)` is still `Alice`.

## Out of scope

Op-wire emit-once for mixed legacy/modern Defines, single-combination DFS, honeycomb thaw.
