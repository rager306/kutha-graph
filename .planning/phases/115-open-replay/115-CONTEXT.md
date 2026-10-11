---
phase: 115-open-replay
milestone: v0.46
harness: M052
slice: S04
ce_finding: "whole-repo P1 #11"
---

# Phase 115 — CONTEXT

## Kill-claim

Snapshot-free Define/terms open paths skip `replay_check` and accept logs that
snapshot-backed open rejects.

## Goal

Run replay validation on every reconstruction path before `finish_open`.

## Out of scope

SEM helper split (#12). Honeycomb Proposed.
