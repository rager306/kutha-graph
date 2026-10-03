# M002 intake: cloud kernel I/O writeback stalls

**Status:** note only — M002 remains **frozen** until `.kutha/STATE.md` leases it.  
**Captured:** 2026-10-02  
**Card:** `.compound-engineering/artifacts/research/applicability/cards/paper-kernel-io-dirty-throttle-cloud-oltp.md`

## Sources

- Mitsumasa Kondo. *Invisible Yet Dominant: Big Stalls of Kernel I/O Mechanisms in Cloud OLTP Databases.* arXiv:2609.12597 (SOSP 2026 poster).
- Companion: *SteelDB* arXiv:2603.29052.

## Claim (for M002 planning)

On cloud distributed block storage, buffered write-back is limited by a **single kernel flusher thread per device**. When drain lags, **IO-less Dirty Throttle** pauses writers; stalls are invisible to standard disk counters. Adding **drains** (multi-volume placement) beats adding bandwidth on one volume for TPC-C-style write load.

## Implications when M002 is leased

1. Segmented append-only log / WAL framing should not rely on “one volume + buffered I/O is fine.”
2. Prefer isolating the event-log path from index/Rocks leases across devices when on cloud block.
3. Acceptance / soak tests for durability must instrument writeback / dirty-throttle (or equivalent), not only `iostat`.
4. Rocks remains **indexes only** behind the durability protocol — this note does not thaw Rocks as SoT.

## Non-goals

- Do not start M002 from this note alone.
- Do not invent a fifth legal clock or change honeycomb Status.
- Do not treat SteelDB / Aurora architecture as Kutha product design — only the kernel-drain lesson transfers.
