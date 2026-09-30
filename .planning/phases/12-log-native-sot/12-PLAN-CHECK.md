## VERIFICATION FAILED

**Phase:** 12-log-native-sot (Log-native SoT)
**Plans checked:** 3 (12-01, 12-02, 12-03)
**Issues:** 2 blocker(s), 0 warning(s), 2 info

Plans **do** map LOG-01 → 12-01, LOG-02 → 12-02, LOG-03 → 12-03 and trace ROADMAP success criteria 1–4 into executable tasks with valid waves (`01` → `02` → `03`) and `depends_on: ["12-01"]` / `["12-02"]`. `verify.plan-structure` reports **valid** for all three plans. Revision is required before execute on Nyquist failing-direction and one vacuous lib-test verify chain.

### Coverage Summary

| Requirement / criterion | Plan(s) | Status |
|-------------------------|---------|--------|
| LOG-01 | 12-01 | Covered (named oracle + M011 crash update) |
| LOG-02 | 12-02 | Covered (justifications + resume + e2e lease discard) |
| LOG-03 | 12-03 | Covered (fingerprint mix + LOG-03 oracle) |
| Success criterion 4 (GATE-01 governor) | 12-03 task 2 | Covered (fsm/checks/bridges/honeycomb + `kutha-gov ci`) |
| CONTEXT D-01..D-04 | 12-01..03 | Honored; Phase 13–15 non-goals not scheduled |

### Plan Summary

| Plan | Tasks | Wave | depends_on | Structure |
|------|-------|------|------------|-----------|
| 12-01 | 3 | 1 | [] | valid |
| 12-02 | 2 | 2 | 12-01 | valid |
| 12-03 | 2 | 3 | 12-02 | valid |

**Estimates (advisory):** 12-01 72k, 12-02 64k, 12-03 54k — all under 100k budget; `confidence: low` (no calibrated actuals).

### Blockers — these properties must hold

**1. [Nyquist / failing direction] Every runnable `<automated>` command has a stated `<fails_when>`**

- Plan: 12-01, 12-02, 12-03 (7 commands total)
- Evidence: `gsd-tools check verify-failure-directions 12` → `status: blocked`, `counts.blocker: 7`; no `<fails_when>` siblings in any `<verify>` block.
- Example fix (non-binding): Add `<fails_when>non-zero exit from cargo/uv, or test summary shows failed &gt; 0</fails_when>` after each `<automated>` (one statement per command when chained).

**2. [Verify command format] `<verify>` must not pass when zero tests ran**

- Plan: 12-01
- Task: 2 — Keep snapshot offset and graph_len on fold-affecting ops only
- Evidence: Today `cargo test -p kutha-runtime --offline --lib cascade_idles_without_inverse_loop -- --exact` reports `running 0 tests` / exit 0. Lib unit tests are named `quantum::tests::…`; short `--exact` filters are a no-op pass. Same pattern for `snapshot_plus_tail_matches_full_replay` and `persist_open_round_trip` in that command.
- Example fix (non-binding): Drop `--exact` for `--lib` filters, or use full names (`quantum::tests::cascade_idles_without_inverse_loop`), or run a single `--lib` without per-test exact filters.

### Advisories (info)

**1. [research_resolution] Dimension 11 / Nyquist VALIDATION.md gate**

- Evidence: Phase directory has CONTEXT + PLANs only; no `12-RESEARCH.md` or `*-VALIDATION.md`. With `workflow.nyquist_validation: true`, validation architecture was not materialized for this phase (Dimension 8 checks 8a–8d skipped per skip rules).

**2. [scope_sanity] Estimate confidence**

- Evidence: All three plans mark `estimate.confidence: low` (`sample_count: 0`). Token estimates are directional only; task counts (3/2/2) are within targets.

### Structured Issues

```yaml
issues:
  - dimension: nyquist_compliance
    severity: blocker
    required_property: "Each runnable automated verify command names what output constitutes failure"
    description: "Seven automated commands across 12-01..12-03 have no fails_when (probe counts.blocker=7)"
    plan: "12-01"
    fix_hint: "Add fails_when after each automated block per failing-direction.md"

  - dimension: verify_command_format
    severity: blocker
    required_property: "Every verify separates pass from fail (no vacuous zero-test success)"
    description: "12-01 task 2 lib cargo test --exact uses short test names; cargo runs 0 tests with exit 0"
    plan: "12-01"
    task: 2
    fix_hint: "Use quantum::tests::… full names or drop --exact for --lib invocations"

  - dimension: research_resolution
    severity: info
    required_property: "RESEARCH.md carries no unresolved open question when research workflow is enabled"
    description: "No phase RESEARCH.md or VALIDATION.md present under 12-log-native-sot"
    plan: null
    fix_hint: "Optional follow-up research artifact; plans are otherwise self-contained"

  - dimension: scope_sanity
    severity: info
    required_property: "Token estimates are interpreted with appropriate confidence"
    description: "All plans estimate.confidence low with zero calibration samples"
    plan: null
    fix_hint: "None required for execute; re-slice only if executor hits context limits"
```

### Recommendation

2 blocker(s) require planner revision (failing directions + lib verify chain). After revision, re-run plan-checker. LOG-01..03 **requirement coverage** is complete; failures are verification hygiene and one false-green command, not missing LOG tasks.
