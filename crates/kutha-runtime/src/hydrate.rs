//! Hydrate, snapshot, fork (droppable leases; log remains SoT).
use crate::fold::GraphFold;
use crate::log::EventLog;
use crate::quantum::{
    IngestExamine, PersistedQuantumOutcome, Runtime, RuntimeError, op_affects_fold,
};
use crate::snapshot::Snapshot;
use kutha_common::{Event, Op, TermDictionary};
use rustc_hash::{FxHashMap, FxHashSet};

type IngestMap<K, V> = FxHashMap<K, V>;

impl Runtime {
    /// Rebuild the droppable operator index from the log, in log order.
    ///
    /// Retire is applied inline so a pack that retires and a later pack that
    /// reuses its name rebuild exactly as admission and replay see them.
    /// A manifest that cannot be loaded leaves the index incomplete; replay is
    /// the gate that fails closed on it, so callers that skip `replay_check`
    /// must not treat the index as authoritative.
    fn rebuild_operator_index(&mut self) {
        let mut index = crate::pack_manifest::OperatorIndex::new();
        let mut lifecycle = crate::PackLifecycleTracker::new();
        let mut trusted = true;
        let mut enrichment_cache_keys = std::collections::HashSet::default();
        let mut log_ids = rustc_hash::FxHashSet::default();
        for event in self.log.iter() {
            match &event.op {
                Op::PackInstall { .. } | Op::PackActivate { .. } | Op::PackRetire { .. } => {
                    // A hand-edited invalid transition leaves the lease short.
                    // `replay_check` rejects it on the store path; a caller
                    // that skips it makes admission fail closed via
                    // `meta_leases_trusted`.
                    if lifecycle.replay(&event.op).is_err() {
                        trusted = false;
                    }
                }
                _ => {}
            }
            match &event.op {
                Op::EnrichmentProposal { cache_key, .. } => {
                    enrichment_cache_keys.insert(cache_key.clone());
                }
                _ => {}
            }
            log_ids.insert(event.id);
            match &event.op {
                Op::PackRetire { pack_id, .. } => index.retire(pack_id),
                Op::PackInstall {
                    pack_id,
                    manifest_digest: Some(_),
                    ..
                } => {
                    match crate::pack_manifest::declared_operators_for(&self.fold.blobs, &event.op)
                    {
                        Ok(names) => {
                            if index.declare(pack_id, names).is_err() {
                                trusted = false;
                            }
                        }
                        Err(_) => trusted = false,
                    }
                }
                _ => {}
            }
        }
        self.operators = index;
        self.lifecycle = lifecycle;
        self.meta_leases_trusted = trusted;
        self.enrichment_cache_keys = enrichment_cache_keys;
        self.log_ids = log_ids;
    }

    pub(crate) fn hydrate_from_log(&mut self) {
        self.outcomes = outcomes_from_events(self.log.iter());
        self.fold.rebuild_blobs(self.log.as_slice());
        self.fold.rebuild_tt_anchors(self.log.as_slice());
        self.fold.bind_reserved(&self.dict);
        self.rebuild_operator_index();
        self.rebuild_log_indexes();
    }
}

pub(crate) fn outcomes_from_events<'a>(
    events: impl Iterator<Item = &'a Event>,
) -> Vec<PersistedQuantumOutcome> {
    let mut outcomes = Vec::new();
    for e in events {
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
    outcomes
}

impl Runtime {
    pub fn snapshot(&self) -> Snapshot {
        Snapshot {
            log_offset: self.graph_len(),
            next_tt: self.next_tt,
            max_cascade: self.max_cascade,
            fold: self.fold.clone(),
            dict_strings: self.dict.strings().to_vec(),
            knows: self.knows,
            known_by: self.known_by,
            behaviors: self.fold.compacted_behaviors.clone(),
            behaviors_exact: true,
        }
    }

    pub fn from_snapshot(snap: Snapshot, all_events: Vec<Event>) -> Self {
        let mut dict = TermDictionary::from_strings(snap.dict_strings);
        for e in &all_events {
            if let Op::Define { name } = &e.op {
                let _ = dict.intern(name);
            }
        }
        // Snapshot offset indexes fold-affecting ops. Keep QuantumOutcome on the log (LOG-01).
        let graph: Vec<Event> = all_events
            .into_iter()
            .filter(|e| !matches!(e.op, Op::Define { .. }))
            .collect();
        let mut fold = snap.fold;
        fold.compacted_behaviors = snap.behaviors;
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
            retract_examine: IngestExamine::default(),
            unindex_cause_examine: IngestExamine::default(),
            unindex_premise_examine: IngestExamine::default(),
            unindex_live_examine: IngestExamine::default(),
            derived_by_cause: IngestMap::default(),
            cause_of_behavior: IngestMap::default(),
            live_behaviors: FxHashSet::default(),
            behaviors_by_premise_rel: IngestMap::default(),
            premise_rels_of_behavior: IngestMap::default(),
            behaviors_by_premise_fact: IngestMap::default(),
            premise_facts_of_behavior: IngestMap::default(),
            compacted_behavior_by_id: IngestMap::default(),
            compacted_assert_by_id: IngestMap::default(),
            operators: crate::pack_manifest::OperatorIndex::new(),
            lifecycle: crate::PackLifecycleTracker::new(),
            meta_leases_trusted: true,
            enrichment_cache_keys: std::collections::HashSet::default(),
            log_ids: rustc_hash::FxHashSet::default(),
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
            retract_examine: IngestExamine::default(),
            unindex_cause_examine: IngestExamine::default(),
            unindex_premise_examine: IngestExamine::default(),
            unindex_live_examine: IngestExamine::default(),
            derived_by_cause: IngestMap::default(),
            cause_of_behavior: IngestMap::default(),
            live_behaviors: FxHashSet::default(),
            behaviors_by_premise_rel: IngestMap::default(),
            premise_rels_of_behavior: IngestMap::default(),
            behaviors_by_premise_fact: IngestMap::default(),
            premise_facts_of_behavior: IngestMap::default(),
            compacted_behavior_by_id: IngestMap::default(),
            compacted_assert_by_id: IngestMap::default(),
            operators: crate::pack_manifest::OperatorIndex::new(),
            lifecycle: crate::PackLifecycleTracker::new(),
            meta_leases_trusted: true,
            enrichment_cache_keys: std::collections::HashSet::default(),
            log_ids: rustc_hash::FxHashSet::default(),
        };
        rt.hydrate_from_log();
        Ok(rt)
    }

    /// Named overlay: replay a log prefix (ADR-061 P0). Does not share tentatives.
    /// Compacted checkpoint prefix is a cut: `n == 0` is empty Ok; `1..=c` and
    /// `n > log.len()` are [`RuntimeError::ForkCutUnavailable`].
    pub fn fork_at(&self, n: usize) -> Result<Self, RuntimeError> {
        let log_len = self.log.len();
        if n > log_len {
            return Err(RuntimeError::ForkCutUnavailable { cut: n, log_len });
        }
        if n > 0 {
            if let Some(c) = self
                .log
                .as_slice()
                .iter()
                .position(|e| matches!(e.op, Op::Checkpoint { .. }))
            {
                if n <= c {
                    return Err(RuntimeError::ForkCutUnavailable { cut: n, log_len });
                }
            }
        }
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
            retract_examine: IngestExamine::default(),
            unindex_cause_examine: IngestExamine::default(),
            unindex_premise_examine: IngestExamine::default(),
            unindex_live_examine: IngestExamine::default(),
            derived_by_cause: IngestMap::default(),
            cause_of_behavior: IngestMap::default(),
            live_behaviors: FxHashSet::default(),
            behaviors_by_premise_rel: IngestMap::default(),
            premise_rels_of_behavior: IngestMap::default(),
            behaviors_by_premise_fact: IngestMap::default(),
            premise_facts_of_behavior: IngestMap::default(),
            compacted_behavior_by_id: IngestMap::default(),
            compacted_assert_by_id: IngestMap::default(),
            operators: crate::pack_manifest::OperatorIndex::new(),
            lifecycle: crate::PackLifecycleTracker::new(),
            meta_leases_trusted: true,
            enrichment_cache_keys: std::collections::HashSet::default(),
            log_ids: rustc_hash::FxHashSet::default(),
        };
        rt.hydrate_from_log();
        Ok(rt)
    }
}
