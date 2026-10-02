//! Fold query surface: as_of / live_at / dictionary / fingerprint / registry cuts.
use kutha_common::{EventId, SupportPolarity, TermId, TransactionTime, ValidTime};
use sha2::{Digest, Sha256};

use crate::fold::{Fact, GraphFold, LiveFactFilter};

impl GraphFold {
    pub(crate) fn live_facts_at(
        &self,
        tt: impl Into<TransactionTime>,
        vt: impl Into<ValidTime>,
    ) -> Vec<&Fact> {
        self.collect_live_facts_at(tt, vt, LiveFactFilter::Product)
    }

    /// All live facts including dictionary relations (fingerprint/replay walks `facts` directly).
    fn all_live_facts_at(
        &self,
        tt: impl Into<TransactionTime>,
        vt: impl Into<ValidTime>,
    ) -> Vec<&Fact> {
        self.collect_live_facts_at(tt, vt, LiveFactFilter::All)
    }

    fn dictionary_live_facts_at(
        &self,
        tt: impl Into<TransactionTime>,
        vt: impl Into<ValidTime>,
    ) -> Vec<&Fact> {
        self.collect_live_facts_at(tt, vt, LiveFactFilter::Dictionary)
    }

    fn collect_live_facts_at(
        &self,
        tt: impl Into<TransactionTime>,
        vt: impl Into<ValidTime>,
        filter: LiveFactFilter,
    ) -> Vec<&Fact> {
        let tt = tt.into().as_u64();
        let vt = vt.into().as_u64();
        let keep = |f: &Fact| filter.keep(self, f);
        if !self.hot_maps_warm {
            return self
                .facts
                .iter()
                .filter(|f| {
                    self.hot_examine.bump();
                    f.is_live_at(tt, vt) && keep(f)
                })
                .collect();
        }
        self.vt_by_from
            .range(..=vt)
            .flat_map(|(_, idxs)| idxs.iter().copied())
            .filter_map(|idx| {
                self.hot_examine.bump();
                let fact = &self.facts[idx];
                (fact.is_live_at(tt, vt) && keep(fact)).then_some(fact)
            })
            .collect()
    }

    pub fn live_count(&self, tt: impl Into<TransactionTime>, vt: impl Into<ValidTime>) -> usize {
        self.live_at(tt, vt).len()
    }

    /// Live interned triples at an explicit transaction-time × valid-time cut.
    /// There is no “now” default (FF5 / MemStrata).
    pub fn live_at(
        &self,
        tt: impl Into<TransactionTime>,
        vt: impl Into<ValidTime>,
    ) -> Vec<(TermId, TermId, TermId)> {
        self.live_facts_at(tt, vt)
            .into_iter()
            .map(|f| (f.subject, f.relation, f.object()))
            .collect()
    }

    /// AS OF valid-time on the current transaction picture (`tt = MAX`).
    /// Product picture: dictionary (`kutha:`) relations are excluded (D-05).
    pub fn as_of(&self, vt: impl Into<ValidTime>) -> Vec<(TermId, TermId, TermId)> {
        self.live_at(u64::MAX, vt)
    }

    /// Dictionary entries at a valid-time cut (kutha: relations only).
    pub fn dictionary_as_of(&self, vt: impl Into<ValidTime>) -> Vec<(TermId, TermId, TermId)> {
        self.dictionary_live_at(u64::MAX, vt)
    }

    pub fn dictionary_live_at(
        &self,
        tt: impl Into<TransactionTime>,
        vt: impl Into<ValidTime>,
    ) -> Vec<(TermId, TermId, TermId)> {
        self.dictionary_live_facts_at(tt, vt)
            .into_iter()
            .map(|f| (f.subject, f.relation, f.object()))
            .collect()
    }

    pub fn fingerprint(&self) -> [u8; 32] {
        let mut h = Sha256::new();
        for f in &self.facts {
            h.update(f.seq.to_le_bytes());
            h.update(f.subject.to_le_bytes());
            h.update(f.relation.to_le_bytes());
            h.update(f.object().to_le_bytes());
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
    pub(crate) fn fact_index_by_event_id(&self, event_id: EventId) -> Option<usize> {
        if !self.hot_maps_warm {
            return self
                .facts
                .iter()
                .enumerate()
                .filter(|(_, f)| {
                    self.hot_examine.bump();
                    f.event_id == event_id
                })
                .map(|(i, _)| i)
                .find(|&i| self.facts[i].invalidated_at.is_none())
                .or_else(|| self.facts.iter().rposition(|f| f.event_id == event_id));
        }
        let idxs = self.fact_by_event.get(&event_id)?;
        idxs.iter()
            .copied()
            .find(|&i| self.facts[i].invalidated_at.is_none())
            .or_else(|| idxs.last().copied())
    }

    /// Whether any Fact (live or not) carries this claim id.
    pub(crate) fn claim_known(&self, claim: EventId) -> bool {
        if !self.hot_maps_warm {
            return self.facts.iter().any(|f| {
                self.hot_examine.bump();
                f.claim_id == claim
            });
        }
        self.claim_facts.contains_key(&claim)
    }

    /// Live triple presence for residual cascade checks (knows → knownBy).
    pub(crate) fn has_live_triple(
        &self,
        subject: TermId,
        relation: TermId,
        object: TermId,
    ) -> bool {
        if !self.hot_maps_warm {
            return self.facts.iter().any(|f| {
                self.hot_examine.bump();
                f.invalidated_at.is_none()
                    && f.subject == subject
                    && f.relation == relation
                    && f.object() == object
            });
        }
        match self.live_triples.get(&(subject, relation, object)) {
            Some(&idx) => self.facts[idx].invalidated_at.is_none(),
            None => false,
        }
    }

    /// Mark a fact slot invalidated and drop it from the live-triple lease.
    pub(crate) fn invalidate_fact_at(&mut self, idx: usize, at: impl Into<TransactionTime>) {
        self.facts[idx].invalidated_at = Some(at.into().as_u64());
        let fact = &self.facts[idx];
        let key = (fact.subject, fact.relation, fact.object());
        if self.live_triples.get(&key) == Some(&idx) {
            self.live_triples.remove(&key);
        }
    }

    /// Live Facts that support `claim` at an explicit cut (product facts only, D-05).
    pub fn live_supports(
        &self,
        claim: EventId,
        tt: impl Into<TransactionTime>,
        vt: impl Into<ValidTime>,
    ) -> Vec<&Fact> {
        let tt = tt.into().as_u64();
        let vt = vt.into().as_u64();
        if !self.hot_maps_warm {
            return self
                .facts
                .iter()
                .filter(|f| {
                    self.hot_examine.bump();
                    f.claim_id == claim
                        && f.is_live_at(tt, vt)
                        && !self.is_dictionary_rel(f.relation)
                })
                .collect();
        }
        let Some(idxs) = self.claim_facts.get(&claim) else {
            return Vec::new();
        };
        idxs.iter()
            .filter_map(|&idx| {
                self.hot_examine.bump();
                let fact = &self.facts[idx];
                (fact.is_live_at(tt, vt) && !self.is_dictionary_rel(fact.relation)).then_some(fact)
            })
            .collect()
    }

    pub fn live_support_count(
        &self,
        claim: EventId,
        tt: impl Into<TransactionTime>,
        vt: impl Into<ValidTime>,
    ) -> usize {
        self.live_supports(claim, tt, vt).len()
    }

    pub fn claim_supported_at(
        &self,
        claim: EventId,
        tt: impl Into<TransactionTime>,
        vt: impl Into<ValidTime>,
    ) -> bool {
        self.live_support_count(claim, tt, vt) > 0
    }

    /// Fold-cut allowlist: a live `kutha:allows` fact whose delivery_key is `name`.
    /// Warm path is O(facts for that name), not O(live product facts).
    pub fn relation_allowed_at(
        &self,
        name: &str,
        tt: impl Into<TransactionTime>,
        vt: impl Into<ValidTime>,
    ) -> bool {
        let tt = tt.into().as_u64();
        let vt = vt.into().as_u64();
        let Some(rel) = self.rel_allows else {
            return false;
        };
        if let Some(idxs) = self.allows_by_name.get(name) {
            return idxs.iter().copied().any(|idx| {
                self.hot_examine.bump();
                self.facts[idx].is_live_at(tt, vt)
            });
        }
        if !self.allows_by_name.is_empty() {
            self.hot_examine.bump();
            return false;
        }
        // Lease empty (replay before bind): scan only kutha:allows rows.
        self.facts.iter().any(|f| {
            self.hot_examine.bump();
            f.relation == rel && f.delivery_key.as_deref() == Some(name) && f.is_live_at(tt, vt)
        })
    }

    /// Fold-cut rule registry: a live `kutha:rule` fact whose delivery_key is `pin`.
    pub fn rule_hash_live_at(
        &self,
        pin: &str,
        tt: impl Into<TransactionTime>,
        vt: impl Into<ValidTime>,
    ) -> bool {
        let Some(rel) = self.rel_rule else {
            return false;
        };
        self.all_live_facts_at(tt, vt)
            .into_iter()
            .any(|f| f.relation == rel && f.delivery_key.as_deref() == Some(pin))
    }

    /// Fold-cut policy pin: a live `kutha:policy` fact whose delivery_key is `pin`.
    pub fn policy_hash_live_at(
        &self,
        pin: &str,
        tt: impl Into<TransactionTime>,
        vt: impl Into<ValidTime>,
    ) -> bool {
        let Some(rel) = self.rel_policy else {
            return false;
        };
        self.all_live_facts_at(tt, vt)
            .into_iter()
            .any(|f| f.relation == rel && f.delivery_key.as_deref() == Some(pin))
    }

    /// Latest ingested live policy hash at `(tt, vt)`.
    pub fn live_policy_pin_at(
        &self,
        tt: impl Into<TransactionTime>,
        vt: impl Into<ValidTime>,
    ) -> Option<String> {
        let rel = self.rel_policy?;
        self.all_live_facts_at(tt, vt)
            .into_iter()
            .filter(|f| f.relation == rel)
            .max_by_key(|f| f.ingested_at)
            .and_then(|f| f.delivery_key.clone())
    }

    /// Latest ingested live admission status for `justification_id` at `(tt, vt)`.
    pub fn admission_status_at(
        &self,
        justification_id: &str,
        tt: impl Into<TransactionTime>,
        vt: impl Into<ValidTime>,
    ) -> Option<bool> {
        self.live_admission_fact(justification_id, tt, vt)
            .map(|f| f.polarity == Some(SupportPolarity::Positive))
    }

    pub(crate) fn live_admission_at(
        &self,
        justification_id: &str,
        tt: impl Into<TransactionTime>,
        vt: impl Into<ValidTime>,
    ) -> Option<(bool, String)> {
        let f = self.live_admission_fact(justification_id, tt, vt)?;
        let admitted = f.polarity == Some(SupportPolarity::Positive);
        let policy = f.delivery_key.clone().unwrap_or_default();
        Some((admitted, policy))
    }

    fn live_admission_fact(
        &self,
        justification_id: &str,
        tt: impl Into<TransactionTime>,
        vt: impl Into<ValidTime>,
    ) -> Option<&Fact> {
        let tt = tt.into().as_u64();
        let vt = vt.into().as_u64();
        let rel = self.rel_admission?;
        if let Ok(eid) = justification_id.parse::<EventId>() {
            if let Some(idx) = self.fact_index_by_event_id(eid) {
                let f = &self.facts[idx];
                if f.relation == rel && f.is_live_at(tt, vt) {
                    return Some(f);
                }
            }
        }
        self.all_live_facts_at(tt, vt)
            .into_iter()
            .filter(|f| f.relation == rel && f.delivery_key.as_deref() == Some(justification_id))
            .max_by_key(|f| f.ingested_at)
    }

    pub(crate) fn live_action_fact(
        &self,
        justification_id: &str,
        tt: impl Into<TransactionTime>,
        vt: impl Into<ValidTime>,
    ) -> Option<&Fact> {
        let rel = self.rel_action?;
        self.all_live_facts_at(tt, vt)
            .into_iter()
            .filter(|f| f.relation == rel && f.delivery_key.as_deref() == Some(justification_id))
            .max_by_key(|f| f.ingested_at)
    }
}
