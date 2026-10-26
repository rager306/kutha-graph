# Remote baseline sample, 2026-10-26

One sitting on `aleksei@43.152.237.209` (`bkk-edge01`): 2 cores, 1.9 GB RAM, Rust 1.99.1. Kutha checkout `c21a9b3`. FalkorDB image `falkordb/falkordb:latest`. Samyama image `ghcr.io/samyama-ai/samyama-graph:1.10.0` with embeddings off. Containers were stopped after the run. This is not the B3 kill-rule budget. The workloads are not the same operation, and the sample is one pass (Falkor and Samyama medians are three query repeats).

## Kutha (`cargo bench`, release profile)

`KUTHA_BENCH_N=10000`: `ingest_plain.n` 38.5 ms, `as_of.n` 183 µs.

`KUTHA_HOP_N=256`: `hop2.walk.n` 92.3 ms (in-process walk of every layer-0 start after one CSR build), `csr_lease.n` 113 µs. Walk scaled linearly (2n/n 2.05, 4n/2n 1.97).

## Competitors (Redis `GRAPH.QUERY` on localhost)

10 000 edges, one `UNWIND range(0,9999) CREATE (:A {id:i})-[:R]->(:B {id:i})`, median of 3:

| Engine | Time |
|--------|------|
| FalkorDB | 26.3 ms |
| Samyama | 205 ms |

Layered fan-out 4, two edge layers, one Cypher count of all 2-hop paths:

| Engine | Graph | Paths | Median |
|--------|-------|------:|--------|
| FalkorDB | n=256 | 4096 | 4.8 ms |
| FalkorDB | n=64 | 1024 | 6.7 ms |
| Samyama | n=64 | 1024 | 4.9 ms |

Samyama's `MATCH`+`CREATE` of the n=256 edge layers was OOM-killed at about 1.4 GB (`exit 137`). The n=64 edges were inserted one statement at a time (2.6 s for 512 edges); that time is Redis round trips. FalkorDB built the n=256 edge layers as two queries, about 84 ms each.

Kutha's 38.5 ms ingest is 10 000 admitted asserts into the fold. The competitors' 10 000-edge number is one Cypher `CREATE`. Kutha's 92 ms hop time is 256 in-process walks. The competitors' hop time is one Cypher count. Do not treat either pair as a regression gate.
