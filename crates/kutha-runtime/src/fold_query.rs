//! Fold query surface: as_of / live_at / dictionary / fingerprint / registry cuts.
use kutha_common::{EventId, SupportPolarity, TermId, TransactionTime, ValidTime, WallTime};
use sha2::{Digest, Sha256};

use crate::fold::{Fact, GraphFold, LiveFactFilter, leftover_halves, vt_intersect};

impl GraphFold {
    pub(crate) fn live_facts_at(
        &self,
        tt: impl Into<TransactionTime>,
        vt: impl Into<ValidTime>,
    ) -> Vec<&Fact> {
        self.collect_live_facts_at(tt, vt, LiveFactFilter::Product)
    }

    /// Live product facts of `rel` at `(tt, vt)`. Warm path is O(facts of that
    /// relation), not O(all live product facts). Point-cut sibling of
    /// [`Self::facts_overlapping_relation`] (Horn discovery uses the latter).
    #[allow(dead_code)] // retained point-cut API; Horn path is overlap-aware (M097)
    pub(crate) fn live_facts_for_relation(
        &self,
        rel: TermId,
        tt: impl Into<TransactionTime>,
        vt: impl Into<ValidTime>,
    ) -> Vec<&Fact> {
        let tt = tt.into().as_u64();
        let vt = vt.into().as_u64();
        if !self.hot_maps_warm {
            return self
                .live_facts_at(tt, vt)
                .into_iter()
                .filter(|f| f.relation == rel)
                .collect();
        }
        let Some(idxs) = self.facts_by_relation.get(&rel) else {
            return Vec::new();
        };
        idxs.iter()
            .filter_map(|&idx| {
                let fact = &self.facts[idx];
                (fact.is_live_at(tt, vt) && !self.is_dictionary_rel(fact.relation)).then_some(fact)
            })
            .collect()
    }

    /// TT-live product facts of `rel` whose VT overlaps `[win_from, win_to)`.
    /// Used by multi-premise Horn discovery so a later-starting premise is not
    /// missed when the trigger's `valid_from` is earlier (M097 / CE #4).
    pub(crate) fn facts_overlapping_relation(
        &self,
        rel: TermId,
        tt: impl Into<TransactionTime>,
        win_from: u64,
        win_to: Option<u64>,
    ) -> Vec<&Fact> {
        let tt = tt.into().as_u64();
        let overlaps = |f: &Fact| {
            f.is_known_at(tt)
                && !self.is_dictionary_rel(f.relation)
                && f.relation == rel
                && vt_intersect(f.valid_from, f.valid_to, win_from, win_to).is_some()
        };
        if !self.hot_maps_warm {
            return self.facts.iter().filter(|f| overlaps(f)).collect();
        }
        let Some(idxs) = self.facts_by_relation.get(&rel) else {
            return Vec::new();
        };
        idxs.iter()
            .filter_map(|&idx| {
                let fact = &self.facts[idx];
                overlaps(fact).then_some(fact)
            })
            .collect()
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
    ///
    /// Takes [`ValidTime`] (not `impl Into`): bare `u64` and wrong-role clocks
    /// (`WallTime` / `TransactionTime`) do not compile (M025 S02 / G-C09).
    pub fn as_of(&self, vt: ValidTime) -> Vec<(TermId, TermId, TermId)> {
        self.live_at(u64::MAX, vt)
    }

    /// Product triples ingested (and not invalidated) at `tt`, ignoring valid-time.
    ///
    /// Closed VT intervals remain visible. Dictionary (`kutha:`) relations are excluded.
    /// When hot maps are warm, only facts with `ingested_at <= tt` are examined
    /// (`tt_by_ingest`); Lazy cold path still scans every fact until rebuild.
    pub fn known_at(&self, tt: impl Into<TransactionTime>) -> Vec<(TermId, TermId, TermId)> {
        let tt = tt.into().as_u64();
        let keep = |f: &Fact| LiveFactFilter::Product.keep(self, f);
        if !self.hot_maps_warm {
            return self
                .facts
                .iter()
                .filter(|f| {
                    self.hot_examine.bump();
                    f.is_known_at(tt) && keep(f)
                })
                .map(|f| (f.subject, f.relation, f.object()))
                .collect();
        }
        self.tt_by_ingest
            .range(..=tt)
            .flat_map(|(_, idxs)| idxs.iter().copied())
            .filter_map(|idx| {
                self.hot_examine.bump();
                let fact = &self.facts[idx];
                (fact.is_known_at(tt) && keep(fact)).then_some((
                    fact.subject,
                    fact.relation,
                    fact.object(),
                ))
            })
            .collect()
    }

    /// Greatest `TtAnchor` wall ≤ `wall` → knowledge transaction time.
    ///
    /// Shared by [`Self::as_known_on`] and [`crate::Runtime::norm_view`].
    pub fn knowledge_tt_at(&self, wall: WallTime) -> Option<u64> {
        let wall = wall.as_u64();
        self.tt_anchors
            .range(..=wall)
            .next_back()
            .map(|(_, &tt)| tt)
    }

    /// Product triples as known on a wall date.
    ///
    /// Greatest `Op::TtAnchor` with `wall` at or before `wall`, then
    /// [`Self::known_at`] at that transaction time (TT-only; no ValidTime cut).
    /// `None` means no anchor at or before the date.
    ///
    /// Takes [`WallTime`] (not `impl Into`): bare `u64` and wrong-role clocks
    /// (`ValidTime` / `TransactionTime`) do not compile (M025 S02 / G-C09).
    pub fn as_known_on(&self, wall: WallTime) -> Option<Vec<(TermId, TermId, TermId)>> {
        let tt = self.knowledge_tt_at(wall)?;
        Some(self.known_at(tt))
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
    /// Falls back to a dead row so callers can distinguish "never known" from "dead".
    /// User Retract/Correct gates must use [`Self::live_fact_indices_by_event_id`].
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

    /// Every live residual minted under `event_id` (CorrectInterval may share one id).
    pub(crate) fn live_fact_indices_by_event_id(&self, event_id: EventId) -> Vec<usize> {
        if !self.hot_maps_warm {
            return self
                .facts
                .iter()
                .enumerate()
                .filter(|(_, f)| {
                    self.hot_examine.bump();
                    f.event_id == event_id && f.invalidated_at.is_none()
                })
                .map(|(i, _)| i)
                .collect();
        }
        match self.fact_by_event.get(&event_id) {
            Some(idxs) => idxs
                .iter()
                .copied()
                .filter(|&i| self.facts[i].invalidated_at.is_none())
                .collect(),
            None => Vec::new(),
        }
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

    /// Live triple presence whose VT fully covers `valid_from`/`valid_to` (Horn dedup).
    pub(crate) fn has_live_triple(
        &self,
        subject: TermId,
        relation: TermId,
        object: TermId,
        valid_from: u64,
        valid_to: Option<u64>,
    ) -> bool {
        self.has_live_triple_except(
            subject,
            relation,
            object,
            valid_from,
            valid_to,
            &std::collections::HashSet::new(),
        )
    }

    /// Like [`Self::has_live_triple`], ignoring facts whose `event_id` is in `except`
    /// (same-batch retracts scheduled before re-fire). Dedup is cover-aware: only a
    /// live residual whose VT fully covers the candidate window blocks. Partial
    /// overlap emits leftover residuals ([`Self::uncovered_vt_windows`]).
    pub(crate) fn has_live_triple_except(
        &self,
        subject: TermId,
        relation: TermId,
        object: TermId,
        valid_from: u64,
        valid_to: Option<u64>,
        except: &std::collections::HashSet<EventId>,
    ) -> bool {
        let covers = |f: &crate::fold::Fact| {
            f.invalidated_at.is_none()
                && !except.contains(&f.event_id)
                && f.subject == subject
                && f.relation == relation
                && f.object() == object
                && vt_fully_covers(f.valid_from, f.valid_to, valid_from, valid_to)
        };
        if !self.hot_maps_warm {
            return self.facts.iter().any(|f| {
                self.hot_examine.bump();
                covers(f)
            });
        }
        match self.live_triples.get(&(subject, relation, object)) {
            Some(list) => list.iter().any(|&idx| {
                self.hot_examine.bump();
                covers(&self.facts[idx])
            }),
            None => false,
        }
    }

    /// Candidate VT minus live same-triple windows (except scheduled retracts).
    /// Empty when a live row fully covers the candidate (see [`Self::has_live_triple_except`]).
    pub(crate) fn uncovered_vt_windows(
        &self,
        subject: TermId,
        relation: TermId,
        object: TermId,
        valid_from: u64,
        valid_to: Option<u64>,
        except: &std::collections::HashSet<EventId>,
    ) -> Vec<(u64, Option<u64>)> {
        let covered = if except.is_empty() {
            self.has_live_triple(subject, relation, object, valid_from, valid_to)
        } else {
            self.has_live_triple_except(subject, relation, object, valid_from, valid_to, except)
        };
        if covered {
            return Vec::new();
        }
        let same = |f: &crate::fold::Fact| {
            f.invalidated_at.is_none()
                && !except.contains(&f.event_id)
                && f.subject == subject
                && f.relation == relation
                && f.object() == object
        };
        let mut windows = vec![(valid_from, valid_to)];
        if !self.hot_maps_warm {
            for f in &self.facts {
                self.hot_examine.bump();
                if same(f) {
                    subtract_vt_window(&mut windows, f.valid_from, f.valid_to);
                    if windows.is_empty() {
                        break;
                    }
                }
            }
            return windows;
        }
        if let Some(list) = self.live_triples.get(&(subject, relation, object)) {
            for &idx in list {
                self.hot_examine.bump();
                let f = &self.facts[idx];
                if same(f) {
                    subtract_vt_window(&mut windows, f.valid_from, f.valid_to);
                    if windows.is_empty() {
                        break;
                    }
                }
            }
        }
        windows
    }

    /// Mark a fact slot invalidated and drop it from the live-triple lease.
    pub(crate) fn invalidate_fact_at(&mut self, idx: usize, at: impl Into<TransactionTime>) {
        self.unindex_horn_slot(idx);
        self.facts[idx].invalidated_at = Some(at.into().as_u64());
        let fact = &self.facts[idx];
        let key = (fact.subject, fact.relation, fact.object());
        if let Some(list) = self.live_triples.get_mut(&key) {
            list.retain(|i| *i != idx);
            if list.is_empty() {
                self.live_triples.remove(&key);
            }
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

    /// Whether any not-yet-invalidated Fact of `claim` on `relation` carries
    /// `delivery_key == key`.
    ///
    /// `Correct` / `CorrectInterval` replace the minting fact with rows under their
    /// own event id but keep `claim_id`, `relation`, and `delivery_key`, so this
    /// follows the delivery through corrections (M023 Phase 75). Liveness is
    /// relation-scoped: a sibling relation sharing claim+key must not keep a
    /// retracted support short-circuitable (M056 / CE #26).
    pub(crate) fn has_live_keyed_support(
        &self,
        claim: EventId,
        relation: TermId,
        key: &str,
    ) -> bool {
        let keyed = |f: &Fact| {
            f.relation == relation
                && f.invalidated_at.is_none()
                && f.delivery_key.as_deref() == Some(key)
        };
        if !self.hot_maps_warm {
            return self.facts.iter().any(|f| {
                self.hot_examine.bump();
                f.claim_id == claim && keyed(f)
            });
        }
        let Some(idxs) = self.claim_facts.get(&claim) else {
            return false;
        };
        idxs.iter().any(|&idx| {
            self.hot_examine.bump();
            keyed(&self.facts[idx])
        })
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
        let tt = tt.into().as_u64();
        let vt = vt.into().as_u64();
        if self.rel_rule.is_none() {
            return false;
        }
        if let Some(idxs) = self.rules_by_pin.get(pin) {
            return idxs.iter().copied().any(|idx| {
                self.hot_examine.bump();
                self.facts[idx].is_live_at(tt, vt)
            });
        }
        self.hot_examine.bump();
        false
    }

    /// Fold-cut policy pin: a live `kutha:policy` fact whose delivery_key is `pin`.
    pub fn policy_hash_live_at(
        &self,
        pin: &str,
        tt: impl Into<TransactionTime>,
        vt: impl Into<ValidTime>,
    ) -> bool {
        let tt = tt.into().as_u64();
        let vt = vt.into().as_u64();
        if self.rel_policy.is_none() {
            return false;
        }
        if let Some(idxs) = self.policies_by_pin.get(pin) {
            return idxs.iter().copied().any(|idx| {
                self.hot_examine.bump();
                self.facts[idx].is_live_at(tt, vt)
            });
        }
        self.hot_examine.bump();
        false
    }

    /// Latest ingested live policy hash at `(tt, vt)`.
    pub fn live_policy_pin_at(
        &self,
        tt: impl Into<TransactionTime>,
        vt: impl Into<ValidTime>,
    ) -> Option<String> {
        let tt = tt.into().as_u64();
        let vt = vt.into().as_u64();
        self.rel_policy?;
        for &idx in self.policy_by_ingest.iter().rev() {
            self.hot_examine.bump();
            let f = &self.facts[idx];
            if f.is_live_at(tt, vt) {
                return f.delivery_key.clone();
            }
        }
        None
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

/// Half-open `[exist_from, exist_to)` fully covers `[cand_from, cand_to)`.
fn vt_fully_covers(
    exist_from: u64,
    exist_to: Option<u64>,
    cand_from: u64,
    cand_to: Option<u64>,
) -> bool {
    exist_from <= cand_from && exist_to.unwrap_or(u64::MAX) >= cand_to.unwrap_or(u64::MAX)
}

/// Subtract `[cut_from, cut_to)` from each remaining window via [`leftover_halves`].
fn subtract_vt_window(windows: &mut Vec<(u64, Option<u64>)>, cut_from: u64, cut_to: Option<u64>) {
    let mut next = Vec::new();
    for &(w_from, w_to) in windows.iter() {
        match vt_intersect(w_from, w_to, cut_from, cut_to) {
            None => next.push((w_from, w_to)),
            Some((i_from, i_to)) => {
                let (prefix, suffix) = leftover_halves(w_from, w_to, i_from, i_to);
                if let Some(p) = prefix {
                    next.push(p);
                }
                if let Some(s) = suffix {
                    next.push(s);
                }
            }
        }
    }
    *windows = next;
}

/// Fact-set picture diff: what turns fold `a` into fold `b` (ADR-061).
/// Identity is `(event_id, valid_from, valid_to)` plus rank; `seq` does not participate.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct FoldDiff {
    pub added: Vec<Fact>,
    pub removed: Vec<Fact>,
    pub changed: Vec<FactChange>,
}

/// One `(key, rank)` pair whose non-`seq` fields differ between the two folds.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct FactChange {
    pub before: Fact,
    pub after: Fact,
}

/// What turns `a` into `b`. Compile-only empty stub (D-01); real query replaces this.
pub fn diff_folds(_a: &GraphFold, _b: &GraphFold) -> FoldDiff {
    FoldDiff::default()
}

impl GraphFold {
    /// Thin wrapper around [`diff_folds`].
    pub fn diff(&self, other: &GraphFold) -> FoldDiff {
        diff_folds(self, other)
    }
}
