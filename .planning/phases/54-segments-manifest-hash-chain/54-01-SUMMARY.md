# 54-01 / 54-02 SUMMARY (rolled for artifact-budget n≤2)

Rolled so 54-03 can keep `54-03-SUMMARY.md`. Full text: git `39af84b` / `be57466`.

## 54-01 — reader

Commits: `de1c565` RED, `189757c` GREEN, `8326c09` no-trailer, `39af84b` docs. RED failed 12 format/open oracles; preserved `legacy_dir_still_opens_and_appends_as_before`. Bench vs `3c469fe`: ingest +0.6%, as_of +3.1%, open.n −5.2%.

## 54-02 — persist / Persister / rollover

Commits: `0eed2d4` RED, `4f8bdc2` GREEN, `be57466` docs. RED failed 18 persist/append/rollover oracles; workspace 271 (`m002_seg01` 33). Empty dir = no manifest, no jsonl, no wal. Bench vs `39af84b`: ingest −5.9%, as_of +0.0%, open.n −7.8%. Nine kill-mutations died.
