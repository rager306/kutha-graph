---
module: kutha-runtime
tags: [m010, semantic-recovery, store, intern]
problem_type: capability-gap
date: 2026-09-15
---

# Open without snapshot needs retained term strings

## Problem

Events carry dense `TermId`s. Dropping `snapshot.json` used to make `store::open` fail, so the droppable lease was treated as required for meanings.

## Pattern

Persist authoritative `terms.jsonl` (dense id order) with the event log. Prefer snapshot when present; else rebuild dictionary from terms and full-replay the fold. Fail closed if events exist without terms. Put the named capability test in `tests/*.rs` so FSM `observe_cargo.required` matches `test <name> ... ok`.

## Not this

Not Rocks (M002). Not logging DefineTerm ops yet (M010 S02). Not promoting ADR-011 to Accepted.
