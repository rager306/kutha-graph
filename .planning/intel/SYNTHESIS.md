# SYNTHESIS

Entry point for `gsd-roadmapper`. Intel from `/gsd-ingest-docs` (MODE=new).

## Doc counts by type

- Classifications consumed: 37
- ADR: 31
- SPEC: 2
- PRD: 0
- DOC: 4
- UNKNOWN: 0
- Classification `locked: true`: 0
- Confidence low: 0
- Confidence medium: 2 (`docs/architecture/stca-guide.md` SPEC; `docs/process/kutha-harness.md` DOC)
- Confidence high: 35

## Decisions

- File: `.planning/intel/decisions.md`
- ADR entries: 31 (ADR-000, ADR-001, ADR-002, ADR-010–014, ADR-020–022, ADR-030–031, ADR-040–043, ADR-050–052, ADR-060–062, ADR-070–071, ADR-080–081, ADR-090–093)
- Extra foundation-lock extracts from ADR-000: D1–D10 (10 entries)
- Total decision headings: 41
- GSD locked decisions: 0
- All ADR status in intel: `proposed` (source Status Proposed, not Accepted)

Locked-in-narrative (not GSD locked) sources: `docs/ADR/ADR-000-kutha-hybrid-architecture-research.md` (D1–D10); `docs/ADR/README.md` idea-stack framing.

## Requirements

- File: `.planning/intel/requirements.md`
- Count: 0
- IDs: none (no PRD classifications)

## Constraints

- File: `.planning/intel/constraints.md`
- Count: 13
- type protocol: 8 (STCA paradigm, Algorithms 1–3, isolation laws, agent change protocol, semantic three-consumers, remaining encoding choices)
- type schema: 1 (STCA Cargo workspace target shape)
- type nfr: 4 (STCA verification regimes, semantic ownership/limits, candidate fixture oracles, adversarial probes)
- type api-contract: 0
- SPEC sources: `docs/architecture/stca-guide.md`, `docs/architecture/semantic-contract-validation.md`

## Context topics

- File: `.planning/intel/context.md`
- Count: 4
- Topics: ADR index and honeycomb process; Kutha harness — parallel control plane; Governor intake — control loop to check; Codex subagent workflow
- DOC sources: `docs/ADR/README.md`, `docs/process/kutha-harness.md`, `docs/process/governor-intake.md`, `docs/process/codex-subagents.md`

## Conflicts

- Report: `.planning/INGEST-CONFLICTS.md`
- BLOCKERS: 0
- WARNINGS (competing-variants / overlap): 1 (stca-guide SPEC vs ADR-002)
- INFO (auto-resolved / transparency): 6

## Cycle detection

- Run: DFS three-color + Tarjan SCC on resolved ingest cross_refs
- Depth abort (>50): 0
- SCC size>1: 1 component of 35 docs
- Docs outside SCC (still synthesized): `docs/process/governor-intake.md`, `docs/process/codex-subagents.md`
- Treatment: INFO, not BLOCKER — per-doc extraction, no recursive merge

## Precedence applied

- Default: ADR > SPEC > PRD > DOC
- Per-doc `precedence` overrides: none (all null)
- MODE: new (no merge vs existing CONTEXT.md)

## Pointers

- Decisions: `.planning/intel/decisions.md`
- Requirements: `.planning/intel/requirements.md`
- Constraints: `.planning/intel/constraints.md`
- Context: `.planning/intel/context.md`
- Conflicts: `.planning/INGEST-CONFLICTS.md`
- Classifications: `.planning/intel/classifications/`
