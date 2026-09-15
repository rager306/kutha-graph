---
title: "Process-relation membership editions must be one snapshot, not per-member open chains"
date: 2026-09-15
category: architecture-patterns
module: kutha-harness-h4
problem_type: architecture_pattern
component: harness_process_overlay
severity: high
applies_when:
  - "Logging allowlist or dictionary membership that later drops names"
  - "Projecting process JSONL into kutha-tenant via emit_chained"
  - "Need AS OF a prior cut after a later tip edition"
tags: [H4, KTD3, processAllows, membership, emit_chained, ADR-090]
---

# Process-relation membership editions must be one snapshot, not per-member open chains

## Context

H4 dogfoods ADR-090 overlay on the harness plane: tip YAML stays the admit lease, while membership history is logged so tenant AS OF can answer what was allowed at a prior cut. A natural first design is one assert per allowed relation name, then retract or omit names when the tip drops them.

That design fails under the existing tenant helper `emit_chained`: each subject/relation/object stream is an open interval until a later event closes it. Dropping a name from a new tip edition without an explicit retract leaves the old member live at later cuts.

## Guidance

Encode each membership **edition** as a single snapshot assert:

- subject: `process.relations`
- relation: `allows` (process JSONL) → product token `processAllows`
- object: deterministic sorted CSV of member names (`encode_membership_object`)

Chain editions with `emit_chained` so each new snapshot closes the previous edition. A prior AS OF cut then still sees the earlier object string (including members the tip later dropped). Tip YAML continues to admit writes; it is not fold-derived from the log.

## Why This Matters

Per-member open chains silently keep revoked permissions in the live projection. Reviewers and tests can look green on “latest tip” while historical cuts and dropped-member semantics are wrong — exactly the AE1 failure mode H4 had to prove.

## When to Apply

- Any harness or product allowlist whose membership must be queryable AS OF
- Any use of `emit_chained` where the meaningful unit is a set, not an individual edge that is retracted one-by-one
- Before modeling “N asserts + later omit” as time travel

## Examples

Wrong (dropped `status` stays live under chained open intervals):

```text
assert process.relations / allows / status
assert process.relations / allows / cargo
# later tip drops status — no retract → status still live at tip cut
```

Right (one object per edition; later edition replaces the set):

```text
assert process.relations / allows / cargo,status
assert process.relations / allows / cargo
# prior cut still sees cargo,status; tip cut sees cargo only
```

See `scripts/kutha_gov/time_log.py` (`encode_membership_object`, `append_membership_edition`) and `crates/kutha-runtime/src/tenant.rs` (membership → `processAllows`).

## Related

- Plan KTD3: `.compound-engineering/artifacts/plans/2026-09-15-1459-feat-h4-adr-090-process-overlay-plan.md`
- Tests: `crates/kutha-runtime/tests/h4_process_allows.rs`, membership cases in `scripts/tests/test_kutha_gov.py`
