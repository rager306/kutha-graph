# Requirements: Kutha

**Defined:** 2026-09-30
**Core Value:** Legal PIT stays falsifiable; governor CI stays honest; one Active Slice at a time — not the honeycomb.
**Milestone:** v0.03 Lean context + semantic governor

Derived from the 2026-09-30 architecture and roadmap review (findings F1–F8 below) and from user direction: slim `AGENTS.md`, assess ADR corrections, make the governor check meaning instead of needle presence. Research skipped: the domain is this repository and the evidence is already in hand.

**Boundary:** harness + docs plane only. No `crates/` edits. Product fixes stay deferred to a separately leased M012a. Freeze holds; every honeycomb cell stays **Proposed**.

## Review findings driving this milestone

| ID | Finding (verified in source or by measurement) |
|----|-----------------------------------------------|
| F1 | Two sources of truth: quantum outcomes and justifications live in sidecars outside the log and outside `provenance_fingerprint` |
| F2 | References are fold-local: `Retract{fact_seq}` and `source_fact_seqs` depend on fold numbering |
| F3 | A `claim` is a slot, not a proposition; conflict polarity is caller-supplied; ingest is not idempotent |
| F4 | Derivation: single-parent `caused_by`, one-hop non-transitive eligibility, hard-coded `inverse_knows`, free-string `rule_version` |
| F5 | Admission is advisory (no non-test caller); allowlist is ambient file/env state with no policy version in the log; no "action" entity |
| F6 | Durability: `persist` is destructive and non-atomic; `open` trusts the snapshot without `replay_check`; `Define` ids change on every persist |
| F7 | Time: `ValidTime` is a bare `u64` with an undeclared scale; `TransactionTime` has no wall-clock mapping |
| F8 | Reads are linear scans; `Materializer` is not on the CSR path; STCA Space/Composition axes and the ADR-061 diff half are unexercised |

## v0.03 Requirements

Each requirement maps to exactly one roadmap phase.

### Harness lease

- [ ] **LEASE-01**: `.kutha/STATE.md`, `.kutha/ROADMAP.md`, `README.md`, and `docs/process/kutha-harness.md` name the **H5** rung (semantic governor + lean context) consistently; Active Milestone, lifecycles, and freeze are unchanged and `uv run kutha-gov ci` stays at 0 HIGH

### Lean agent context

- [ ] **CTX-01**: `AGENTS.md` contains no lease or trajectory values (no `L_delivery=` tokens, no "current slice/milestone is …" narrative); it points to `.kutha/STATE.md` instead
- [ ] **CTX-02**: Content an agent does not need on every turn (P0 spike inventory, literature/matrix counts and card lists, research notes such as Leapfrog, long subagent workflow, full repo tree) is moved to its owning doc and referenced by one-line pointers; no fact is lost, only relocated
- [ ] **CTX-03**: Durable rules survive the diet unchanged in meaning: language policy, two planes, D1–D10 guard, CE skill routing, commands, freeze, CBM/subagent rules, working conventions
- [ ] **CTX-04**: The stale `L_delivery=M011-S03-done` claims are removed or corrected in `AGENTS.md` and `docs/architecture/semantic-contract-validation.md`

### ADR and roadmap correction

- [ ] **ADR-01**: A review artifact maps every finding F1–F8 to the affected ADR-000/001/002 and honeycomb cells with a verdict per cell (`no change` | `amend` | `open question`), each with evidence (file + symbol or measurement)
- [ ] **ADR-02**: Cells with an `amend` verdict carry an explicit, dated amendment or Open Question in the ADR body; Status stays **Proposed**; ADR-000 D1–D10 wording is not rewritten (a needed lock change is raised as a question, not applied silently)
- [ ] **ADR-03**: `.kutha/ROADMAP.md` "Later milestones" is re-ordered and labelled non-authoritative: M012a (single-log SoT + stable references) precedes M012; M012 names the admission/rule-registry subset it thaws; M002 is described as log durability first (Rocks only for indexes); benchmarks and a thin real-text legal golden fixture move earlier; no new milestone is leased
- [ ] **ADR-04**: `honeycomb.yaml` rows and `docs/ADR/README.md` stay consistent with the amendments (map/delivery/capability orthogonal, no cell promoted to Accepted)

### Semantic governor

- [ ] **SEM-01**: `kutha-gov selftest` proves each mutable YAML check can fail: it applies a declared mutation to a temporary copy of the tree and requires a HIGH; checks that cannot be mutated are listed with a reason
- [ ] **SEM-02**: A new check kind proves a named Rust test is non-vacuous (function exists, not `#[ignore]`, body contains an assertion, optionally references required symbols); all existing `fn …` needle checks use it
- [ ] **SEM-03**: A new check kind proves every citation of a lease value in docs (`L_delivery`, Active Milestone/Slice, Phase) equals `.kutha/STATE.md`; a wrong citation fails
- [ ] **SEM-04**: `honeycomb.yaml` `evidence` and `capability: named` are resolved: each named evidence item points to an existing test or path, and a cell claiming a named capability has at least one resolving evidence item
- [ ] **SEM-05**: Every `ADR-NNN` and `D1`–`D10` reference in `AGENTS.md`, `README.md`, `.kutha/`, and `docs/process/` resolves to an existing ADR or lock; a dangling reference fails
- [ ] **SEM-06**: `AGENTS.md` has an enforced size budget and forbids lease tokens, so CTX-01/02 cannot regress silently
- [ ] **SEM-07**: Product gaps F1–F8 are recorded as `disposition: deferred` invariants with `until` naming their target milestone; they are not enforced and not implemented
- [ ] **SEM-08**: Each new kind has a red-path pytest, META.md and `docs/process/governor-intake.md` document it, and CHANGELOG records the process change under **Process**

## Future Requirements

Product plane, separate lease (M012a): single-log SoT for outcomes/justifications, stable `EventId`/content-addressed references, proposition ids and idempotent ingest, n-ary derive + rule registry, `open`→`replay_check`, atomic segmented persist, stable `Define` ids, declared `TimeScale`, fold-internal indexes. Then M012 (dictionaries as facts subset), M002 (log durability, then Rocks for indexes), M003 Cypher skin, thin legal golden fixture, later verticals as in `.kutha/ROADMAP.md`.

## Out of Scope

| Feature | Reason |
|---------|--------|
| Any `crates/` change | Boundary of v0.03; product fixes need an explicit M012a lease |
| Promoting any ADR to Accepted | GATE-03; cells become Accepted only when running in the engine |
| Rewriting ADR-000 D1–D10 | Locks change by explicit question and review, not by amendment sweep |
| Starting M012a / M012 / M002 | Requires a STATE lease; this milestone only records the corrected order |
| Enforcing F1–F8 in the governor | They are product gaps; the governor only records them as deferred invariants |
| Reverse-audit of every ADR sentence | Explicitly excluded by `docs/process/governor-intake.md` |
| New Python `Check` subclasses | Checks stay YAML rows; new behaviour is a kind in `kinds.py` |
| CI service / GitHub Actions | Not requested; unpushed history stays a separate decision |

## Traceability

Updated during roadmap creation.

| Requirement | Phase | Status |
|-------------|-------|--------|
| LEASE-01 | Phase 9 | Pending |
| CTX-01 | Phase 9 | Pending |
| CTX-02 | Phase 9 | Pending |
| CTX-03 | Phase 9 | Pending |
| CTX-04 | Phase 9 | Pending |
| ADR-01 | Phase 10 | Pending |
| ADR-02 | Phase 10 | Pending |
| ADR-03 | Phase 10 | Pending |
| ADR-04 | Phase 10 | Pending |
| SEM-01 | Phase 11 | Pending |
| SEM-02 | Phase 11 | Pending |
| SEM-03 | Phase 11 | Pending |
| SEM-04 | Phase 11 | Pending |
| SEM-05 | Phase 11 | Pending |
| SEM-06 | Phase 11 | Pending |
| SEM-07 | Phase 11 | Pending |
| SEM-08 | Phase 11 | Pending |

**Coverage:** 17/17 v0.03 requirements mapped ✓

---
*Requirements defined: 2026-09-30*
