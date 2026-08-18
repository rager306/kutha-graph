use crate::csr::CsrLease;
use crate::fold::GraphFold;

/// Reversible view contract (ADR-040). Pack install is ADR-021, not this trait.
pub trait Materializer {
    fn name(&self) -> &'static str;
    fn build(&mut self, fold: &GraphFold, vertex_count: usize, tt: u64, vt: u64, log_offset: usize);
    fn unload(&mut self);
    fn is_mounted(&self) -> bool;
}

#[derive(Default)]
pub struct CsrMaterializer {
    lease: Option<CsrLease>,
    log_offset: Option<usize>,
}

impl CsrMaterializer {
    pub fn lease(&self) -> Option<&CsrLease> {
        self.lease.as_ref()
    }

    pub fn log_offset(&self) -> Option<usize> {
        self.log_offset
    }
}

impl Materializer for CsrMaterializer {
    fn name(&self) -> &'static str {
        "csr"
    }

    fn build(
        &mut self,
        fold: &GraphFold,
        vertex_count: usize,
        tt: u64,
        vt: u64,
        log_offset: usize,
    ) {
        self.lease = Some(CsrLease::from_fold(fold, tt, vt, vertex_count));
        self.log_offset = Some(log_offset);
    }

    fn unload(&mut self) {
        self.lease = None;
        self.log_offset = None;
    }

    fn is_mounted(&self) -> bool {
        self.lease.is_some()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::quantum::Runtime;
    use kutha_common::Op;

    #[test]
    fn csr_materializer_build_unload() {
        let mut rt = Runtime::default();
        let a = rt.intern("A");
        let b = rt.intern("B");
        let r = rt.intern("knows");
        rt.emit(Op::Assert {
            subject: a,
            relation: r,
            object: b,
            valid_from: 0,
            valid_to: None,
        })
        .unwrap();
        let mut m = CsrMaterializer::default();
        m.build(
            rt.fold(),
            rt.dictionary().len(),
            u64::MAX,
            0,
            rt.log().len(),
        );
        assert!(m.is_mounted());
        assert_eq!(m.lease().unwrap().neighbors(a), &[b]);
        m.unload();
        assert!(!m.is_mounted());
        m.build(
            rt.fold(),
            rt.dictionary().len(),
            u64::MAX,
            0,
            rt.log().len(),
        );
        assert_eq!(m.lease().unwrap().neighbors(a), &[b]);
    }
}
