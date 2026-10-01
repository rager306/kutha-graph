use crate::allow::load_allowed_names;
use crate::csr::{CsrLease, TypedCsrLease};
use crate::fold::GraphFold;
use crate::log::EventLog;
use crate::materializer::{CsrMaterializer, Materializer};
use crate::receipt::{digest_to_hex, QuantumReceipt};
use crate::snapshot::Snapshot;
use kutha_common::{Event, EventId, Op, TermDictionary, TermId};
use sha2::{Digest, Sha256};
use std::collections::HashSet;
use std::fmt;

#[derive(Debug)]
pub enum RuntimeError {
    ReplayDivergence {
        expected: [u8; 32],
        actual: [u8; 32],
    },
    ProvenanceMismatch {
        expected: [u8; 32],
        actual: [u8; 32],
    },
    UnknownFact {
        event_id: EventId,
    },
    UnknownRelation {
        name: String,
    },
    UnknownClaim {
        claim: EventId,
    },
    BrokenLineage {
        caused_by: EventId,
    },
    IntervalPatchRejected {
        event_id: EventId,
    },
    DuplicateResume {
        resume_of: String,
    },
    AdmissionDenied {
        justification_id: String,
        reason: &'static str,
    },
    /// Public `emit` of fold-noop meta ops is fail-closed (LOG-01 / T-12-02).
    MetaOpRejected,
    /// Same non-empty delivery key with a different Assert payload (ING-01 / T-14-02).
    DeliveryKeyConflict {
        key: String,
    },
}

impl fmt::Display for RuntimeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RuntimeError::ReplayDivergence { .. } => write!(f, "ReplayDivergenceError"),
            RuntimeError::ProvenanceMismatch { .. } => write!(f, "ProvenanceMismatchError"),
            RuntimeError::UnknownFact { event_id } => write!(f, "unknown fact {event_id}"),
            RuntimeError::UnknownRelation { name } => {
                write!(f, "unknown relation {name} (not in allowlist)")
            }
            RuntimeError::UnknownClaim { claim } => write!(f, "unknown claim {claim}"),
            RuntimeError::BrokenLineage { caused_by } => {
                write!(f, "broken lineage caused_by={caused_by}")
            }
            RuntimeError::IntervalPatchRejected { event_id } => {
                write!(f, "interval patch rejected for fact {event_id}")
            }
            RuntimeError::DuplicateResume { resume_of } => {
                write!(f, "duplicate resume for quantum {resume_of}")
            }
            RuntimeError::AdmissionDenied { .. } => write!(f, "AdmissionDeniedError"),
            RuntimeError::MetaOpRejected => write!(f, "MetaOpRejectedError"),
            RuntimeError::DeliveryKeyConflict { key } => {
                write!(f, "delivery key conflict {key}")
            }
        }
    }
}

pub use kutha_common::OutcomeDisposition;

/// Authoritative justification / admission cite (D-F2).
/// SoT is `Op::JustificationCite` on the log; `justifications.jsonl` is a lease (LOG-02 / D-02).
#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct Justification {
    pub justification_id: String,
    pub target_claim: EventId,
    pub source_claim_ids: Vec<EventId>,
    pub source_event_ids: Vec<EventId>,
    pub rule_version: String,
    pub tt: u64,
    pub vt: u64,
}

/// Thin conflict evidence at a cut (D-F3). Report only — no winner.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ConflictReport {
    pub positive_supports: Vec<u64>,
    pub negative_supports: Vec<u64>,
}

/// Authoritative row for `quantum_outcomes.jsonl` (D-O1). Not a droppable lease.
#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct PersistedQuantumOutcome {
    pub quantum_id: String,
    pub disposition: OutcomeDisposition,
    pub aborted_on_budget: bool,
    pub events_in_quantum: usize,
    pub event_ids: Vec<EventId>,
    pub receipt_digest_hex: String,
    pub resume_of: Option<String>,
}

/// Map emit abort × committed count to Zero / Partial / Full (D-O2).
pub fn disposition(aborted: bool, events_in_quantum: usize) -> OutcomeDisposition {
    match (aborted, events_in_quantum) {
        (true, 0) => OutcomeDisposition::Zero,
        (true, _) => OutcomeDisposition::Partial,
        (false, _) => OutcomeDisposition::Full,
    }
}

fn op_relation(op: &Op) -> Option<TermId> {
    match op {
        Op::Assert { relation, .. } | Op::Behavior { relation, .. } => Some(*relation),
        Op::Retract { .. }
        | Op::Correct { .. }
        | Op::CorrectInterval { .. }
        | Op::Define { .. }
        | Op::QuantumOutcome { .. }
        | Op::JustificationCite { .. }
        | Op::AllowRelation { .. } => None,
    }
}

/// Fold-affecting ops only. `Define` and `QuantumOutcome` stay on the log as meta (LOG-01).
fn op_affects_fold(op: &Op) -> bool {
    !matches!(
        op,
        Op::Define { .. }
            | Op::QuantumOutcome { .. }
            | Op::JustificationCite { .. }
            | Op::AllowRelation { .. }
    )
}

impl std::error::Error for RuntimeError {}

pub(crate) fn cascade_limit() -> usize {
    crate::allow::apply_dotenv();
    std::env::var("KUTHA_MAX_CASCADE")
        .ok()
        .and_then(|s| s.parse().ok())
        .filter(|n: &usize| *n > 0)
        .unwrap_or(32)
}

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
    /// In-memory buffer of quantum outcomes. SoT is `Op::QuantumOutcome` on the log;
    /// `quantum_outcomes.jsonl` is a droppable lease after those Events exist (LOG-01 / D-02).
    outcomes: Vec<PersistedQuantumOutcome>,
    /// In-memory buffer of justification cites. SoT is `Op::JustificationCite` on the log.
    /// Open hydrates from those Events; it never infers cites from Assert/Behavior triples.
    justifications: Vec<Justification>,
}

impl Default for Runtime {
    fn default() -> Self {
        Self::new(cascade_limit())
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
            outcomes: Vec::new(),
            justifications: Vec::new(),
        }
    }

    /// Intern a term string. New strings append `Op::Define` to the live log (fold no-op).
    /// Bootstrap `knows`/`knownBy` from [`Runtime::new`] stay silent until persist synthesizes them.
    pub fn intern(&mut self, s: &str) -> TermId {
        if let Some(id) = self.dict.id(s) {
            return id;
        }
        let id = self.dict.intern(s);
        let event = Event::new(
            Op::Define {
                name: s.to_string(),
            },
            self.next_tt(),
        );
        self.fold.apply(&event);
        self.log.append(event);
        id
    }

    pub fn log(&self) -> &EventLog {
        &self.log
    }

    /// Count of fold-affecting events. Snapshot `log_offset` uses this so open-with-snapshot
    /// stays aligned with the graph stream (`Define` and `QuantumOutcome` excluded).
    pub fn graph_len(&self) -> usize {
        self.log.iter().filter(|e| op_affects_fold(&e.op)).count()
    }

    pub fn fold(&self) -> &GraphFold {
        &self.fold
    }

    pub fn dictionary(&self) -> &TermDictionary {
        &self.dict
    }

    /// Persisted quantum outcome rows buffered for `store::persist` (D-O1).
    pub fn outcome_records(&self) -> &[PersistedQuantumOutcome] {
        &self.outcomes
    }

    /// Attach rows loaded from the outcomes sidecar when the log has no `QuantumOutcome` Events.
    pub fn attach_outcomes(&mut self, rows: Vec<PersistedQuantumOutcome>) {
        self.outcomes = rows;
    }

    /// Rebuild outcome and justification rows from log Events (D-01). Sidecars are not consulted.
    pub(crate) fn hydrate_from_log(&mut self) {
        let mut outcomes = Vec::new();
        let mut justifications = Vec::new();
        for e in self.log.iter() {
            match &e.op {
                Op::QuantumOutcome {
                    quantum_id,
                    disposition,
                    aborted_on_budget,
                    events_in_quantum,
                    event_ids,
                    receipt_digest_hex,
                    resume_of,
                } => outcomes.push(PersistedQuantumOutcome {
                    quantum_id: quantum_id.clone(),
                    disposition: *disposition,
                    aborted_on_budget: *aborted_on_budget,
                    events_in_quantum: *events_in_quantum,
                    event_ids: event_ids.clone(),
                    receipt_digest_hex: receipt_digest_hex.clone(),
                    resume_of: resume_of.clone(),
                }),
                Op::JustificationCite {
                    justification_id,
                    target_claim,
                    source_claim_ids,
                    source_event_ids,
                    rule_version,
                    tt,
                    vt,
                } => justifications.push(Justification {
                    justification_id: justification_id.clone(),
                    target_claim: *target_claim,
                    source_claim_ids: source_claim_ids.clone(),
                    source_event_ids: source_event_ids.clone(),
                    rule_version: rule_version.clone(),
                    tt: *tt,
                    vt: *vt,
                }),
                _ => {}
            }
        }
        self.outcomes = outcomes;
        self.justifications = justifications;
        self.fold.rebuild_allow_entries(self.log.as_slice());
    }

    pub(crate) fn log_has_quantum_outcome(&self) -> bool {
        self.log
            .iter()
            .any(|e| matches!(e.op, Op::QuantumOutcome { .. }))
    }

    pub(crate) fn log_has_justification_cite(&self) -> bool {
        self.log
            .iter()
            .any(|e| matches!(e.op, Op::JustificationCite { .. }))
    }

    fn append_meta(&mut self, op: Op) {
        let event = Event::new(op, self.next_tt());
        self.fold.apply(&event);
        self.log.append(event);
    }

    fn outcome_op(row: &PersistedQuantumOutcome) -> Op {
        Op::QuantumOutcome {
            quantum_id: row.quantum_id.clone(),
            disposition: row.disposition,
            aborted_on_budget: row.aborted_on_budget,
            events_in_quantum: row.events_in_quantum,
            event_ids: row.event_ids.clone(),
            receipt_digest_hex: row.receipt_digest_hex.clone(),
            resume_of: row.resume_of.clone(),
        }
    }

    fn justification_op(row: &Justification) -> Op {
        Op::JustificationCite {
            justification_id: row.justification_id.clone(),
            target_claim: row.target_claim,
            source_claim_ids: row.source_claim_ids.clone(),
            source_event_ids: row.source_event_ids.clone(),
            rule_version: row.rule_version.clone(),
            tt: row.tt,
            vt: row.vt,
        }
    }

    /// Persisted justification rows buffered for `store::persist` (D-F2).
    pub fn justification_records(&self) -> &[Justification] {
        &self.justifications
    }

    /// Attach rows loaded from the justifications sidecar when the log has no `JustificationCite` Events.
    pub fn attach_justifications(&mut self, rows: Vec<Justification>) {
        self.justifications = rows;
    }

    /// Append one justification cite. Does not run from `emit`. Returns the minted id.
    pub fn record_justification(
        &mut self,
        target_claim: EventId,
        source_claim_ids: Vec<EventId>,
        source_event_ids: Vec<EventId>,
        rule_version: impl Into<String>,
        tt: u64,
        vt: u64,
    ) -> String {
        let rule_version = rule_version.into();
        let mut justification_id = format!("j:{target_claim}:{tt}:{vt}");
        let mut n = 0u32;
        while self
            .justifications
            .iter()
            .any(|j| j.justification_id == justification_id)
        {
            n += 1;
            justification_id = format!("j:{target_claim}:{tt}:{vt}:{n}");
        }
        self.justifications.push(Justification {
            justification_id: justification_id.clone(),
            target_claim,
            source_claim_ids,
            source_event_ids,
            rule_version,
            tt,
            vt,
        });
        let row = self.justifications.last().expect("just pushed").clone();
        self.append_meta(Self::justification_op(&row));
        justification_id
    }

    /// Fail-closed admission for a persisted cite. Locked reasons: unknown_justification,
    /// stale_support, ineligible, rule_version.
    pub fn check_admission(&self, justification_id: &str) -> Result<(), RuntimeError> {
        let row = self
            .justifications
            .iter()
            .find(|j| j.justification_id == justification_id)
            .ok_or(RuntimeError::AdmissionDenied {
                justification_id: justification_id.into(),
                reason: "unknown_justification",
            })?;
        // Cited minting EventIds must still be live on the current picture at the row's VT.
        // Historical row.tt records the claimed cut; using it alone would never
        // stale a cite after CorrectInterval/Retract (FIX-02).
        for event_id in &row.source_event_ids {
            let live = self
                .fold
                .facts()
                .iter()
                .any(|f| f.event_id == *event_id && f.is_live_at(u64::MAX, row.vt));
            if !live {
                return Err(RuntimeError::AdmissionDenied {
                    justification_id: row.justification_id.clone(),
                    reason: "stale_support",
                });
            }
        }
        if !self.derivation_eligible_at(row.target_claim, row.tt, row.vt) {
            return Err(RuntimeError::AdmissionDenied {
                justification_id: row.justification_id.clone(),
                reason: "ineligible",
            });
        }
        let pin = match self.log.iter().find(|e| e.id == row.target_claim) {
            Some(e) => match &e.op {
                Op::Behavior { rule_version, .. } => rule_version.as_str(),
                _ => "",
            },
            None => "",
        };
        if pin != row.rule_version.as_str() {
            return Err(RuntimeError::AdmissionDenied {
                justification_id: row.justification_id.clone(),
                reason: "rule_version",
            });
        }
        Ok(())
    }

    /// Partition live supports of `claim` by stored `Fact.polarity` (ING-03 / D-03).
    /// `None` polarity is in neither bucket. Caller TermIds are not the polarity source.
    pub fn conflict_report_at(&self, claim: EventId, tt: u64, vt: u64) -> ConflictReport {
        let mut positive_supports = Vec::new();
        let mut negative_supports = Vec::new();
        for f in self.fold.live_supports(claim, tt, vt) {
            match f.polarity {
                Some(kutha_common::SupportPolarity::Positive) => positive_supports.push(f.seq),
                Some(kutha_common::SupportPolarity::Negative) => negative_supports.push(f.seq),
                None => {}
            }
        }
        ConflictReport {
            positive_supports,
            negative_supports,
        }
    }

    /// Explicit resume citation for a prior quantum (D-O3). Open never calls this.
    pub fn record_resume(&mut self, quantum_id: &str) -> Result<(), RuntimeError> {
        if self
            .outcomes
            .iter()
            .any(|r| r.resume_of.as_deref() == Some(quantum_id))
        {
            return Err(RuntimeError::DuplicateResume {
                resume_of: quantum_id.to_string(),
            });
        }
        self.outcomes.push(PersistedQuantumOutcome {
            quantum_id: format!("resume:{quantum_id}"),
            disposition: OutcomeDisposition::Resume,
            aborted_on_budget: false,
            events_in_quantum: 0,
            event_ids: Vec::new(),
            receipt_digest_hex: String::new(),
            resume_of: Some(quantum_id.to_string()),
        });
        let row = self.outcomes.last().expect("just pushed").clone();
        self.append_meta(Self::outcome_op(&row));
        Ok(())
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
            allowed: load_allowed_names(),
            outcomes: Vec::new(),
            justifications: Vec::new(),
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
        let mut rt = Self {
            log: EventLog::from_events(all_events),
            fold,
            dict,
            next_tt,
            max_cascade,
            knows,
            known_by,
            allowed: load_allowed_names(),
            outcomes: Vec::new(),
            justifications: Vec::new(),
        };
        rt.hydrate_from_log();
        Ok(rt)
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
        let mut rt = Self {
            log: prefix,
            fold,
            dict: self.dict.clone(),
            next_tt,
            max_cascade: self.max_cascade,
            knows: self.knows,
            known_by: self.known_by,
            allowed: self.allowed.clone(),
            outcomes: Vec::new(),
            justifications: Vec::new(),
        };
        rt.hydrate_from_log();
        rt
    }

    /// CSR lease at an explicit cut. Callers must name valid-time (no silent “now”).
    /// Built through [`CsrMaterializer`] then unloaded so Runtime does not keep a mounted view.
    pub fn csr_lease_at(&self, tt: u64, vt: u64) -> CsrLease {
        let mut materializer = CsrMaterializer::default();
        materializer.build(&self.fold, self.dict.len(), tt, vt, self.log.len());
        let lease = materializer
            .lease()
            .expect("CsrMaterializer mounts a lease on build")
            .clone();
        materializer.unload();
        lease
    }

    /// Typed CSR lease at an explicit cut (labels + support multiplicity). Droppable; not SoT.
    pub fn typed_csr_lease_at(&self, tt: u64, vt: u64) -> TypedCsrLease {
        TypedCsrLease::from_fold(&self.fold, tt, vt, self.dict.len())
    }

    fn next_tt(&mut self) -> u64 {
        let t = self.next_tt;
        self.next_tt += 1;
        t
    }

    fn admit(&self, op: &Op) -> Result<(), RuntimeError> {
        if let Op::AllowRelation { name, .. } = op {
            if name.is_empty() {
                return Err(RuntimeError::UnknownRelation {
                    name: String::new(),
                });
            }
            return Ok(());
        }
        let Some(rel) = op_relation(op) else {
            return Ok(());
        };
        let name = self.dict.lookup(rel).unwrap_or("").to_string();
        let vt = match op {
            Op::Assert { valid_from, .. } | Op::Behavior { valid_from, .. } => *valid_from,
            _ => 0,
        };
        let tt = self.next_tt.saturating_sub(1);
        if !name.is_empty() && self.fold.relation_allowed_at(&name, tt, vt) {
            return Ok(());
        }
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

    fn admit_claim(&self, op: &Op) -> Result<(), RuntimeError> {
        let Op::Assert {
            claim: Some(claim), ..
        } = op
        else {
            return Ok(());
        };
        if self.fold.facts().iter().any(|f| f.claim_id == *claim) {
            return Ok(());
        }
        Err(RuntimeError::UnknownClaim { claim: *claim })
    }

    fn delivery_key_active(key: &Option<String>) -> Option<&str> {
        match key {
            Some(s) if !s.is_empty() => Some(s.as_str()),
            _ => None,
        }
    }

    fn assert_payload_matches(left: &Op, right: &Op) -> bool {
        match (left, right) {
            (
                Op::Assert {
                    subject: s1,
                    relation: r1,
                    object: o1,
                    valid_from: vf1,
                    valid_to: vt1,
                    claim: c1,
                    polarity: p1,
                    delivery_key: d1,
                },
                Op::Assert {
                    subject: s2,
                    relation: r2,
                    object: o2,
                    valid_from: vf2,
                    valid_to: vt2,
                    claim: c2,
                    polarity: p2,
                    delivery_key: d2,
                },
            ) => {
                s1 == s2
                    && r1 == r2
                    && o1 == o2
                    && vf1 == vf2
                    && vt1 == vt2
                    && c1 == c2
                    && p1 == p2
                    && d1 == d2
            }
            _ => false,
        }
    }

    /// Identical keyed Assert returns the original EventId without appending (ING-01 / D-01).
    fn delivery_key_retry(&self, op: &Op) -> Result<Option<QuantumOutcome>, RuntimeError> {
        let Op::Assert { delivery_key, .. } = op else {
            return Ok(None);
        };
        let Some(key) = Self::delivery_key_active(delivery_key) else {
            return Ok(None);
        };
        for e in self.log.iter() {
            let Op::Assert {
                delivery_key: existing,
                ..
            } = &e.op
            else {
                continue;
            };
            let Some(found) = Self::delivery_key_active(existing) else {
                continue;
            };
            if found != key {
                continue;
            }
            if Self::assert_payload_matches(&e.op, op) {
                let receipt = QuantumReceipt::from_events(vec![e.id], &[e.digest_bytes()], false);
                return Ok(Some(QuantumOutcome {
                    receipt,
                    events_in_quantum: 0,
                }));
            }
            return Err(RuntimeError::DeliveryKeyConflict {
                key: key.to_string(),
            });
        }
        Ok(None)
    }

    /// Admit a user op, append, fold, cascade inverse-`knows` until idle or budget.
    pub fn emit(&mut self, op: Op) -> Result<QuantumOutcome, RuntimeError> {
        if matches!(op, Op::QuantumOutcome { .. } | Op::JustificationCite { .. }) {
            return Err(RuntimeError::MetaOpRejected);
        }
        if let Op::Retract { event_id } = &op {
            let known_fact = self.fold.facts().iter().any(|f| f.event_id == *event_id);
            if !known_fact && !self.fold.has_allow_entry(*event_id) {
                return Err(RuntimeError::UnknownFact {
                    event_id: *event_id,
                });
            }
        } else if let Op::Correct { event_id, .. } | Op::CorrectInterval { event_id, .. } = &op {
            if !self.fold.facts().iter().any(|f| f.event_id == *event_id) {
                return Err(RuntimeError::UnknownFact {
                    event_id: *event_id,
                });
            }
        }
        if let Op::CorrectInterval {
            event_id,
            patch_from,
            patch_to,
            ..
        } = &op
        {
            let fact = self
                .fold
                .facts()
                .iter()
                .find(|f| f.event_id == *event_id && f.invalidated_at.is_none())
                .or_else(|| self.fold.facts().iter().find(|f| f.event_id == *event_id))
                .expect("UnknownFact gate already ran");
            if fact.invalidated_at.is_some()
                || patch_to.is_some_and(|t| *patch_from >= t)
                || crate::fold::vt_intersect(fact.valid_from, fact.valid_to, *patch_from, *patch_to)
                    .is_none()
            {
                return Err(RuntimeError::IntervalPatchRejected {
                    event_id: *event_id,
                });
            }
        }
        self.admit(&op)?;
        if matches!(&op, Op::AllowRelation { .. }) {
            let event = Event::new(op, self.next_tt());
            let id = event.id;
            let digest = event.digest_bytes();
            self.fold.apply(&event);
            self.log.append(event);
            let receipt = QuantumReceipt::from_events(vec![id], &[digest], false);
            return Ok(QuantumOutcome {
                receipt,
                events_in_quantum: 1,
            });
        }
        self.admit_claim(&op)?;
        if let Some(retry) = self.delivery_key_retry(&op)? {
            return Ok(retry);
        }
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

        let receipt = QuantumReceipt::from_events(ids, &digests, aborted);
        let events_in_quantum = used;
        let receipt_digest_hex = digest_to_hex(&receipt.digest);
        let disp = disposition(aborted, events_in_quantum);
        self.outcomes.push(PersistedQuantumOutcome {
            quantum_id: receipt_digest_hex.clone(),
            disposition: disp,
            aborted_on_budget: aborted,
            events_in_quantum,
            event_ids: receipt.event_ids.clone(),
            receipt_digest_hex,
            resume_of: None,
        });
        let row = self.outcomes.last().expect("just pushed").clone();
        self.append_meta(Self::outcome_op(&row));

        Ok(QuantumOutcome {
            receipt,
            events_in_quantum,
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
                ..
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
                        rule_version: String::new(),
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
        let mut seen = HashSet::new();
        for e in self.log.iter() {
            if let Op::Behavior { caused_by, .. } = &e.op {
                if !seen.contains(caused_by) {
                    return Err(RuntimeError::BrokenLineage {
                        caused_by: *caused_by,
                    });
                }
            }
            seen.insert(e.id);
        }
        Ok(rebuilt)
    }

    /// Lineage digest over Behavior plus log-native outcome/cite Events (ADR-060 obligation 2, LOG-03).
    /// Behavior mix is `(event.id, caused_by, name, rule_version)` with length-prefixed strings.
    /// QuantumOutcome and JustificationCite mix `Event::digest_bytes` in the same log walk.
    /// Does not include Fact triples and is not called from [`Self::replay_check`].
    pub fn provenance_fingerprint(&self) -> [u8; 32] {
        let mut h = Sha256::new();
        h.update(b"kutha-prov-log-native");
        for e in self.log.iter() {
            match &e.op {
                Op::Behavior {
                    name,
                    caused_by,
                    rule_version,
                    ..
                } => {
                    h.update(e.id.as_bytes());
                    h.update(caused_by.as_bytes());
                    h.update((name.len() as u64).to_le_bytes());
                    h.update(name.as_bytes());
                    h.update((rule_version.len() as u64).to_le_bytes());
                    h.update(rule_version.as_bytes());
                }
                Op::QuantumOutcome { .. }
                | Op::JustificationCite { .. }
                | Op::AllowRelation { .. } => {
                    h.update(e.digest_bytes());
                }
                _ => {}
            }
        }
        h.finalize().into()
    }

    /// Compare a previously captured lineage digest. State replay stays [`Self::replay_check`].
    pub fn provenance_check(&self, expected: [u8; 32]) -> Result<(), RuntimeError> {
        let actual = self.provenance_fingerprint();
        if expected != actual {
            return Err(RuntimeError::ProvenanceMismatch { expected, actual });
        }
        Ok(())
    }

    /// Thin P→Q oracle (M011 S03): a Behavior-derived claim is eligible at a cut
    /// iff the derived fact is still live and its premise claim still has a live support.
    /// `caused_by` may name any prior Assert/Behavior event; eligibility keys on that
    /// event's claim identity, so withdrawing one of several supports does not drop Q.
    pub fn derivation_eligible_at(&self, derived: EventId, tt: u64, vt: u64) -> bool {
        if !self.fold.claim_supported_at(derived, tt, vt) {
            return false;
        }
        let Some(event) = self.log.iter().find(|e| e.id == derived) else {
            return false;
        };
        let Op::Behavior { caused_by, .. } = &event.op else {
            return false;
        };
        let Some(cause) = self.log.iter().find(|e| e.id == *caused_by) else {
            return false;
        };
        let premise = match &cause.op {
            Op::Assert { claim, .. } => claim.unwrap_or(cause.id),
            Op::Behavior { .. } => cause.id,
            Op::Retract { .. }
            | Op::Correct { .. }
            | Op::CorrectInterval { .. }
            | Op::Define { .. }
            | Op::QuantumOutcome { .. }
            | Op::JustificationCite { .. }
            | Op::AllowRelation { .. } => return false,
        };
        self.fold.claim_supported_at(premise, tt, vt)
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

            claim: None,
            delivery_key: None,
            polarity: None,
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

            claim: None,
            delivery_key: None,
            polarity: None,
        })
        .unwrap();
        let minting = rt.fold().facts()[0].event_id;
        rt.emit(Op::Retract { event_id: minting }).unwrap();
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

                claim: None,
                delivery_key: None,
                polarity: None,
            })
            .unwrap();
        assert_eq!(q.events_in_quantum, 2);
        assert!(!q.receipt.aborted_on_budget);
        assert_eq!(rt.graph_len(), 2);
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

                claim: None,
                delivery_key: None,
                polarity: None,
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

            claim: None,
            delivery_key: None,
            polarity: None,
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

                claim: None,
                delivery_key: None,
                polarity: None,
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

                claim: None,
                delivery_key: None,
                polarity: None,
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

            claim: None,
            delivery_key: None,
            polarity: None,
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

            claim: None,
            delivery_key: None,
            polarity: None,
        })
        .unwrap();
        // Fixture facts use valid_from=0; name that cut (no silent “now”).
        let csr = rt.csr_lease_at(u64::MAX, 0);
        assert_eq!(csr.neighbors(a), &[b]);
        assert_eq!(csr.seek(a, b), Some(b));
        assert_eq!(csr.seek(a, b.saturating_add(1)), None);
        drop(csr);
        let csr2 = rt.csr_lease_at(u64::MAX, 0);
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

            claim: None,
            delivery_key: None,
            polarity: None,
        })
        .unwrap();
        let cut = rt.log().len();
        rt.emit(Op::Assert {
            subject: n1,
            relation: rel,
            object: n2,
            valid_from: 0,
            valid_to: None,

            claim: None,
            delivery_key: None,
            polarity: None,
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
