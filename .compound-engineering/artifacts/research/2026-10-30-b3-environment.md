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

## Not done

No layered workload rendered or loaded; no timed comparison; no B3 floor decision. The next step is a single back-to-back run of the phase-251 layered workload (256 nodes/layer, fan-out 4) through both engines with calibrated client overhead, then the B4/B5 decision. The environment is ready; this note is not evidence of a competitor ratio.
