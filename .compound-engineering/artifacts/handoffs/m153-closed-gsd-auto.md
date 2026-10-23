---
ce-handoff/v1: true
title: "M153 closed (Verus on-ramp) — Arc B next, B1 vacuum candidate"
date: 2026-10-22
branch: work/main-line
head: 89257ca
resume_focus: "After the user asks to continue: lease B1 (ADR-012 thin vacuum with receipt) with a fresh RED, bundle STATE+ROADMAP+CHANGELOG+README. Harness moratorium applies. Verus proofs ride inside the product slice that touches the kernel. Do not thaw Cypher/HNSW/SIX-01/legal pack without their own lease."
---

Superseded by m154-closed-gsd-auto.md

# Handoff — M153 closed, Arc B next

## Closed

- **M153 / GSD v1.47 / Phase 227:** Verus on-ramp. `tools/verus/{VERSION,SHA256SUMS}` pin release `0.2026.10.18.7d82994` (rustc 1.99.1; checksum equals the GitHub release digest). `verus/vt_intersect.rs` carries the production body verbatim and verifies: `2 verified, 0 errors`. Two body mutations fail in Verus. Check `verus-proof-sync` (invariant `I-verus-vt-proof-sync`, matrix `SEM-m153-verus-vt-sync`) pins body sync; budget 105.
- Closeout: `kutha-gov ci --jobs 1` `run_high=0`, 105 checks.
- Verus is evidence, not a gate: CI never runs it, no crate depends on it.

## Local setup (not in git)

- Binary: `~/.local/share/kutha/verus/0.2026.10.18.7d82994/verus-x86-linux/verus`; archive in `.../verus/dl/` (487 MB, deletable). `.env` has `KUTHA_VERUS_BIN` and budget 105.
- Run: `RUSTUP_TOOLCHAIN=1.99.1 "$KUTHA_VERUS_BIN" verus/vt_intersect.rs`.
- The official PDD Kit (GitHub Next + Microsoft Research) is a research preview with no public install; this slice is only its Verus prerequisite. Do not confuse it with `pdd-kit` / `pdd-toolkit` (Product-Driven, creates `product-model/`, would add a second SoT).

## Next

1. Lease B1 (ADR-012 thin vacuum with a receipt op).
2. Further proofs (`conjunct_vt`, `leftover_halves`, `windows_cover`; later `leapfrog_intersect` needs an index-based mirror) inside the slice that edits them.
3. Disk and baseline environment before B3; owner decision pending on `reviews/2026-10-11-ce-whole-repo/report.md` (left uncommitted).
