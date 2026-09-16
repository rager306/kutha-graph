---
module: kutha-runtime
tags: [m011, claim, lineage, replay]
problem_type: architecture-pattern
---

# Unknown claim fail-closed and replay caused_by

## Problem

After S01, `Assert { claim: Some(ghost) }` still appended. `replay_check` matched fold fingerprints even when a Behavior pointed at a never-logged cause (ADR-060 obligation 2).

## Solution

- `admit_claim` before Event construction: claim must already exist on some Fact (including retracted).
- After fingerprint match, walk the log in order; Behavior `caused_by` must be in the preceding id set.

## Do not

- Treat this as execution replay of inverse_knows.
- Start P→Q or M002.
