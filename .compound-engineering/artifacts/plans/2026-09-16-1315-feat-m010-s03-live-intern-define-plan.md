---
artifact_contract: ce-unified-plan/v1
product_contract_source: ce-brainstorm
title: "M010 S03 — live intern appends Op::Define"
date: 2026-09-16
---

# Goal Capsule

Lease M010 S03: `Runtime::intern` appends `Op::Define` for newly created terms so the in-memory event log is SoT for meanings (not only persist-time synthesis). Bootstrap `knows`/`knownBy` stay silent. Close lease drift after S02. No M002/legal pack.

# Product Contract

## Problem

S02 writes Define only at persist. Live log before persist lacks term definitions (documented gap). STATE still names Active Slice S02 after ROADMAP checked it done.

## In scope

- Lease STATE/ROADMAP to S03.
- `intern` on new string → append Define (fold no-op); existing id → no log growth.
- `Runtime::new` still uses `dict.intern` directly (no Define for bootstrap).
- `graph_len()` counts non-Define events; update log-growth tests that meant graph ops.
- Named integration test; FSM observe; governor needles.
- After S03 green, close M010 milestone (success criteria met) OR leave Active Slice S03 done with milestone open — prefer close M010 in same wave if tests green.

## Out of scope

- M002 Rocks, Cypher, HNSW, P→Q fixture.
- Changing admit/allowlist to cover Define.

## Success

- `intern_appends_define_for_new_terms_only` green.
- `kutha-gov ci` green; log-growth allowlist tests still prove fail-closed.
- Lease text matches ROADMAP.
