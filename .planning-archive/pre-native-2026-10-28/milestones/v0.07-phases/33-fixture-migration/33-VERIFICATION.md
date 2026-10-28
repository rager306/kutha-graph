---
independent: true
verifier_model: composer-2.5
status: passed
score: 3/3
exit: pass
covered_files: [.planning/REQUIREMENTS.md, .planning/ROADMAP.md, CHANGELOG.md, crates/kutha-runtime/tests/ff5_legal_pit.rs, crates/kutha-runtime/tests/ff6_allowlist.rs, crates/kutha-runtime/tests/m011_e2e_fixture.rs]
covered_digest: "v2:sha256:950f166be13af3139ec030ff0c62f6a928895283bdd9842bdac348238ebfe805"
commands:
  - cmd: "git diff 8d6513a..58b3bc9 -- FF5/FF6/M011–M013 tests"; exit: 0
  - cmd: "! git diff -U0 8d6513a..58b3bc9 -- crates/kutha-runtime/tests | rg '^[+-].*valid_from:'"; exit: 0
  - cmd: "cargo test -p kutha-runtime --offline (named FF5–M013 oracles)"; exit: 0
  - cmd: "cargo test --workspace --offline"; exit: 0
  - cmd: "git diff --exit-code -- .kutha/STATE.md"; exit: 0
read_only_inputs:
  - .planning/REQUIREMENTS.md
  - .planning/ROADMAP.md
  - .planning/phases/33-fixture-migration/33-CONTEXT.md
  - .planning/phases/33-fixture-migration/33-RESEARCH.md
  - crates/kutha-runtime/tests/ff5_legal_pit.rs
  - crates/kutha-runtime/tests/ff6_allowlist.rs
  - crates/kutha-runtime/tests/m011_e2e_fixture.rs
  - crates/kutha-runtime/tests/m012a_hot_indexes.rs
  - crates/kutha-runtime/tests/m013_dual_mode.rs
  - CHANGELOG.md
  - .kutha/STATE.md
cbm: { list_projects: kutha-graph, detect_changes: inbound on cited test paths }
---

# Phase 33 — independent verification

**ROADMAP SC:** (1) FF5/M011/M012/M013 explicit clock types at public cuts. (2) Oracle assertions unchanged; wrap table in executor SUMMARY. (3) `kutha-gov ci` 0 HIGH at milestone close (not re-run here; D-03).

| REQ | Status | Evidence |
|-----|--------|----------|
| FIX-01 | satisfied | `8d6513a..58b3bc9`: `ValidTime::from` / `TransactionTime::MAX` on `as_of`, `csr_lease_at`, `live_count`, `claim_supported_at`, `derivation_eligible_at`, `record_action`, etc.; `Op` `valid_from` absent from diff |
| FIX-02 | satisfied | No oracle message-string edits; assert expected literals/polarity unchanged (only clock args wrapped, incl. multi-line fmt); `m014_as_known_on.rs` out of diff |

**Kill-tests:** (1) constructor wraps only — pass. (2) Named oracles — pass. (3) Workspace `--offline` — 35× `test result: ok.`, no `FAILED`.

**Score:** 3/3 (FIX-01, FIX-02, SC1–2). D-05: verifier ≠ executor; harness STATE untouched. M015 not started. CBM `detect_changes` shows inbound impact on `ff6_allowlist`, `m011_e2e_fixture` helpers — consistent with wraps only.
