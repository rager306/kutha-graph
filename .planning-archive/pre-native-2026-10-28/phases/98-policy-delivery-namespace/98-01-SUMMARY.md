# Phase 98-01 SUMMARY

`by_delivery_key` is `(relation, delivery_key)`. Policy re-pin after admission is idempotent; admission keeps the policy pin on `delivery_key`. Honeycomb Proposed. Oracles: `m038_policy_delivery_namespace.rs`.
