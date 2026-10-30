//! Horn evaluation over live product triples (M016 S03 / RULE-02).
//! Named bodies are pins only; they do not auto-fire.

use crate::fold::{GraphFold, leftover_halves, vt_intersect};
use crate::quantum::{Runtime, RuntimeError};
use kutha_common::{Event, EventId, Op, RuleAst, RuleAtom, RuleTerm, TermDictionary, TermId};
use std::collections::HashSet;

const MAX_VAR: usize = 16;

pub(crate) fn inverse_knows_rule() -> RuleAst {
    let head = RuleAtom::new(
        "knownBy",
        RuleTerm::var(1).expect("v1"),
        RuleTerm::var(0).expect("v0"),
    )
    .expect("head");
    let prem = RuleAtom::new(
        "knows",
        RuleTerm::var(0).expect("v0"),
        RuleTerm::var(1).expect("v1"),
    )
    .expect("prem");
    RuleAst::horn(head, vec![prem]).expect("inverse Horn")
}

type Bind = [Option<TermId>; MAX_VAR];

fn pin_window_overlaps(f: &crate::fold::Fact, win_from: u64, win_to: Option<u64>) -> bool {
    f.is_known_at(u64::MAX) && vt_intersect(f.valid_from, f.valid_to, win_from, win_to).is_some()
}

impl Runtime {
    pub(crate) fn rule_follow_ons(&self, event: &Event) -> Result<Vec<Event>, RuntimeError> {
        match &event.op {
            Op::Assert {
                subject,
                relation,
                object,
                valid_from,
                valid_to,
                ..
            }
            | Op::Behavior {
                subject,
                relation,
                object,
                valid_from,
                valid_to,
                ..
            } => self.horn_fires(
                event.id,
                event.ingested_at,
                *subject,
                *relation,
                *object,
                *valid_from,
                *valid_to,
            ),
            Op::Retract { event_id } => Ok(self.retract_derived(event, *event_id)),
            Op::Correct {
                event_id,
                object,
                valid_from,
                valid_to,
            } => {
                // Mirror CorrectInterval: except-set + fires-then-retracts so a
                // same-triple Correct (e.g. VT-only) is not suppressed by the
                // still-live old derived row before LIFO applies the retract.
                let retracts = self.retract_derived(event, *event_id);
                let except: HashSet<EventId> = retracts
                    .iter()
                    .filter_map(|e| match &e.op {
                        Op::Retract { event_id: rid } => Some(*rid),
                        _ => None,
                    })
                    .collect();
                let mut fires = Vec::new();
                for idx in self.fold.live_fact_indices_by_event_id(*event_id) {
                    let fact = &self.fold.facts()[idx];
                    let (s, r) = (fact.subject, fact.relation);
                    fires.extend(self.horn_fires_except(
                        event.id,
                        event.ingested_at,
                        s,
                        r,
                        *object,
                        *valid_from,
                        *valid_to,
                        &except,
                    )?);
                }
                let mut out = fires;
                out.extend(retracts);
                Ok(out)
            }
            Op::CorrectInterval {
                event_id,
                object,
                patch_from,
                patch_to,
            } => {
                let retracts = self.retract_derived(event, *event_id);
                let except: HashSet<EventId> = retracts
                    .iter()
                    .filter_map(|e| match &e.op {
                        Op::Retract { event_id: rid } => Some(*rid),
                        _ => None,
                    })
                    .collect();
                let mut fires = Vec::new();
                for idx in self.fold.live_fact_indices_by_event_id(*event_id) {
                    let fact = &self.fold.facts()[idx];
                    let (s, r, old_o) = (fact.subject, fact.relation, fact.object());
                    if let Some((inter_from, inter_to)) =
                        vt_intersect(fact.valid_from, fact.valid_to, *patch_from, *patch_to)
                    {
                        let (prefix, suffix) =
                            leftover_halves(fact.valid_from, fact.valid_to, inter_from, inter_to);
                        if let Some((vf, vt)) = prefix {
                            fires.extend(self.horn_fires_except(
                                event.id,
                                event.ingested_at,
                                s,
                                r,
                                old_o,
                                vf,
                                vt,
                                &except,
                            )?);
                        }
                        fires.extend(self.horn_fires_except(
                            event.id,
                            event.ingested_at,
                            s,
                            r,
                            *object,
                            inter_from,
                            inter_to,
                            &except,
                        )?);
                        if let Some((vf, vt)) = suffix {
                            fires.extend(self.horn_fires_except(
                                event.id,
                                event.ingested_at,
                                s,
                                r,
                                old_o,
                                vf,
                                vt,
                                &except,
                            )?);
                        }
                    }
                }
                // Fires first, retracts last → LIFO cascade applies retracts before re-fires.
                let mut out = fires;
                out.extend(retracts);
                Ok(out)
            }
            _ => Ok(vec![]),
        }
    }

    fn retract_derived(&self, trigger: &Event, premise: EventId) -> Vec<Event> {
        let mut out = Vec::new();
        let mut retracting = HashSet::new();
        let children: Vec<EventId> = self
            .derived_by_cause
            .get(&premise)
            .cloned()
            .unwrap_or_default();
        for id in children {
            self.retract_examine.bump();
            let Some((event_id, caused_by, rule_version)) = self.log_event(id).and_then(|e| {
                let Op::Behavior {
                    caused_by,
                    rule_version,
                    ..
                } = &e.op
                else {
                    return None;
                };
                Some((e.id, *caused_by, rule_version.clone()))
            }) else {
                continue;
            };
            if caused_by != premise {
                continue;
            }
            let Some(bytes) = self.fold.blobs.get(&rule_version) else {
                continue;
            };
            if !matches!(RuleAst::from_canonical(bytes), Some(RuleAst::Horn { .. })) {
                continue;
            }
            if self.behavior_fact_live(event_id) && retracting.insert(event_id) {
                out.push(Event::stable_derived(
                    Op::Retract { event_id },
                    trigger.ingested_at,
                ));
            }
        }
        // Keep a single caused_by. Also drop live Horn heads whose body no
        // longer matches after treating P as dead (non-trigger witness).
        // Skip P itself: the trigger Retract already names that id; a second
        // DET-02-stable copy would leave drop-tamper stamps unchanged.
        // Candidates are Behaviors that listed P as a supporting premise fact
        // (behaviors_by_premise_fact), not the same-rel relation bucket (M139).
        // Single-premise Horns depend only on caused_by — phase 1 already
        // covered them; arity skip avoids re-examining them when also indexed
        // under caused_by (M136).
        let except = HashSet::from([premise]);
        let candidates: Vec<EventId> = self
            .behaviors_by_premise_fact
            .get(&premise)
            .cloned()
            .unwrap_or_default();
        for id in candidates {
            if let Some(e) = self.log_event(id) {
                let Op::Behavior { rule_version, .. } = &e.op else {
                    continue;
                };
                let event_id = e.id;
                if event_id == premise
                    || retracting.contains(&event_id)
                    || !self.behavior_fact_live(event_id)
                {
                    continue;
                }
                let Some(bytes) = self.fold.blobs.get(rule_version) else {
                    continue;
                };
                if let Some(RuleAst::Horn { premises, .. }) = RuleAst::from_canonical(bytes) {
                    if premises.len() <= 1 {
                        continue;
                    }
                }
                self.retract_examine.bump();
                if self.horn_body_matches_except(e, &except) {
                    continue;
                }
                retracting.insert(event_id);
                out.push(Event::stable_derived(
                    Op::Retract { event_id },
                    trigger.ingested_at,
                ));
            }
        }
        out
    }

    /// Supporting premise EventIds for a live Horn Behavior (droppable lease).
    /// Always includes `caused_by`; multi-premise adds non-trigger witnesses via
    /// re-search under the head VT (M139).
    pub(crate) fn supporting_premise_ids_for_behavior(&self, e: &Event) -> Vec<EventId> {
        let Op::Behavior {
            caused_by,
            rule_version,
            subject,
            relation,
            object,
            valid_from,
            valid_to,
            ..
        } = &e.op
        else {
            return Vec::new();
        };
        let mut out = vec![*caused_by];
        let Some(bytes) = self.fold.blobs.get(rule_version) else {
            return out;
        };
        let Some(RuleAst::Horn { head, premises }) = RuleAst::from_canonical(bytes) else {
            return out;
        };
        if premises.len() <= 1 {
            return out;
        }
        let Some(trigger) = self.fold.facts().iter().find(|f| f.event_id == *caused_by) else {
            return out;
        };
        for (i, prem) in premises.iter().enumerate() {
            let Some(prem_rel) = self.dict.id(&prem.relation) else {
                continue;
            };
            if prem_rel != trigger.relation {
                continue;
            }
            let mut bind = [None; MAX_VAR];
            if !unify_term(&prem.subject, trigger.subject, &self.dict, &mut bind) {
                continue;
            }
            if !unify_term(&prem.object, trigger.object(), &self.dict, &mut bind) {
                continue;
            }
            let hits = search_all(
                self,
                &self.fold,
                &premises,
                i,
                bind,
                *valid_from,
                *valid_to,
                u64::MAX,
                &HashSet::new(),
            );
            for hit in hits {
                let Some(hs) = instantiate(&head.subject, &hit.bind, &self.dict) else {
                    continue;
                };
                let Some(ho) = instantiate(&head.object, &hit.bind, &self.dict) else {
                    continue;
                };
                let Some(hr) = self.dict.id(&head.relation) else {
                    continue;
                };
                if hs != *subject || hr != *relation || ho != *object {
                    continue;
                }
                for id in hit.premise_ids {
                    if !out.contains(&id) {
                        out.push(id);
                    }
                }
                return out;
            }
        }
        out
    }

    pub(crate) fn log_event(&self, id: EventId) -> Option<&Event> {
        if let Some(pos) = self.by_event_id.get(&id) {
            return self.log.as_slice().get(*pos);
        }
        self.compacted_behavior_by_id
            .get(&id)
            .or_else(|| self.compacted_assert_by_id.get(&id))
    }

    pub(crate) fn behavior_fact_live(&self, event_id: EventId) -> bool {
        self.fold
            .fact_index_by_event_id(event_id)
            .and_then(|i| self.fold.facts().get(i))
            .is_some_and(|f| f.invalidated_at.is_none())
    }

    /// Horn body still matches over `head` VT with `except` treated as dead.
    /// Named bodies are skipped (treated as still matching).
    fn horn_body_matches_except(&self, e: &Event, except: &HashSet<EventId>) -> bool {
        let Op::Behavior {
            rule_version,
            subject,
            relation,
            object,
            valid_from,
            valid_to,
            ..
        } = &e.op
        else {
            return true;
        };
        let Some(bytes) = self.fold.blobs.get(rule_version) else {
            return true;
        };
        let Some(ast) = RuleAst::from_canonical(bytes) else {
            return true;
        };
        let RuleAst::Horn { head, premises } = ast else {
            return true;
        };
        let Some(head_rel) = self.dict.id(&head.relation) else {
            return true;
        };
        if head_rel != *relation {
            return true;
        }
        let mut bind = [None; MAX_VAR];
        if !unify_term(&head.subject, *subject, &self.dict, &mut bind) {
            return true;
        }
        if !unify_term(&head.object, *object, &self.dict, &mut bind) {
            return true;
        }
        search_all(
            self,
            &self.fold,
            &premises,
            premises.len(),
            bind,
            *valid_from,
            *valid_to,
            u64::MAX,
            except,
        )
        .iter()
        .any(|hit| windows_cover(&hit.windows, *valid_from, *valid_to))
    }

    fn horn_fires(
        &self,
        caused_by: EventId,
        ingested_at: u64,
        subject: TermId,
        relation: TermId,
        object: TermId,
        valid_from: u64,
        valid_to: Option<u64>,
    ) -> Result<Vec<Event>, RuntimeError> {
        self.horn_fires_except(
            caused_by,
            ingested_at,
            subject,
            relation,
            object,
            valid_from,
            valid_to,
            &HashSet::new(),
        )
    }

    fn horn_fires_except(
        &self,
        caused_by: EventId,
        ingested_at: u64,
        subject: TermId,
        relation: TermId,
        object: TermId,
        valid_from: u64,
        valid_to: Option<u64>,
        except: &HashSet<EventId>,
    ) -> Result<Vec<Event>, RuntimeError> {
        let mut out = Vec::new();
        let no_dead = HashSet::new();
        for horn in self.live_horns(valid_from, valid_to, relation) {
            let RuleAst::Horn { head, premises } = horn.ast else {
                continue;
            };
            for (i, prem) in premises.iter().enumerate() {
                let Some(prem_rel) = self.dict.id(&prem.relation) else {
                    continue;
                };
                if prem_rel != relation {
                    continue;
                }
                let mut bind = [None; MAX_VAR];
                if !unify_term(&prem.subject, subject, &self.dict, &mut bind) {
                    continue;
                }
                if !unify_term(&prem.object, object, &self.dict, &mut bind) {
                    continue;
                }
                let hits = search_all(
                    self,
                    &self.fold,
                    &premises,
                    i,
                    bind,
                    valid_from,
                    valid_to,
                    u64::MAX,
                    &no_dead,
                );
                if hits.is_empty() {
                    continue;
                }
                if self.dict.id(&head.relation).is_none() {
                    return Err(RuntimeError::UnknownRelation {
                        name: head.relation.clone(),
                    });
                }
                for hit in hits {
                    let mut parts = vec![(valid_from, valid_to), (horn.pin_from, horn.pin_to)];
                    parts.extend(hit.windows);
                    let Some((vf, vt)) = conjunct_vt(parts) else {
                        continue;
                    };
                    out.extend(self.head_event(
                        &head,
                        &horn.pin,
                        &hit.bind,
                        caused_by,
                        ingested_at,
                        vf,
                        vt,
                        except,
                    ));
                }
            }
        }
        Ok(out)
    }

    /// Horn pins whose valid-time overlaps `[win_from, win_to)` and that are
    /// still known at the transaction tip. A pin that starts after the trigger
    /// is kept; `horn_fires_except` conjuncts the pin window with the premises.
    fn live_horns(&self, win_from: u64, win_to: Option<u64>, relation: TermId) -> Vec<LiveHorn> {
        if self.fold.rel_rule.is_none() {
            return vec![];
        }
        let Some(name) = self.dict.lookup(relation) else {
            return vec![];
        };
        if let Some(slots) = self.fold.horns_by_premise.get(name) {
            let mut out = Vec::new();
            for slot in slots {
                self.fold.horn_examine.bump();
                let f = &self.fold.facts()[slot.fact_idx];
                if !pin_window_overlaps(f, win_from, win_to) {
                    continue;
                }
                out.push(LiveHorn {
                    pin: slot.pin.clone(),
                    ast: slot.ast.clone(),
                    pin_from: f.valid_from,
                    pin_to: f.valid_to,
                });
            }
            return out;
        }
        if self.fold.horn_lease_ready {
            return vec![];
        }
        self.live_horns_scan(win_from, win_to)
    }

    fn live_horns_scan(&self, win_from: u64, win_to: Option<u64>) -> Vec<LiveHorn> {
        let Some(rel) = self.fold.rel_rule else {
            return vec![];
        };
        let mut out = Vec::new();
        for f in self.fold.facts() {
            self.fold.horn_examine.bump();
            if f.relation != rel || !pin_window_overlaps(f, win_from, win_to) {
                continue;
            }
            let Some(pin) = f.delivery_key.as_deref() else {
                continue;
            };
            let Some(bytes) = self.fold.blobs.get(pin) else {
                continue;
            };
            let Some(ast) = RuleAst::from_canonical(bytes) else {
                continue;
            };
            if matches!(ast, RuleAst::Horn { .. }) {
                out.push(LiveHorn {
                    pin: pin.to_string(),
                    ast,
                    pin_from: f.valid_from,
                    pin_to: f.valid_to,
                });
            }
        }
        out
    }

    fn head_event(
        &self,
        head: &RuleAtom,
        pin: &str,
        bind: &Bind,
        caused_by: EventId,
        ingested_at: u64,
        valid_from: u64,
        valid_to: Option<u64>,
        except: &HashSet<EventId>,
    ) -> Vec<Event> {
        let Some(rel) = self.dict.id(&head.relation) else {
            return Vec::new();
        };
        let Some(subject) = instantiate(&head.subject, bind, &self.dict) else {
            return Vec::new();
        };
        let Some(object) = instantiate(&head.object, bind, &self.dict) else {
            return Vec::new();
        };
        let windows = self
            .fold
            .uncovered_vt_windows(subject, rel, object, valid_from, valid_to, except);
        let name = if head.relation == "knownBy" {
            "inverse_knows".into()
        } else {
            head.relation.clone()
        };
        windows
            .into_iter()
            .map(|(vf, vt)| {
                Event::stable_derived(
                    Op::Behavior {
                        name: name.clone(),
                        caused_by,
                        rule_version: pin.to_string(),
                        subject,
                        relation: rel,
                        object,
                        valid_from: vf,
                        valid_to: vt,
                    },
                    ingested_at,
                )
            })
            .collect()
    }

    /// Horn-pinned Behavior is reproducible from premises overlapping head VT.
    /// Named bodies skip. Missing blob or unmatched Horn fails closed (RULE-04).
    pub(crate) fn horn_behavior_reproducible(&self, e: &Event) -> bool {
        match self.covering_horn_hits_in(&self.fold, e) {
            None => true,
            Some(hits) => !hits.is_empty(),
        }
    }

    /// Covering `search_all` hits for a Horn Behavior (same window rule as
    /// [`Self::horn_behavior_reproducible`]). `None` = named / non-Horn skip.
    /// `Some([])` = missing blob, unparseable body, unmatched head, or no covering hit.
    fn covering_horn_hits_in(&self, fold: &GraphFold, e: &Event) -> Option<Vec<PremMatch>> {
        let Op::Behavior {
            rule_version,
            subject,
            relation,
            object,
            valid_from,
            valid_to,
            ..
        } = &e.op
        else {
            return None;
        };
        let Some(bytes) = fold.blobs.get(rule_version) else {
            return Some(Vec::new());
        };
        let Some(ast) = RuleAst::from_canonical(bytes) else {
            return Some(Vec::new());
        };
        let RuleAst::Horn { head, premises } = ast else {
            return None;
        };
        let Some(head_rel) = self.dict.id(&head.relation) else {
            return Some(Vec::new());
        };
        if head_rel != *relation {
            return Some(Vec::new());
        }
        let mut bind = [None; MAX_VAR];
        if !unify_term(&head.subject, *subject, &self.dict, &mut bind) {
            return Some(Vec::new());
        }
        if !unify_term(&head.object, *object, &self.dict, &mut bind) {
            return Some(Vec::new());
        }
        Some(
            search_all(
                self,
                fold,
                &premises,
                premises.len(),
                bind,
                *valid_from,
                *valid_to,
                e.ingested_at,
                &HashSet::new(),
            )
            .into_iter()
            .filter(|hit| windows_cover(&hit.windows, *valid_from, *valid_to))
            .collect(),
        )
    }

    /// Compacted Horn: a covering hit's `premise_ids` must contain `caused_by`.
    /// Named / non-Horn skip. Missing blob or no covering hit:
    /// `ExecutionReplayFailed`. Covering hit without that cause: `BrokenLineage`.
    pub(crate) fn compacted_horn_cause(
        &self,
        fold: &GraphFold,
        e: &Event,
    ) -> Result<(), RuntimeError> {
        let Some(hits) = self.covering_horn_hits_in(fold, e) else {
            return Ok(());
        };
        if hits.is_empty() {
            return Err(RuntimeError::ExecutionReplayFailed { event_id: e.id });
        }
        let Op::Behavior { caused_by, .. } = &e.op else {
            return Ok(());
        };
        if hits.iter().any(|hit| hit.premise_ids.contains(caused_by)) {
            Ok(())
        } else {
            Err(RuntimeError::BrokenLineage {
                caused_by: *caused_by,
            })
        }
    }
}

struct LiveHorn {
    pin: String,
    ast: RuleAst,
    pin_from: u64,
    pin_to: Option<u64>,
}

struct PremMatch {
    bind: Bind,
    windows: Vec<(u64, Option<u64>)>,
    /// Supporting premise fact EventIds (droppable index input; M139).
    premise_ids: Vec<EventId>,
}

fn conjunct_vt(
    windows: impl IntoIterator<Item = (u64, Option<u64>)>,
) -> Option<(u64, Option<u64>)> {
    let mut acc: Option<(u64, Option<u64>)> = None;
    for w in windows {
        acc = Some(match acc {
            None => w,
            Some((a, b)) => vt_intersect(a, b, w.0, w.1)?,
        });
    }
    acc
}

/// `windows` conjunction covers `[from, to)`. Empty windows cover any head window.
fn windows_cover(windows: &[(u64, Option<u64>)], from: u64, to: Option<u64>) -> bool {
    if windows.is_empty() {
        return true;
    }
    let Some((cf, ct)) = conjunct_vt(windows.iter().copied()) else {
        return false;
    };
    vt_intersect(cf, ct, from, to) == Some((from, to))
}

fn unify_term(term: &RuleTerm, id: TermId, dict: &TermDictionary, bind: &mut Bind) -> bool {
    match term {
        RuleTerm::Var(v) => {
            let i = *v as usize;
            if i >= MAX_VAR {
                return false;
            }
            match bind[i] {
                Some(existing) => existing == id,
                None => {
                    bind[i] = Some(id);
                    true
                }
            }
        }
        RuleTerm::Const(s) => dict.id(s) == Some(id),
    }
}

fn instantiate(term: &RuleTerm, bind: &Bind, dict: &TermDictionary) -> Option<TermId> {
    match term {
        RuleTerm::Var(v) => bind.get(*v as usize).copied().flatten(),
        RuleTerm::Const(s) => dict.id(s),
    }
}

fn search_all(
    rt: &Runtime,
    fold: &GraphFold,
    premises: &[RuleAtom],
    skip: usize,
    bind: Bind,
    win_from: u64,
    win_to: Option<u64>,
    tt: u64,
    except: &HashSet<EventId>,
) -> Vec<PremMatch> {
    let mut out = Vec::new();
    search_at_all(
        rt,
        fold,
        premises,
        0,
        skip,
        bind,
        win_from,
        win_to,
        tt,
        Vec::new(),
        except,
        &mut out,
    );
    out
}

fn search_at_all(
    rt: &Runtime,
    fold: &GraphFold,
    premises: &[RuleAtom],
    idx: usize,
    skip: usize,
    bind: Bind,
    win_from: u64,
    win_to: Option<u64>,
    tt: u64,
    windows: Vec<(u64, Option<u64>)>,
    except: &HashSet<EventId>,
    out: &mut Vec<PremMatch>,
) {
    if idx >= premises.len() {
        out.push(PremMatch {
            bind,
            windows,
            premise_ids: Vec::new(),
        });
        return;
    }
    if idx == skip {
        search_at_all(
            rt,
            fold,
            premises,
            idx + 1,
            skip,
            bind,
            win_from,
            win_to,
            tt,
            windows,
            except,
            out,
        );
        return;
    }
    let Some(prem) = premises.get(idx) else {
        return;
    };
    let Some(rel) = rt.dict.id(&prem.relation) else {
        return;
    };
    for f in fold.facts_overlapping_relation(rel, tt, win_from, win_to) {
        fold.horn_examine.bump();
        if except.contains(&f.event_id) {
            continue;
        }
        if f.relation != rel {
            continue;
        }
        let mut next = bind;
        if !unify_term(&prem.subject, f.subject, &rt.dict, &mut next) {
            continue;
        }
        if !unify_term(&prem.object, f.object(), &rt.dict, &mut next) {
            continue;
        }
        let mut next_windows = windows.clone();
        next_windows.push((f.valid_from, f.valid_to));
        let before = out.len();
        search_at_all(
            rt,
            fold,
            premises,
            idx + 1,
            skip,
            next,
            win_from,
            win_to,
            tt,
            next_windows,
            except,
            out,
        );
        for hit in out.iter_mut().skip(before) {
            hit.premise_ids.push(f.event_id);
        }
    }
}
