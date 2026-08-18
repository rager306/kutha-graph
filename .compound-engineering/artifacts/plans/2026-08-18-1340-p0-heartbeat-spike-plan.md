---
title: P0 Heartbeat Spike and ADR Proposed Graduation
type: research
date: 2026-08-18
artifact_contract: ce-unified-plan/v1
artifact_readiness: implementation-ready
product_contract_source: ce-plan-bootstrap
execution: code
---

# P0 Heartbeat Spike and ADR Proposed Graduation

## Goal Capsule

- **Objective:** (1) Graduate honeycomb + vision ADRs from Research to **Proposed** (decisions exist; not Accepted). Keep ADR-000 Research until its own exit checklist is met. (2) Ship a falsifiable **in-memory P0 runtime spike**: emit → cascade → idle + receipt + strict replay (ADR-010/011/013/014/060; intern from 011). No RocksDB, no Cypher, no HNSW, no Consensus.
- **Product authority:** STRATEGY track 1 P0; ADR-000 D1–D10; honeycomb 010–014.
- **Open blockers:** None. User authorized “promote all” after honeycomb opening.

## Product Contract

### Key Decisions

- KD1. **Proposed ≠ Accepted.** Accepted requires implemented product runtime. Spike does not Accept 041/080/090.
- KD2. **ADR-000 stays Research** until replay-vs-length and cascade bounds are measured (its checklist). Honeycomb drafts item can be checked.
- KD3. **In-memory log is valid P0 SoT for the spike.** Rocks WAL remains the crash cousin (010), not this unit’s store.
- KD4. **No LLM write path.** Typed assert/retract/correct only.
- KD5. **Intern map in common; no agent dictionaries.**

### Requirements

- R1. Status **Proposed** on ADR-001, 002, and all honeycomb 010–093. README index matches.
- R2. Cargo workspace: `crates/kutha-common`, `crates/kutha-runtime`.
- R3. Tests: fold after assert; retract keeps loser; inverse-edge cascade idles; budget abort; replay equality; `ReplayDivergenceError` on tamper.
- R4. No Graphiti/Samyama/Rocks as SoT. No `src/` at repo root (crates layout).

## Implementation Units

### U1. ADR status graduation

- Files: `docs/ADR/*.md`, `docs/ADR/README.md`
- Approach: first Status Research → Proposed except ADR-000.

### U2. P0 crates

- Files: `Cargo.toml`, `crates/kutha-common/**`, `crates/kutha-runtime/**`, `.gitignore`
- Approach: UUID v7 events, interned ids, in-memory log, fold, quantum with max-cascade budget, SHA-256 receipt, replay harness.

## Verification Contract

`cargo test --workspace` green. README Status column Proposed except 000.

## Definition of Done

U1+U2. ADR-000 checklist: honeycomb drafts + spike demo checked; measurements noted if tests print them. No commit unless asked.
