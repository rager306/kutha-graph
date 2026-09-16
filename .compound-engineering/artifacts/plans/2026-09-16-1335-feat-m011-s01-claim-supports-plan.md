---
artifact_contract: ce-unified-plan/v1
title: "M011 S01 — independent claim supports"
date: 2026-09-16
---

# Goal Capsule

Lease **M011 S01**: give each folded fact a portable `claim_id` so two independent supports for the same claim stay distinguishable; retracting one support must not erase the other. First slice of claim/support identity after M010 semantic recovery. No M002, no derivation engine, no legal pack.

# Product Contract

## Problem

ADR-011 / semantic-contract: independent supports must survive withdrawal of another. Today each Assert gets a local `fact_seq` only; `live_at` returns triples without a claim/support API. Duplicate Asserts already create two Facts, but there is no shared claim identity or named oracle.

## In scope (S01)

- `Op::Assert { claim: Option<EventId> }` — `None` means this event opens a new claim (`claim_id = event.id`); `Some(id)` attaches another support to an existing claim.
- `Fact.claim_id` persisted in the fold/snapshot (serde default for old snapshots).
- Fold helpers: `live_supports(claim, tt, vt)`, `claim_supported_at(claim, tt, vt)`.
- Named integration test: two supports for one claim → retract one → other still supports at the cut.
- STATE/ROADMAP/README/AGENTS lease; governor FSM + check needles; CHANGELOG.
- Digest includes claim when present.

## Out of scope

- Full P→Q derivation fixture (later S0x).
- Interval-patch Correct residuals; provenance polynomials; ADR-050 six kinds.
- M002 Rocks, Cypher, HNSW, legal ontology.
- Changing Relation allowlist semantics for claim links.

## Success

- Named test green; `kutha-gov ci` green.
- Lease text matches ROADMAP open checkbox for S01.
- Cells remain Proposed.

## Execution direction

Test-first. Reuse existing Retract + `is_live_at`; do not invent a second SoT.

## Key files

- `crates/kutha-common/src/event.rs` — Assert claim field + digest
- `crates/kutha-runtime/src/fold.rs` — Fact.claim_id + helpers
- `crates/kutha-runtime/src/quantum.rs` — emit paths that build Assert
- `crates/kutha-runtime/tests/m011_claim_supports.rs` — oracle
- `.kutha/STATE.md`, `ROADMAP.md`, governor dicts

## Test scenarios

1. New Assert without claim → `claim_id == event.id`; one live support.
2. Second Assert with `claim: Some(first_id)` same (s,r,o) → two live supports for that claim.
3. Retract first support’s `fact_seq` → `claim_supported_at` still true via second support.
4. Retract both → claim unsupported; earlier TT cut still sees prior support (bitemporal).
5. Snapshot round-trip preserves `claim_id` (or nil default for legacy).
