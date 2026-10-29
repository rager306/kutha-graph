---
phase: "246"
name: "bytes-honesty"
created: 2026-10-29
---

# Phase 246: Bytes honesty — Context

## Decisions

1. A zero RSS delta is a measurement miss (warm allocator), not `0 B` per fact.
2. `bytes_per_fact` prints `n/a` when `delta == 0` or `live == 0`.
3. No product semantics; bench output only.
4. Rerun one HEAD bench to record the new key.

## Discretion Areas

Whether to attempt a cold-process measurement later; this phase only stops the false zero.

## Deferred Ideas

A cold-process RSS measurement is a separate probe.
