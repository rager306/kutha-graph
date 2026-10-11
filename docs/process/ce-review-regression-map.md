# CE whole-repo regression map (2026-10-11)

Process note. Maps review findings to matrix rows and oracles so the same
defect classes cannot silent-regress. Not an ADR. Honeycomb stays **Proposed**.

Source review: `.compound-engineering/artifacts/reviews/2026-10-11-ce-whole-repo/`
plus `advisor-correction.md`.

## Closed / demoted → killed regressions

| Finding / claim | Matrix id | Oracle(s) | Owner |
|-----------------|-----------|-----------|-------|
| #1 Op decode (demoted; documented break) | `SEM-m050-op-compat-policy`, `SEM-m053-legacy-meta-ops-fail-open` | `m050_op_compat` + `all_five_legacy_dictionary_meta_ops_fail_deserialize` | M050-S01 / M053-S01 |
| #7 lock before heal | `SEM-m052-lock-before-heal` | `m052_lock_before_heal` | M052-S03 |
| #8 cascade atomicity | `SEM-m052-cascade-atomic` | `m052_cascade_atomic` | M052-S02 |
| #11 snapshot-free open replay | `SEM-m052-open-replay` | `m052_open_replay` | M052-S04 |
| Withdrawal: hydrate outcomes | `SEM-m053-hydrate-outcomes` | `fork_at_rebuilds_outcomes_from_log_quantum_outcomes` | M053-S01 |
| Withdrawal: persist_dir anchor | `SEM-m053-persist-dir-anchor` | `common_persist_dir_anchors_under_manifest_tmp_not_cwd` | M053-S01 |
| Withdrawal: rollover layout seam | `SEM-m053-rollover-layout-seam` | `try_persist_with_layout_preserves_rollover_seam_contract` | M053-S01 |

Pin scheme (full-branch #24 / M051) stays `SEM-m051-pin-compat-policy` — distinct from whole-repo EventId #24.

## Open problem classes → gap rows (M054 backlog)

| Class | Findings | Matrix id | Owner |
|-------|----------|-----------|-------|
| Horn soundness | #2 #3 #4 #5 | `SEM-gap-horn-soundness` | M054-S01 |
| Persist durability | #6 #9 #10 | `SEM-gap-persist-durability` | M054-S02 |
| Governor honesty | #14 #22 #23 #34 | `SEM-gap-governor-honesty` | M054-S03 |
| Scaling + bench | #15 #16 #17 #19 #25 #27 | `SEM-gap-scaling-bench` | M054-S04 |
| ID + structure | #24 #12 #20 #21 | `SEM-gap-id-and-structure` | M054-S05 |

Adjacent residuals (#26 #28 #29 #30 #35 #36 and P2 eligibility) stay on the advisory follow-on list until a leased slice names them.

## Rule

New CE findings that land a fix must add a `killed` matrix row with a real
`kill_mutation` and named `#[test]` / check oracle in the same wave. Gap rows
are the honest parking lot until a slice is leased — not a silent pass.
