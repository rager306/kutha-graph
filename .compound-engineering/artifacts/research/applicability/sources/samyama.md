# Samyama dossier (Wave 1)

CBM: `root-samyama-graph` path `/root/samyama-graph`.

## Extracted

| card | confidence | note |
|------|------------|------|
| samyama-csr-frozen-adjacency | code | store.rs CSR tiers |
| samyama-leapfrog-triejoin | code | leapfrog.rs |
| samyama-mvcc-version-chains | code | Node.version + mvcc tests |
| samyama-late-materialization | code | bench + stub/column load |
| samyama-agentic-enrichment-gak | observed | example + CLI dependency |
| samyama-raft-ha | code | openraft apply persists mutations; ExecuteQuery rows:0 |

## Not extracted

OpenCypher parser (~90%) is implied by leapfrog/executor; not a separate card (would duplicate Query axis).
