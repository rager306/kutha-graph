---
id: paper-graph-branch-fork
source: paper
axes: [Verify, Time, Space, Agent]
usefulness: high
optimality: med
demand: high
confidence: paper
layer5: no niche
status: closed
channels_failed: []
---

# Named branches are overlays on the log — not CRDT merge and not clock snapshots

Papers: [ForkBase](https://consensus.app/papers/details/6dbec732fe4259e3a81c4ab424aa222f/?utm_source=cursor) (Lin et al., 2020, ICDE) Git-for-data co-designed with the store: immutable, tamper-evident, branch/merge below file granularity, dedup across versions [8]; [GitLake](https://consensus.app/papers/details/7315b8d012f250839dedd57fee99e87c/?utm_source=cursor) (Sheng et al., 2026) lakehouse-wide commits; agents work on isolated branches; pipelines publish by merge so outputs appear atomically or not at all [5]; [Toward Systems Foundations for Agentic Exploration](https://consensus.app/papers/details/644ed22c71595cbc93313ffb55495f5d/?utm_source=cursor) (Xu et al., 2025) generic CRIU/container fork is too slow; open problems are fork *semantics* (what tentative writes leak), external side-effects, and microsecond native clone [10]; ChronoGraph [3][6] is **system-time versioning** of a TinkerPop graph (SQL:2011 cousin) — not this noun; DeltaGraph [17] / GDBAlive [15] / GraphOne [19] are historical-snapshot / ingest-vs-analytics cousins of `paper-lsm-snapshot-compaction`. Distinct from `paper-crdt-graph-eventual` (merge throws away losers), `paper-activegraph-log-is-sot` (cheap fork at an *event offset* is replay, not a named branch), `hindsight-four-network-tempr` (git-as-memory store).

## 1. Raw idea

Three “version” nouns. (1) **System-time / snapshot**: every write is a timestamp; recreate history from snapshot+log [3][9][15][17]. (2) **CRDT / eventual**: concurrent writes commute; losers vanish. (3) **Named branch**: a first-class overlay (like Git refs) that diverges, is reviewed, and merges — ForkBase’s Git-for-data [8]; GitLake’s agent branches with atomic publish [5]. Agent exploration needs fork of *execution state* plus hiding of tentative writes; container checkpoint is the wrong primitive [10]. BIM graph merge via graph transformation is the same merge problem in another domain [7]. GitOfThoughts [11] is an agent-*memory* git repo — skip (Hindsight cousin).

## 2. STCA applicability

Verify 061: fork-and-diff is a **named ref on the event log**, not a second SoT. Space: a branch is a Space overlay (pack of events with a ref); merge is a logged compensating/commit event, not CRDT. Time: system-time snapshots stay honeycomb 012; this card is *user-visible* branches. Agent: LLM experiments belong on a branch; publish is merge to `main` of the fold [5]. ActiveGraph’s “fork at any event” is replay from an offset — keep it; add refs so two agents can fork the *same* offset without sharing tentative writes [10].

## 3. Quality / cost

Usefulness high: honeycomb 061 is empty without a ref type. Optimality med: ForkBase is a storage substrate; native microsecond clone is still research [10]. Cost: P0 = single `main` + log-offset replay; honeycomb = `ref → event-id` + copy-on-write leases; do not vendor Git, Iceberg, or ChronoGraph as the Kutha log.

## 4. Demand

Pack authors and agents will experiment. Engine demand: branch id on compiled queries + merge as an event, so fork-diff is a first-class Verify operator.

## 5. Niche → effect

`no niche`
