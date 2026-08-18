---
id: paper-graph-ssi-isolation
source: paper
axes: [Time, Verify, Agent]
usefulness: high
optimality: high
demand: high
confidence: paper
layer5: no niche
status: closed
channels_failed: []
---

# Snapshot isolation on the fold still write-skews — SSI is the protocol, not MVCC storage

Papers: [Making SI serializable](https://consensus.app/papers/details/909ad817dbbd5f86a86b56691e485e96/?utm_source=cursor) (Fekete et al., 2005, TODS) SI ≠ serializability; write-skew / dangerous structures [3]; [SSI](https://consensus.app/papers/details/992c6cb6360555b98667510cf98f61fa/?utm_source=cursor) (Cahill, 2009, TODS) detect anomalies at runtime; readers do not block writers [2]; [PostgreSQL SSI](https://consensus.app/papers/details/ddcc144157f2502f870bc20ec31f0fd0/?utm_source=cursor) (Ports & Grittner, 2012, VLDB) first production SSI [4]; PSSI cycle-breaking vs conservative dangerous-structure aborts [8]; read-only SI anomaly even when updates are serializable [15]; PolySI black-box SI checker [5]. Distinct from `samyama-mvcc-version-chains` (how versions are *stored*), `paper-cordon-semantic-tx` (stage *external* tool effects), `paper-graph-tenant-isolation` (noisy-neighbor quotas), `paper-mas-isolation-lattice` (agent memory isolation, not DB isolation levels).

## 1. Raw idea

Each transaction reads a **snapshot** of committed state; writes conflict at commit (first-committer-wins). That kills dirty/non-repeatable reads and still allows **write skew**: two transactions each preserve a constraint, together they break it [2][3]. SSI tracks rw-dependencies and aborts *dangerous structures* [2][4]. On a graph the phantom is often a **path**: MATCH `()-[:R*]-()` sees an edge another concurrent append just committed. The log being append-only does not make the *fold* serializable.

## 2. STCA applicability

Time 010: the event log is the SoT; isolation is a **property of concurrent folds / agent appends**, not of the WAL format. Verify: SSI abort is a receipt (“dangerous rw-rw”). Agent: two packs appending complementary edges are the write-skew pair. Do not confuse MVCC chains (Samyama) with the isolation *level*. Cordon commits/rolls back *side effects outside* the log; this card serializes *log appends relative to reads of the fold*.

## 3. Quality / cost

Usefulness high: agents will race on AS-OF + MERGE. Optimality high: SSI is the production answer (Postgres). Cost: P0 = single-writer log (serial append); honeycomb = SSI-style rw-dependency tracking on concurrent readers of a snapshot fold; skip lock-based 2PL as default.

## 4. Demand

Without SSI, two legal AS-OF writers can violate a uniqueness/disjointness constraint that each thought they checked. Engine demand: declare isolation of a session (SI vs SSI), fail-closed abort.

## 5. Niche → effect

`no niche`
