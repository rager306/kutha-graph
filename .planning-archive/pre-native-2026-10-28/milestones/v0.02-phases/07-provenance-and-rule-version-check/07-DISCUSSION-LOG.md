# Phase 7: Provenance and rule-version check - Discussion Log

> **Audit trail only.** Do not use as input to planning, research, or execution agents.
> Decisions are captured in CONTEXT.md — this log preserves the alternatives considered.

**Date:** 2026-09-30
**Phase:** 7-provenance-and-rule-version-check
**Areas discussed:** PROV-01 caused_by oracle, rule version carrier, verification API, fixture width

---

## PROV-01 caused_by oracle

| Option | Description | Selected |
|--------|-------------|----------|
| Ghost only | Rely on existing BrokenLineage for absent prior | |
| Swap valid prior | Two priors; rewrite caused_by A1→A2; state FP match; provenance fails | ✓ |
| Mutate live SoT API | Production in-place log rewrite helper | |

**User's choice:** Claude discretion («реши эти вопросы разумно»)
**Notes:** CBM showed `replay_check` already accepts any earlier id; ghost test exists and does not prove PROV-01. Fingerprint is Facts-only; fold ignores `caused_by`.

---

## Rule version carrier

| Option | Description | Selected |
|--------|-------------|----------|
| Field on Behavior | `rule_version: String` with serde default; fold ignores | ✓ |
| Outcomes sidecar only | Pin in quantum_outcomes.jsonl | |
| Define / ADR-050 | Term or meta_prompt dictionary | |

**User's choice:** Claude discretion
**Notes:** Sidecar is S05 outcomes SoT, wrong plane for rule pin. ADR-050 frozen. Behavior already has `name` for behavior id.

---

## Verification API

| Option | Description | Selected |
|--------|-------------|----------|
| Extend replay_check only | One method mixes state + provenance | |
| Separate provenance surface | Keep replay_check; add provenance_fingerprint / check | ✓ |
| Full execution replay | Rerun behaviors / CA cache | |

**User's choice:** Claude discretion
**Notes:** ADR-060 three obligations; execution replay explicitly out of REQUIREMENTS for S07. Inbound callers of `replay_check` must not break.

---

## Fixture width

| Option | Description | Selected |
|--------|-------------|----------|
| Two thin oracles | PROV-01 + PROV-02 named tests only | ✓ |
| Observation 4 slice | Summary/action cites in this phase | |
| Full t1–t3 fixture | Semantic-contract narrative | |

**User's choice:** Claude discretion
**Notes:** Observation 4 / FIX-02 deferred to Phase 8 / S08.

---

## Claude's Discretion

All four gray areas — user requested reasonable decisions + CBM during execution.

## Deferred Ideas

- FIX-02 summary/action cites → Phase 8
- Execution replay → later ADR-060 work
- Provenance polynomials → not P0
