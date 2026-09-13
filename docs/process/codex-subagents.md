# Codex subagent workflow

This is an agent workflow, not a product ADR or a delivery milestone. Repository instructions and `.kutha/STATE.md` determine the authorized scope.

## Local configuration

Configured against Codex CLI 0.154.0 on 2026-09-13. Personal configuration is in `~/.codex/config.toml`; custom roles are standalone TOML files under `~/.codex/agents/`. These machine-local settings are not distributed by cloning this repository.

```toml
[agents]
enabled = true
max_concurrent_threads_per_session = 3
interrupt_message = true
```

The cap excludes the primary agent; the runtime may impose a lower limit. Model selection remains inherited from the parent. The local parent currently uses `gpt-6-astra` with `high` reasoning. Effort is assigned by role to avoid requiring maximum reasoning for every helper.

| Role | Responsibility | Effort | Writes |
| --- | --- | --- | --- |
| `codebase-memory-scout` | Narrow positive discovery; provisional findings | medium | None |
| `codebase-memory` | Task-directed graph verification and source evidence | high | None |
| `codebase-memory-auditor` | Bounded audit with coverage and disclosed gaps | high | None |
| `implementation-worker` | Assigned implementation in owned files | medium | Assigned scope only |
| `correctness-reviewer` | Independent review of a stable diff and evidence | high | None |

Keep ambiguous architecture, temporal truth changes, and difficult correctness reasoning with the parent or an explicitly assigned high-effort worker. Role files that set effort override spawn defaults; use a suitable available role when a different effort is necessary. If custom roles are unavailable, use built-in roles with the same task constraints.

## Delegation contract

Use parallel agents when the work is independently useful. One or two helpers are usually enough. Keep the critical path with the parent, avoid duplicate investigation, and reuse an existing helper for related follow-ups. Do not start additional work merely to occupy slots.

Give each helper this compact brief:

```text
Objective and acceptance criteria:
Owned files and allowed actions:
Relevant STATE/freeze and product/harness constraints:
Evidence tier, graph project/generation/freshness:
Queries, pagination, symbols, paths, coverage gaps and source fallback:
Checks already run and shared outputs to avoid:
Expected result and unresolved questions:
You share this workspace; preserve other edits. Do not spawn more agents.
```

For work outside the code graph, explicitly mark graph analysis not applicable and give the exact source evidence. If a helper lacks MCP access, it must disclose the limitation and use source reads without claiming graph verification.

Assign one writer per file and shared artifact. The integrator owns shared lockfiles, `.kutha/events.jsonl`, `.kutha/tenant`, and graph-index mutations. Coordinate test commands sharing `target` or uv environments. Use an isolated worktree when concurrent work would otherwise overlap, and define how its changes will be integrated.

Reviewers inspect source and supplied test evidence. Checks that write caches or build outputs belong to the integrator or an assigned worker. Read-only intent remains binding even when live runtime permissions override a role's sandbox defaults.

Return a concise outcome, file/symbol references or changed paths, check exit status, and unresolved limitations. The parent inspects the combined diff and runs relevant integration checks after edits stabilize. Repeat checks only when subsequent changes affect their evidence. Product tests and governor checks keep their separate meanings.

## Verification and operation

After changing personal settings, start a new Codex session so role definitions and instructions are loaded together. CLI `/agent` provides access to subagent threads.

Use `codex --strict-config doctor --summary` to check configuration parsing and installation health. Network failures inside a restricted sandbox are not sufficient evidence of broken provider configuration. Doctor does not prove a delegated workflow completed: also run a small, read-only task using the configured roles and verify the returned evidence.

Validation on 2026-09-13: all five role TOML files parsed with their expected effort and inherited model; comparison with the backup confirmed that only the global `agents` table changed in `config.toml`. Strict Doctor completed with no failures and one pre-existing warning for the unset optional `CBM_CACHE_DIR` variable. A separate ephemeral read-only session reported that both `correctness-reviewer` and `implementation-worker` were accepted, spawned, and completed their policy checks. This tests role availability and delegation, not enforcement against every possible instruction or runtime override. No product tests were needed for these configuration and documentation changes.

Reference: [OpenAI subagent documentation](https://learn.chatgpt.com/docs/agent-configuration/subagents) and [configuration reference](https://learn.chatgpt.com/docs/config-file/config-reference). These establish supported settings; the ownership, scope, and validation choices above are local operating decisions.
