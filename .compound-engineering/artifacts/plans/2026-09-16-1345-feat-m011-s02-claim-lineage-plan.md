---
artifact_contract: ce-unified-plan/v1
title: "M011 S02 — unknown claim fail-closed + replay lineage"
date: 2026-09-16
---

# Goal Capsule

Lease M011 S02: close S01 identity hole (Assert onto a never-seen claim must not append) and ADR-060 obligation-2 stub (Behavior.caused_by must name an earlier log event). No P→Q engine, no M002.

# Product Contract

## Problem

`claim: Some(uuid)` currently becomes a Fact even when no prior support opened that claim. `replay_check` only compares fold fingerprints; a dangling `caused_by` still passes.

## In scope

- `RuntimeError::UnknownClaim` / `BrokenLineage`.
- emit: `Assert.claim = Some(id)` requires an existing `Fact.claim_id` (live or retracted).
- `replay_check`: after fingerprint match, every `Op::Behavior` has `caused_by` equal to some earlier event id in the same log.
- Named tests; FSM + governor needles; lease STATE/ROADMAP.

## Out of scope

- P→Q derivation fixture (later slice).
- Execution replay of inverse_knows; receipt persistence.
- M002 / legal pack.

## Success

Named tests green; `kutha-gov ci` green; log does not grow on unknown claim.
