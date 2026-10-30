# B3 competitor environment (2026-10-30)

Status: **phase 252 setup**, not a comparison and not a B4/B5 decision.

## Images

| engine | image | digest | container / ports |
|---|---|---|---|
| FalkorDB | `falkordb/falkordb:latest` | `sha256:725b74d400124862c9e0f318c56bfb3c55b97cd7f301d9dfe8685980fb3c8195` | `kutha-b3-falkor`, 127.0.0.1:6381 -> 6379 |
| Samyama | `public.ecr.aws/f9f6l5u4/samyama-graph:1.1.0` | `sha256:458895059c24b8b809f7e9fa42b62a16734254b0cc4dc7562a34cca12506f517` | `kutha-b3-samyama`, 127.0.0.1:6380 -> 6379, 8081 -> 8080 |

Both containers started 2026-10-30 ~15:47 +07. Samyama health `healthy`; Falkor `PONG`.

## Smoke

- FalkorDB: `GRAPH.QUERY b3 "RETURN 1"` returns 1.
- Samyama: `POST /api/query {"query":"RETURN 1 AS one","graph":"b3"}` returns `{"columns":["one"],...,"records":[[1]]}`.

## Smoke load

- FalkorDB loaded the phase-251 shape: 768 nodes (L0/L1/L2 x 256) and 2,048 `REL` edges; `MATCH (a:L0)-[:REL*2]->(c:L2) RETURN count(c)` returns 4,096; internal execution 5.0 ms. This is a smoke number, not a calibrated comparison: the client path differs from Kutha's in-process walk.
- Samyama `RETURN 1` works, but `UNWIND range(0,255) AS i CREATE (:L0 {id:i})` is rejected by its parser (`expected query`). A full load needs a supported create/batch form or per-row statements; do not call this a Samyama failure until the documented API shape is tried.

## Not done

No back-to-back calibrated comparison and no B3 floor decision. The next step is a supported Samyama load path plus one same-sitting run of the phase-251 layered workload through both engines, then the B4/B5 decision. This note is not evidence of a competitor ratio.
