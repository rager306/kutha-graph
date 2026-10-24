# Repository baseline (2026-10-23)

Public-repo baseline used for this pass: GitHub community-health files, OpenSSF Scorecard checks (security policy, license, dependency update tool, token permissions, pinned actions, SAST, branch protection, dangerous workflow), and the arsumbris entry repo's habit of stating status and risk in plain files (`README` status, `SAFETY.md`) instead of implying a guarantee the project does not have.

## In the tree

| Piece | Where |
|-------|--------|
| Security report path | `SECURITY.md` (private GitHub advisory) |
| How to change the repo | `CONTRIBUTING.md` |
| Conduct | `CODE_OF_CONDUCT.md` |
| Issue and PR forms | `.github/ISSUE_TEMPLATE/`, `.github/PULL_REQUEST_TEMPLATE.md` |
| Owner hint | `.github/CODEOWNERS` (does not block a push) |
| Dependency PRs | `.github/dependabot.yml` (cargo, uv, github-actions, weekly) |
| CI | `.github/workflows/ci.yml` runs `uv run kutha-gov ci --jobs 1` after `cargo fetch --locked` |
| SAST | `.github/workflows/codeql.yml` (Rust and Python) |
| Scorecard | `.github/workflows/scorecard.yml` (publishes on `main`) |
| License | `LICENSE` — Apache License 2.0 (SPDX `Apache-2.0`) |
| Release id | `VERSION` — `N.N.NN` (now `0.0.01`). Cargo and pyproject use `0.0.1` |
| Line endings | `.gitattributes`, `.editorconfig` |
| Rust format edition | `rustfmt.toml` (`edition = "2024"`) |

Workflow `permissions` are read-only except the two jobs that must upload SARIF. Actions are pinned to commit SHAs. `pull_request_target` is not used.

## GitHub settings (not in git)

Enabled on `rager306/kutha-graph` on 2026-10-23:

- Dependabot alerts and Dependabot security updates
- Secret scanning and push protection
- Private vulnerability reporting
- Delete head branches on merge
- Ruleset `protect-main-lines` on `main` and `work/main-line`: no branch deletion, no non-fast-forward push. Direct fast-forward pushes stay allowed. Required reviews are not on, because one maintainer pushes the working line.

Secret-scanning validity checks and non-provider patterns stayed disabled. The settings API accepted a patch and left both `disabled`.

## Deliberately not added

| Item | Why |
|------|-----|
| `cargo deny` allow-list | SPDX is Apache-2.0. A `deny.toml` is still not configured. |
| Required pull-request review | Solo working line. The ruleset still blocks force-push and deletion. |
| crates.io publish | `publish` stays false. The GitHub Release is the tag `0.0.01`, not a crate publish. |
| Fuzzing in CI | No parser boundary is leased (B6). A token fuzzer would not test the engine. |
| Verus in CI | Evidence tool, not a gate (`AGENTS.md`, `tools/verus/`). |
| CII Best Practices badge | Not registered. |
| Required status checks | The new workflows have not gone green on GitHub yet. Do not require them until a run passes. |

## ADR recheck (same day)

32 ADR files, 32 index rows in `docs/ADR/README.md`. Every `## Status` is **Proposed**. None are Accepted, Deprecated, or Superseded. Shipped slices (checkpoint, fork diff, thin query IR) are evidence inside Proposed cells.
