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
use rustc_hash::{FxHashMap, FxHashSet};
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
    /// Duplicate or backwards `TtAnchor` wall (ANK-ADM-01).
    TtAnchorRejected {
        wall: u64,
        max_wall: Option<u64>,
    },
    /// Horn-pinned Behavior cannot be re-derived at its ingested_at (RULE-04).
    ExecutionReplayFailed {
        event_id: EventId,
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
            RuntimeError::TtAnchorRejected { wall, max_wall } => match max_wall {
                Some(max) => write!(f, "tt_anchor rejected wall {wall} max {max}"),
                None => write!(f, "tt_anchor rejected wall {wall}"),
            },
            RuntimeError::ExecutionReplayFailed { event_id } => {
                write!(f, "execution replay failed for {event_id}")
            }
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

/// One product fact in a successful [`NormView`] (M025 S03 / G-B01).
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct NormTriple {
    pub subject: TermId,
    pub relation: TermId,
    pub object: TermId,
    pub claim_id: EventId,
    pub event_id: EventId,
    /// Always true in [`NormView::Ok`]; eligibility was the admission gate.
    pub eligible: bool,
}

/// Legal answer envelope: force (VT) ∩ knowledge (wall) ∩ eligibility (R-22).
///
/// Does not change [`Runtime::as_of`] or ASOF-ELIG-01. Typed non-success per
/// TR-06 / G-B04 / G-C10 (no Incomplete in this thin slice).
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum NormView {
    /// Eligible live product facts at the force∩knowledge cut.
    Ok(Vec<NormTriple>),
    /// Both polarities live for a claim intersecting the cut.
    Conflict {
        claim: EventId,
        report: ConflictReport,
    },
    /// Live facts exist but none pass eligibility (e.g. retracted-premise derived).
    Abstain,
    /// Anchor present; force∩knowledge has no product facts.
    Unknown,
    /// No `TtAnchor` at or before the wall.
    MissingAnchor,
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
    /// `(relation, delivery_key)` → EventId for identical Assert retry (lease).
    /// Namespaced by relation so policy / admission / action / product pins that
    /// share a key string do not collide (M038 / CE #29).
    pub(crate) by_delivery_key: IngestMap<(TermId, String), EventId>,
    /// Rows visited on ingest lookup paths (tests; not SoT).
    pub(crate) ingest_examine: IngestExamine,
    /// Log rows visited in `retract_derived` (tests; not SoT).
    pub(crate) retract_examine: IngestExamine,
    /// Cause buckets visited in `unindex_behavior` (tests; not SoT).
    pub(crate) unindex_cause_examine: IngestExamine,
    /// Premise-rel buckets visited in `unindex_behavior` (tests; not SoT).
    pub(crate) unindex_premise_examine: IngestExamine,
    pub(crate) unindex_live_examine: IngestExamine,
    /// Live Behavior ids keyed by `caused_by` (droppable; rebuilt from the log).
    pub(crate) derived_by_cause: IngestMap<EventId, Vec<EventId>>,
    /// Behavior id → `caused_by` for O(1) unindex of `derived_by_cause` (M104).
    pub(crate) cause_of_behavior: IngestMap<EventId, EventId>,
    /// Currently live Horn Behavior event ids (droppable; rebuilt from the log).
    pub(crate) live_behaviors: FxHashSet<EventId>,
    /// Live Behavior ids keyed by Horn body premise relation (droppable; rebuilt
    /// from the log). Second-phase retract/correct walks this index instead of
    /// every live Behavior (M095 / CE #3).
    pub(crate) behaviors_by_premise_rel: IngestMap<TermId, Vec<EventId>>,
    /// Behavior id → premise relations indexed for it (M106). Missing key means
    /// rebuild gap: unindex falls back to every premise-rel bucket.
    pub(crate) premise_rels_of_behavior: IngestMap<EventId, Vec<TermId>>,
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
            retract_examine: IngestExamine::default(),
            unindex_cause_examine: IngestExamine::default(),
            derived_by_cause: IngestMap::default(),
            cause_of_behavior: IngestMap::default(),
            live_behaviors: FxHashSet::default(),
            behaviors_by_premise_rel: IngestMap::default(),
            premise_rels_of_behavior: IngestMap::default(),
            unindex_premise_examine: IngestExamine::default(),
            unindex_live_examine: IngestExamine::default(),
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
        rt.emit_rule_ast(&crate::eval::inverse_knows_rule(), 0, None)
            .expect("genesis inverse_knows Horn");
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
    ///
    /// SelfCommit `Define` (not a User quantum / no QO). Crash window: Defines stay
    /// visible via `committed_prefix`; a following NeedsOutcome Assert without QO is
    /// dropped (M024 S02 / G-A09).
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

    /// Reset retract-derived examine counter (integration tests; not fingerprint input).
    pub fn reset_retract_examine_count(&self) {
        self.retract_examine.reset();
    }

    /// Log rows visited in `retract_derived` since reset.
    pub fn retract_examine_count(&self) -> u64 {
        self.retract_examine.get()
    }

    /// Reset cause-bucket visits in `unindex_behavior` (tests; not fingerprint input).
    pub fn reset_unindex_cause_examine_count(&self) {
        self.unindex_cause_examine.reset();
    }

    /// Cause buckets visited in `unindex_behavior` since reset.
    pub fn unindex_cause_examine_count(&self) -> u64 {
        self.unindex_cause_examine.get()
    }

    /// Reset premise-rel bucket visits in `unindex_behavior` (tests; not fingerprint input).
    pub fn reset_unindex_premise_examine_count(&self) {
        self.unindex_premise_examine.reset();
    }

    /// Premise-rel buckets visited in `unindex_behavior` since reset.
    pub fn unindex_premise_examine_count(&self) -> u64 {
        self.unindex_premise_examine.get()
    }

    /// Reset visits of the live-behavior vector in `unindex_behavior` (tests).
    pub fn reset_unindex_live_examine_count(&self) {
        self.unindex_live_examine.reset();
    }

    /// Entries removed from `live_behaviors` by `unindex_behavior` since reset.
    pub fn unindex_live_examine_count(&self) -> u64 {
        self.unindex_live_examine.get()
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
        // Live premises only (CE review #4): match retract/correct. A retracted
        // fact must not become caused_by for a new Behavior.
        let premise = self
            .fold
            .facts()
            .iter()
            .rev()
            .find(|f| {
                f.subject == subject
                    && f.invalidated_at.is_none()
                    && !self.fold.is_dictionary_rel(f.relation)
            })
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

    /// AS OF valid-time (`tt = MAX`). See [`GraphFold::as_of`].
    pub fn as_of(&self, vt: ValidTime) -> Vec<(TermId, TermId, TermId)> {
        self.fold.as_of(vt)
    }

    /// Product triples as known on a wall date.
    ///
    /// Greatest `Op::TtAnchor` with `wall` at or before `wall`, then
    /// [`GraphFold::known_at`] at that transaction time (TT-only; no ValidTime cut).
    /// `None` means no anchor at or before the date.
    ///
    /// Takes [`WallTime`] (not `impl Into`): bare `u64` and wrong-role clocks do
    /// not compile (M025 S02 / G-C09).
    pub fn as_known_on(&self, wall: WallTime) -> Option<Vec<(TermId, TermId, TermId)>> {
        self.fold.as_known_on(wall)
    }

    /// Norm in force on `vt`, as known on `wall` (M025 S03 / Phase 83).
    ///
    /// Resolves knowledge TT from the greatest `TtAnchor` ≤ `wall`, then
    /// `live_at(tt, vt)`. Claim lineage keyed on `claim_id`: when
    /// `event_by_id(claim_id)` is `Op::Behavior`, require
    /// [`Self::derivation_eligible_at`] and that the live row still matches the
    /// Behavior head proposition (Correct/CorrectInterval keep `claim_id` but
    /// may change object — M037 / CE #13; M056 / CE #28). Assert claims pass.
    /// Conflict (both polarities) wins over Ok. `as_of` / ASOF-ELIG-01 unchanged.
    pub fn norm_view(&self, vt: ValidTime, wall: WallTime) -> NormView {
        let Some(tt) = self.fold.knowledge_tt_at(wall) else {
            return NormView::MissingAnchor;
        };
        let vt_u = vt.as_u64();
        let live = self.fold.live_facts_at(tt, vt);
        if live.is_empty() {
            return NormView::Unknown;
        }

        let mut seen_claims = std::collections::BTreeSet::new();
        for f in &live {
            if !seen_claims.insert(f.claim_id) {
                continue;
            }
            let report = self.conflict_report_at(
                f.claim_id,
                TransactionTime::from(tt),
                ValidTime::from(vt_u),
            );
            if !report.positive_supports.is_empty() && !report.negative_supports.is_empty() {
                return NormView::Conflict {
                    claim: f.claim_id,
                    report,
                };
            }
        }

        let mut eligible = Vec::new();
        for f in live {
            // Key off claim lineage, not minting event type: Correct/CI keep
            // claim_id of the Behavior while minting a new event_id.
            let behavior_head = match self.event_by_id(f.claim_id).map(|e| &e.op) {
                Some(Op::Behavior {
                    subject,
                    relation,
                    object,
                    ..
                }) => Some((*subject, *relation, *object)),
                _ => None,
            };
            let ok = if let Some((hs, hr, ho)) = behavior_head {
                // Premise+pin walk alone is not enough: a Correct that changes
                // the derived proposition must not inherit rule eligibility
                // (M056 / CE #28). Live row must still match the Behavior head.
                self.derivation_eligible_at(
                    f.claim_id,
                    TransactionTime::from(tt),
                    ValidTime::from(vt_u),
                ) && f.subject == hs
                    && f.relation == hr
                    && f.object() == ho
            } else {
                true
            };
            if ok {
                eligible.push(NormTriple {
                    subject: f.subject,
                    relation: f.relation,
                    object: f.object(),
                    claim_id: f.claim_id,
                    event_id: f.event_id,
                    eligible: true,
                });
            }
        }

        if eligible.is_empty() {
            NormView::Abstain
        } else {
            NormView::Ok(eligible)
        }
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
