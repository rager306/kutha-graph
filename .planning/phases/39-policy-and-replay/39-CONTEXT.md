# Phase 39 Context: Policy evaluation and execution replay

**Milestone:** v0.09 / harness M016 S04
**Requirements:** RULE-03, RULE-04
**Depends on:** Phase 38 (RULE-02)
**Source:** `.kutha/ROADMAP.md` M016 S04; F16; `check_admission_row`; `replay_check`
**Run rules:** `docs/process/gsd-auto-contract.md`

## Locked decisions

- **D-01 (RULE-03):** `check_admission_row` evaluates the live policy blob at `(tt, vt)` (`live_policy_pin_at` + StoreBlob). Trimmed body `deny` denies (`reason: "policy"`). Any other non-empty live body admits (keeps `leased-policy`). Changing the live pin to `deny` changes a subsequent decision; prior admission facts remain queryable AS OF.
- **D-02 (RULE-04):** `replay_check` keeps fingerprint + caused_by lineage. Additionally, every Horn-pinned `Behavior` on the log must be reproducible from premises live at its `ingested_at` (same pin, endpoints, relation). Empty-pin or unmatched Horn fails closed. Do not change fingerprint contents (facts-only).
- **D-03:** Preserve FF5, FF6, `admission_cites_pinned_policy_version`, two-hop eligibility, M012 registry assertions. Mechanical constructor/helper renames only.
- **D-04 (non-goals):** Golden fixture / five clocks (40); MATCH; polynomials; M002; honeycomb Accepted; PolicyAst (UTF-8 body stays).
- **D-05:** Verifier ≠ executor. Do not write 39-VERIFICATION. Never edit `.kutha/STATE.md`. No `kutha-gov ci`.
- **D-06:** CBM graph-first; GitNexus secondary; `rg` arbiter.

## Verifier kill-tests

- Live policy body `deny` fails `check_admission` with reason policy; `leased-policy` still admits.
- After switching live pin to `deny`, a new justification decision changes; prior admission AS OF unchanged.
- Log with empty-pin `inverse_knows` (same triple) fails `replay_check` even if fingerprint matches.

## Non-goals

See D-04.

## Harness

Active Milestone M016, Active Slice **S04**.
