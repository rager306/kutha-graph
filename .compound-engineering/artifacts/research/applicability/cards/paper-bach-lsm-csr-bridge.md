---
id: paper-bach-lsm-csr-bridge
source: paper
axes: [Data, Time, Query]
usefulness: high
optimality: high
demand: high
confidence: paper
layer5: no niche
status: closed
channels_failed: []
---

# BACH: LSM-tree that ages adjacency lists into CSR for HGTAP

Paper: [BACH: Bridging Adjacency List and CSR Format using LSM-Trees for HGTAP Workloads](https://doi.org/10.14778/3718057.3718076) (Huang et al., 2025). Consensus: https://consensus.app/papers/details/96ea5ef380255d9981a0c959f4f3b362/?utm_source=cursor

## 1. Raw idea

TP-friendly adjacency lists at the top of an LSM; AP-friendly CSR after compaction into deeper levels. Elastic merge for skew degree and mixed workloads. Lightweight MVCC snapshots for concurrent read/write.

## 2. STCA applicability

Data/Time: this *is* the Kutha compaction story — log/mutable adj → frozen CSR lease — without making CSR the SoT. Query: HGTAP so OLTP writes and analytics hops share one aging pipeline. Complements Rocks WAL + Samyama frozen CSR.

## 3. Quality / cost

Highest storage-engine fit this wave. Cost: disk LSM graph, not in-process-only P0; elastic merge policy is research.

## 4. Demand

Without a bridge, Kutha either stays on a slow adj list or rebuilds CSR from scratch.

## 5. Niche → effect

`no niche`
