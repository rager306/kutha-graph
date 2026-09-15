---
artifact_contract: ce-unified-plan/v1
product_contract_source: ce-brainstorm
title: "Post-M001 process debt closure (wave 2) — live H4 membership emit"
date: 2026-09-15
---

# Goal Capsule

Close the H4 process gap where membership snapshots exist in tests/API but the live CI `emit_log` quantum never writes them. Keep tip YAML as the admit lease; append one edition only when tip differs from the last logged snapshot (meta-prompt FSM, no hardcoding of relation names beyond the existing reserved `allows`).

# Product Contract

## Problem

`append_membership_edition` is only called from pytest. Live `kutha-gov ci` → `emit_log` never records tip membership, so H4 dogfood on the process tenant lags the tip allowlist unless tests synthesize editions.

## In scope (Wave 2)

- Detect last logged `process.relations`/`allows` snapshot from JSONL.
- Sync tip YAML membership into the process log when it differs.
- Call sync from FSM `emit_log`.
- TDD coverage; refresh stale kutha-changelog example lease text.
- Changelog + commit.

## Out of scope

- M002 Rocks / legal pack / Cypher / HNSW.
- Full semantic recovery without snapshots (needs Active Milestone lease).
- Rewriting historical CHANGELOG Trajectory lines that correctly described past leases.

## Success

- `kutha-gov ci` green; pytest proves sync skips unchanged tip and appends on tip change.
- GitNexus reindexed after the change.
