use crate::fold::GraphFold;
use kutha_common::TermId;

/// Droppable CSR picture of live adjacency (ADR-041). Not SoT.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CsrLease {
    offsets: Vec<u32>,
    neighbors: Vec<TermId>,
}

impl CsrLease {
    pub fn from_fold(fold: &GraphFold, tt: u64, vt: u64, vertex_count: usize) -> Self {
        let mut buckets: Vec<Vec<TermId>> = vec![Vec::new(); vertex_count];
        for f in fold.facts() {
            if !f.is_live_at(tt, vt) {
                continue;
            }
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
