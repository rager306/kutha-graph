# Governor CLI / GSD session hardening (2026-10-08)

Post-session notes after M023 close debt + M024 S01–S02 (fold identity + write-path). Not a lease.

## What went wrong this session (trajectory drift)

1. **Uncommitted milestone close** sat in the tree while `docs-coupling` stayed HIGH — close wave must include CHANGELOG in the same commit as harness edits, then lease next work.
2. **Workspace `m019_dur_a` flake** looked like product red; leftover `kutha.lock` / orphaned hold-lock children from parallel runs. Serial and clean-tmp were green. Governor does not distinguish "lock storm" vs fold bug.
3. **GitNexus MCP staleness** reported `commitsBehind` after CLI `analyze --index-only` — MCP process can lag the on-disk index; agents re-queried stale impact.
4. **GSD auto with Active None** cannot invent a milestone; controller must lease before `--only N`. Combined S01 decisions + implement in one wave (ok for thin debt) but id-map still said v0.17 active until docs catch up.
5. **False CI oracle green**: `CorrectInterval` multi-residual test first used a non-intersecting patch (rejected for VT miss, not multi-count). Kill-tests must intersect first-live.
6. **Open/persist oracle vs Dictionary SelfCommit**: Assert of `kutha:*` / Dict relations is SelfCommit, so a crash-window test that Asserts a product relation after intern must use a non-`kutha:` relation — or assert only on `committed_prefix` / dictionary reopen, not "Assert vanishes from open log".
7. **Bench host variance**: docs-only S02 still swung `as_of.n` / `ingest_plain.n` >10% on single runs; median of ≥3 runs needed before claiming gate fail.

## Governor CLI strengtheners (proposals)

| Gap | Proposal | Plane |
|-----|----------|-------|
| Close without CHANGELOG | already `docs-coupling`; add needle that Active None + ROADMAP CLOSED requires audit path present | process |
| Product close without workspace+ci | already `product-close-gate`; keep fail-closed | process |
| Bench numbers missing from close CHANGELOG | optional `file_contains` on last Process/Product H2 citing `ingest_plain.n` when crates changed in same commit window | process |
| Single-run bench false red | record `runs≥3` / median in VERIFICATION or allow LOW note when fold bytes unchanged (diff is docs-only) | process |
| Stale VERIFICATION after edit | already in gsd-auto-contract; governor could HIGH if `verified:` timestamp older than file mtime of cited crates (expensive) — defer | process |
| Parallel store-lock flakes | document in META: clean `tmp/p60-store-*` before full workspace; or selftest that `m019_dur_a` ×3 under `--test-threads=8` | process note |
| Cite lease after close | README/STATE already; extend `cite-lease` to flag `M0xx ACTIVE` in ROADMAP when STATE is None | process |
| ROADMAP index vs section | check enumerated "Later milestones" line matches CLOSED/ACTIVE heading above | process |
| Session log / GSD | require `.planning/STATE.md` `last_updated` bump when ROADMAP ticks; optional check | GSD overlay |
| Honeycomb evidence ↔ oracle names | already needles; fail if ADR Clarification cites oracle absent from honeycomb evidence list | process |

## GSD session log checklist (operators)

1. Reindex CBM (`index_repository`) + GitNexus (`analyze --index-only`) at session start and after large product commits; treat MCP `commitsBehind` as signal to re-analyze from the controller.
2. Commit close debt before leasing the next milestone (STATE+CHANGELOG lease commit is separate from product).
3. RED oracles before fold edits; record bench pre/post on the same host (median of ≥3 if variance >5%).
4. Independent `NN-VERIFICATION.md` before clearing Active Slice.
5. Do not promote honeycomb Accepted from governor green.
6. After S02-style characterization: prefer `committed_prefix` unit oracles for SelfCommit windows; keep open/persist for meaning recovery only.
7. **Knowledge-cut fixture order:** for “known but not in force”, Assert must be ingested **before** the `TtAnchor` that maps the wall cut (else `ingested_at > tt` hides the fact and the pin looks like a product bug).
