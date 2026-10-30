# Gate GC — toy pack with zero core edits

Process evidence, not an ADR and not honeycomb Acceptance. Gate GC asks: does a
toy pack install, query through GPML `AS OF`, and retire **without editing the
core**? If core edits were needed, ADR-021 must be fixed before Arc A.

## Result

**Passed** on 2026-10-31 (M179-S01). The probe is
`crates/kutha-runtime/tests/m179_gate_gc.rs`; it adds no production code.

| Step | Surface used | Core edit |
|------|--------------|-----------|
| Install + activate a toy pack with a manifest declaring one operator | C1 `emit_pack_install_with_manifest` / `emit_pack_activate`, C1b `PackManifest` | none |
| Write product facts owned by the host, not the pack | existing `Op::Assert` | none |
| Query at two cuts through GPML `AS OF`, with the pack installed and after retire | M003a `parse_gpml` + `RuntimeBinder` + `execute`; the query path does **not** consult pack state | none |
| Retire | C1 `emit_pack_retire` | none |
| Reopen and compare | existing `persist` / `Runtime::open` | none |

Observed:

- lifecycle `Active` → `Retired`; operator registry rows follow the pack state;
- `AS OF 2015` returns `50-kmh`, `AS OF 2020` returns `30-kmh`, before and after retire;
- fold fingerprint and fact count are unchanged by install, activate, and retire;
- reopen equals the live runtime on facts, fingerprint, lifecycle, and registry.

## What this does not prove

- **Query sufficiency.** The GPML leg never consults pack state (`RuntimeBinder` is dictionary + FF6 only). Deleting the pack leaves both `AS OF` assertions green. What is proven is *neutrality*: install and retire do not disturb queries. Gate GC's "queries via Cypher AS OF" is met in that weaker sense; it is not evidence that a pack participates in query resolution.
- No operator executes. The manifest names an operator; nothing resolves or runs it.
- No dictionary facets, no schema SMO, no lease rebuild, no HNSW, no WASM, no marketplace.
- The pack carries no code and no legal corpus. ADR-021 and ADR-052 stay **Proposed**.
- Gate GC is a sufficiency probe for the current pack surface, not product readiness.

## Non-goal boundary

Gate GC passing does **not** thaw a legal pack (`I-legal-pack-not-next` stays) and
does not make C2/C3/C6 unnecessary. Those still need their own leases.
