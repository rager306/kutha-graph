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

/// Lean write operators (TGMS-shaped; ADR-010 D010-3).
#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum Op {
    /// N-ary objects: subject, relation, object (interned).
    /// `claim`: when set, this Assert is another support for that claim id;
    /// when absent, the new event id becomes the claim id (ADR-011 / M011 S01).
    Assert {
        subject: TermId,
        relation: TermId,
        object: TermId,
        valid_from: ValidTime,
        valid_to: Option<ValidTime>,
        #[serde(default)]
        claim: Option<EventId>,
    },
    Retract {
        fact_seq: u64,
    },
    Correct {
        fact_seq: u64,
        object: TermId,
        valid_from: ValidTime,
        valid_to: Option<ValidTime>,
    },
    /// Explicit interval patch (M011 S04). Does not change whole-version `Correct`.
    CorrectInterval {
        fact_seq: u64,
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
            Op::Retract { .. } | Op::Define { .. } => vec![],
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
            }
            Op::Retract { fact_seq } => {
                h.update(b"retract");
                h.update(fact_seq.to_le_bytes());
            }
            Op::Correct {
                fact_seq,
                object,
                valid_from,
                valid_to,
            } => {
                h.update(b"correct");
                h.update(fact_seq.to_le_bytes());
                h.update(object.to_le_bytes());
                h.update(valid_from.to_le_bytes());
                h.update(valid_to.unwrap_or(u64::MAX).to_le_bytes());
            }
            Op::CorrectInterval {
                fact_seq,
                object,
                patch_from,
                patch_to,
            } => {
                h.update(b"correct-interval");
                h.update(fact_seq.to_le_bytes());
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
        }
        h.finalize().into()
    }
}
