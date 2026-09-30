use kutha_common::{Event, EventId, Op, SupportPolarity, TermId, TransactionTime, ValidTime};
use sha2::{Digest, Sha256};
use std::collections::{BTreeMap, HashMap};
use std::sync::atomic::{AtomicU64, Ordering};

/// Integration-test examine counter (not fingerprint / JSON). `cfg(test)` is off for `tests/*.rs`.
#[derive(Debug, Default)]
struct ExamineCounter(AtomicU64);

impl Clone for ExamineCounter {
    fn clone(&self) -> Self {
        Self(AtomicU64::new(self.0.load(Ordering::Relaxed)))
    }
}

impl ExamineCounter {
    fn reset(&self) {
        self.0.store(0, Ordering::Relaxed);
    }

    fn get(&self) -> u64 {
        self.0.load(Ordering::Relaxed)
    }

    fn bump(&self) {
        self.0.fetch_add(1, Ordering::Relaxed);
    }
}

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

fn flipped_polarity(p: SupportPolarity) -> SupportPolarity {
    match p {
        SupportPolarity::Positive => SupportPolarity::Negative,
        SupportPolarity::Negative => SupportPolarity::Positive,
    }
}

/// Residuals copy polarity. An object-changing replacement row flips when old polarity is Some.
fn polarity_for_row(
    old: Option<SupportPolarity>,
    old_object: TermId,
    new_object: TermId,
) -> Option<SupportPolarity> {
    if new_object == old_object {
        old
    } else {
        old.map(flipped_polarity)
    }
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
    /// Portable claim / proposition identity (ADR-011, ING-02). Multiple Facts may share one
    /// `claim_id` as independent supports. Not the support slot and not `delivery_key`.
    #[serde(default = "nil_claim")]
    pub claim_id: EventId,
    /// Minting Event.id — the support slot (REF-01, ING-02). Retract/Correct look up this field, not fold-local seq.
    #[serde(default = "nil_claim")]
    pub event_id: EventId,
    /// Durable retry key copied from `Op::Assert` (ING-01 / D-01). Not claim identity.
    #[serde(default)]
    pub delivery_key: Option<String>,
    /// Stored support polarity copied from `Op::Assert` (ING-03). `None` is in neither conflict bucket.
    #[serde(default)]
    pub polarity: Option<SupportPolarity>,
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
/// Hot maps are leases: skip-serialized and rebuilt from `facts` (D-02).
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct GraphFold {
    facts: Vec<Fact>,
    next_seq: u64,
    /// Hot-path examine count. Not SoT; skipped in snapshot JSON.
    #[serde(skip)]
    hot_examine: ExamineCounter,
    /// Fact indices grouped by `valid_from` so a cut can skip future-start decoys.
    #[serde(skip)]
    vt_by_from: BTreeMap<ValidTime, Vec<usize>>,
    /// Fact indices grouped by portable `claim_id`.
    #[serde(skip)]
    claim_facts: HashMap<EventId, Vec<usize>>,
}

impl<'de> serde::Deserialize<'de> for GraphFold {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        #[derive(serde::Deserialize)]
        struct GraphFoldDe {
            facts: Vec<Fact>,
            next_seq: u64,
        }
        let de = GraphFoldDe::deserialize(deserializer)?;
        let mut fold = GraphFold {
            facts: de.facts,
            next_seq: de.next_seq,
            hot_examine: ExamineCounter::default(),
            vt_by_from: BTreeMap::new(),
            claim_facts: HashMap::new(),
        };
        fold.rebuild_hot_maps();
        Ok(fold)
    }
}

impl GraphFold {
    pub fn facts(&self) -> &[Fact] {
        &self.facts
    }

    /// Reset the hot-path examine counter (integration tests; not fingerprint input).
    pub fn reset_hot_examine_count(&self) {
        self.hot_examine.reset();
    }

    /// Facts (or index slots) considered for liveness on the last hot reads since reset.
    pub fn hot_examine_count(&self) -> u64 {
        self.hot_examine.get()
    }

    fn index_slot(&mut self, idx: usize) {
        let fact = &self.facts[idx];
        self.vt_by_from.entry(fact.valid_from).or_default().push(idx);
        self.claim_facts.entry(fact.claim_id).or_default().push(idx);
    }

    fn rebuild_hot_maps(&mut self) {
        self.vt_by_from.clear();
        self.claim_facts.clear();
        for idx in 0..self.facts.len() {
            self.index_slot(idx);
        }
    }

    fn ensure_hot_maps(&mut self) {
        if self.vt_by_from.is_empty() && !self.facts.is_empty() {
            self.rebuild_hot_maps();
        }
    }

    fn push_fact(&mut self, fact: Fact) {
        let idx = self.facts.len();
        self.facts.push(fact);
        self.index_slot(idx);
    }

    /// Live facts at an explicit cut, walking the VT index rather than every fact.
    pub(crate) fn live_facts_at(&self, tt: TransactionTime, vt: ValidTime) -> Vec<&Fact> {
        self.vt_by_from
            .range(..=vt)
            .flat_map(|(_, idxs)| idxs.iter().copied())
            .filter_map(|idx| {
                self.hot_examine.bump();
                let fact = &self.facts[idx];
                fact.is_live_at(tt, vt).then_some(fact)
            })
            .collect()
    }

    pub fn live_count(&self, tt: TransactionTime, vt: ValidTime) -> usize {
        self.live_at(tt, vt).len()
    }

    /// Live interned triples at an explicit transaction-time × valid-time cut.
    /// There is no “now” default (FF5 / MemStrata).
    pub fn live_at(&self, tt: TransactionTime, vt: ValidTime) -> Vec<(TermId, TermId, TermId)> {
        self.live_facts_at(tt, vt)
            .into_iter()
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
            h.update(f.event_id.as_bytes());
            match &f.delivery_key {
                Some(k) => {
                    h.update(b"dkey");
                    h.update((k.len() as u64).to_le_bytes());
                    h.update(k.as_bytes());
                }
                None => h.update(b"no-dkey"),
            }
            h.update(match f.polarity {
                None => b"pol-none".as_slice(),
                Some(SupportPolarity::Positive) => b"pol-pos",
                Some(SupportPolarity::Negative) => b"pol-neg",
            });
        }
        h.finalize().into()
    }

    /// First Fact minted (or residual-replaced) by `event_id`. Prefer a live row.
    fn fact_index_by_event_id(&self, event_id: EventId) -> Option<usize> {
        self.facts
            .iter()
            .position(|f| f.event_id == event_id && f.invalidated_at.is_none())
            .or_else(|| self.facts.iter().position(|f| f.event_id == event_id))
    }

    /// Live Facts that support `claim` at an explicit cut.
    pub fn live_supports(&self, claim: EventId, tt: TransactionTime, vt: ValidTime) -> Vec<&Fact> {
        let Some(idxs) = self.claim_facts.get(&claim) else {
            return Vec::new();
        };
        idxs.iter()
            .filter_map(|&idx| {
                self.hot_examine.bump();
                let fact = &self.facts[idx];
                fact.is_live_at(tt, vt).then_some(fact)
            })
            .collect()
    }

    pub fn live_support_count(&self, claim: EventId, tt: TransactionTime, vt: ValidTime) -> usize {
        self.live_supports(claim, tt, vt).len()
    }

    pub fn claim_supported_at(&self, claim: EventId, tt: TransactionTime, vt: ValidTime) -> bool {
        self.live_support_count(claim, tt, vt) > 0
    }

    pub fn apply(&mut self, event: &Event) {
        self.ensure_hot_maps();
        match &event.op {
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
                let seq = self.next_seq;
                self.next_seq += 1;
                let claim_id = claim.unwrap_or(event.id);
                self.push_fact(Fact {
                    seq,
                    subject: *subject,
                    relation: *relation,
                    object: *object,
                    valid_from: *valid_from,
                    valid_to: *valid_to,
                    ingested_at: event.ingested_at,
                    invalidated_at: None,
                    claim_id,
                    event_id: event.id,
                    delivery_key: delivery_key.clone(),
                    polarity: *polarity,
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
                self.push_fact(Fact {
                    seq,
                    subject: *subject,
                    relation: *relation,
                    object: *object,
                    valid_from: *valid_from,
                    valid_to: *valid_to,
                    ingested_at: event.ingested_at,
                    invalidated_at: None,
                    claim_id: event.id,
                    event_id: event.id,
                    delivery_key: None,
                    polarity: None,
                });
            }
            Op::Retract { event_id } => {
                if let Some(idx) = self.fact_index_by_event_id(*event_id) {
                    if self.facts[idx].invalidated_at.is_none() {
                        self.facts[idx].invalidated_at = Some(event.ingested_at);
                    }
                }
            }
            Op::Correct {
                event_id,
                object,
                valid_from,
                valid_to,
            } => {
                if let Some(idx) = self.fact_index_by_event_id(*event_id) {
                    if self.facts[idx].invalidated_at.is_none() {
                        self.facts[idx].invalidated_at = Some(event.ingested_at);
                        let s = self.facts[idx].subject;
                        let r = self.facts[idx].relation;
                        let claim_id = self.facts[idx].claim_id;
                        let old_object = self.facts[idx].object();
                        let old_polarity = self.facts[idx].polarity;
                        let seq = self.next_seq;
                        self.next_seq += 1;
                        self.push_fact(Fact {
                            seq,
                            subject: s,
                            relation: r,
                            object: *object,
                            valid_from: *valid_from,
                            valid_to: *valid_to,
                            ingested_at: event.ingested_at,
                            invalidated_at: None,
                            claim_id,
                            event_id: event.id,
                            delivery_key: None,
                            polarity: polarity_for_row(old_polarity, old_object, *object),
                        });
                    }
                }
            }
            Op::CorrectInterval {
                event_id,
                object,
                patch_from,
                patch_to,
            } => {
                let Some(idx) = self.fact_index_by_event_id(*event_id) else {
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
                let old_polarity = old.polarity;
                self.facts[idx].invalidated_at = Some(event.ingested_at);

                let mut push_row = |obj: TermId, vf: ValidTime, vt: Option<ValidTime>| {
                    let seq = self.next_seq;
                    self.next_seq += 1;
                    self.push_fact(Fact {
                        seq,
                        subject: s,
                        relation: r,
                        object: obj,
                        valid_from: vf,
                        valid_to: vt,
                        ingested_at: event.ingested_at,
                        invalidated_at: None,
                        claim_id,
                        event_id: event.id,
                        delivery_key: None,
                        polarity: polarity_for_row(old_polarity, old_object, obj),
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
            Op::Define { .. } | Op::QuantumOutcome { .. } | Op::JustificationCite { .. } => {}
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
