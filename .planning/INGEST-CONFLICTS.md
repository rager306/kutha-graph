## Conflict Detection Report

### BLOCKERS (0)

None. No UNKNOWN/low-confidence classifications. No ADR with locked: true. No LOCKED-vs-LOCKED contradiction. MODE=new; no existing PROJECT.md / REQUIREMENTS.md / ROADMAP.md / CONTEXT.md to contradict. Cross-ref citation cycles were detected and recorded under INFO (per-doc extraction, not recursive merge).

### WARNINGS (1)

[WARNING] SPEC stca-guide (medium confidence) overlaps ADR-002 STCA paradigm
  Found: docs/architecture/stca-guide.md classified SPEC (confidence medium, locked false) restates STCA Space/Time/Composition, deterministic fold, Relation Behaviors, Cui max-convolution, UUID v7, Strict Replay, Fork-and-Diff, and optional Regimes Gated Loop
  Found: docs/ADR/ADR-002-stca-paradigm.md classified ADR (confidence high, locked false) adopts STCA as Kutha’s paradigm and names docs/architecture/stca-guide.md as the home for detail algorithms and Cargo isolation laws
  Impact: A downstream roadmapper could treat the SPEC as a second independent STCA product decision, or treat Algorithm 1 JSON merge-patch as the write surface
  → Keep ADR-002 as the paradigm decision; keep stca-guide extracted only as constraints; do not mint a competing STCA lock. Write-operator clash with ADR-010 is auto-resolved under INFO

### INFO (6)

[INFO] Cycle detection: honeycomb citation SCC, synthesis used per-doc extraction
  Note: Directed graph from classification cross_refs (resolved only to other ingest sources): 37 nodes, 326 edges, DFS depth abort 0 (cap 50). One strongly connected component of 35 documents (all ADRs plus docs/ADR/README.md, docs/architecture/stca-guide.md, docs/architecture/semantic-contract-validation.md, docs/process/kutha-harness.md). 135 simple back-edge cycles (19 two-cycles). Outside the SCC: docs/process/governor-intake.md, docs/process/codex-subagents.md. Mutual Related-to citations are expected honeycomb structure. Extraction did not recurse through cross_refs, so the cyclic set was synthesized independently rather than skipped

[INFO] Auto-resolved: ADR-010 > stca-guide on write operators
  Note: docs/architecture/stca-guide.md Algorithm 1 / §5 names object.created / relation.created / object.patched (JSON merge-patch). docs/ADR/ADR-010-event-log-runtime-quantum.md D010-3 names a closed TGMS family assert / retract / correct. docs/process/kutha-harness.md forbids copying stca-guide §5 JSON merge-patch; product and process use typed Op. Precedence ADR > SPEC (and ADR > DOC). Synthesized constraints retain the SPEC algorithm text with a pointer to this resolution; synthesized decisions keep D010-3

[INFO] Auto-resolved: ADR-002 > stca-guide on STCA as paradigm
  Note: Both sources adopt the same three-axis STCA (vertical slices + Ports; event log SoT + fold; Cui max-convolution). ADR-002 is the decision record; stca-guide is the manifest-spec it cites. No second paradigm winner was chosen beyond default precedence. Classification locked remains false on both (Status Proposed)

[INFO] All ingested ADRs are Proposed / locked:false (not Accepted, not GSD locked)
  Note: docs/ADR/README.md and each ADR Status line: Proposed. ADR-000 Status text: foundation ADR — D1–D10 locked; honeycomb Proposed; not Accepted product runtime. D1–D10 were extracted as separate proposed decisions (foundation-lock narrative), not as GSD status: locked. MODE=new does not promote them to Accepted

[INFO] No PRDs in the ingest set
  Note: 0 documents classified type PRD. requirements.md contains no REQ-* entries. TR-01–TR-10 remain inside ADR-090/093 decisions. Semantic-contract fixture oracles remain SPEC constraints

[INFO] semantic-contract-validation SPEC defers to honeycomb ADRs
  Note: docs/architecture/semantic-contract-validation.md states it is a proposed validation design, not implemented capability, not a new ADR, and that normative homes remain the linked ADRs. No contradiction vs ADR-011/013/060/010/014 extracted as a competing decision; constraints record current limits and oracles only
