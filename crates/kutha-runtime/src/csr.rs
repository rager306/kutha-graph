use crate::fold::GraphFold;
use kutha_common::{EventId, TermId};

/// Droppable CSR picture of live adjacency (ADR-041). Not SoT.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CsrLease {
    offsets: Vec<u32>,
    neighbors: Vec<TermId>,
}

impl CsrLease {
    pub fn from_fold(fold: &GraphFold, tt: u64, vt: u64, vertex_count: usize) -> Self {
        let mut buckets: Vec<Vec<TermId>> = vec![Vec::new(); vertex_count];
        for f in fold.live_facts_at(tt, vt) {
            let s = f.subject as usize;
            if s < buckets.len() {
                buckets[s].push(f.object());
            }
        }
        let mut offsets = Vec::with_capacity(vertex_count + 1);
        let mut neighbors = Vec::new();
        offsets.push(0);
        for mut row in buckets {
            row.sort_unstable();
            row.dedup();
            neighbors.extend(row);
            offsets.push(neighbors.len() as u32);
        }
        Self { offsets, neighbors }
    }

    pub fn neighbors(&self, v: TermId) -> &[TermId] {
        let i = v as usize;
        if i + 1 >= self.offsets.len() {
            return &[];
        }
        let a = self.offsets[i] as usize;
        let b = self.offsets[i + 1] as usize;
        &self.neighbors[a..b]
    }

    /// Sorted seek: first neighbor ≥ key (LFTJ iterator cousin).
    pub fn seek(&self, v: TermId, key: TermId) -> Option<TermId> {
        let row = self.neighbors(v);
        let i = row.partition_point(|x| *x < key);
        row.get(i).copied()
    }
}

/// One live Fact as a typed CSR edge (relation + object + claim support identity).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct TypedEdge {
    pub relation: TermId,
    pub object: TermId,
    pub claim_id: EventId,
    pub fact_seq: u64,
}

/// Droppable CSR picture of live typed edges (ADR-041). Not SoT.
/// Parallel to [`CsrLease`]: one edge per live Fact; does not collapse supports or labels.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TypedCsrLease {
    offsets: Vec<u32>,
    edges: Vec<TypedEdge>,
}

impl TypedCsrLease {
    /// Rebuild typed adjacency from the same live-fact cut as [`CsrLease::from_fold`].
    /// Keeps every live Fact as its own edge (no object-only dedup).
    pub fn from_fold(fold: &GraphFold, tt: u64, vt: u64, vertex_count: usize) -> Self {
        let mut buckets: Vec<Vec<TypedEdge>> = vec![Vec::new(); vertex_count];
        for f in fold.live_facts_at(tt, vt) {
            let s = f.subject as usize;
            if s < buckets.len() {
                buckets[s].push(TypedEdge {
                    relation: f.relation,
                    object: f.object(),
                    claim_id: f.claim_id,
                    fact_seq: f.seq,
                });
            }
        }
        let mut offsets = Vec::with_capacity(vertex_count + 1);
        let mut edges = Vec::new();
        offsets.push(0);
        for mut row in buckets {
            row.sort_unstable_by_key(|e| (e.relation, e.object, e.claim_id, e.fact_seq));
            edges.extend(row);
            offsets.push(edges.len() as u32);
        }
        Self { offsets, edges }
    }

    pub fn edges_out(&self, v: TermId) -> &[TypedEdge] {
        let i = v as usize;
        if i + 1 >= self.offsets.len() {
            return &[];
        }
        let a = self.offsets[i] as usize;
        let b = self.offsets[i + 1] as usize;
        &self.edges[a..b]
    }
}
