---
phase: "247"
name: "asof-bisect"
created: 2026-10-29
---

# Phase 247: as_of bisect — Context

## Decisions

1. Use N=2000 for the bisect predicate to keep each build+run short; compare the stable 2N/4N `as_of` values, not the noisy single `.n` first call.
2. Prime suspect `cec5b64` (dictionary facts) is tested against its parent `7e79912`.
3. Old `cdf1942` already reproduced baseline; no need to re-test it at every step.
4. This is a regression hunt, not B3.

## Discretion Areas

If the suspect pair does not separate, continue with `git bisect` over the remaining range.

## Deferred Ideas

Ingest delta (+12.6%) is a separate bisect.
