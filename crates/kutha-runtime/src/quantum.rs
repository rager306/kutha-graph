use crate::allow::load_allowed_names;
use crate::csr::{CsrLease, TypedCsrLease};
use crate::fold::GraphFold;
use crate::log::EventLog;
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
        fact_seq: u64,
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
        fact_seq: u64,
    },
    DuplicateResume {
        resume_of: String,
    },
    AdmissionDenied {
        justification_id: String,
        reason: &'static str,
    },
}

impl fmt::Display for RuntimeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RuntimeError::ReplayDivergence { .. } => write!(f, "ReplayDivergenceError"),
            RuntimeError::ProvenanceMismatch { .. } => write!(f, "ProvenanceMismatchError"),
            RuntimeError::UnknownFact { fact_seq } => write!(f, "unknown fact {fact_seq}"),
            RuntimeError::UnknownRelation { name } => {
                write!(f, "unknown relation {name} (not in allowlist)")
            }
            RuntimeError::UnknownClaim { claim } => write!(f, "unknown claim {claim}"),
            RuntimeError::BrokenLineage { caused_by } => {
                write!(f, "broken lineage caused_by={caused_by}")
            }
            RuntimeError::IntervalPatchRejected { fact_seq } => {
                write!(f, "interval patch rejected for fact {fact_seq}")
            }
            RuntimeError::DuplicateResume { resume_of } => {
                write!(f, "duplicate resume for quantum {resume_of}")
            }
            RuntimeError::AdmissionDenied { .. } => write!(f, "AdmissionDeniedError"),
        }
    }
}

/// Durable progress encoding for a quantum (OUT-01 / D-O2). Call `Ok` is not completion.
#[derive(Clone, Copy, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum OutcomeDisposition {
    Zero,
    Partial,
    Full,
    Resume,
}

/// Authoritative justification / admission cite (D-F2). Not a droppable lease.
/// SoT is `justifications.jsonl`; do not mix these bytes into [`Runtime::provenance_fingerprint`].
#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct Justification {
    pub justification_id: String,
    pub target_claim: EventId,
    pub source_claim_ids: Vec<EventId>,
    pub source_event_ids: Vec<EventId>,
    pub source_fact_seqs: Vec<u64>,
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
        | Op::Define { .. } => None,
    }
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
    /// In-memory buffer of durable quantum outcomes (SoT is `quantum_outcomes.jsonl`).
    outcomes: Vec<PersistedQuantumOutcome>,
    /// In-memory buffer of durable justification cites (SoT is `justifications.jsonl`).
    /// Open never infers cites from the log. `fork_at` starts empty (like outcomes).
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

    /// Count of non-`Define` events (graph ops). Snapshot `log_offset` uses this so open-with-snapshot
    /// stays aligned with the Define-stripped durable graph stream.
    pub fn graph_len(&self) -> usize {
        self.log
            .iter()
            .filter(|e| !matches!(e.op, Op::Define { .. }))
            .count()
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

    /// Attach rows loaded from `quantum_outcomes.jsonl` (store open paths).
    pub fn attach_outcomes(&mut self, rows: Vec<PersistedQuantumOutcome>) {
        self.outcomes = rows;
    }

    /// Persisted justification rows buffered for `store::persist` (D-F2).
    pub fn justification_records(&self) -> &[Justification] {
        &self.justifications
    }

    /// Attach rows loaded from `justifications.jsonl`. Open never infers cites from the log.
    /// `fork_at` starts empty (like outcomes); record cites again on the fork if needed.
    pub fn attach_justifications(&mut self, rows: Vec<Justification>) {
        self.justifications = rows;
    }

    /// Append one justification cite. Does not run from `emit`. Returns the minted id.
    pub fn record_justification(
        &mut self,
        target_claim: EventId,
        source_claim_ids: Vec<EventId>,
        source_event_ids: Vec<EventId>,
        source_fact_seqs: Vec<u64>,
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
            source_fact_seqs,
            rule_version,
            tt,
            vt,
        });
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
        // Cited seqs must still be live on the current picture at the row's VT.
        // Historical row.tt records the claimed cut; using it alone would never
        // stale a cite after CorrectInterval/Retract (FIX-02).
        for seq in &row.source_fact_seqs {
            let live = self
                .fold
                .facts()
                .iter()
                .any(|f| f.seq == *seq && f.is_live_at(u64::MAX, row.vt));
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

    /// Partition live supports of `claim` by interned positive vs negative object (D-F3).
    pub fn conflict_report_at(
        &self,
        claim: EventId,
        tt: u64,
        vt: u64,
        positive: TermId,
        negative: TermId,
    ) -> ConflictReport {
        let mut positive_supports = Vec::new();
        let mut negative_supports = Vec::new();
        for f in self.fold.live_supports(claim, tt, vt) {
            if f.object() == positive {
                positive_supports.push(f.seq);
            } else if f.object() == negative {
                negative_supports.push(f.seq);
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
        // Snapshot offset indexes the graph stream; Define ops are durable SoT but not fold/offset.
        let graph: Vec<Event> = all_events
            .into_iter()
            .filter(|e| !matches!(e.op, Op::Define { .. }))
            .collect();
        let mut fold = snap.fold;
        let mut next_tt = snap.next_tt;
        let offset = snap.log_offset.min(graph.len());
        for e in &graph[offset..] {
            fold.apply(e);
            next_tt = next_tt.max(e.ingested_at.saturating_add(1));
        }
        Self {
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
        }
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
        Ok(Self {
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
        })
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
            outcomes: Vec::new(),
            justifications: Vec::new(),
        }
    }

    /// CSR lease at an explicit cut. Callers must name valid-time (no silent “now”).
    pub fn csr_lease_at(&self, tt: u64, vt: u64) -> CsrLease {
        CsrLease::from_fold(&self.fold, tt, vt, self.dict.len())
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

    /// Admit a user op, append, fold, cascade inverse-`knows` until idle or budget.
    pub fn emit(&mut self, op: Op) -> Result<QuantumOutcome, RuntimeError> {
        if let Op::Retract { fact_seq }
        | Op::Correct { fact_seq, .. }
        | Op::CorrectInterval { fact_seq, .. } = &op
        {
            if !self.fold.facts().iter().any(|f| f.seq == *fact_seq) {
                return Err(RuntimeError::UnknownFact {
                    fact_seq: *fact_seq,
                });
            }
        }
        if let Op::CorrectInterval {
            fact_seq,
            patch_from,
            patch_to,
            ..
        } = &op
        {
            let fact = self
                .fold
                .facts()
                .iter()
                .find(|f| f.seq == *fact_seq)
                .expect("UnknownFact gate already ran");
            if fact.invalidated_at.is_some()
                || patch_to.is_some_and(|t| *patch_from >= t)
                || crate::fold::vt_intersect(fact.valid_from, fact.valid_to, *patch_from, *patch_to)
                    .is_none()
            {
                return Err(RuntimeError::IntervalPatchRejected {
                    fact_seq: *fact_seq,
                });
            }
        }
        self.admit(&op)?;
        self.admit_claim(&op)?;
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
                claim: _,
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

    /// Lineage digest over Behavior rows in log order (ADR-060 obligation 2).
    /// Mix is `(event.id, caused_by, name, rule_version)` with length-prefixed strings.
    /// Does not include Fact triples and is not called from [`Self::replay_check`].
    pub fn provenance_fingerprint(&self) -> [u8; 32] {
        let mut h = Sha256::new();
        h.update(b"kutha-prov-v1");
        for e in self.log.iter() {
            if let Op::Behavior {
                name,
                caused_by,
                rule_version,
                ..
            } = &e.op
            {
                h.update(e.id.as_bytes());
                h.update(caused_by.as_bytes());
                h.update((name.len() as u64).to_le_bytes());
                h.update(name.as_bytes());
                h.update((rule_version.len() as u64).to_le_bytes());
                h.update(rule_version.as_bytes());
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
            | Op::Define { .. } => return false,
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

                claim: None,
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
