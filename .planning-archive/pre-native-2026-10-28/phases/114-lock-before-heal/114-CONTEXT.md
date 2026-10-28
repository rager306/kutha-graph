---
phase: 114-lock-before-heal
milestone: v0.46
harness: M052
slice: S03
ce_finding: "whole-repo P1 #7"
---

# Phase 114 — CONTEXT

## Kill-claim

`Persister::attach` runs `attach_holding` (heal may write) before
`acquire_store_write_lock`, contradicting the no-writes-before-lock contract.

## Goal

Lock first (or split read-only validate from heal). Concurrent second attach
denied with no byte change.

## Out of scope

Torn-tail heal (#9), generation names (#10). Honeycomb Proposed.
