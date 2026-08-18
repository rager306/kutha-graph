use kutha_common::{Event, Op, TermId, TransactionTime, ValidTime};
use sha2::{Digest, Sha256};

/// One asserted (or behavior-asserted) fact in the fold. Losers stay on retract (ADR-013).
#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct Fact {
    pub seq: u64,
    pub subject: TermId,
    pub relation: TermId,
    object: TermId,
    pub valid_from: ValidTime,
    pub valid_to: Option<ValidTime>,
    pub ingested_at: TransactionTime,
    pub invalidated_at: Option<TransactionTime>,
}

impl Fact {
    pub fn object(&self) -> TermId {
        self.object
    }

    pub fn is_live_at(&self, tt: TransactionTime, vt: ValidTime) -> bool {
        if self.ingested_at > tt {
            return false;
        }
        if self.invalidated_at.is_some_and(|inv| inv <= tt) {
            return false;
        }
        if vt < self.valid_from {
            return false;
        }
        if self.valid_to.is_some_and(|to| vt >= to) {
            return false;
        }
        true
    }
}

/// Deterministic fold of the log. Droppable picture, not SoT.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct GraphFold {
    facts: Vec<Fact>,
    next_seq: u64,
}

impl GraphFold {
    pub fn facts(&self) -> &[Fact] {
        &self.facts
    }

    pub fn live_count(&self, tt: TransactionTime, vt: ValidTime) -> usize {
        self.live_at(tt, vt).len()
    }

    /// Live interned triples at an explicit transaction-time × valid-time cut.
    /// There is no “now” default (FF5 / MemStrata).
    pub fn live_at(&self, tt: TransactionTime, vt: ValidTime) -> Vec<(TermId, TermId, TermId)> {
        self.facts
            .iter()
            .filter(|f| f.is_live_at(tt, vt))
            .map(|f| (f.subject, f.relation, f.object()))
            .collect()
    }

    /// AS OF valid-time on the current transaction picture (`tt = MAX`).
    pub fn as_of(&self, vt: ValidTime) -> Vec<(TermId, TermId, TermId)> {
        self.live_at(u64::MAX, vt)
    }

    pub fn fingerprint(&self) -> [u8; 32] {
        let mut h = Sha256::new();
        for f in &self.facts {
            h.update(f.seq.to_le_bytes());
            h.update(f.subject.to_le_bytes());
            h.update(f.relation.to_le_bytes());
            h.update(f.object.to_le_bytes());
            h.update(f.valid_from.to_le_bytes());
            h.update(f.valid_to.unwrap_or(u64::MAX).to_le_bytes());
            h.update(f.ingested_at.to_le_bytes());
            h.update(f.invalidated_at.unwrap_or(u64::MAX).to_le_bytes());
        }
        h.finalize().into()
    }

    pub fn apply(&mut self, event: &Event) {
        match &event.op {
            Op::Assert {
                subject,
                relation,
                object,
                valid_from,
                valid_to,
            }
            | Op::Behavior {
                subject,
                relation,
                object,
                valid_from,
                valid_to,
                ..
            } => {
                let seq = self.next_seq;
                self.next_seq += 1;
                self.facts.push(Fact {
                    seq,
                    subject: *subject,
                    relation: *relation,
                    object: *object,
                    valid_from: *valid_from,
                    valid_to: *valid_to,
                    ingested_at: event.ingested_at,
                    invalidated_at: None,
                });
            }
            Op::Retract { fact_seq } => {
                if let Some(f) = self.facts.iter_mut().find(|f| f.seq == *fact_seq) {
                    if f.invalidated_at.is_none() {
                        f.invalidated_at = Some(event.ingested_at);
                    }
                }
            }
            Op::Correct {
                fact_seq,
                object,
                valid_from,
                valid_to,
            } => {
                if let Some(old) = self.facts.iter_mut().find(|f| f.seq == *fact_seq) {
                    if old.invalidated_at.is_none() {
                        old.invalidated_at = Some(event.ingested_at);
                        let s = old.subject;
                        let r = old.relation;
                        let seq = self.next_seq;
                        self.next_seq += 1;
                        self.facts.push(Fact {
                            seq,
                            subject: s,
                            relation: r,
                            object: *object,
                            valid_from: *valid_from,
                            valid_to: *valid_to,
                            ingested_at: event.ingested_at,
                            invalidated_at: None,
                        });
                    }
                }
            }
        }
    }

    pub fn replay(events: &[Event]) -> Self {
        let mut fold = Self::default();
        for e in events {
            fold.apply(e);
        }
        fold
    }

    /// Test-only: mutate the picture without a log event (must fail replay).
    #[cfg(test)]
    pub(crate) fn tamper_invalidate_first(&mut self) {
        if let Some(f) = self.facts.get_mut(0) {
            f.invalidated_at = Some(999);
        }
    }
}
