use crate::allow::load_allowed_names;
use crate::csr::{CsrLease, TypedCsrLease};
use crate::fold::{FoldIndexMode, GraphFold};
use crate::log::EventLog;
use crate::materializer::{CsrMaterializer, Materializer};
use crate::receipt::QuantumReceipt;
use kutha_common::{
    rule_definition_hash, Event, EventId, Op, TermDictionary, TermId, TransactionTime, ValidTime,
    WallTime, KUTHA_RESERVED_RELS,
};
use rustc_hash::FxHashMap;
use std::fmt;
use std::path::Path;
use std::sync::atomic::{AtomicU64, Ordering};

type IngestMap<K, V> = FxHashMap<K, V>;

/// Integration-test ingest examine counter (not fingerprint / JSON).
#[derive(Debug, Default)]
pub(crate) struct IngestExamine(AtomicU64);

impl Clone for IngestExamine {
    fn clone(&self) -> Self {
        Self(AtomicU64::new(self.0.load(Ordering::Relaxed)))
    }
}

impl IngestExamine {
    pub(crate) fn reset(&self) {
        self.0.store(0, Ordering::Relaxed);
    }

    pub(crate) fn get(&self) -> u64 {
        self.0.load(Ordering::Relaxed)
    }

    pub(crate) fn bump(&self) {
        self.0.fetch_add(1, Ordering::Relaxed);
    }
}

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
    /// User Behavior pin is not a live registry hash (RULE-02 / RULE-03).
    UnknownRuleVersion {
        pin: String,
    },
    /// Missing or empty live policy pin (ADM-02).
    UnknownPolicyVersion {
        pin: String,
    },
    /// Missing cite or missing live admission for an Action bind (ACT-01 / T-20-02).
    UnknownAdmission {
        justification_id: String,
    },
    /// Persist/open IO (facade wrap of `store`).
    StoreIo {
        message: String,
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
            RuntimeError::UnknownRuleVersion { pin } => {
                write!(f, "unknown rule version {pin}")
            }
            RuntimeError::UnknownPolicyVersion { pin } => {
                write!(f, "unknown policy version {pin}")
            }
            RuntimeError::UnknownAdmission { justification_id } => {
                write!(f, "unknown admission {justification_id}")
            }
            RuntimeError::StoreIo { message } => write!(f, "store io {message}"),
        }
    }
}

pub use kutha_common::OutcomeDisposition;

/// Thin Action bind (ACT-01). Reconstructed from `kutha:action` + `JustificationCite`.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ActionRecord {
    pub justification_id: String,
    pub target_claim: EventId,
    pub source_claim_ids: Vec<EventId>,
    pub source_event_ids: Vec<EventId>,
    pub rule_version: String,
    pub admitted: bool,
    pub policy_version: String,
}

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

pub(crate) fn op_relation(op: &Op) -> Option<TermId> {
    match op {
        Op::Assert { relation, .. } | Op::Behavior { relation, .. } => Some(*relation),
        Op::Retract { .. }
        | Op::Correct { .. }
        | Op::CorrectInterval { .. }
        | Op::Define { .. }
        | Op::QuantumOutcome { .. }
        | Op::JustificationCite { .. }
        | Op::StoreBlob { .. }
        | Op::TtAnchor { .. } => None,
    }
}

/// Fold-affecting ops only. `Define`, `QuantumOutcome`, `JustificationCite`, `StoreBlob`,
/// and `TtAnchor` stay on the log as meta (LOG-01 / D-02). Dictionary `Assert`s are fold facts.
pub(crate) fn op_affects_fold(op: &Op) -> bool {
    !matches!(
        op,
        Op::Define { .. }
            | Op::QuantumOutcome { .. }
            | Op::JustificationCite { .. }
            | Op::StoreBlob { .. }
            | Op::TtAnchor { .. }
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
    pub(crate) log: EventLog,
    pub(crate) fold: GraphFold,
    pub(crate) dict: TermDictionary,
    pub(crate) next_tt: u64,
    pub max_cascade: usize,
    pub(crate) knows: TermId,
    pub(crate) known_by: TermId,
    /// In-memory buffer of quantum outcomes. SoT is `Op::QuantumOutcome` on the log;
    /// `quantum_outcomes.jsonl` is a droppable lease after those Events exist (LOG-01 / D-02).
    pub(crate) outcomes: Vec<PersistedQuantumOutcome>,
    /// Log position by EventId (lease; rebuilt on hydrate / fork).
    pub(crate) by_event_id: IngestMap<EventId, usize>,
    /// Delivery key → EventId for identical Assert retry (lease).
    pub(crate) by_delivery_key: IngestMap<String, EventId>,
    /// Rows visited on ingest lookup paths (tests; not SoT).
    pub(crate) ingest_examine: IngestExamine,
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
        let mut rt = Self {
            log: EventLog::default(),
            fold: GraphFold::with_index_mode(FoldIndexMode::from_env()),
            dict,
            next_tt: 0,
            max_cascade,
            knows,
            known_by,
            outcomes: Vec::new(),
            by_event_id: IngestMap::default(),
            by_delivery_key: IngestMap::default(),
            ingest_examine: IngestExamine::default(),
        };
        for name in KUTHA_RESERVED_RELS {
            rt.intern_silent(name);
        }
        rt.intern_silent("true");
        rt.intern_silent("false");
        rt.fold.bind_reserved(&rt.dict);
        // Empty-log genesis: silent intern of YAML names + Assert kutha:allows valid_from 0 (D-01).
        let mut names: Vec<String> = load_allowed_names().into_iter().collect();
        names.sort();
        for name in names {
            rt.emit_allow(name, 0, None).expect("genesis kutha:allows");
        }
        rt
    }

    /// Construct with an explicit fold index policy (Eager = accept write-cost, Lazy = thin write).
    pub fn with_index_mode(max_cascade: usize, index_mode: FoldIndexMode) -> Self {
        let mut rt = Self::new(max_cascade);
        rt.set_index_mode(index_mode);
        rt
    }

    pub fn index_mode(&self) -> FoldIndexMode {
        self.fold.index_mode()
    }

    pub fn set_index_mode(&mut self, mode: FoldIndexMode) {
        self.fold.set_index_mode(mode);
    }

    /// Rebuild fold secondary leases after Lazy ingest (optional before hot reads).
    pub fn ensure_fold_indexes(&mut self) {
        self.fold.ensure_hot_maps();
    }

    /// Intern a term string. New strings append `Op::Define` to the live log (fold no-op).
    /// Bootstrap `knows`/`knownBy` and genesis reserved/YAML names stay silent until persist synthesizes them.
    /// Names with prefix `kutha:` are marked as dictionary relations (D-01 / D-05).
    pub fn intern(&mut self, s: &str) -> TermId {
        if let Some(id) = self.dict.id(s) {
            return id;
        }
        let id = self.intern_silent(s);
        let event = Event::new(
            Op::Define {
                name: s.to_string(),
            },
            self.next_tt(),
        );
        self.fold.apply(&event);
        self.append_indexed(event);
        id
    }

    pub(crate) fn intern_silent(&mut self, s: &str) -> TermId {
        if let Some(id) = self.dict.id(s) {
            if s.starts_with("kutha:") {
                self.fold.bind_reserved(&self.dict);
            }
            return id;
        }
        let id = self.dict.intern(s);
        if s.starts_with("kutha:") {
            self.fold.bind_reserved(&self.dict);
        }
        id
    }

    pub fn log(&self) -> &EventLog {
        &self.log
    }

    /// Reset ingest examine counter (integration tests; not fingerprint input).
    pub fn reset_ingest_examine_count(&self) {
        self.ingest_examine.reset();
    }

    /// Log/fact rows visited on ingest lookup paths since reset.
    pub fn ingest_examine_count(&self) -> u64 {
        self.ingest_examine.get()
    }

    /// Pre-size ingest index leases for an expected bulk load (bench / importer).
    /// Does not change fold semantics; avoids rehash on the write path.
    pub fn reserve_ingest_indexes(&mut self, expected_events: usize, expected_facts: usize) {
        if expected_events > 0 {
            self.by_event_id.reserve(expected_events);
            self.by_delivery_key.reserve(expected_events);
        }
        self.fold.reserve_ingest_indexes(expected_facts);
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
    /// CSR lease at an explicit cut. Callers must name valid-time (no silent “now”).
    /// Built through [`CsrMaterializer`] then unloaded so Runtime does not keep a mounted view.
    pub fn csr_lease_at(
        &self,
        tt: impl Into<TransactionTime>,
        vt: impl Into<ValidTime>,
    ) -> CsrLease {
        let tt = tt.into().as_u64();
        let vt = vt.into().as_u64();
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
    pub fn typed_csr_lease_at(
        &self,
        tt: impl Into<TransactionTime>,
        vt: impl Into<ValidTime>,
    ) -> TypedCsrLease {
        let tt = tt.into().as_u64();
        let vt = vt.into().as_u64();
        TypedCsrLease::from_fold(&self.fold, tt, vt, self.dict.len())
    }

    /// Internable Assert (PIPE-03). Goes through the write pipeline.
    pub fn assert(
        &mut self,
        subject: impl AsRef<str>,
        relation: impl AsRef<str>,
        object: impl AsRef<str>,
        valid_from: u64,
        valid_to: Option<u64>,
    ) -> Result<QuantumOutcome, RuntimeError> {
        let subject = self.intern(subject.as_ref());
        let relation = self.intern(relation.as_ref());
        let object = self.intern(object.as_ref());
        self.pipeline(Op::Assert {
            subject,
            relation,
            object,
            valid_from,
            valid_to,
            claim: None,
            delivery_key: None,
            polarity: None,
        })
    }

    /// Retract the latest live product fact for `subject` starting at `valid_from`.
    pub fn retract(
        &mut self,
        subject: impl AsRef<str>,
        valid_from: u64,
    ) -> Result<QuantumOutcome, RuntimeError> {
        let subject = self.intern(subject.as_ref());
        let event_id = self
            .fold
            .facts()
            .iter()
            .rev()
            .find(|f| {
                f.subject == subject
                    && f.valid_from == valid_from
                    && f.invalidated_at.is_none()
                    && !self.fold.is_dictionary_rel(f.relation)
            })
            .map(|f| f.event_id)
            .ok_or(RuntimeError::UnknownFact {
                event_id: EventId::nil(),
            })?;
        self.pipeline(Op::Retract { event_id })
    }

    /// Correct the latest live fact for `subject`/`relation` starting at `valid_from`.
    pub fn correct(
        &mut self,
        subject: impl AsRef<str>,
        relation: impl AsRef<str>,
        object: impl AsRef<str>,
        valid_from: u64,
    ) -> Result<QuantumOutcome, RuntimeError> {
        let subject = self.intern(subject.as_ref());
        let relation = self.intern(relation.as_ref());
        let object = self.intern(object.as_ref());
        let fact = self
            .fold
            .facts()
            .iter()
            .rev()
            .find(|f| {
                f.subject == subject
                    && f.relation == relation
                    && f.valid_from == valid_from
                    && f.invalidated_at.is_none()
            })
            .ok_or(RuntimeError::UnknownFact {
                event_id: EventId::nil(),
            })?;
        let event_id = fact.event_id;
        let valid_to = fact.valid_to;
        self.pipeline(Op::Correct {
            event_id,
            object,
            valid_from,
            valid_to,
        })
    }

    /// Behavior-derived follow-on pinned to `rule_definition_hash(name)` (PIPE-03).
    pub fn derive(
        &mut self,
        name: impl AsRef<str>,
        subject: impl AsRef<str>,
        valid_from: u64,
    ) -> Result<QuantumOutcome, RuntimeError> {
        let name = name.as_ref();
        let pin = rule_definition_hash(name);
        if !self.fold.rule_hash_live_at(&pin, u64::MAX, valid_from) {
            self.emit_rule(name, valid_from, None)?;
        }
        let subject = self.intern(subject.as_ref());
        let premise = self
            .fold
            .facts()
            .iter()
            .rev()
            .find(|f| f.subject == subject && !self.fold.is_dictionary_rel(f.relation))
            .ok_or(RuntimeError::UnknownFact {
                event_id: EventId::nil(),
            })?;
        let caused_by = premise.event_id;
        let relation = premise.relation;
        let object = premise.object();
        self.pipeline(Op::Behavior {
            name: name.to_string(),
            caused_by,
            rule_version: pin,
            subject,
            relation,
            object,
            valid_from,
            valid_to: None,
        })
    }

    pub fn as_of(&self, vt: impl Into<ValidTime>) -> Vec<(TermId, TermId, TermId)> {
        self.fold.as_of(vt)
    }

    /// Product triples as known on a wall date.
    ///
    /// Greatest `Op::TtAnchor` with `wall` at or before `wall`, then
    /// [`Self::live_at`] at that event's transaction time and `vt = u64::MAX`.
    /// `None` means no anchor at or before the date.
    pub fn as_known_on(&self, wall: impl Into<WallTime>) -> Option<Vec<(TermId, TermId, TermId)>> {
        let wall = wall.into().as_u64();
        let tt = *self.fold.tt_anchors.range(..=wall).next_back()?.1;
        Some(self.live_at(tt, u64::MAX))
    }

    pub fn live_at(
        &self,
        tt: impl Into<TransactionTime>,
        vt: impl Into<ValidTime>,
    ) -> Vec<(TermId, TermId, TermId)> {
        self.fold.live_at(tt, vt)
    }

    pub fn persist(&self, dir: impl AsRef<Path>) -> Result<(), RuntimeError> {
        crate::store::persist(self, dir.as_ref()).map_err(|e| RuntimeError::StoreIo {
            message: e.to_string(),
        })
    }

    pub fn open(dir: impl AsRef<Path>) -> Result<Self, RuntimeError> {
        crate::store::open(dir.as_ref()).map_err(|e| RuntimeError::StoreIo {
            message: e.to_string(),
        })
    }
}

#[cfg(test)]
#[path = "quantum_invariants.rs"]
mod tests;
