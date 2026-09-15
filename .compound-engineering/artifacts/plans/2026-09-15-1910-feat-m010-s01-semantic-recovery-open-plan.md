---
artifact_contract: ce-unified-plan/v1
product_contract_source: ce-brainstorm
title: "M010 S01 — open without snapshot recovers intern meanings"
date: 2026-09-15
---

# Goal Capsule

Lease M010 and deliver S01: `store::open` recovers term meanings and fold state from retained history when `snapshot.json` is absent, without Rocks/Cypher/HNSW or a legal pack.

# Product Contract

## Problem

Events store dense `TermId`s. Today `open` refuses without `snapshot.json`, so discarding the droppable snapshot lease loses meanings (ADR-011 gap). Process debt waves closed; roadmap was idle (`Active Milestone: None`).

## In scope (S01)

- Persist authoritative `terms.jsonl` (dense TermId order) with the event log.
- `open` without snapshot: load terms + replay events; fail explicitly if events exist but terms are missing.
- Snapshot path unchanged when present.
- Lease M010/S01 in STATE/ROADMAP/README; governor observes the named test.
- Changelog + commit.

## Out of scope

- M002 Rocks, Cypher, HNSW, ADR-050 six kinds, P→Q fixture, Op::DefineTerm in the event schema (later S0x).

## Success

- Named test `open_without_snapshot_recovers_intern_meanings` green.
- `kutha-gov ci` green with Active Milestone M010.
