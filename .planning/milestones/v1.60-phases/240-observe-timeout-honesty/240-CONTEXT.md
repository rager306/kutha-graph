---
phase: "240"
name: "observe-timeout-honesty"
created: 2026-10-28
---

# Phase 240: Observe timeout honesty — Context

## Decisions

1. A cargo timeout (`124`) or missing binary (`127`) must not retry the test invocation. The retry loop exists for incomplete output, not for a spent wall-clock budget.
2. The build step stays skipped for `124`/`127`; non-zero test status still builds so tenant ingest can see fail.
3. The regression is the existing `test_observe_skips_build_after_timeout` plus one explicit retries-env test.
4. No product code, no Op, no thaw.

## Discretion Areas

Whether the new regression is a separate test or an assertion inside the existing one; prefer a separate name so the retries env is explicit.

## Deferred Ideas

Retry policy for partial output remains as shipped in M149; do not change it.
