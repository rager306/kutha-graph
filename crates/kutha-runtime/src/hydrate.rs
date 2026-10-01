//! Hydrate, snapshot, fork (droppable leases; log remains SoT).
use crate::fold::GraphFold;
use crate::log::EventLog;
use crate::quantum::{
    op_affects_fold, IngestExamine, PersistedQuantumOutcome, Runtime, RuntimeError,
};
use crate::snapshot::Snapshot;
use kutha_common::{Event, Op, TermDictionary};
use rustc_hash::FxHashMap;

type IngestMap<K, V> = FxHashMap<K, V>;

impl Runtime {
    pub(crate) fn hydrate_from_log(&mut self) {
        let mut outcomes = Vec::new();
        for e in self.log.iter() {
            if let Op::QuantumOutcome {
                quantum_id,
                disposition,
                aborted_on_budget,
                events_in_quantum,
                event_ids,
                receipt_digest_hex,
                resume_of,
            } = &e.op
            {
                outcomes.push(PersistedQuantumOutcome {
                    quantum_id: quantum_id.clone(),
                    disposition: *disposition,
                    aborted_on_budget: *aborted_on_budget,
                    events_in_quantum: *events_in_quantum,
                    event_ids: event_ids.clone(),
                    receipt_digest_hex: receipt_digest_hex.clone(),
                    resume_of: resume_of.clone(),
                });
            }
        }
        self.outcomes = outcomes;
        self.fold.rebuild_blobs(self.log.as_slice());
        self.fold.bind_reserved(&self.dict);
        self.rebuild_log_indexes();
    }

    pub(crate) fn log_has_quantum_outcome(&self) -> bool {
        self.log
            .iter()
            .any(|e| matches!(e.op, Op::QuantumOutcome { .. }))
    }
    pub fn snapshot(&self) -> Snapshot {
        Snapshot {
            log_offset: self.graph_len(),
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
        // Snapshot offset indexes fold-affecting ops. Keep QuantumOutcome on the log (LOG-01).
        let graph: Vec<Event> = all_events
            .into_iter()
            .filter(|e| !matches!(e.op, Op::Define { .. }))
            .collect();
        let mut fold = snap.fold;
        let mut next_tt = snap.next_tt;
        let offset = snap.log_offset;
        let mut fold_seen = 0usize;
        for e in &graph {
            let affects = op_affects_fold(&e.op);
            let in_tail = fold_seen >= offset;
            if affects {
                if in_tail {
                    fold.apply(e);
                    next_tt = next_tt.max(e.ingested_at.saturating_add(1));
                }
                fold_seen += 1;
            } else if in_tail {
                fold.apply(e);
                next_tt = next_tt.max(e.ingested_at.saturating_add(1));
            }
        }
        let mut rt = Self {
            log: EventLog::from_events(graph),
            fold,
            dict,
            next_tt,
            max_cascade: snap.max_cascade,
            knows: snap.knows,
            known_by: snap.known_by,
            outcomes: Vec::new(),
            by_event_id: IngestMap::default(),
            by_delivery_key: IngestMap::default(),
            ingest_examine: IngestExamine::default(),
        };
        rt.hydrate_from_log();
        rt
    }

    /// Rebuild from authoritative term strings + full event replay (no snapshot lease).
    pub fn from_dict_and_events(
        dict_strings: Vec<String>,
        all_events: Vec<Event>,
        max_cascade: usize,
    ) -> Result<Self, RuntimeError> {
        let dict = TermDictionary::from_strings(dict_strings);
        let knows = dict
            .id("knows")
            .ok_or_else(|| RuntimeError::UnknownRelation {
                name: "knows".into(),
            })?;
        let known_by = dict
            .id("knownBy")
            .ok_or_else(|| RuntimeError::UnknownRelation {
                name: "knownBy".into(),
            })?;
        let mut fold = GraphFold::default();
        let mut next_tt = 0u64;
        for e in &all_events {
            fold.apply(e);
            next_tt = next_tt.max(e.ingested_at.saturating_add(1));
        }
        // Replay above may leave Lazy maps cold; Eager ensure for a consistent open picture.
        fold.ensure_hot_maps();
        let mut rt = Self {
            log: EventLog::from_events(all_events),
            fold,
            dict,
            next_tt,
            max_cascade,
            knows,
            known_by,
            outcomes: Vec::new(),
            by_event_id: IngestMap::default(),
            by_delivery_key: IngestMap::default(),
            ingest_examine: IngestExamine::default(),
        };
        rt.hydrate_from_log();
        Ok(rt)
    }

    /// Named overlay: replay a log prefix (ADR-061 P0). Does not share tentatives.
    pub fn fork_at(&self, n: usize) -> Self {
        let prefix = self.log.prefix(n);
        let mut fold = GraphFold::with_index_mode(self.index_mode());
        let mut next_tt = 0;
        for e in prefix.as_slice() {
            fold.apply(e);
            next_tt = next_tt.max(e.ingested_at.saturating_add(1));
        }
        fold.ensure_hot_maps();
        let mut rt = Self {
            log: prefix,
            fold,
            dict: self.dict.clone(),
            next_tt,
            max_cascade: self.max_cascade,
            knows: self.knows,
            known_by: self.known_by,
            outcomes: Vec::new(),
            by_event_id: IngestMap::default(),
            by_delivery_key: IngestMap::default(),
            ingest_examine: IngestExamine::default(),
        };
        rt.hydrate_from_log();
        rt
    }
}
