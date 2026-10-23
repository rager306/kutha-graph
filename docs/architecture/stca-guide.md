# Guide: Spatiotemporal Compositional Architecture (STCA) in Rust

**Status:** working manifesto-specification for Kutha / kutha-graph  
**Relation:** normative support **ADR-002 (STCA)**; product vision — **ADR-001**; D1–D10 foundation — **ADR-000**; detailing — honeycomb ADR-010+  
**Date:** 2026-08-16

Every concept is grounded in the source materials. Author citations are in the text; passage numbers from the original synthesis are marked `[i]` where a trace back to the Studio source is needed.

---

## 1. Synthetic architectural paradigm (STCA)

The **Spatiotemporal Compositional Architecture (STCA)** paradigm unites:

1. **Spatial contour** — static domain separation (vertical slices, ports).
2. **Temporal contour** — event-generated graph (append-only event log → deterministic projection).
3. **Compositional bridge** — mathematics of generalized items and max-convolution (Cui).

```
  [ SPACE (Space) ]                     [ TIME (Temporal) ]
  Vertical domain slices                 Append-only Event Log
  Cockburn / Palermo / Martin            ActiveGraph (Nakajima)
  + Vertical Slices (Drotbohm)           arXiv:2605.21997
           │                                      │
           ▼                                      ▼
  Free domain + Ports                   Deterministic fold
  (adapters outside)                    (graph = log projection)
           │                                      │
           └──────────────────┬───────────────────┘
                              ▼
                   [ STCA CORE ]
              Spatiotemporal Context
                              ▲
                              │
                   [ Mapping and convolution ]
                 Cui «背包问题九讲» (max-convolution)
```

**Kutha formula:** STCA core + pluggable hot materializations (CSR/HNSW/…) + dict-first agents + native bi-temporal semantics (Graphiti reference, without Graphiti runtime).

### 1.1. Spatial isolation (Space)

- **Classics:** hexagonal architecture (Alistair Cockburn, 2005), onion (Jeffrey Palermo, 2008), clean architecture (Robert C. Martin, 2012) — dependencies point toward the pure domain.
- **Evolution 2025–2026:** a technical cut into `ports` / `adapters` / `domain` folders yields low cohesion when requirements change. Per Oliver Drotbohm (Spring I/O / CodeCrafts 2026) — **functional decomposition into vertical slices (Vertical Slices / Sliced Onion)**. The hexagon lives *inside* the domain module; modules communicate only through **Ports**.
- **Metaframework:** spatiotemporal composability (Cordis / DeepSeek Harness) — components as autonomous plugins (Pack), interaction through context; effects are reversible.

### 1.2. Temporal isolation (Time)

- State is **not** overwritten in place (not State-on-Write).
- Per ActiveGraph (Yohei Nakajima, 2026, “The Log is the Agent”, arXiv:2605.21997): **the sole SoT is an ordered append-only Event Log**.
- The graph \(G_t = \mathrm{foldl}(\mathrm{apply}, G_0, L_t)\) is a deterministic projection.
- **Behaviors** and **Relation Behaviors** react to topology/events, propose **Patches**; the runtime validates and writes to the log.
- Principle: “The graph is the world. Behaviors are the physics. The log is the proof” (Nakajima, 2026).

### 1.3. Algorithmic bridge (Cui)

- Module \(M_i\) ≈ **generalized item**: local \(h_i(v)\) at resource budget \(v\).
- Global state is the **max-convolution** of local functions (Tianyi Cui, “Nine Lectures on the Knapsack Problem”, lecture 8).
- In Kutha: allocation of cascade budgets, agent limits, competition among materialization plugins — without a hard workflow-engine.

---

## 2. Mathematical and algorithmic specification

### Algorithm 1. Deterministic fold (graph projection)

**Input:** \(G_0 = \emptyset\), \(L_t = [e_1,\ldots,e_t]\).  
**Output:** \(G_t\).

1. \(G \leftarrow G_0\).
2. For each \(e_k \in L_t\): apply `object.created` / `relation.created` / `object.patched` (JSON merge-patch), etc.
3. Return \(G\).

### Algorithm 2. Relation Behaviors

**Input:** event \(e\), graph \(G\), registry of edge behaviors.  
**Output:** list of produced events.

For each behavior with a matching trigger and edge type: if the event is incident to the edge endpoints — `execute(rel, e, G)`.

### Algorithm 3. Cui Max-Convolution

**Input:** budget \(V\), plugins \(M_1..M_n\) with \(h_i(v)\).  
**Output:** vector \(H\), where \(H[V]\) is the optimal composition.

\[
H_{\mathrm{next}}[W] = \max_{0 \le v \le W}\{ h_i(v) + H[W-v] \}
\]

---

## 3. Structure in Rust (Cargo Workspace)

```
[workspace]
 ├── app-shell/          # Composition Root (the only binary)
 ├── common/             # Event, Patch, UuidV7, …
 └── modules/           # Vertical slices (packs)
      └── <domain>-module/
           ├── domain/      # pure domain (no sqlx/ORM)
           ├── internal/    # pub(crate)
           ├── adapters/    # outgoing ports
           └── lib.rs       # public Ports (traits)
```

### Isolation laws (AI-Compiler Gates)

1. **Domain Ignorance** — `domain/` without infrastructure.
2. **Private-by-Default** — only Ports go outward.
3. **Single Composition Root** — only `app-shell` knows concrete adapters.

For the Kutha-engine: `common` + runtime crate + materializer packs + dictionary/agent packs; hot path without mandatory network adapters (see ideation idea-21).

---

## 4. Testing strategy

### 4.1. Strict Replay

Replay of the historical log; external I/O — content-addressed cache (`SHA256(prompt…)`); divergence → `ReplayDivergenceError`.

### 4.2. Fork-and-Diff

`fork_at(n)` takes a `usize` offset into the **current** log. A compacted checkpoint prefix is atomic: cuts inside it (or past `log.len()`) fail closed (`ForkCutUnavailable`). Then change a behavior → stabilization → structural graph diff. `diff_folds` is a `facts()` query including losers, not named refs, GED, or merge; dictionaries, rules, and blobs are out of domain.

### 4.3. Regimes Gated Loop (Nakajima, arXiv:2606.10241)

Diagnose → Route onto the Action Seam → Static / Sandbox / In-Sample / Held-Out gates; held-out failure → discard (anti-overfit).

---

## 5. Reference scaffold (summary)

The full typed ActiveRuntime scaffold (Event, Patch, MaterializedGraph::apply_event, BehaviorFn, RelationBehaviorFn, emit-cycle until stabilization) is in the original Studio artifact and in the prototype sketches in ADR-000 §Prototype. Identifiers: **UUID v7**.

---

## 6. Protocol for a coding agent

1. Change only at the seams: Behavior / Relation Behavior / Port.
2. Strict event payload schema in `common/`.
3. Domain — pure unit tests without the network.
4. For adapters — replay fixtures with a CA-cache.

---

## 7. Relation to Kutha product refinements (2026-08-16)

| Refinement | Place in STCA |
|-----------|----------------|
| Self-contained Rust engine, without mandatory external APIs | Space: adapters optional; Time: SoT is the local log (RocksDB) |
| Bi-temporal semantics “like Graphiti”, natively | Time: fact fields + invalidation behaviors; not Graphiti runtime |
| Quantum Receipt / cascade budgets | Time + Cui: emit quantum and convolution budgets |
| Dict-first agents | Space pack + control plane on top of STCA |
| Hot CSR/HNSW | Data-plane materializations *on top of* the fold (D5 ADR-000) |

---

*The document is stripped of filler for architect reading and LLM-agent parsing. The full reference Rust listing of ActiveRuntime — restore if needed from Studio `stca-guide` / ADR-000 prototype appendix.*
