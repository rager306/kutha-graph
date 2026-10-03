# 54-01 / 54-02 SUMMARY (rolled; n≤2 with 54-03-SUMMARY)

Rolled so 54-03 can keep `54-03-SUMMARY.md`. Full 54-01/02 text: git `39af84b` / `be57466`.

## 54-01 — reader

Commits: `de1c565` RED, `189757c` GREEN, `8326c09` no-trailer, `39af84b` docs. Bench vs `3c469fe`: ingest +0.6%, as_of +3.1%, open.n −5.2%.

## 54-02 — persist / Persister / rollover

Commits: `0eed2d4` RED, `4f8bdc2` GREEN, `be57466` docs. Bench vs `39af84b`: ingest −5.9%, as_of +0.0%, open.n −7.8%. Empty dir = no manifest, no jsonl, no wal.

## 54-03 — migrate / export / tamper

Commits: `c79927d` RED, `4810eaf` GREEN, `e695d98` smashed-jsonl, `424bda6` docs. Bench vs `be57466`: ingest +7.1%, as_of +8.6%, open.n +0.3%.

## Second round (independent verifier)

Commits: `1239585` RED migrate-verify, `987f263` GREEN failpoint+unflip, `4711492` RED stitch oracles, `e46d7c6` GREEN header prev + named needles, `2cc3639` hash-chain vs chain-link needles, this docs commit. Did not edit STATE/README/REQUIREMENTS/ROADMAP/AGENTS or `54-VERIFICATION.md`. Honeycomb stays Proposed. Workspace `cargo test --workspace --offline` green.

Header 44 bytes (prev tip; H0 only for seq 1). Open: contiguous sealed+active names, then per-file missing / chain-link / CRC / trailer / count / bytes / hash chain / manifest; active links to last sealed tip. Does **not** stop a full rewrite of files+manifest+hashes; manifest tip is not anchored.

Living jsonl-SoT: dated 2026-10-04 pointer under ADR-010 Clarification 2026-10-03 (and D010-4 / ADR-000 D6 2026-10-03); 2026-10-03 text kept. Dated Evidence stays.

Mutations (restore `git checkout --`; tree clean): skip verify-reopen (o); skip chain-link (swap/replace/delete-middle/active-from-other); skip contiguity (reorder); skip CRC / count / missing / bytes / hash-chain — each failed an oracle.

Controller (not edited): `.kutha/ROADMAP.md` L265 thaw “keeps `events.jsonl` as SoT”; L281–282 S06 “the jsonl export is no longer the SoT”. `AGENTS.md` / `README.md` / `.planning/REQUIREMENTS.md` DOC-06 as listed in `54-03-SUMMARY.md`.

Bench vs `424bda6` (2 interleaved pairs): ingest_plain.n −9.4%, as_of.n −9.7%, open.n +1.6% — D-09 met.
