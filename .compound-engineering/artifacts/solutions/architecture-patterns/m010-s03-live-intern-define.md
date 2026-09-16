---
module: kutha-runtime
tags: [m010, define, intern, snapshot, lease-drift]
problem_type: architecture-pattern
---

# Live intern Define with graph-oriented snapshot offset

## Problem

After M010 S02, persist synthesized `Op::Define` but live `Runtime::intern` did not. Lease text stayed on S02 after ROADMAP checked it done. Putting Define into `log.len()` broke `snapshot.log_offset` vs Define-stripped open.

## Solution

- New terms: `intern` appends `Op::Define` (fold no-op); bootstrap `knows`/`knownBy` stay silent.
- `graph_len()` counts non-Define events; `snapshot.log_offset` uses it; `from_snapshot` strips Define before offset apply.
- `encoded_log` still synthesizes a full durable Define prefix from the dictionary.
- Governor `pointer_in_other_file` gains `require_template` so an Active Slice must still be `- [ ]` on ROADMAP.

## Do not

- Treat Define as a graph fact or admit it through the relation allowlist.
- Start M002/legal pack because M010 closed.
- Collapse ADR-011 to Accepted.
