---
id: paper-blockchain-graph-ads
source: paper
axes: [Verify, Query, Packaging]
usefulness: high
optimality: med
demand: med
confidence: paper
layer5: no niche
status: closed
channels_failed: []
---

# Blockchain-assisted graph ADS: proofs for an untrusted responder; ledger is not SoT

Papers: [PAGB](https://consensus.app/papers/details/a6b00d73fe9d59a091b3fcaf6a1d14ac/?utm_source=cursor) (Wu et al., 2023, TKDE, DOI: 10.1109/tkde.2023.3249279) RSA accumulator + completeness set for authenticated **graph** queries on blockchain-assisted clouds [1]; [vChain](https://consensus.app/papers/details/1c3931305dd1535ab08ca15fb8ff5e2f/?utm_source=cursor) (Xu et al., 2019, SIGMOD, DOI: 10.1145/3299869.3300083) verifiable Boolean range over blockchain DBs [3]; [NetChain](https://consensus.app/papers/details/b13057a05426591fa66f821ac7de8b27/?utm_source=cursor) (Zhao et al., 2025, arXiv:2501.15077) authenticated top-k on on-chain graphs [2]; [VGQ](https://consensus.app/papers/details/023fb668ee5056409d69280cb23576bc/?utm_source=cursor) (Yao et al., 2025, ICDE) verifiable graph queries without changing chain storage [5]; [VeriDKG](https://consensus.app/papers/details/e2e2f6c6401156b1810a8a4a71cb72fb/?utm_source=cursor) (Zhou et al., 2023) SPARQL over decentralized KGs with RGB-Trie ADS [10]; subgraph-match ADS (MELTree) [11]. Distinct from `paper-constant-size-evidence` (per-event receipts *on our log*) and Crosby/Wallach (tamper-evident *local* logs). Goodrich graph ADS already cited there as untrusted-responder cousin.

## 1. Raw idea

Hybrid storage: fat data off-chain / in a cloud or full node; a small **ADS digest** (accumulator, Merkle, Sorted Merkle) on-chain so a light client can check completeness and soundness of a query without the whole ledger. PAGB targets **graph** queries with privacy (do not leak the graph via the chain) [1]. vChain started the blockchain-DB verifiable-query line for Boolean/range [3]. NetChain/VGQ extend to top-k and general graph patterns [2][5]. VeriDKG authenticates SPARQL on a DKG [10]. The chain stores **digests**, not the working graph.

## 2. STCA applicability

Verify: this is the **untrusted-responder** pack — a service may return a hop/path; the client checks a VO against a digest. Time: freshness schemes (veffChain) are extra; Kutha freshness is the log’s AS-OF, not “latest K on a chain.” Packaging: optional **epoch root-of-roots** as an external anchor (same as constant-size evidence). Do **not** make Ethereum/Hyperledger the event log or the CSR. Distinct from quantum receipts: receipts certify that *our* engine event existed; ADS proofs certify that *their* answer matches a published digest.

## 3. Quality / cost

Usefulness high when someone else hosts the graph (auditor, partner cloud). Optimality med: RSA accumulators and on-chain ADS updates are gas/CPU heavy; VGQ’s “don’t change chain storage” is the right instinct. Cost: emit Crosby/Kao receipts locally; if an auditor needs a public pin, publish the Merkle root — not the tuples. Skip PAGB/vChain as a GDBMS.

## 4. Demand

Web3/KG-sharing RFPs will ask to “put the graph on chain.” Answer: digest on chain, log in Rocks, proofs optional. Engine demand: VO-shaped verify for exported views.

## 5. Niche → effect

`no niche`
