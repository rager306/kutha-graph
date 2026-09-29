use kutha_common::{Event, EventId, Op, TermId, TransactionTime, ValidTime};
use sha2::{Digest, Sha256};

fn nil_claim() -> EventId {
    EventId::nil()
}

/// Half-open intersection of `[a_from, a_to)` and `[b_from, b_to)`. `None` end is +∞.
/// Empty when `from` is not strictly less than `to` after substituting infinity (D-C7).
pub(crate) fn vt_intersect(
    a_from: ValidTime,
    a_to: Option<ValidTime>,
    b_from: ValidTime,
    b_to: Option<ValidTime>,
) -> Option<(ValidTime, Option<ValidTime>)> {
    let from = a_from.max(b_from);
    let a_end = a_to.unwrap_or(u64::MAX);
    let b_end = b_to.unwrap_or(u64::MAX);
    let to_raw = a_end.min(b_end);
    if from >= to_raw {
        return None;
    }
    let to = if to_raw == u64::MAX {
        None
    } else {
        Some(to_raw)
    };
    Some((from, to))
}

/// Leftover halves of fact VT minus a non-empty intersection (prefix, suffix). Skip empty halves.
fn leftover_halves(
    fact_from: ValidTime,
    fact_to: Option<ValidTime>,
    inter_from: ValidTime,
    inter_to: Option<ValidTime>,
) -> (
    Option<(ValidTime, Option<ValidTime>)>,
    Option<(ValidTime, Option<ValidTime>)>,
) {
    let prefix = if fact_from < inter_from {
        Some((fact_from, Some(inter_from)))
    } else {
        None
    };
    let suffix = match inter_to {
        None => None,
        Some(i_to) => {
            let fact_end = fact_to.unwrap_or(u64::MAX);
            if i_to < fact_end {
                Some((i_to, fact_to))
            } else {
                None
            }
        }
    };
    (prefix, suffix)
}

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
    /// Portable claim identity (ADR-011). Multiple Facts may share one claim_id (supports).
    #[serde(default = "nil_claim")]
    pub claim_id: EventId,
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
            h.update(f.claim_id.as_bytes());
        }
        h.finalize().into()
    }

    /// Live Facts that support `claim` at an explicit cut.
    pub fn live_supports(&self, claim: EventId, tt: TransactionTime, vt: ValidTime) -> Vec<&Fact> {
        self.facts
            .iter()
            .filter(|f| f.claim_id == claim && f.is_live_at(tt, vt))
            .collect()
    }

    pub fn live_support_count(&self, claim: EventId, tt: TransactionTime, vt: ValidTime) -> usize {
        self.live_supports(claim, tt, vt).len()
    }

    pub fn claim_supported_at(&self, claim: EventId, tt: TransactionTime, vt: ValidTime) -> bool {
        self.live_support_count(claim, tt, vt) > 0
    }

    pub fn apply(&mut self, event: &Event) {
        match &event.op {
            Op::Assert {
                subject,
                relation,
                object,
                valid_from,
                valid_to,
                claim,
            } => {
                let seq = self.next_seq;
                self.next_seq += 1;
                let claim_id = claim.unwrap_or(event.id);
                self.facts.push(Fact {
                    seq,
                    subject: *subject,
                    relation: *relation,
                    object: *object,
                    valid_from: *valid_from,
                    valid_to: *valid_to,
                    ingested_at: event.ingested_at,
                    invalidated_at: None,
                    claim_id,
                });
            }
            Op::Behavior {
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
                    claim_id: event.id,
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
                        let claim_id = old.claim_id;
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
                            claim_id,
                        });
                    }
                }
            }
            Op::CorrectInterval {
                fact_seq,
                object,
                patch_from,
                patch_to,
            } => {
                let Some(idx) = self.facts.iter().position(|f| f.seq == *fact_seq) else {
                    return;
                };
                if self.facts[idx].invalidated_at.is_some() {
                    return;
                }
                let old = &self.facts[idx];
                let Some((inter_from, inter_to)) =
                    vt_intersect(old.valid_from, old.valid_to, *patch_from, *patch_to)
                else {
                    return;
                };
                let (prefix, suffix) =
                    leftover_halves(old.valid_from, old.valid_to, inter_from, inter_to);
                let s = old.subject;
                let r = old.relation;
                let claim_id = old.claim_id;
                let old_object = old.object();
                self.facts[idx].invalidated_at = Some(event.ingested_at);

                let mut push_row = |obj: TermId, vf: ValidTime, vt: Option<ValidTime>| {
                    let seq = self.next_seq;
                    self.next_seq += 1;
                    self.facts.push(Fact {
                        seq,
                        subject: s,
                        relation: r,
                        object: obj,
                        valid_from: vf,
                        valid_to: vt,
                        ingested_at: event.ingested_at,
                        invalidated_at: None,
                        claim_id,
                    });
                };
                if let Some((vf, vt)) = prefix {
                    push_row(old_object, vf, vt);
                }
                push_row(*object, inter_from, inter_to);
                if let Some((vf, vt)) = suffix {
                    push_row(old_object, vf, vt);
                }
            }
            Op::Define { .. } => {}
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
