---
module: kutha-gov
tags: [governor, lease, docs-coupling, gh, trajectory]
problem_type: process-drift
date: 2026-09-15
---

# Keep lease text and docs coupling from drifting after milestone close

## Problem

Closing M001 left process text and governor messages still talking about “before M001 L_capability” / “H4 next”, while agents repeatedly forgot CHANGELOG or README when editing STATE or architecture notes. CE babysit also failed once on an old Ubuntu `gh` that lacked `baseRefOid`.

## Pattern

1. Prefer dictionary checks over hardcoded Python: new conditional lease rule → `when_match_then_match` kind + YAML row.
2. Keep freeze messages tied to the live lease (“until Active Milestone names M002”), not a closed milestone gate.
3. Extend `docs-coupling` so STATE and `docs/architecture/**` diffs require CHANGELOG.
4. Treat host `gh` version as harness tooling: CE `pr-snapshot` needs a `gh` that exposes `baseRefOid` (official GitHub CLI package, not the Ubuntu 2.45.0 pin alone).

## Not this

Do not lease M002 or a legal pack from a debt wave. Do not encode “next steel thread” in architecture notes without STATE Active Milestone.
