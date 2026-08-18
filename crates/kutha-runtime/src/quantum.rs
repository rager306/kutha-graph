use crate::allow::load_allowed_names;
use crate::csr::CsrLease;
use crate::fold::GraphFold;
use crate::log::EventLog;
use crate::receipt::QuantumReceipt;
use crate::snapshot::Snapshot;
use kutha_common::{Event, Op, TermDictionary, TermId};
use std::collections::HashSet;
use std::fmt;

#[derive(Debug)]
pub enum RuntimeError {
    ReplayDivergence {
        expected: [u8; 32],
        actual: [u8; 32],
    },
    UnknownFact {
        fact_seq: u64,
    },
    UnknownRelation {
        name: String,
    },
}

impl fmt::Display for RuntimeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RuntimeError::ReplayDivergence { .. } => write!(f, "ReplayDivergenceError"),
            RuntimeError::UnknownFact { fact_seq } => write!(f, "unknown fact {fact_seq}"),
            RuntimeError::UnknownRelation { name } => {
                write!(f, "unknown relation {name} (not in allowlist)")
            }
        }
    }
}

fn op_relation(op: &Op) -> Option<TermId> {
    match op {
        Op::Assert { relation, .. } | Op::Behavior { relation, .. } => Some(*relation),
        Op::Retract { .. } | Op::Correct { .. } => None,
    }
}

impl std::error::Error for RuntimeError {}

#[derive(Clone, Debug)]
pub struct QuantumOutcome {
    pub receipt: QuantumReceipt,
    pub events_in_quantum: usize,
}

/// Inverse-edge relation name interned as `knownBy` when `knows` is asserted (demo cascade).
pub struct Runtime {
    log: EventLog,
    fold: GraphFold,
    dict: TermDictionary,
    next_tt: u64,
    pub max_cascade: usize,
    knows: TermId,
    known_by: TermId,
    allowed: HashSet<String>,
}

impl Default for Runtime {
    fn default() -> Self {
        let v = {
            crate::allow::apply_dotenv();
            std::env::var("KUTHA_MAX_CASCADE")
                .ok()
                .and_then(|s| s.parse().ok())
                .filter(|n: &usize| *n > 0)
                .unwrap_or(32)
        };
        Self::new(v)
    }
}

impl Runtime {
    pub fn new(max_cascade: usize) -> Self {
        let mut dict = TermDictionary::default();
        let knows = dict.intern("knows");
        let known_by = dict.intern("knownBy");
        Self {
            log: EventLog::default(),
            fold: GraphFold::default(),
            dict,
            next_tt: 0,
            max_cascade,
            knows,
            known_by,
            allowed: load_allowed_names(),
        }
    }

    pub fn intern(&mut self, s: &str) -> TermId {
        self.dict.intern(s)
    }

    pub fn log(&self) -> &EventLog {
        &self.log
    }

    pub fn fold(&self) -> &GraphFold {
        &self.fold
    }

    pub fn dictionary(&self) -> &TermDictionary {
        &self.dict
    }

    pub fn snapshot(&self) -> Snapshot {
        Snapshot {
            log_offset: self.log.len(),
            next_tt: self.next_tt,
            max_cascade: self.max_cascade,
            fold: self.fold.clone(),
            dict_strings: self.dict.strings().to_vec(),
            knows: self.knows,
            known_by: self.known_by,
        }
    }

    pub fn from_snapshot(snap: Snapshot, all_events: Vec<Event>) -> Self {
        let dict = TermDictionary::from_strings(snap.dict_strings);
        let mut fold = snap.fold;
        let mut next_tt = snap.next_tt;
        let offset = snap.log_offset.min(all_events.len());
        for e in &all_events[offset..] {
            fold.apply(e);
            next_tt = next_tt.max(e.ingested_at.saturating_add(1));
        }
        Self {
            log: EventLog::from_events(all_events),
            fold,
            dict,
            next_tt,
            max_cascade: snap.max_cascade,
            knows: snap.knows,
            known_by: snap.known_by,
            allowed: load_allowed_names(),
        }
    }

    /// Named overlay: replay a log prefix (ADR-061 P0). Does not share tentatives.
    pub fn fork_at(&self, n: usize) -> Self {
        let prefix = self.log.prefix(n);
        let mut fold = GraphFold::default();
        let mut next_tt = 0;
        for e in prefix.as_slice() {
            fold.apply(e);
            next_tt = next_tt.max(e.ingested_at.saturating_add(1));
        }
        Self {
            log: prefix,
            fold,
            dict: self.dict.clone(),
            next_tt,
            max_cascade: self.max_cascade,
            knows: self.knows,
            known_by: self.known_by,
            allowed: self.allowed.clone(),
        }
    }

    pub fn csr_lease(&self) -> CsrLease {
        self.csr_lease_at(u64::MAX, 0)
    }

    /// CSR lease at an explicit cut. Callers must name valid-time (no silent “now”).
    pub fn csr_lease_at(&self, tt: u64, vt: u64) -> CsrLease {
        CsrLease::from_fold(&self.fold, tt, vt, self.dict.len())
    }

    fn next_tt(&mut self) -> u64 {
        let t = self.next_tt;
        self.next_tt += 1;
        t
    }

    fn admit(&self, op: &Op) -> Result<(), RuntimeError> {
        let Some(rel) = op_relation(op) else {
            return Ok(());
        };
        let name = self.dict.lookup(rel).unwrap_or("").to_string();
        if self.allowed.contains(&name) {
            return Ok(());
        }
        Err(RuntimeError::UnknownRelation {
            name: if name.is_empty() {
                format!("#{rel}")
            } else {
                name
            },
        })
    }

    /// Admit a user op, append, fold, cascade inverse-`knows` until idle or budget.
    pub fn emit(&mut self, op: Op) -> Result<QuantumOutcome, RuntimeError> {
        if let Op::Retract { fact_seq } | Op::Correct { fact_seq, .. } = &op {
            if !self.fold.facts().iter().any(|f| f.seq == *fact_seq) {
                return Err(RuntimeError::UnknownFact {
                    fact_seq: *fact_seq,
                });
            }
        }
        self.admit(&op)?;
        let first = Event::new(op, self.next_tt());
        for follow in self.follow_ons(&first) {
            self.admit(&follow.op)?;
        }
        let mut ids = Vec::new();
        let mut digests = Vec::new();
        let mut aborted = false;
        let mut pending = vec![first];
        let mut used = 0usize;

        while let Some(event) = pending.pop() {
            if used >= self.max_cascade {
                aborted = true;
                break;
            }
            used += 1;
            ids.push(event.id);
            digests.push(event.digest_bytes());
            let follow = self.follow_ons(&event);
            self.fold.apply(&event);
            self.log.append(event);
            for e in follow {
                pending.push(e);
            }
        }

        Ok(QuantumOutcome {
            receipt: QuantumReceipt::from_events(ids, &digests, aborted),
            events_in_quantum: used,
        })
    }

    fn follow_ons(&self, event: &Event) -> Vec<Event> {
        match &event.op {
            Op::Assert {
                subject,
                relation,
                object,
                valid_from,
                valid_to,
            } if *relation == self.knows => {
                let exists = self.fold.facts().iter().any(|f| {
                    f.subject == *object
                        && f.relation == self.known_by
                        && f.object() == *subject
                        && f.invalidated_at.is_none()
                });
                if exists {
                    return vec![];
                }
                vec![Event::new(
                    Op::Behavior {
                        name: "inverse_knows".into(),
                        caused_by: event.id,
                        subject: *object,
                        relation: self.known_by,
                        object: *subject,
                        valid_from: *valid_from,
                        valid_to: *valid_to,
                    },
                    event.ingested_at,
                )]
            }
            _ => vec![],
        }
    }

    /// Strict replay from the log (ADR-060). LLM is not consulted.
    pub fn replay_check(&self) -> Result<GraphFold, RuntimeError> {
        let rebuilt = GraphFold::replay(self.log.as_slice());
        let expected = self.fold.fingerprint();
        let actual = rebuilt.fingerprint();
        if expected != actual {
            return Err(RuntimeError::ReplayDivergence { expected, actual });
        }
        Ok(rebuilt)
    }

    #[cfg(test)]
    fn tamper_fold(&mut self) {
        self.fold.tamper_invalidate_first();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use kutha_common::Op;
    use std::time::Instant;

    #[test]
    fn assert_then_fold_has_live_edge() {
        let mut rt = Runtime::default();
        let alice = rt.intern("Alice");
        let bob = rt.intern("Bob");
        let knows = rt.intern("knows");
        rt.emit(Op::Assert {
            subject: alice,
            relation: knows,
            object: bob,
            valid_from: 0,
            valid_to: None,
        })
        .unwrap();
        assert_eq!(rt.fold().live_count(u64::MAX, 0), 2); // knows + inverse
        rt.replay_check().unwrap();
    }

    #[test]
    fn retract_keeps_loser() {
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
        let seq = rt.fold().facts()[0].seq;
        rt.emit(Op::Retract { fact_seq: seq }).unwrap();
        assert_eq!(rt.fold().facts().len(), 2); // knows retracted, inverse still live
        assert!(!rt.fold().facts()[0].is_live_at(u64::MAX, 0));
        assert!(rt.fold().facts().iter().any(|f| f.invalidated_at.is_some()));
        rt.replay_check().unwrap();
    }

    #[test]
    fn cascade_idles_without_inverse_loop() {
        let mut rt = Runtime::default();
        let a = rt.intern("A");
        let b = rt.intern("B");
        let r = rt.intern("knows");
        let q = rt
            .emit(Op::Assert {
                subject: a,
                relation: r,
                object: b,
                valid_from: 0,
                valid_to: None,
            })
            .unwrap();
        assert_eq!(q.events_in_quantum, 2);
        assert!(!q.receipt.aborted_on_budget);
        assert_eq!(rt.log().len(), 2);
    }

    #[test]
    fn budget_aborts_storm() {
        let mut rt = Runtime::new(1);
        let a = rt.intern("A");
        let b = rt.intern("B");
        let r = rt.intern("knows");
        let q = rt
            .emit(Op::Assert {
                subject: a,
                relation: r,
                object: b,
                valid_from: 0,
                valid_to: None,
            })
            .unwrap();
        assert!(q.receipt.aborted_on_budget);
        assert_eq!(q.events_in_quantum, 1);
    }

    #[test]
    fn replay_detects_tamper() {
        let mut rt = Runtime::default();
        let a = rt.intern("A");
        let b = rt.intern("B");
        let r = rt.intern("relatedTo");
        rt.emit(Op::Assert {
            subject: a,
            relation: r,
            object: b,
            valid_from: 0,
            valid_to: None,
        })
        .unwrap();
        rt.tamper_fold();
        let err = rt.replay_check().unwrap_err();
        assert!(matches!(err, RuntimeError::ReplayDivergence { .. }));
    }

    #[test]
    fn replay_time_vs_log_length() {
        let mut rt = Runtime::default();
        let rel = rt.intern("relatedTo");
        for i in 0..200u32 {
            let s = rt.intern(&format!("n{i}"));
            let o = rt.intern(&format!("n{}", i + 1));
            rt.emit(Op::Assert {
                subject: s,
                relation: rel,
                object: o,
                valid_from: 0,
                valid_to: None,
            })
            .unwrap();
        }
        let start = Instant::now();
        rt.replay_check().unwrap();
        let ms = start.elapsed().as_secs_f64() * 1000.0;
        eprintln!(
            "kutha P0 measure: replay_check log_len={} live={} elapsed_ms={:.3}",
            rt.log().len(),
            rt.fold().live_count(u64::MAX, 0),
            ms
        );
        assert!(ms < 1000.0, "replay unexpectedly slow: {ms} ms");
    }

    #[test]
    fn snapshot_plus_tail_matches_full_replay() {
        let mut rt = Runtime::default();
        let rel = rt.intern("relatedTo");
        for i in 0..10u32 {
            let s = rt.intern(&format!("s{i}"));
            let o = rt.intern(&format!("o{i}"));
            rt.emit(Op::Assert {
                subject: s,
                relation: rel,
                object: o,
                valid_from: 0,
                valid_to: None,
            })
            .unwrap();
        }
        let snap = rt.snapshot();
        let events = rt.log().as_slice().to_vec();
        let restored = Runtime::from_snapshot(snap, events);
        assert_eq!(restored.fold().fingerprint(), rt.fold().fingerprint());
        restored.replay_check().unwrap();
    }

    #[test]
    fn persist_open_round_trip() {
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
        let dir = std::env::temp_dir().join(format!("kutha-p0-{}", uuid_like()));
        crate::store::persist(&rt, &dir).unwrap();
        let opened = crate::store::open(&dir).unwrap();
        assert_eq!(opened.fold().fingerprint(), rt.fold().fingerprint());
        assert_eq!(opened.dictionary().lookup(a), Some("A"));
        opened.replay_check().unwrap();
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn csr_drop_rebuild_and_seek() {
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
        let csr = rt.csr_lease();
        assert_eq!(csr.neighbors(a), &[b]);
        assert_eq!(csr.seek(a, b), Some(b));
        assert_eq!(csr.seek(a, b.saturating_add(1)), None);
        drop(csr);
        let csr2 = rt.csr_lease();
        assert_eq!(csr2.neighbors(a), &[b]);
    }

    #[test]
    fn fork_at_isolates_prefix() {
        let mut rt = Runtime::default();
        let rel = rt.intern("relatedTo");
        let n0 = rt.intern("n0");
        let n1 = rt.intern("n1");
        let n2 = rt.intern("n2");
        rt.emit(Op::Assert {
            subject: n0,
            relation: rel,
            object: n1,
            valid_from: 0,
            valid_to: None,
        })
        .unwrap();
        let cut = rt.log().len();
        rt.emit(Op::Assert {
            subject: n1,
            relation: rel,
            object: n2,
            valid_from: 0,
            valid_to: None,
        })
        .unwrap();
        let fork = rt.fork_at(cut);
        assert_eq!(fork.log().len(), cut);
        assert!(fork.fold().live_count(u64::MAX, 0) < rt.fold().live_count(u64::MAX, 0));
        fork.replay_check().unwrap();
    }

    fn uuid_like() -> u64 {
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos() as u64
    }
}
