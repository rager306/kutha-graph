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
    Assert {
        subject: TermId,
        relation: TermId,
        object: TermId,
        valid_from: ValidTime,
        valid_to: Option<ValidTime>,
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
    /// Behavior-emitted follow-on (still a log event; never LLM narrative).
    Behavior {
        name: String,
        caused_by: EventId,
        subject: TermId,
        relation: TermId,
        object: TermId,
        valid_from: ValidTime,
        valid_to: Option<ValidTime>,
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
            Op::Retract { .. } => vec![],
            Op::Correct { object, .. } => vec![*object],
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
            } => {
                h.update(b"assert");
                h.update(subject.to_le_bytes());
                h.update(relation.to_le_bytes());
                h.update(object.to_le_bytes());
                h.update(valid_from.to_le_bytes());
                h.update(valid_to.unwrap_or(u64::MAX).to_le_bytes());
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
            Op::Behavior {
                name,
                caused_by,
                subject,
                relation,
                object,
                valid_from,
                valid_to,
            } => {
                h.update(b"behavior");
                h.update(name.as_bytes());
                h.update(caused_by.as_bytes());
                h.update(subject.to_le_bytes());
                h.update(relation.to_le_bytes());
                h.update(object.to_le_bytes());
                h.update(valid_from.to_le_bytes());
                h.update(valid_to.unwrap_or(u64::MAX).to_le_bytes());
            }
        }
        h.finalize().into()
    }
}
