---
artifact_contract: ce-unified-plan/v1
product_contract_source: ce-brainstorm
title: "Post-M001 process debt closure (wave 1)"
date: 2026-09-15
---

# Goal Capsule

Close process/doc/governor drift after M001 closed and H4 shipped, without leasing M002 or a legal pack. Keep meta-prompt+FSM as the control plane (YAML checks, not hardcoded Python rules).

# Product Contract

## Problem

Session work left trajectory debt: freeze check still talks about “before M001 L_capability”, architecture note still says STATE is H3/H4-next, kutha-graph was not in GitNexus, and agents repeatedly tripped docs-coupling / stale lease text. Full M002 and semantic recovery remain later waves.

## In scope (Wave 1)

- Index kutha-graph in GitNexus.
- Correct freeze governor message to M002/STATE freeze language.
- Refresh stale lease references in `docs/architecture/semantic-contract-validation.md`.
- Add a dictionary check that `Active Milestone: None` implies a closed delivery lease wording in STATE.
- Changelog + commit.

## Out of scope

- M002 Rocks adapter / rocksdb crate.
- Legal pack / Cypher / HNSW / ADR-050 six kinds.
- Full semantic recovery without snapshots (Wave 2 candidate).
- Live CI membership emit each quantum (optional polish).

## Success

- `kutha-gov precommit` and `ci` green.
- GitNexus `list_repos` includes kutha-graph.
- No freeze message mentioning closed M001 capability gate.
- Tests cover new lease-closed check behavior.
