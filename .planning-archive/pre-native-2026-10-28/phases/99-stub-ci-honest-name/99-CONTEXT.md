# Phase 99 CONTEXT — stub CI quantum is not live cargo

**Source:** CE full-branch review P1 #7 (G3 governor silent-pass).  
**Lease:** M039 S01 / GSD v0.33.

## Goal

The pytest full-quantum test stubs `cargo` / `kutha-tenant` and still claims the tree is green. Any Rust regression stays green in that test. Rename it to stub-scoped FSM mechanics. Live `observe_cargo` remains `uv run kutha-gov ci` (product-close-gate) plus an opt-in `KUTHA_LIVE_CARGO=1` pytest. Governor YAML forbids the lying name.

## Oracles

- `test_stub_ci_is_not_named_green_on_this_tree`
- `test_ci_fsm_reaches_ok_with_stubbed_observe`
- `test_ci_observe_cargo_live` (skip unless `KUTHA_LIVE_CARGO=1`)
