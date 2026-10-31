---
phase: "269"
slug: "meta-surface-threat-audit-and-adversarial-tests"
status: verified
threats_open: 0
asvs_level: 1
created: "2026-10-31"
---

# Phase 269 — Security

> Per-phase security contract for the C1/C1b/C5 meta surfaces and the replay reader gate.

---

## Trust Boundaries

| Boundary | Description | Data Crossing |
|----------|-------------|---------------|
| Log writer → admission | Untrusted caller input becomes log events | pack identity, operator names, enrichment pins, payloads |
| Stored log → replay/open | A segment on disk may be hand-edited or foreign-written | full event log, blobs, lifecycle state |
| Manifest → operator registry | Declaration data claims operator names | operator name strings |
| Enrichment → derived ledger | Proposed derived content, never kernel truth | model/prompt/cache pins, payload digest |

---

## Threat Register

| Threat ID | Category | Component | Severity | Disposition | Mitigation | Status |
|-----------|----------|-----------|----------|-------------|------------|--------|
| T-269-01 | Denial of service | `replay_check` meta walk | high | mitigate | Single-pass tracker; `m182_meta_threats::replay_meta_is_not_quadratic` (4000 ops < 3 s; was ~2.18 s at 2000) | closed |
| T-269-02 | Spoofing | manifest digest | high | mitigate | Raw SHA-256 re-verified in `load_manifest`; `manifest_blob_digest_mismatch_fails_closed` | closed |
| T-269-03 | Tampering | manifest order | high | mitigate | Prefix-scoped replay; `manifest_blob_after_install_fails_closed` | closed |
| T-269-04 | Elevation of privilege | operator namespace | medium | mitigate | Cross-pack collision check; `cross_pack_operator_collision_fails_on_replay` | closed |
| T-269-05 | Spoofing | pack identity | high | mitigate | Exact id/name/version match; `manifest_identity_spoof_fails_on_replay` | closed |
| T-269-06 | Repudiation / forgery | public `emit` | high | mitigate | `reject_forged_meta`; `user_forged_meta_ops_are_rejected` | closed |
| T-269-07 | Tampering | enrichment payload | medium | mitigate | `StoreBlob` digest + replay digest check; `enrichment_payload_tamper_fails_on_replay` | closed |
| T-269-08 | Inconsistent enforcement | admission vs replay | medium | mitigate | One shared `PackLifecycleTracker`; `admission_and_replay_agree_on_lifecycle` | closed |
| T-269-09 | Information disclosure | enrichment source ids | low | accept | See accepted risks | closed |
| T-269-10 | Elevation of privilege | operator execution | high | accept | No executor exists; a future executor needs C3/C6 | closed |

*Status: open · closed*
*Severity: critical > high > medium > low*

---

## Accepted Risks Log

| Risk ID | Threat Ref | Rationale | Accepted By | Date |
|---------|------------|-----------|-------------|------|
| R-269-01 | T-269-09 | Enrichment source ids are existence-only and global cache keys are unique; a proposal can reveal that some event id exists. Enrichment is derived data, never kernel truth, and no cross-tenant boundary exists yet. Revisit when tenant slices land. | owner-lease M182-S01 | 2026-10-31 |
| R-269-02 | T-269-10 | The manifest names operators but nothing resolves or executes them, so there is no code-execution surface to attack. Accepting this is only sound while no executor exists; C3/C6 must re-open it. | owner-lease M182-S01 | 2026-10-31 |

---

## Security Audit Trail

| Audit Date | Threats Total | Closed | Open | Run By |
|------------|---------------|--------|------|--------|
| 2026-10-31 | 10 | 10 | 0 | M182-S01 + independent verifier |

---

## Sign-Off

- [x] All threats have a disposition (mitigate / accept / transfer)
- [x] Accepted risks documented in Accepted Risks Log
- [x] `threats_open: 0` confirmed
- [x] `status: verified` set in frontmatter

**Approval:** verified 2026-10-31
