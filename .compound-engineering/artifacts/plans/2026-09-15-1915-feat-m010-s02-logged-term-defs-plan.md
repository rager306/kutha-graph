---
artifact_contract: ce-unified-plan/v1
product_contract_source: ce-brainstorm
title: "M010 S02 — term definitions in the persisted event log"
date: 2026-09-15
---

# Goal Capsule

Close S01 lease drift, then deliver M010 S02: persist encodes `Op::Define` in the durable event stream so `open` recovers intern meanings without `snapshot.json` and without `terms.jsonl`. Snapshot stays a droppable lease. No M002/legal pack.

# Product Contract

## Problem

S01 keeps meanings in companion `terms.jsonl`. STATE still says “deliver S01” after ROADMAP marked S01 done. ADR-011 requires term strings in retained history (logged ops or integrity-checked objects), not only a sidecar. Governor observe misses `module::tests::name` cargo lines.

## In scope

- Lease STATE/ROADMAP/README to M010 S02.
- `Op::Define { name }` (no relation allowlist; fold no-op).
- persist writes Define prefix (dictionary order) + graph events; WAL matches.
- open without snapshot: Define ops → dict; else `terms.jsonl`; else fail if events exist.
- Named integration test; FSM observe; governor check needles.
- observe_cargo accepts optional `path::` prefix on test names.
- Changelog + commit.

## Out of scope

- Intern auto-appending Define into the live in-memory log (would inflate log.len tests).
- M002 Rocks, Cypher, HNSW, P→Q fixture, ADR-050 six kinds.

## Success

- `open_without_snapshot_or_terms_file_recovers_from_define_ops` green.
- `kutha-gov ci` green with Active Slice S02.
