# Phase 73: Harness emit isolate — Context

**Milestone:** v0.16 / harness M022 S03  
**Requirements:** SKIP-EMIT-01, OVERLAP-01  
**Depends on:** Phase 72  
**Mode:** Controller + owner (GSD auto)

## Locked

- **D-SKIP-EMIT:** `Context.skip_emit` skips `emit_log` / `emit_tenant` side effects (parallel to `skip_cargo`). FSM unit tests that only need the trace set it.
- **D-OVERLAP:** At `jobs>1`, `observe_cargo` without `overlap_with` is a HIGH finding (not silent skip). Shipped fsm.yaml already has overlap; pin with a synthetic machine.

## Verifier kill-tests

- `test_ci_quantum_*` uses `skip_emit=True`; mock/pin proves append_run not called.
- Synthetic FSM: jobs=2, observe without overlap → HIGH.
