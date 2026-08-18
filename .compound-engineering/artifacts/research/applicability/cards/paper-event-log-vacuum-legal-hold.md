---
id: paper-event-log-vacuum-legal-hold
source: paper
axes: [Time, Verify, Packaging, Security]
usefulness: high
optimality: med
demand: high
confidence: paper
layer5: Legal/finance/clinical: a retention schedule and a litigation hold are first-class policies on the event log — vacuum is irrevocable physical removal, not LSM compaction of snapshots
status: closed
channels_failed: []
---

# Vacuum is policy GC of the SoT — not snapshot placement and not crash recovery

Papers: [Skyt et al.](https://consensus.app/papers/details/b7a5c5f7ac615549b23540b1f13751d5/?utm_source=cursor) (2002) semantic foundation for vacuuming *transaction-time* append-only databases: laws and business policy require physical removal; queries/updates against a vacuumed past must be defined, not silent holes [1][3]; [Hasan et al.](https://consensus.app/papers/details/0c0f0a1f662b500da6af5d1587400554/?utm_source=cursor) (2010) litigation hold: tuples under hold cannot be vacuumed; an illegal delete must not masquerade as vacuum; auditable hold implementation [2]; schema vacuuming alongside data vacuuming when historical schemata still interpret remaining facts [4]. Distinct from `paper-lsm-snapshot-compaction` (where a snapshot *lives*, not whether an event may be erased), `rocksdb-wal-recovery` (crash replay of DRAM), `paper-semi-external-graph` (hot vertices vs SSD edges), `samyama-mvcc-version-chains` (version GC of a fold, not the log). TVA versioned graph storage [14] stays a cousin of LSM. Agent-memory “what to remember” stays queued.

## 1. Raw idea

The log cannot grow forever. **Vacuum** physically deletes expired history under a declared policy [1]. A **legal hold** pins rows so vacuum cannot touch them [2]. After vacuum, AS-OF of a removed state is a defined failure, not a lie.

## 2. STCA applicability

Time: retention is a clock on *transaction-time* events (and optionally valid-time), not a third axis. Verify: vacuum is an audited event on the remaining log (“we dropped offset range X under policy P”); a hold is a grant-shaped fact. Packaging: cold archive vs delete is a pack. Security: hold vs ordinary retention is path-ABAC’s cousin only in *who* may vacuum. LLM does not choose what to forget.

## 3. Quality / cost

Usefulness high: GDPR/erasure, medical/finance retention, e-discovery. Optimality med: semantics exist; implementations are indexes + policy [8]. Cost: P0 = append-only forever; honeycomb = vacuum+hold pack with fail-closed “query affected by vacuum” [3]. Do not vacuum the live fold in place and call it compaction.

## 4. Demand

Counsel will ask both “keep this matter” and “delete this subject.” Engine demand: policy vacuum with holds, fail-closed to retain.

## 5. Niche → effect

Legal/finance/clinical: a retention schedule and a litigation hold are first-class policies on the event log — vacuum is irrevocable physical removal, not LSM compaction of snapshots
