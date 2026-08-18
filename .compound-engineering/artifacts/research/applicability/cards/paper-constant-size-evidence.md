---
id: paper-constant-size-evidence
source: paper
axes: [Verify, Time, Packaging]
usefulness: high
optimality: med
demand: high
confidence: paper
layer5: "Clinical/pharma/finance AI workflows: one fixed-size receipt per log event; Merkle/hash-chain anchor; PQ-migrate signatures without changing the tuple layout"
status: closed
channels_failed: []
---

# Constant-size evidence items as quantum-receipt cousins

Papers: [Constant-Size Cryptographic Evidence Structures for Regulated AI Workflows](https://consensus.app/papers/details/c7650cf7f6dd5f0dbfa15036584aa3b1/?utm_source=cursor) (Kao, 2025, arXiv:2511.17118, DOI: 10.48550/arxiv.2511.17118); companion [Quantum-Adversary-Resilient Evidence Structures](https://arxiv.org/abs/2512.00110) (arXiv:2512.00110). Log ADS: [Efficient Data Structures For Tamper-Evident Logging](https://consensus.app/papers/details/ec9fa840998950a18864a6c11bdb5147/?utm_source=cursor) (Crosby and Wallach, 2009). Graph ADS (do not make blockchain SoT): [Authenticated data structures for graph connectivity](https://consensus.app/papers/details/cec147ed0bfe5fedb0ccf7aa8b582232/?utm_source=cursor) (Goodrich et al., 2009); PAGB (Wu et al., 2023). Distinct from `paper-proof-carrying-llm-envelope` (certificates *around* the model) and PACT (tool-argument provenance).

## 1. Raw idea

Each regulated workflow event emits a **fixed-size** cryptographic tuple (hashes of work unit, I/O commitments, environment/attestation digest, link into a chain/Merkle root, authenticator). Size is independent of payload — no metadata leak from record length, uniform verify cost. Hash-and-sign construction; games for **binding**, **tamper detection**, **non-equivocation** under CRHF + EUF-CMA. Composes with hash-chained logs, Merkle anchoring, optional TEE. Prototype: predictable per-event overhead. Crosby/Wallach: tree-based inclusion/consistency proofs, logarithmic vs linear hash-chain traces (3 KB vs 800 MB on 80M events). PQ companion: Q-Audit Integrity / Q-Non-Equivocation / Q-Binding in QROM so classical signatures can be migrated without changing the evidence *layout*.

## 2. STCA applicability

Verify: this is the literature shape of Kutha **quantum receipts** — one evidence item per log event, not a syslog sidecar. Time: `Link` is the chain/Merkle over the append-only log. Packaging: constant-size tuples are portable (RVF witness cousin), not a second store. Do **not** put the event log on a public chain; optional epoch root-of-roots is an *anchor*, Rocks WAL remains SoT. Distinct from LLM-envelope proofs: those certify a pipeline; these certify **that the engine event existed**.

## 3. Quality / cost

Usefulness high: ADR-000 already names receipts; this gives syntax + games + PQ migration. Optimality med: hash-and-sign is standard, not a new join algorithm; blockchain ADS papers (vChain, vProChain, PAGB) are untrusted-responder designs — borrow proofs, not the ledger as GDBMS. Cost: emit a constant-size receipt on every log append; Crosby tree for auditors; keep signatures swappable (PQ). Retrofit warning in the paper: translating variable logs into events is a trust boundary — Kutha avoids it by making the event the native record.

## 4. Demand

Clinical trial / FDA electronic records / EU AI Act / HIPAA audits need inspectable trails years later without leaking PHI via record size. Engine demand: receipt type on the log, uniform verify, non-equivocation of forks.

## 5. Niche → effect

Clinical/pharma/finance AI workflows: one fixed-size receipt per log event; Merkle/hash-chain anchor; PQ-migrate signatures without changing the tuple layout.
