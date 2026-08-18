---
id: paper-pmem-graph-store
source: paper
axes: [Data, Time, Composition]
usefulness: high
optimality: med
demand: med
confidence: paper
layer5: no niche
status: closed
channels_failed: []
---

# Byte-addressable persistence is a placement tier — not SEM SSD and not SoT

Papers: Optane DCPMM: byte-addressable, denser/cheaper than DRAM, persist across power loss; read≈DRAM, write is the tax [1][5][14]; [Gill et al.](https://consensus.app/papers/details/1e20e62fbce15aa499f18a598b1077b1/?utm_source=cursor) (2019, PVLDB) 6 TB Optane; graph frameworks that respect PMEM principles beat naive ports and rival clusters [4]; DRAM+NVM hybrid and write-isolation save energy/bandwidth vs DRAM-as-cache [6]; [XPGraph](https://consensus.app/papers/details/8d7e8144816854239d20a9cbaf056024/?utm_source=cursor) (Wang et al., 2022/2025) XPLine-friendly vertex-centric buffers + NUMA-aware access; 3–5× vs DRAM graph stores dumped onto PMEM [10][16]; HANA-style cost model for which structures sit in PMEM vs DRAM [7]; post-Optane: CXL pooling keeps crash-consistency research alive even on volatile pooled memory [20][12]. Distinct from `paper-semi-external-graph` (block SSD; vertices in RAM, edges fetched), `paper-lsm-snapshot-compaction` (versioned files), `rocksdb-wal-recovery` (WAL replay of DRAM/LSM), `samyama-csr-frozen-adjacency` (hot DRAM CSR). GPU/CXL µs-latency stays queued.

## 1. Raw idea

A graph can live in **byte-addressable** memory that survives a crash [2][14]. It is not a disk and not DRAM: pointer-rich layouts must respect write amplification, NUMA, and XPLine granularity [10]. Persistence of *placement* is not persistence of *truth*.

## 2. STCA applicability

Data: PMEM/CXL is a **tier for leases** (CSR, vertex state), not the event-log SoT. Time: crash-consistency of in-memory structures is WAL/Rocks’ cousin; the log still reconstructs the fold. Composition: HANA-like placement of which materializations sit in DRAM vs PMEM is a Cui budget [7]. Query: analytics that scan CSR can run in-place; MATCH still sees the fold. Verify: a torn XPLine is not a quantum receipt. LLM does not allocate PMEM.

## 3. Quality / cost

Usefulness high while Optane existed; demand med after cancellation, with CXL as the open interface [20]. Optimality med: XPGraph shows naive ports fail. Cost: P0 = DRAM/SSD SEM; honeycomb = optional CXL/PMEM placement for large leases. Do not treat Intel Optane as a product dependency. Do not make PMEM the log.

## 4. Demand

Folds that exceed DRAM but need pointer-chasing will appear. Engine demand: an explicit placement policy DRAM | CXL/PMEM | NVMe, fail-closed to SEM.

## 5. Niche → effect

`no niche`
