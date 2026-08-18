---
id: paper-graph-tenant-isolation
source: paper
axes: [Security, Space, Composition]
usefulness: high
optimality: med
demand: high
confidence: paper
layer5: "Legal/enterprise: counsel–client graphs as tenant slices; noisy-neighbor isolation is not path-ABAC"
status: closed
channels_failed: []
---

# Tenant isolation is a slice of the fold — not path-ABAC and not agent-runtime lattice

Papers: [Ddi](https://consensus.app/papers/details/2abaddf5f4db5731be063253a5d3c502/?utm_source=cursor) (Fu et al., 2025, VLDB) per-*operation* isolation on graph traversals; extracts concurrency that database-wide serializability cannot, while keeping structural consistency [13]; [SQLVM](https://consensus.app/papers/details/a56434cc43435b558f66f584fe83c968/?utm_source=cursor) (Narasayya et al., 2013) CPU/I/O/memory *reservations* without static partition [4]; [DRFT](https://consensus.app/papers/details/bc4d5667c433592fa9ed0d1ee46cf8c7/?utm_source=cursor) (Cheng et al., 2025) fair transaction scheduling, share guarantee + strategy-proof [3]; [ABase](https://consensus.app/papers/details/192540b6c4855ca098d7d7252c03b612/?utm_source=cursor) cache-aware isolation (hits change resource accounting) [5]; [Silo/Pool/Bridge RAG](https://consensus.app/papers/details/b44cb980b02956d9a3c0a2768da9926e/?utm_source=cursor) four planes including vector plane; embedding leakage [1]. Distinct from `paper-xacml4g-path-abac` (who may *see* a path), `paper-temporal-grants-as-facts` (valid-time of a grant), `paper-mas-isolation-lattice` (LLM tool-effect anomalies), `paper-graph-partition-vertex-cut` (min-cut layout).

## 1. Raw idea

Three different “isolation” nouns: (1) **correctness** — Ddi assigns isolation *per traversal op* so long RW graph tx can overlap without breaking graph invariants [13]; (2) **performance** — noisy-neighbor CPU/I/O quotas (SQLVM/DRFT) [3][4]; (3) **tenancy of data** — named-graph / bank / silo vs pool vs bridge, including the *vector index* as a leak surface [1]. Cache hits make traffic control lie [5]. Generic cloud-graph papers that only say “model tenants as vertices” are not this card.

## 2. STCA applicability

Security 080: a tenant is a **Space slice of the fold** (pack + dictionary + grants), not a second log. Path-ABAC answers “may this hop return?”; tenant isolation answers “is this vertex even in the slice, and does this tenant starve another?” Composition: Ddi-style per-op levels belong on compiled Cypher, not as a global SERIALIZABLE on the event log. Vector-plane silo/pool is Query 071 tenancy, cousin of Hindsight `bank_id` tags — still not SoT.

## 3. Quality / cost

Usefulness high: STRATEGY already names multi-tenancy as a buyer constraint. Optimality med: Ddi is new; SQLVM is RDBMS. Cost: P0 = single tenant; honeycomb 080 = named-graph/bank descriptor + resource quotas on leases; do not silo a second Postgres per counsel.

## 4. Demand

Counsel/client and SaaS RAG will share hardware. Engine demand: slice id on every compiled query + quota on compact/recall.

## 5. Niche → effect

Legal/enterprise: counsel–client graphs as tenant slices; noisy-neighbor isolation is not path-ABAC.
