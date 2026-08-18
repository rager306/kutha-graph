---
id: paper-temporal-grants-as-facts
source: paper
axes: [Security, Time, Verify]
usefulness: high
optimality: med
demand: high
confidence: paper
layer5: "Legal/HIPAA and clinical: permissions have valid-time like norms; AS-OF query must apply the grant that was in force, not today’s role table"
status: closed
channels_failed: []
---

# Grants are bi-temporal facts (valid-time of permission)

Papers: [A Temporal Access Control Mechanism for Database Systems](https://consensus.app/papers/details/5c2a963f45e0555caf30c28a0a46c5cf/?utm_source=cursor) (Bertino, Bettini, Ferrari, Samarati, 1996, IEEE TKDE, DOI: 10.1109/69.485637); [TRBAC](https://consensus.app/papers/details/9e02682b80245775a6a68071e6eca2ca/?utm_source=cursor) (Bertino, Bonatti, Ferrari, 2001, TISSEC, DOI: 10.1145/501978.501979); graph-history [HO(T)-ReBAC](https://consensus.app/papers/details/fdaa10d51a9b5130b6799ff025f9c85e/?utm_source=cursor) (Arora et al., 2022, SACMAT, DOI: 10.1145/3532105.3535026). Demand survey: [Fine-Grained Temporal Access Control in SQL for HIPAA](https://consensus.app/papers/details/54ff2657409654709bbba38a6c81d382/?utm_source=cursor) (Balogun et al., 2025). Distinct from `paper-xacml4g-path-abac` (path shape, not time of the grant).

## 1. Raw idea

Authorizations carry **temporal intervals of validity**; they auto-revoke when the interval expires. New grants can be derived from presence/absence of others in specific periods (positive and negative). Bertino et al. materialize the valid set for efficient checks. TRBAC adds periodic **role enabling/disabling** and role triggers with priority to resolve conflicts. HO(T)-ReBAC decides access from the **history of relationship changes** on a graph (Medical IoT), not only the current ReBAC snapshot. HIPAA-oriented FGTAC surveys time-interval constraints, query rewrite, retroactive audit, and forward-looking permissions on SQL stores.

## 2. STCA applicability

Security+Time: a grant is an event on the same log as facts — `valid_from`/`valid_to` (and transaction-time) like TOKI/Engram edges. Query rewrite (XACML4G) must be **AS-OF the grant**, or a later role change rewrites history. Verify: derivation/conflict ordering is a receipt for “why this access was allowed then.” Agent: dict-first tools inherit the grant fold; LLM is not the PEP. HO(T)-ReBAC’s history-of-relationships is a graph view over grant/relationship events, not a second SoT.

## 3. Quality / cost

Usefulness high: XACML4G card already flagged missing temporal ABAC; this closes it. Optimality med: 1990s materialization + TRBAC-on-DBMS; HO(T)-ReBAC is a prototype matcher. Cost: treat `Grant`/`Revoke`/`RoleEnable` as log event types and compile them into the same pattern rewrite as path-ABAC. Do not train LSTM/GAN on access logs as the decision procedure (Mohammadi 2025 is a demand signal, not an engine).

## 4. Demand

Counsel, HIPAA, and emergency-role exceptions are time-bounded. Without dated grants, replay/fork of a legal pack is unsound: today’s ACL leaks yesterday’s file. Engine demand: PIT on **both** graph facts and permissions.

## 5. Niche → effect

Legal/HIPAA and clinical: permissions have valid-time like norms; AS-OF query must apply the grant that was in force, not today’s role table.
