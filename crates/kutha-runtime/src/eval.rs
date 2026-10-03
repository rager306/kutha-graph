//! Horn evaluation over live product triples (M016 S03 / RULE-02).
//! Named bodies are pins only; they do not auto-fire.

use crate::quantum::Runtime;
use kutha_common::{Event, EventId, Op, RuleAst, RuleAtom, RuleTerm, TermDictionary, TermId};

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

impl Runtime {
    pub(crate) fn rule_follow_ons(&self, event: &Event) -> Vec<Event> {
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
            Op::Retract { event_id } => self.retract_derived(event, *event_id),
            _ => vec![],
        }
    }

    fn retract_derived(&self, trigger: &Event, premise: EventId) -> Vec<Event> {
        let mut out = Vec::new();
        for e in self.log.iter() {
            let Op::Behavior {
                caused_by,
                rule_version,
                ..
            } = &e.op
            else {
                continue;
            };
            if *caused_by != premise {
                continue;
            }
            let Some(bytes) = self.fold.blobs.get(rule_version) else {
                continue;
            };
            if !matches!(RuleAst::from_canonical(bytes), Some(RuleAst::Horn { .. })) {
                continue;
            }
            if self
                .fold
                .fact_index_by_event_id(e.id)
                .and_then(|i| self.fold.facts().get(i))
                .is_some_and(|f| f.invalidated_at.is_none())
            {
                out.push(Event::new(
                    Op::Retract { event_id: e.id },
                    trigger.ingested_at,
                ));
            }
        }
        out
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
    ) -> Vec<Event> {
        let mut out = Vec::new();
        for (pin, ast) in self.live_horns(valid_from) {
            let RuleAst::Horn { head, premises } = ast else {
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
                if let Some(bind) = search_prems(self, &premises, i, bind, valid_from, u64::MAX) {
                    if let Some(ev) = self.head_event(
                        &head,
                        &pin,
                        &bind,
                        caused_by,
                        ingested_at,
                        valid_from,
                        valid_to,
                    ) {
                        out.push(ev);
                    }
                }
            }
        }
        out
    }

    fn live_horns(&self, vt: u64) -> Vec<(String, RuleAst)> {
        let Some(rel) = self.fold.rel_rule else {
            return vec![];
        };
        let mut out = Vec::new();
        for f in self.fold.facts() {
            self.fold.horn_examine.bump();
            if f.relation != rel || !f.is_live_at(u64::MAX, vt) {
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
                out.push((pin.to_string(), ast));
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
    ) -> Option<Event> {
        let rel = self.dict.id(&head.relation)?;
        let subject = instantiate(&head.subject, bind, &self.dict)?;
        let object = instantiate(&head.object, bind, &self.dict)?;
        if self.fold.has_live_triple(subject, rel, object) {
            return None;
        }
        let name = if head.relation == "knownBy" {
            "inverse_knows".into()
        } else {
            head.relation.clone()
        };
        Some(Event::new(
            Op::Behavior {
                name,
                caused_by,
                rule_version: pin.to_string(),
                subject,
                relation: rel,
                object,
                valid_from,
                valid_to,
            },
            ingested_at,
        ))
    }

    /// Horn-pinned Behavior is reproducible from premises live at `ingested_at`.
    /// Named bodies skip. Missing blob or unmatched Horn fails closed (RULE-04).
    pub(crate) fn horn_behavior_reproducible(&self, e: &Event) -> bool {
        let Op::Behavior {
            rule_version,
            subject,
            relation,
            object,
            valid_from,
            ..
        } = &e.op
        else {
            return true;
        };
        let Some(bytes) = self.fold.blobs.get(rule_version) else {
            return !rule_version.is_empty();
        };
        let Some(ast) = RuleAst::from_canonical(bytes) else {
            return false;
        };
        let RuleAst::Horn { head, premises } = ast else {
            return true;
        };
        let Some(head_rel) = self.dict.id(&head.relation) else {
            return false;
        };
        if head_rel != *relation {
            return false;
        }
        let mut bind = [None; MAX_VAR];
        if !unify_term(&head.subject, *subject, &self.dict, &mut bind) {
            return false;
        }
        if !unify_term(&head.object, *object, &self.dict, &mut bind) {
            return false;
        }
        search_prems(
            self,
            &premises,
            premises.len(),
            bind,
            *valid_from,
            e.ingested_at,
        )
        .is_some()
    }
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

fn search_prems(
    rt: &Runtime,
    premises: &[RuleAtom],
    skip: usize,
    bind: Bind,
    vt: u64,
    tt: u64,
) -> Option<Bind> {
    search_at(rt, premises, 0, skip, bind, vt, tt)
}

fn search_at(
    rt: &Runtime,
    premises: &[RuleAtom],
    idx: usize,
    skip: usize,
    bind: Bind,
    vt: u64,
    tt: u64,
) -> Option<Bind> {
    if idx >= premises.len() {
        return Some(bind);
    }
    if idx == skip {
        return search_at(rt, premises, idx + 1, skip, bind, vt, tt);
    }
    let prem = &premises[idx];
    let rel = rt.dict.id(&prem.relation)?;
    for f in rt.fold.live_facts_at(tt, vt) {
        rt.fold.horn_examine.bump();
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
        if let Some(ok) = search_at(rt, premises, idx + 1, skip, next, vt, tt) {
            return Some(ok);
        }
    }
    None
}
