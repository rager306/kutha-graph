use sha2::{Digest, Sha256};
use uuid::Uuid;

/// UUID v7 event identifier (ADR-000 D2 / ADR-011).
pub type EventId = Uuid;

/// Interned term (hot-path integer; strings live in the dictionary lease).
pub type TermId = u32;

/// Valid-time instant as an opaque integer clock (world).
pub type ValidTime = u64;

/// Transaction-time as log sequence (system).
pub type TransactionTime = u64;

/// Durable progress encoding for a quantum (OUT-01 / D-O2). Call `Ok` is not completion.
#[derive(Clone, Copy, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum OutcomeDisposition {
    Zero,
    Partial,
    Full,
    Resume,
}

/// Stored support polarity for conflict views (ING-03). `None` on Assert is unbucketed.
#[derive(Clone, Copy, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SupportPolarity {
    Positive,
    Negative,
}

/// Lean write operators (TGMS-shaped; ADR-010 D010-3).
#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum Op {
    /// N-ary objects: subject, relation, object (interned).
    /// `claim`: when set, this Assert is another support for that claim id;
    /// when absent, the new event id becomes the claim id (ADR-011 / M011 S01).
    /// `delivery_key`: non-empty retry identity (ING-01 / D-01). `None` or empty always mints.
    /// `polarity`: stored support polarity; default `None` (ING-03 wiring is Plan 14-02).
    Assert {
        subject: TermId,
        relation: TermId,
        object: TermId,
        valid_from: ValidTime,
        valid_to: Option<ValidTime>,
        #[serde(default)]
        claim: Option<EventId>,
        #[serde(default)]
        delivery_key: Option<String>,
        #[serde(default)]
        polarity: Option<SupportPolarity>,
    },
    Retract {
        event_id: EventId,
    },
    Correct {
        event_id: EventId,
        object: TermId,
        valid_from: ValidTime,
        valid_to: Option<ValidTime>,
    },
    /// Explicit interval patch (M011 S04). Does not change whole-version `Correct`.
    CorrectInterval {
        event_id: EventId,
        object: TermId,
        patch_from: ValidTime,
        patch_to: Option<ValidTime>,
    },
    /// Behavior-emitted follow-on (still a log event; never LLM narrative).
    Behavior {
        name: String,
        caused_by: EventId,
        /// Pinned rule identity for provenance (ADR-060 obligation 2). Fold ignores this.
        /// Missing JSON decodes to empty so legacy WAL/JSONL rows stay readable.
        #[serde(default)]
        rule_version: String,
        subject: TermId,
        relation: TermId,
        object: TermId,
        valid_from: ValidTime,
        valid_to: Option<ValidTime>,
    },
    /// Logged term definition (ADR-011 / M010 S02). Fold no-op; not a graph fact.
    Define {
        name: String,
    },
    /// Logged quantum outcome (M012a S01 / LOG-01). Fold no-op; not a graph fact.
    QuantumOutcome {
        quantum_id: String,
        disposition: OutcomeDisposition,
        aborted_on_budget: bool,
        events_in_quantum: usize,
        event_ids: Vec<EventId>,
        receipt_digest_hex: String,
        resume_of: Option<String>,
    },
    /// Logged justification cite (M012a S01 / LOG-02). Fold no-op; not a graph fact.
    JustificationCite {
        justification_id: String,
        target_claim: EventId,
        source_claim_ids: Vec<EventId>,
        source_event_ids: Vec<EventId>,
        rule_version: String,
        tt: u64,
        vt: u64,
    },
}

#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct Event {
    pub id: EventId,
    pub op: Op,
    pub ingested_at: TransactionTime,
    pub object_ids: Vec<TermId>,
}

impl Event {
    pub fn new(op: Op, ingested_at: TransactionTime) -> Self {
        let object_ids = match &op {
            Op::Assert {
                subject,
                relation,
                object,
                ..
            }
            | Op::Behavior {
                subject,
                relation,
                object,
                ..
            } => vec![*subject, *relation, *object],
            Op::Retract { .. }
            | Op::Define { .. }
            | Op::QuantumOutcome { .. }
            | Op::JustificationCite { .. } => vec![],
            Op::Correct { object, .. } | Op::CorrectInterval { object, .. } => vec![*object],
        };
        Self {
            id: Uuid::now_v7(),
            op,
            ingested_at,
            object_ids,
        }
    }

    pub fn digest_bytes(&self) -> [u8; 32] {
        let mut h = Sha256::new();
        h.update(self.id.as_bytes());
        h.update(self.ingested_at.to_le_bytes());
        match &self.op {
            Op::Assert {
                subject,
                relation,
                object,
                valid_from,
                valid_to,
                claim,
                delivery_key,
                polarity,
            } => {
                h.update(b"assert");
                h.update(subject.to_le_bytes());
                h.update(relation.to_le_bytes());
                h.update(object.to_le_bytes());
                h.update(valid_from.to_le_bytes());
                h.update(valid_to.unwrap_or(u64::MAX).to_le_bytes());
                if let Some(c) = claim {
                    h.update(b"claim");
                    h.update(c.as_bytes());
                }
                match delivery_key {
                    Some(k) => {
                        h.update(b"dkey");
                        h.update((k.len() as u64).to_le_bytes());
                        h.update(k.as_bytes());
                    }
                    None => h.update(b"no-dkey"),
                }
                h.update(polarity_tag(*polarity));
            }
            Op::Retract { event_id } => {
                h.update(b"retract");
                h.update(event_id.as_bytes());
            }
            Op::Correct {
                event_id,
                object,
                valid_from,
                valid_to,
            } => {
                h.update(b"correct");
                h.update(event_id.as_bytes());
                h.update(object.to_le_bytes());
                h.update(valid_from.to_le_bytes());
                h.update(valid_to.unwrap_or(u64::MAX).to_le_bytes());
            }
            Op::CorrectInterval {
                event_id,
                object,
                patch_from,
                patch_to,
            } => {
                h.update(b"correct-interval");
                h.update(event_id.as_bytes());
                h.update(object.to_le_bytes());
                h.update(patch_from.to_le_bytes());
                h.update(patch_to.unwrap_or(u64::MAX).to_le_bytes());
            }
            Op::Behavior {
                name,
                caused_by,
                rule_version,
                subject,
                relation,
                object,
                valid_from,
                valid_to,
            } => {
                h.update(b"behavior");
                h.update(name.as_bytes());
                h.update(caused_by.as_bytes());
                h.update(rule_version.as_bytes());
                h.update(subject.to_le_bytes());
                h.update(relation.to_le_bytes());
                h.update(object.to_le_bytes());
                h.update(valid_from.to_le_bytes());
                h.update(valid_to.unwrap_or(u64::MAX).to_le_bytes());
            }
            Op::Define { name } => {
                h.update(b"define");
                h.update(name.as_bytes());
            }
            Op::QuantumOutcome {
                quantum_id,
                disposition,
                aborted_on_budget,
                events_in_quantum,
                event_ids,
                receipt_digest_hex,
                resume_of,
            } => {
                h.update(b"quantum-outcome");
                h.update(quantum_id.as_bytes());
                h.update(disposition_tag(*disposition));
                h.update([u8::from(*aborted_on_budget)]);
                h.update((*events_in_quantum as u64).to_le_bytes());
                h.update((event_ids.len() as u64).to_le_bytes());
                for id in event_ids {
                    h.update(id.as_bytes());
                }
                h.update(receipt_digest_hex.as_bytes());
                match resume_of {
                    Some(s) => {
                        h.update(b"resume");
                        h.update(s.as_bytes());
                    }
                    None => h.update(b"no-resume"),
                }
            }
            Op::JustificationCite {
                justification_id,
                target_claim,
                source_claim_ids,
                source_event_ids,
                rule_version,
                tt,
                vt,
            } => {
                h.update(b"justification-cite");
                h.update(justification_id.as_bytes());
                h.update(target_claim.as_bytes());
                h.update((source_claim_ids.len() as u64).to_le_bytes());
                for id in source_claim_ids {
                    h.update(id.as_bytes());
                }
                h.update((source_event_ids.len() as u64).to_le_bytes());
                for id in source_event_ids {
                    h.update(id.as_bytes());
                }
                h.update(rule_version.as_bytes());
                h.update(tt.to_le_bytes());
                h.update(vt.to_le_bytes());
            }
        }
        h.finalize().into()
    }
}

fn polarity_tag(p: Option<SupportPolarity>) -> &'static [u8] {
    match p {
        None => b"pol-none",
        Some(SupportPolarity::Positive) => b"pol-pos",
        Some(SupportPolarity::Negative) => b"pol-neg",
    }
}

fn disposition_tag(d: OutcomeDisposition) -> &'static [u8] {
    match d {
        OutcomeDisposition::Zero => b"zero",
        OutcomeDisposition::Partial => b"partial",
        OutcomeDisposition::Full => b"full",
        OutcomeDisposition::Resume => b"resume",
    }
}
