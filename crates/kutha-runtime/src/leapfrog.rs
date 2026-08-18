use kutha_common::TermId;

/// Seekable iterator over a sorted CSR neighbor row (ADR-041 / LFTJ).
#[derive(Clone, Debug)]
pub struct AdjacencyIter<'a> {
    row: &'a [TermId],
    i: usize,
}

impl<'a> AdjacencyIter<'a> {
    pub fn new(row: &'a [TermId]) -> Self {
        Self { row, i: 0 }
    }

    pub fn at_end(&self) -> bool {
        self.i >= self.row.len()
    }

    pub fn key(&self) -> Option<TermId> {
        self.row.get(self.i).copied()
    }

    pub fn next(&mut self) {
        if self.i < self.row.len() {
            self.i += 1;
        }
    }

    pub fn seek(&mut self, key: TermId) {
        let tail = &self.row[self.i.min(self.row.len())..];
        let j = tail.partition_point(|x| *x < key);
        self.i += j;
    }
}

/// Multiway sorted intersection via leapfrog seeks to the current max key.
pub fn leapfrog_intersect(rows: &[&[TermId]]) -> Vec<TermId> {
    if rows.is_empty() || rows.iter().any(|r| r.is_empty()) {
        return Vec::new();
    }
    let mut iters: Vec<AdjacencyIter<'_>> = rows.iter().map(|r| AdjacencyIter::new(r)).collect();
    let mut out = Vec::new();
    loop {
        let mut mn: Option<TermId> = None;
        let mut mx: Option<TermId> = None;
        for it in &iters {
            match it.key() {
                None => return out,
                Some(k) => {
                    mn = Some(mn.map_or(k, |m| m.min(k)));
                    mx = Some(mx.map_or(k, |m| m.max(k)));
                }
            }
        }
        let (mn, mx) = (mn.unwrap(), mx.unwrap());
        if mn == mx {
            out.push(mn);
            for it in &mut iters {
                it.next();
            }
        } else {
            for it in &mut iters {
                it.seek(mx);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn leapfrog_three_lists() {
        let a = [1u32, 3, 5, 7, 9];
        let b = [3u32, 4, 7, 10];
        let c = [0u32, 3, 7, 8];
        assert_eq!(leapfrog_intersect(&[&a, &b, &c]), vec![3, 7]);
    }

    #[test]
    fn leapfrog_common_neighbors_on_csr() {
        use crate::csr::CsrLease;
        use crate::quantum::Runtime;
        use kutha_common::Op;

        let mut rt = Runtime::default();
        let a = rt.intern("A");
        let b = rt.intern("B");
        let c = rt.intern("C");
        let rel = rt.intern("relatedTo");
        for (s, o) in [(a, c), (b, c)] {
            rt.emit(Op::Assert {
                subject: s,
                relation: rel,
                object: o,
                valid_from: 0,
                valid_to: None,
            })
            .unwrap();
        }
        let csr = CsrLease::from_fold(rt.fold(), u64::MAX, 0, rt.dictionary().len());
        let common = leapfrog_intersect(&[csr.neighbors(a), csr.neighbors(b)]);
        assert_eq!(common, vec![c]);
    }

    #[test]
    fn leapfrog_empty_short_circuits() {
        let a = [1u32, 2];
        let b: [u32; 0] = [];
        assert!(leapfrog_intersect(&[&a, &b]).is_empty());
    }
}
