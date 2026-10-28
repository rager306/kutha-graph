---
phase: "241"
name: "ci-gate-truth"
created: 2026-10-28
---

# Phase 241: CI gate truth — Context

## Decisions

1. `dtolnay/rust-toolchain` must receive `toolchain: 1.99.0`; the pinned action otherwise fails at parse and no gate runs.
2. The workflow runs `uv run pytest -q` before `uv run kutha-gov ci --jobs 1` so the cheap harness gate fails first.
3. Local proof is YAML parse plus `precommit`; remote green is a separate claim that needs a push.
4. No product code, no thaw.

## Discretion Areas

Whether pytest is its own job or a step in `gate`; keep one job to reuse the uv/cargo caches.

## Deferred Ideas

Required status checks stay off until a remote run is observed green.
