---
id: oxify-zanzibar-rebac
source: oxify
axes: [Security, Query, Time]
usefulness: high
optimality: med
demand: high
confidence: code
layer5: "Legal/enterprise: Zanzibar tuples as a grant-graph overlay (owner⊃editor⊃viewer); not Cypher path-ABAC rewrite and not clock-on-the-role-table"
status: closed
channels_failed: []
---

# ReBAC is tuples + reachability — not path-ABAC and not a second log

User URL: https://github.com/cool-japan/oxify `oxify-authz` (cloned `/tmp/cool-japan/oxify`, not CBM-indexed). Ported from OxiRS. Read: `crates/oxify-authz/src/lib.rs` — `RelationTuple { namespace, object_id, relation, subject, condition }`; Check/Expand APIs; `src/types.rs` — `RelationshipCondition::TimeWindow { not_before, not_after }`, namespace `inherits_from` (owner→editor→viewer); `src/leopard.rs` — materialize transitive closure, max_depth 10, O(1) check / O(n) write; `src/engine.rs` — SQLite `AuthzEngine` + moka cache + bloom negatives. `src/quantum.rs` is **claim**: Kyber keypair is a commented placeholder. Distinct from `paper-xacml4g-path-abac` (query rewrite on hop shape), `paper-temporal-grants-as-facts` (grant as bi-temporal *fact* on the event log; HO(T)-ReBAC is history-of-relationships), `paper-graph-tenant-isolation` (slice/quota), `paper-mas-isolation-lattice`. Closes the queued AReBAC/Nano-Cypher skip-unless with a *code* surface.

## 1. Raw idea

Authorization as **relation tuples** on a namespace (Zanzibar): `(document, 123, owner, user:alice)`. Check walks/indexes the relation graph; Expand lists subjects. Leopard precomputes inherited relations so `owner` implies `viewer` without DFS at request time. Optional `TimeWindow` on the tuple is *request-now* vs interval — not AS-OF valid-time on the Kutha log. Storage in this crate is SQLite, not the event log.

## 2. STCA applicability

Security 080: ReBAC is a **grant overlay**. Tuples should be events (`Grant`/`Revoke`) folded into a reachability lease (Leopard-shaped), not a SQLite side database. Query: Check is *not* Cypher rewrite; path-ABAC answers “may this hop return?”; ReBAC answers “does this subject-relation-object hold (possibly via inheritance)?” Both can compile into the same fail-closed plan. Time: `TimeWindow` is a cousin of temporal grants — promote it to valid-time on the log or it rewrites history. Verify: `AuthzDecision.depth` is a small why; not a quantum receipt. Ignore `quantum.rs` PQC until it is real crypto.

## 3. Quality / cost

Usefulness high: STRATEGY already wants graph-shaped AC; Zanzibar is the industry noun buyers know. Optimality med: in-memory HashMap index + SQLite; Leopard is a real algorithm, not a README. Cost: P0 = path-ABAC rewrite; honeycomb = tuple events + inherited-relation lease; do not vendor `oxify-authz`’s SQLite/Postgres story.

## 4. Demand

Counsel/client and SaaS docs share objects with inherited relations (owner/editor/viewer). Engine demand: tuple type on the log + Check compiled into the query, not app-layer `IF role`.

## 5. Niche → effect

Legal/enterprise: Zanzibar tuples as a grant-graph overlay (owner⊃editor⊃viewer); not Cypher path-ABAC rewrite and not clock-on-the-role-table.
