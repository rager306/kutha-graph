---
module: kutha-gov
tags: [h4, membership, emit_log, fsm]
problem_type: process-gap
date: 2026-09-15
---

# Live H4 membership must sync from tip on emit_log

## Problem

H4 membership APIs and pytest existed, but FSM `emit_log` never called them, so CI quanta did not version the tip allowlist on the process tenant.

## Pattern

Compare tip `load_process_relations` to `last_logged_membership` from JSONL; call `append_membership_edition` only on change; wire from `emit_log` after `append_run`. Tip YAML stays the admit lease.

## Not this

Do not emit a new edition every quantum. Do not hardcode relation names beyond the reserved `allows` constant already used by H4.
