---
id: paper-k2tree-succinct-graph
source: paper
axes: [Data, Query, Packaging]
usefulness: high
optimality: high
demand: med
confidence: paper
layer5: no niche
status: closed
channels_failed: []
---

# Navigate the adjacency matrix in compressed form — k²-tree is a lease, not a second CSR SoT

Papers: [k²-trees](https://consensus.app/papers/details/7eb88d07593b53ed983efb7a9e38036a/?utm_source=cursor) (Brisaboa et al., 2009) 3.3–5.3 bits/link, successors *and* predecessors [1]; [compact k²-tree](https://consensus.app/papers/details/94df60bbf950543780474a0b9b58cc3d/?utm_source=cursor) (2013) 1–3 bits/link, range/link checks [2]; [WebGraph / Re-Pair](https://consensus.app/papers/details/b41fa00ab8d65f099ed03f2c9e05b33f/?utm_source=cursor) (Claude & Navarro, 2010) grammar compression, faster nav at same space [11]; [Log(Graph)](https://consensus.app/papers/details/b3ce20c90de55eb2b11198bbf915257c/?utm_source=cursor) (Besta et al., 2018) near lower-bound encodings, 20–35% smaller than tuned CSR with comparable speed [20]; [survey](https://consensus.app/papers/details/3764b4f48b5858968597432162a66a06/?utm_source=cursor) (Besta et al., 2018) taxonomy of lossless graph compression [19]; [Fan contraction](https://consensus.app/papers/details/8e1fa06de8f859719842d30f9e27601e/?utm_source=cursor) (2021/22) contract regular/obsolete regions to supernodes with per-query-class synopses; lossless decontract [14]; [dynamic k²-tree](https://consensus.app/papers/details/50b2c6b06ff5538fa10e0972216b3820/?utm_source=cursor) (Coimbra et al., 2021) updates without full rebuild [7]. Distinct from `samyama-csr-frozen-adjacency` (uncompressed CSR) and `paper-bach-lsm-csr-bridge` (LSM layout transform, not succinct bits).

## 1. Raw idea

CSR is already a *picture* of the log; it is still ~dozens of bits per edge. k²-tree recursively subdivides empty regions of the adjacency matrix so you can list neighbors **without decompressing the whole graph** [1][2]. WebGraph/Re-Pair and Log(Graph) are the web/social pole. Fan’s contraction is the *logical* cousin: shrink the graph, keep synopses, expand only when the query needs the atoms [14]. Depth-first k² layouts help matrix-multiply locality [4][15].

## 2. STCA applicability

Data/Query: a succinct structure is another **reversible materialization** (ADR-040/041), beside CSR/HNSW. Cold historical hops and legal archives want bits/link; hot WCOJ still wants CSR/tries. Packaging: RVF/export can carry a k² blob. Do not make k² the write SoT — dynamic variants exist but the log remains append-only. Partitioning (already closed) distributes *uncompressed* shards; succinct is how a *single* lease shrinks.

## 3. Quality / cost

Usefulness high for huge sparse graphs in RAM. Optimality high on space; med on mutating property graphs (labels/props sit beside the bits). Cost: optional cold-path codec; keep CSR for the hot leapfrog.

## 4. Demand

Web/social scale and “fit the graph in RAM” are why these papers exist. Engine demand: a compact adjacency plugin, not P0.

## 5. Niche → effect

`no niche`
