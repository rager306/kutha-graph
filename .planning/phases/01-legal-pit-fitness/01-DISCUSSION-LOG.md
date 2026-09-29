# Phase 1: Legal PIT fitness - Discussion Log

> **Audit trail only.** Do not use as input to planning, research, or execution agents.
> Decisions are captured in CONTEXT.md — this log preserves the alternatives considered.

**Date:** 2026-09-29
**Phase:** 1-Legal PIT fitness
**Areas discussed:** Verification command surface, Evidence & checkbox hygiene, Red fitness policy, H2/H4 depth bound

---

## Verification command surface

| Option | Description | Selected |
|--------|-------------|----------|
| Cargo-named only | Named FIT cargo tests; ci soft/Phase 2 | |
| Cargo + full kutha-gov ci | Both hard for Phase 1 | |
| Cargo + precommit only | Dictionaries without emit | |
| You decide | Claude picks default | ✓ (Q1–Q3) |
| Match harness observe --offline | Align with observe.py | ✓ (Q4) |
| Whole workspace | cargo test --workspace | ✓ (discretion) |
| Skip ci in Phase 1 | ci deferred to Phase 2 | ✓ (discretion) |

**User's choice:** Q1–Q3 You decide; Q4 Match harness observe (`--offline`). User later asked Claude to decide remaining gray areas reasonably.
**Notes:** Locked: `cargo test --workspace --offline`; named tests = evidence map; ci not Phase 1 must_have.

---

## Evidence & checkbox hygiene

| Option | Description | Selected |
|--------|-------------|----------|
| VERIFICATION/SUMMARY SoT | Evidence map; no mid-execute REQUIREMENTS edits | ✓ (Claude) |
| Batch [x] at verify | REQUIREMENTS FIT checkboxes when phase verifies | ✓ (Claude) |
| Edit REQUIREMENTS every task | Piecemeal during execute | |

**User's choice:** User asked Claude to decide reasonably.
**Notes:** Avoid checkbox churn mid-execute.

---

## Red fitness policy

| Option | Description | Selected |
|--------|-------------|----------|
| Hard stop on red FIT | Block phase complete | ✓ (Claude) |
| Quarantine with blocker | Continue other FIT | |
| Green with caveat | Complete despite fail | |

**User's choice:** Claude default — fail closed.
**Notes:** No skip without explicit human lease outside CONTEXT.

---

## H2/H4 depth bound

| Option | Description | Selected |
|--------|-------------|----------|
| Existing tests only | h2_* / h4_* files | ✓ (Claude) |
| Thin new harness assert | Extra without ADR-090 | |
| Expand toward legal pack | Out of scope / freeze | |

**User's choice:** Claude default — existing tests only.
**Notes:** Active Slice None; no ADR-090.

---

## Claude's Discretion

- Verification Q1–Q3 (You decide)
- Evidence hygiene, red policy, H2/H4 bound (user: “decide reasonably”)

## Deferred Ideas

- `kutha-gov ci` hard gate → Phase 2
- File-filtered cargo → optional later if runtime pain
