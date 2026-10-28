---
phase: "03"
slug: "lease-gated-next-slice"
status: verified
threats_open: 0
asvs_level: 1
created: "2026-09-29"
---

# Phase 3 — Security

> Per-phase security contract: threat register, accepted risks, and audit trail.
> Overlay-only (verification / negative proof). No product-crate slice. ASVS L1: plan-authored register, all blocking threats closed in evidence SoT.

---

## Trust Boundaries

| Boundary | Description | Data Crossing |
|----------|-------------|---------------|
| Developer shell → kutha-gov ci | Trajectory honesty gate | Process HIGH/LOW, checks.yaml |
| GSD VERIFICATION.md → phase complete claim | Evidence SoT | Probe rows, gate exits |
| GSD execute → `.kutha/STATE.md` Active Slice | Lease SoT (cite only) | Active Slice name |
| Product crates ↔ harness dictionaries | Plane split (D-L2) | Empty crates porcelain under None |
| Probe commands → VERIFICATION pass cells | Evidence integrity | Path/lease/caption probes |
| VERIFICATION passed → REQUIREMENTS [x] | Checkbox batch | GOV-03 / NEXT-01 / NEXT-02 |
| GSD STATE → harness STATE | Cite-only | Must not overwrite `.kutha/STATE.md` |

---

## Threat Register

| Threat ID | Category | Component | Severity | Disposition | Mitigation | Status |
|-----------|----------|-----------|----------|-------------|------------|--------|
| T-03-01 | Spoofing | Governor green as Accepted / lease grant | high | mitigate | D-10 authority: none + green≠Accepted≠lease in VERIFICATION/SUMMARY | closed |
| T-03-02 | Tampering | Silent HIGH / skipped ci | high | mitigate | D-L3/D-G1/D-G2: ci required; HIGH-free recorded | closed |
| T-03-03 | Elevation of privilege | Product slice without Active Slice lease | high | mitigate | D-L1/D-L2/D-L4: Active Slice None; no slice implementation | closed |
| T-03-04 | Repudiation | Unlogged LOW/WARN | medium | mitigate | D-11: LOW=0 empty WARN ledger | closed |
| T-03-05 | Tampering | False NEXT-01 via pack trees | high | mitigate | Path absence + h4-lease/freeze + ADR-090 Proposed (D-L6) | closed |
| T-03-06 | Spoofing | NEXT-02 via assumed M002 / honeycomb | high | mitigate | Negative caption probes (D-L5) | closed |
| T-03-07 | Tampering | Overview implies infinite hold | medium | mitigate | D-G3 Overview rewrite; negative-proof needle | closed |
| T-03-08 | Information disclosure | Full events.jsonl / FSM dump in SUMMARY | low | accept | D-10 excerpt ≤8 lines; no JSONL paste | closed |
| T-03-09 | Spoofing | Checkbox batch without green VERIFICATION | high | mitigate | status: passed + pre-verify ci/cargo before REQUIREMENTS [x] | closed |
| T-03-10 | Tampering | Accidental uncheck of FIT/Phase 2 IDs | medium | mitigate | Leave-checked greps FIT-01 and MAP-01 | closed |
| T-03-11 | Elevation of privilege | Closeout as freeze thaw / M002 grant | high | mitigate | Next crate work only if STATE names a slice; D-L4 still None | closed |
| T-03-SC | Tampering | npm/pip/cargo installs | high | mitigate | No package installs in Phase 3; Package Legitimacy N/A | closed |

*Status: open · closed · open — below high threshold (non-blocking)*
*Severity: critical > high > medium > low — only open threats at or above workflow.security_block_on count toward threats_open*
*Disposition: mitigate (implementation required) · accept (documented risk) · transfer (third-party)*

---

## Accepted Risks Log

| Risk ID | Threat Ref | Rationale | Accepted By | Date |
|---------|------------|-----------|-------------|------|
| AR-03-01 | T-03-08 | D-10 already caps Trajectory excerpts; residual leak if an agent pastes JSONL anyway is process, not a crate vuln | plan disposition accept | 2026-09-29 |

---

## Security Audit Trail

| Audit Date | Threats Total | Closed | Open | Run By |
|------------|---------------|--------|------|--------|
| 2026-09-29 | 12 | 12 | 0 | gsd-verify-work verify:post (ASVS L1, register authored at plan time) |

---

## Sign-Off

- [x] All threats have a disposition (mitigate / accept / transfer)
- [x] Accepted risks documented in Accepted Risks Log
- [x] `threats_open: 0` confirmed
- [x] `status: verified` set in frontmatter

**Approval:** verified 2026-09-29
