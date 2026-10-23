//! Round-3 independent kill tests for phase 229 (checkpoint cut / Horn lineage).
//! Public API only. Rounds 1–2 covered chain shapes; this file hunts lineage
//! rigor: three-arm T/P/C parity, last-wins EventId, cascade rollback, forks.
//!
//! Normalisation: named live product triples (skip `kutha:*`); `as_of` /
//! `as_known_on` named and sorted; derived = live product triples whose
//! `(s,r,o,vf,vt)` is not a currently tracked user Assert. Fingerprint is
//! compared only while no arm has minted a new UUID-v7 user Assert / rule row
//! since the shared persist (Retract/Correct of existing ids keep support
//! slots). T = never-reopened in-memory Runtime; P = persist + `store::open`
//! (not checkpointed); C = checkpointed + reopened.

use kutha_common::{Event, EventId, Op, RuleAst, RuleAtom, RuleTerm, ValidTime, WallTime};
use kutha_runtime::{Runtime, RuntimeError, store};
use std::collections::HashSet;
use std::fs;
use std::path::Path;

mod common;

struct LimitGuard;

impl Drop for LimitGuard {
    fn drop(&mut self) {
        store::reset_test_segment_limits();
    }
}

fn tiny_limits() -> LimitGuard {
    store::set_test_segment_limits(Some(4096), Some(1));
    LimitGuard
}

fn persist_dir(label: &str) -> common::PersistScratch {
    common::PersistScratch::new(&format!("b3-kill3-{label}"))
}

struct Rng(u64);

impl Rng {
    fn new(seed: u64) -> Self {
        Self(seed | 1)
    }

    fn next(&mut self) -> u64 {
        self.0 = self
            .0
            .wrapping_mul(0x9E3779B97F4A7C15)
            .wrapping_add(0xD1B54A32D192ED03);
        self.0
    }

    fn u(&mut self, n: usize) -> usize {
        if n == 0 {
            return 0;
        }
        (self.next() as usize) % n
    }

    fn coin(&mut self) -> bool {
        self.next() & 1 == 1
    }
}

fn horn_trans() -> RuleAst {
    let v = |i| RuleTerm::var(i).unwrap();
    let atom = |rel: &str, s, o| RuleAtom::new(rel, s, o).unwrap();
    RuleAst::horn(
        atom("relatedTo", v(0), v(2)),
        vec![atom("relatedTo", v(0), v(1)), atom("relatedTo", v(1), v(2))],
    )
    .unwrap()
}

fn horn_inverse() -> RuleAst {
    let v = |i| RuleTerm::var(i).unwrap();
    let atom = |rel: &str, s, o| RuleAtom::new(rel, s, o).unwrap();
    RuleAst::horn(
        atom("relatedTo", v(1), v(0)),
        vec![atom("relatedTo", v(0), v(1))],
    )
    .unwrap()
}

fn ghost_from_known_by() -> RuleAst {
    let v = |i| RuleTerm::var(i).unwrap();
    let atom = |rel: &str, s, o| RuleAtom::new(rel, s, o).unwrap();
    RuleAst::horn(
        atom("ghostRel", v(1), v(0)),
        vec![atom("knownBy", v(0), v(1))],
    )
    .unwrap()
}

fn emit_keyed(
    rt: &mut Runtime,
    subject: &str,
    relation: &str,
    object: &str,
    valid_from: u64,
    valid_to: Option<u64>,
    key: &str,
) -> EventId {
    let subject = rt.intern(subject);
    let relation = rt.intern(relation);
    let object = rt.intern(object);
    let outcome = rt
        .emit(Op::Assert {
            subject,
            relation,
            object,
            valid_from,
            valid_to,
            claim: None,
            delivery_key: Some(key.into()),
            polarity: None,
        })
        .unwrap_or_else(|e| panic!("emit Assert {key}: {e}"));
    let id = rt
        .log()
        .as_slice()
        .iter()
        .rev()
        .find_map(|e| match &e.op {
            Op::Assert {
                delivery_key: Some(k),
                ..
            } if k == key => Some(e.id),
            _ => None,
        })
        .unwrap_or_else(|| {
            panic!(
                "Assert {key} missing from log after emit; receipt={:?}",
                outcome.receipt.event_ids
            )
        });
    assert!(
        rt.fold().facts().iter().any(|f| f.event_id == id),
        "HIGH: Assert {key} id={id} not in fold after emit"
    );
    id
}

fn pad(rt: &mut Runtime, n: usize, tag: &str) {
    for i in 0..n {
        let _ = rt.intern(&format!("{tag}-term-{i}"));
        emit_keyed(
            rt,
            "art-12-speed",
            "inForceAs",
            &format!("{tag}-obj-{i}"),
            2010,
            Some(2020),
            &format!("{tag}-dk-{i}"),
        );
    }
}

fn name(rt: &Runtime, id: kutha_common::TermId) -> String {
    rt.dictionary()
        .lookup(id)
        .unwrap_or("<missing>")
        .to_string()
}

fn named_as_of(rt: &Runtime, vt: u64) -> Vec<(String, String, String)> {
    let mut rows: Vec<_> = rt
        .as_of(ValidTime::from(vt))
        .into_iter()
        .map(|(s, r, o)| (name(rt, s), name(rt, r), name(rt, o)))
        .collect();
    rows.sort();
    rows
}

fn named_known_on(rt: &Runtime, wall: u64) -> Option<Vec<(String, String, String)>> {
    rt.as_known_on(WallTime::from(wall)).map(|triples| {
        let mut rows: Vec<_> = triples
            .into_iter()
            .map(|(s, r, o)| (name(rt, s), name(rt, r), name(rt, o)))
            .collect();
        rows.sort();
        rows
    })
}

fn product_live(rt: &Runtime) -> Vec<(String, String, String, u64, Option<u64>)> {
    let mut rows = Vec::new();
    for f in rt.fold().facts() {
        if f.invalidated_at.is_some() {
            continue;
        }
        let rel = name(rt, f.relation);
        if rel.starts_with("kutha:") {
            continue;
        }
        rows.push((
            name(rt, f.subject),
            rel,
            name(rt, f.object()),
            f.valid_from,
            f.valid_to,
        ));
    }
    rows.sort();
    rows
}

fn live_named(rt: &Runtime, s: &str, r: &str, o: &str) -> bool {
    let (Some(s), Some(r), Some(o)) = (
        rt.dictionary().id(s),
        rt.dictionary().id(r),
        rt.dictionary().id(o),
    ) else {
        return false;
    };
    rt.fold()
        .facts()
        .iter()
        .any(|f| f.subject == s && f.relation == r && f.object() == o && f.invalidated_at.is_none())
}

fn live_count(rt: &Runtime, s: &str, r: &str, o: &str) -> usize {
    let (Some(s), Some(r), Some(o)) = (
        rt.dictionary().id(s),
        rt.dictionary().id(r),
        rt.dictionary().id(o),
    ) else {
        return 0;
    };
    rt.fold()
        .facts()
        .iter()
        .filter(|f| {
            f.subject == s && f.relation == r && f.object() == o && f.invalidated_at.is_none()
        })
        .count()
}

fn derived_named(
    rt: &Runtime,
    user: &HashSet<(String, String, String, u64, Option<u64>)>,
) -> Vec<(String, String, String, u64, Option<u64>)> {
    product_live(rt)
        .into_iter()
        .filter(|row| row.1 == "relatedTo" && !user.contains(row))
        .collect()
}

fn fact_debug(
    rt: &Runtime,
) -> Vec<(
    u64,
    String,
    String,
    String,
    u64,
    Option<u64>,
    Option<u64>,
    String,
)> {
    rt.fold()
        .facts()
        .iter()
        .map(|f| {
            (
                f.seq,
                name(rt, f.subject),
                name(rt, f.relation),
                name(rt, f.object()),
                f.valid_from,
                f.valid_to,
                f.invalidated_at,
                f.event_id.to_string(),
            )
        })
        .collect()
}

const AS_OF_VTS: [u64; 6] = [2009, 2012, 2015, 2017, 2019, 2025];
const KNOWN_WALLS: [u64; 7] = [2000, 2012, 2015, 2018, 2021, 2024, 2030];

fn assert_named(got: &Runtime, want: &Runtime, msg: &str) {
    assert_eq!(
        product_live(got),
        product_live(want),
        "HIGH {msg}: live product triples got={:?} want={:?}",
        product_live(got),
        product_live(want)
    );
    for vt in AS_OF_VTS {
        assert_eq!(
            named_as_of(got, vt),
            named_as_of(want, vt),
            "HIGH {msg}: as_of {vt}"
        );
    }
    for wall in KNOWN_WALLS {
        assert_eq!(
            named_known_on(got, wall),
            named_known_on(want, wall),
            "HIGH {msg}: as_known_on {wall}"
        );
    }
}

fn three_err(
    t: &Runtime,
    p: &Runtime,
    c: &Runtime,
    user: &HashSet<(String, String, String, u64, Option<u64>)>,
    fp: bool,
    msg: &str,
) -> Result<(), String> {
    let check = |got: &Runtime, want: &Runtime, arm: &str| -> Result<(), String> {
        if product_live(got) != product_live(want) {
            return Err(format!(
                "HIGH {msg} {arm}: live product got={:?} want={:?}",
                product_live(got),
                product_live(want)
            ));
        }
        for vt in AS_OF_VTS {
            if named_as_of(got, vt) != named_as_of(want, vt) {
                return Err(format!(
                    "HIGH {msg} {arm}: as_of {vt} got={:?} want={:?}",
                    named_as_of(got, vt),
                    named_as_of(want, vt)
                ));
            }
        }
        for wall in KNOWN_WALLS {
            if named_known_on(got, wall) != named_known_on(want, wall) {
                return Err(format!(
                    "HIGH {msg} {arm}: as_known_on {wall} got={:?} want={:?}",
                    named_known_on(got, wall),
                    named_known_on(want, wall)
                ));
            }
        }
        Ok(())
    };
    check(c, t, "C==T")?;
    check(c, p, "C==P")?;
    check(p, t, "P==T")?;
    if derived_named(c, user) != derived_named(t, user) {
        return Err(format!(
            "HIGH {msg}: derived C==T got={:?} want={:?}",
            derived_named(c, user),
            derived_named(t, user)
        ));
    }
    if derived_named(c, user) != derived_named(p, user) {
        return Err(format!(
            "HIGH {msg}: derived C==P got={:?} want={:?}",
            derived_named(c, user),
            derived_named(p, user)
        ));
    }
    if derived_named(p, user) != derived_named(t, user) {
        return Err(format!(
            "HIGH {msg}: derived P==T got={:?} want={:?}",
            derived_named(p, user),
            derived_named(t, user)
        ));
    }
    if fp {
        if c.fold().fingerprint() != t.fold().fingerprint() {
            return Err(format!(
                "HIGH {msg} C==T fp: got={:?} want={:?}",
                fact_debug(c),
                fact_debug(t)
            ));
        }
        if c.fold().fingerprint() != p.fold().fingerprint() {
            return Err(format!(
                "HIGH {msg} C==P fp: got={:?} want={:?}",
                fact_debug(c),
                fact_debug(p)
            ));
        }
        if p.fold().fingerprint() != t.fold().fingerprint() {
            return Err(format!(
                "HIGH {msg} P==T fp: got={:?} want={:?}",
                fact_debug(p),
                fact_debug(t)
            ));
        }
    }
    Ok(())
}

fn assert_three(
    t: &Runtime,
    p: &Runtime,
    c: &Runtime,
    user: &HashSet<(String, String, String, u64, Option<u64>)>,
    fp: bool,
    msg: &str,
) {
    three_err(t, p, c, user, fp, msg).unwrap_or_else(|e| panic!("{e}"));
}

fn with_log(log: &[String], r: Result<(), String>) -> Result<(), String> {
    r.map_err(|e| format!("{e}; sequence: {}", log.join(" | ")))
}

fn replay_ok(rt: &Runtime, msg: &str) {
    rt.replay_check()
        .unwrap_or_else(|e| panic!("HIGH {msg}: replay_check {e}"));
}

fn sealed_prefix_len(dir: &Path) -> usize {
    let man: store::Manifest =
        serde_json::from_str(&fs::read_to_string(dir.join(store::MANIFEST_REL)).unwrap()).unwrap();
    man.sealed.iter().map(|s| s.count as usize).sum()
}

fn first_checkpoint_index(events: &[Event]) -> usize {
    events
        .iter()
        .position(|e| matches!(e.op, Op::Checkpoint { .. }))
        .expect("Op::Checkpoint on current log")
}

fn twin_prefix_len_for_checkpointed(ckp: &Runtime, twin: &Runtime) -> usize {
    let events = ckp.log().as_slice();
    let c = first_checkpoint_index(events);
    match events.get(c + 1) {
        Some(first_tail) => twin
            .log()
            .as_slice()
            .iter()
            .position(|e| e.id == first_tail.id)
            .unwrap_or_else(|| {
                panic!(
                    "HIGH R1: first tail event {} missing from twin log",
                    first_tail.id
                )
            }),
        None => twin.log().len(),
    }
}

fn copy_store(src: &Path, dst: &Path) {
    fs::create_dir_all(dst).unwrap();
    for ent in fs::read_dir(src).unwrap() {
        let ent = ent.unwrap();
        let name = ent.file_name();
        if name == "kutha.lock" {
            continue;
        }
        let from = ent.path();
        let to = dst.join(&name);
        if from.is_dir() {
            copy_store(&from, &to);
        } else {
            fs::copy(&from, &to).unwrap();
        }
    }
}

fn persist_three(
    rt: &Runtime,
    label: &str,
) -> (common::PersistScratch, common::PersistScratch, usize) {
    let pdir = persist_dir(&format!("{label}-p"));
    let cdir = persist_dir(&format!("{label}-c"));
    store::persist(rt, &pdir).unwrap();
    let prefix_len = sealed_prefix_len(&pdir);
    assert!(
        prefix_len > 0 && prefix_len < store::encoded_events(rt).len(),
        "fixture must seal a strict prefix"
    );
    copy_store(&pdir, &cdir);
    store::checkpoint_sealed_prefix(&cdir).unwrap();
    (pdir, cdir, prefix_len)
}

fn behavior_rows_for(rt: &Runtime, s: &str, r: &str, o: &str) -> Vec<(EventId, EventId)> {
    rt.log()
        .as_slice()
        .iter()
        .filter_map(|e| match &e.op {
            Op::Behavior {
                subject,
                relation,
                object,
                caused_by,
                ..
            } if name(rt, *subject) == s && name(rt, *relation) == r && name(rt, *object) == o => {
                Some((e.id, *caused_by))
            }
            _ => None,
        })
        .collect()
}

fn duplicate_id_causes(rows: &[(EventId, EventId)]) -> Vec<(EventId, Vec<EventId>)> {
    let mut by_id: Vec<(EventId, Vec<EventId>)> = Vec::new();
    for (id, cause) in rows {
        if let Some(slot) = by_id.iter_mut().find(|(i, _)| *i == *id) {
            if !slot.1.contains(cause) {
                slot.1.push(*cause);
            }
        } else {
            by_id.push((*id, vec![*cause]));
        }
    }
    by_id.into_iter().filter(|(_, cs)| cs.len() > 1).collect()
}

#[derive(Clone)]
struct Edge {
    s: usize,
    o: usize,
    vf: u64,
    vt: Option<u64>,
    key: String,
    id_t: EventId,
    id_p: EventId,
    id_c: EventId,
    live: bool,
}

fn node(i: usize) -> String {
    format!("G{i}")
}

fn user_set(edges: &[Edge]) -> HashSet<(String, String, String, u64, Option<u64>)> {
    edges
        .iter()
        .filter(|e| e.live)
        .map(|e| (node(e.s), "relatedTo".into(), node(e.o), e.vf, e.vt))
        .collect()
}

fn emit_edge(rt: &mut Runtime, e: &Edge) -> EventId {
    emit_keyed(rt, &node(e.s), "relatedTo", &node(e.o), e.vf, e.vt, &e.key)
}

fn maybe_rule(rt: &mut Runtime, trans: bool, inv: bool) {
    if trans {
        rt.emit_rule_ast(&horn_trans(), 2010, None).unwrap();
    }
    if inv {
        rt.emit_rule_ast(&horn_inverse(), 2010, None).unwrap();
    }
}

fn run_seed(seed: u64) -> Result<Vec<String>, String> {
    let mut rng = Rng::new(seed);
    let mut log: Vec<String> = Vec::new();
    let n = 4 + rng.u(4);
    let mut pairs: Vec<(usize, usize)> = Vec::new();
    for i in 0..n {
        for j in 0..n {
            if i != j && rng.u(10) < 4 {
                pairs.push((i, j));
            }
        }
    }
    if pairs.len() < n.saturating_sub(1) {
        for i in 0..n.saturating_sub(1) {
            if !pairs.contains(&(i, i + 1)) {
                pairs.push((i, i + 1));
            }
        }
    }
    let trans_b = rng.coin();
    let inv_b = rng.coin();
    let trans_a = rng.coin();
    let inv_a = rng.coin();
    let (trans_b, inv_b, trans_a, inv_a) = if !trans_b && !inv_b && !trans_a && !inv_a {
        (true, false, false, false)
    } else {
        (trans_b, inv_b, trans_a, inv_a)
    };
    let split = 1 + rng.u(pairs.len().max(1));
    let split = split.min(pairs.len());
    let pad_n = 6 + rng.u(5);
    let n_ops = 4 + rng.u(5);
    let do_reopen = seed % 3 == 0;
    let do_append = seed % 5 == 0;
    let do_second = seed % 7 == 0;
    log.push(format!(
        "seed={seed} n={n} edges={} split={split} trans_b={trans_b} inv_b={inv_b} trans_a={trans_a} inv_a={inv_a} pad={pad_n} ops={n_ops} reopen={do_reopen} append={do_append} second={do_second}",
        pairs.len()
    ));

    let mut t = Runtime::default();
    for i in 0..n {
        let _ = t.intern(&node(i));
    }
    let _ = t.intern("relatedTo");
    maybe_rule(&mut t, trans_b, inv_b);
    log.push(format!("rules_before trans={trans_b} inv={inv_b}"));

    let mut edges: Vec<Edge> = Vec::new();
    for (k, (s, o)) in pairs.iter().enumerate() {
        let vf = if k % 3 == 0 { 2012 } else { 2010 };
        let vt = if k % 4 == 0 { Some(2020) } else { None };
        edges.push(Edge {
            s: *s,
            o: *o,
            vf,
            vt,
            key: format!("s{seed}-e{k}"),
            id_t: EventId::nil(),
            id_p: EventId::nil(),
            id_c: EventId::nil(),
            live: false,
        });
    }
    for e in edges.iter_mut().take(split) {
        let id = emit_edge(&mut t, e);
        e.id_t = id;
        e.id_p = id;
        e.id_c = id;
        e.live = true;
        log.push(format!("prefix Assert {}-{} key={}", e.s, e.o, e.key));
    }
    pad(&mut t, pad_n, &format!("s{seed}"));
    t.emit_tt_anchor(2015).unwrap();
    log.push("tt_anchor 2015".into());

    let (pdir, cdir, _prefix_len) = persist_three(&t, &format!("s{seed}"));
    let mut p = store::open(&pdir).map_err(|e| format!("open P: {e}"))?;
    let mut c = store::open(&cdir).map_err(|e| format!("open C: {e}"))?;
    c.replay_check()
        .map_err(|e| format!("seed {seed} C open replay: {e}"))?;
    let mut fp = !trans_a && !inv_a;
    let user = user_set(&edges);
    with_log(
        &log,
        three_err(&t, &p, &c, &user, fp, &format!("seed {seed} after open")),
    )?;

    // Register-after only when that rule was not already in the prefix.
    // Re-registering the same Horn after a cut is a separate kill
    // (`r3_reregister_same_horn_after_checkpoint`).
    let trans_after = trans_a && !trans_b;
    let inv_after = inv_a && !inv_b;
    maybe_rule(&mut t, trans_after, inv_after);
    maybe_rule(&mut p, trans_after, inv_after);
    maybe_rule(&mut c, trans_after, inv_after);
    if trans_after || inv_after {
        fp = false;
        log.push(format!("rules_after trans={trans_after} inv={inv_after}"));
    }
    for e in edges.iter_mut().skip(split) {
        e.id_t = emit_edge(&mut t, e);
        e.id_p = emit_edge(&mut p, e);
        e.id_c = emit_edge(&mut c, e);
        e.live = true;
        fp = false;
        log.push(format!("tail Assert {}-{} key={}", e.s, e.o, e.key));
    }
    let user = user_set(&edges);
    with_log(
        &log,
        three_err(&t, &p, &c, &user, fp, &format!("seed {seed} after tail")),
    )?;

    if do_append {
        store::persist(&c, &cdir).map_err(|e| format!("persist C before append: {e}"))?;
        drop(c);
        let (mut cw, mut persister) =
            store::open_for_append(&cdir).map_err(|e| format!("open_for_append C: {e}"))?;
        if let Some(idx) = edges.iter().position(|e| e.live) {
            cw.emit(Op::Retract {
                event_id: edges[idx].id_c,
            })
            .map_err(|err| format!("append-arm retract: {err}"))?;
            t.emit(Op::Retract {
                event_id: edges[idx].id_t,
            })
            .map_err(|err| format!("T retract during append: {err}"))?;
            p.emit(Op::Retract {
                event_id: edges[idx].id_p,
            })
            .map_err(|err| format!("P retract during append: {err}"))?;
            log.push(format!("open_for_append Retract {}", edges[idx].key));
            edges[idx].live = false;
        }
        persister
            .append(&cw)
            .map_err(|e| format!("persister.append: {e}"))?;
        drop(persister);
        c = cw;
        let user = user_set(&edges);
        with_log(
            &log,
            three_err(
                &t,
                &p,
                &c,
                &user,
                fp,
                &format!("seed {seed} after open_for_append"),
            ),
        )?;
        drop(c);
        c = store::open(&cdir).map_err(|e| format!("reopen C after append: {e}"))?;
        c.replay_check()
            .map_err(|e| format!("seed {seed} C reopen after append: {e}"))?;
        with_log(
            &log,
            three_err(
                &t,
                &p,
                &c,
                &user,
                fp,
                &format!("seed {seed} after append reopen"),
            ),
        )?;
    }

    let mut re_seq = 0u32;
    for step in 0..n_ops {
        let live_idx: Vec<usize> = edges
            .iter()
            .enumerate()
            .filter(|(_, e)| e.live)
            .map(|(i, _)| i)
            .collect();
        let dead_idx: Vec<usize> = edges
            .iter()
            .enumerate()
            .filter(|(_, e)| !e.live)
            .map(|(i, _)| i)
            .collect();
        let kind = rng.u(3);
        if kind == 0 && !live_idx.is_empty() {
            let i = live_idx[rng.u(live_idx.len())];
            t.emit(Op::Retract {
                event_id: edges[i].id_t,
            })
            .map_err(|e| {
                format!(
                    "T Retract {}: {e}; sequence: {}",
                    edges[i].key,
                    log.join(" | ")
                )
            })?;
            p.emit(Op::Retract {
                event_id: edges[i].id_p,
            })
            .map_err(|e| {
                format!(
                    "P Retract {}: {e}; sequence: {}",
                    edges[i].key,
                    log.join(" | ")
                )
            })?;
            c.emit(Op::Retract {
                event_id: edges[i].id_c,
            })
            .map_err(|e| {
                format!(
                    "C Retract {}: {e}; sequence: {}",
                    edges[i].key,
                    log.join(" | ")
                )
            })?;
            edges[i].live = false;
            log.push(format!("Retract {}", edges[i].key));
        } else if kind == 1 && !live_idx.is_empty() {
            let i = live_idx[rng.u(live_idx.len())];
            let mut obj = rng.u(n);
            if obj == edges[i].o {
                obj = (obj + 1) % n;
            }
            let vf = edges[i].vf;
            let vt = edges[i].vt;
            let oname = node(obj);
            let ot = t.intern(&oname);
            let op_ = p.intern(&oname);
            let oc = c.intern(&oname);
            let old_t = edges[i].id_t;
            let old_p = edges[i].id_p;
            let old_c = edges[i].id_c;
            t.emit(Op::Correct {
                event_id: old_t,
                object: ot,
                valid_from: vf,
                valid_to: vt,
            })
            .map_err(|e| format!("T Correct {}: {e}", edges[i].key))?;
            p.emit(Op::Correct {
                event_id: old_p,
                object: op_,
                valid_from: vf,
                valid_to: vt,
            })
            .map_err(|e| format!("P Correct {}: {e}", edges[i].key))?;
            c.emit(Op::Correct {
                event_id: old_c,
                object: oc,
                valid_from: vf,
                valid_to: vt,
            })
            .map_err(|e| format!("C Correct {}: {e}", edges[i].key))?;
            let new_id = |rt: &Runtime, old: EventId| {
                rt.log()
                    .as_slice()
                    .iter()
                    .rev()
                    .find_map(|e| match &e.op {
                        Op::Correct { event_id, .. } if *event_id == old => Some(e.id),
                        _ => None,
                    })
                    .expect("Correct event on log")
            };
            edges[i].id_t = new_id(&t, old_t);
            edges[i].id_p = new_id(&p, old_p);
            edges[i].id_c = new_id(&c, old_c);
            edges[i].o = obj;
            fp = false;
            log.push(format!("Correct {} -> G{obj}", edges[i].key));
        } else if !dead_idx.is_empty() {
            let i = dead_idx[rng.u(dead_idx.len())];
            re_seq += 1;
            edges[i].key = format!("{}-re{re_seq}", edges[i].key);
            edges[i].id_t = emit_edge(&mut t, &edges[i]);
            edges[i].id_p = emit_edge(&mut p, &edges[i]);
            edges[i].id_c = emit_edge(&mut c, &edges[i]);
            edges[i].live = true;
            fp = false;
            log.push(format!("re-assert {}", edges[i].key));
        } else if !live_idx.is_empty() {
            let i = live_idx[rng.u(live_idx.len())];
            t.emit(Op::Retract {
                event_id: edges[i].id_t,
            })
            .map_err(|e| e.to_string())?;
            p.emit(Op::Retract {
                event_id: edges[i].id_p,
            })
            .map_err(|e| e.to_string())?;
            c.emit(Op::Retract {
                event_id: edges[i].id_c,
            })
            .map_err(|e| e.to_string())?;
            edges[i].live = false;
            log.push(format!("Retract {}", edges[i].key));
        }
        let rt_ok = t.replay_check().is_ok();
        let rp_ok = p.replay_check().is_ok();
        let rc = c.replay_check();
        match (rt_ok, rp_ok, rc.as_ref().ok()) {
            (true, true, Some(_)) => {}
            (true, true, None) => {
                return Err(format!(
                    "HIGH R2: C replay_check failed while T and P succeeded: {}; sequence: {}",
                    rc.err().unwrap(),
                    log.join(" | ")
                ));
            }
            _ => {
                // T/P also failed RULE-04 on this graph; not a checkpoint-only hole.
            }
        }
        let user = user_set(&edges);
        with_log(
            &log,
            three_err(
                &t,
                &p,
                &c,
                &user,
                fp,
                &format!("seed {seed} after step {step} ({})", log.last().unwrap()),
            ),
        )?;
        if do_reopen && step == n_ops / 2 {
            let t_rep = match t.replay_check() {
                Ok(_) => "Ok".to_string(),
                Err(e) => e.to_string(),
            };
            store::persist(&p, &pdir).map_err(|e| format!("persist P mid: {e}"))?;
            store::persist(&c, &cdir).map_err(|e| format!("persist C mid: {e}"))?;
            drop(p);
            drop(c);
            p = match store::open(&pdir) {
                Ok(rt) => rt,
                Err(e) => {
                    if t_rep == "Ok" {
                        return Err(format!(
                            "HIGH R2: persist+open P failed while T replay_check Ok: {e}; sequence: {}",
                            log.join(" | ")
                        ));
                    }
                    log.push(format!(
                        "skip mid persist+open: P open failed with dirty T replay ({e})"
                    ));
                    return Ok(log);
                }
            };
            c = match store::open(&cdir) {
                Ok(rt) => rt,
                Err(e) => {
                    if t_rep == "Ok" {
                        return Err(format!(
                            "HIGH R2: persist+open C failed while T replay_check Ok: {e}; sequence: {}",
                            log.join(" | ")
                        ));
                    }
                    log.push(format!(
                        "skip mid persist+open: C open failed with dirty T replay ({e})"
                    ));
                    return Ok(log);
                }
            };
            let rp_ok = p.replay_check().is_ok();
            let rc = c.replay_check();
            match (t_rep.as_str() == "Ok", rp_ok, rc.as_ref().ok()) {
                (true, true, None) => {
                    return Err(format!(
                        "HIGH R2: C replay_check failed after mid persist+open while T and P succeeded: {}; sequence: {}",
                        rc.err().unwrap(),
                        log.join(" | ")
                    ));
                }
                _ => {}
            }
            let user = user_set(&edges);
            with_log(
                &log,
                three_err(
                    &t,
                    &p,
                    &c,
                    &user,
                    fp,
                    &format!("seed {seed} after mid persist+open"),
                ),
            )?;
            log.push("persist+open P and C".into());
        }
    }

    if do_second {
        pad(&mut t, 6, &format!("s{seed}-2"));
        pad(&mut p, 6, &format!("s{seed}-2"));
        pad(&mut c, 6, &format!("s{seed}-2"));
        log.push("pad before second checkpoint".into());
        let replay_status = |rt: &Runtime| match rt.replay_check() {
            Ok(_) => "Ok".into(),
            Err(e) => e.to_string(),
        };
        let t_rep = replay_status(&t);
        let p_rep = replay_status(&p);
        let c_rep = replay_status(&c);
        store::persist(&c, &cdir).map_err(|e| format!("persist C second: {e}"))?;
        store::persist(&p, &pdir).map_err(|e| format!("persist P second: {e}"))?;
        drop(c);
        c = match store::open(&cdir) {
            Ok(rt) => rt,
            Err(e) => {
                if t_rep == "Ok" && p_rep == "Ok" {
                    return Err(format!(
                        "HIGH R2: persist+open C failed before second checkpoint: {e}; T replay={t_rep}; P replay={p_rep}; C in-memory replay={c_rep}; sequence: {}",
                        log.join(" | ")
                    ));
                }
                log.push(format!(
                    "skip second checkpoint: persist+open C failed with dirty T/P replay ({e})"
                ));
                return Ok(log);
            }
        };
        drop(p);
        p = match store::open(&pdir) {
            Ok(rt) => rt,
            Err(e) => {
                log.push(format!(
                    "skip second checkpoint: persist+open P failed ({e})"
                ));
                return Ok(log);
            }
        };
        if sealed_prefix_len(&cdir) > 0 {
            if let Err(e) = store::checkpoint_sealed_prefix(&cdir) {
                if t_rep == "Ok" && p_rep == "Ok" {
                    return Err(format!(
                        "HIGH R2: second checkpoint: {e}; T replay={t_rep}; P replay={p_rep}; C in-memory replay={c_rep}; sequence: {}",
                        log.join(" | ")
                    ));
                }
                log.push(format!(
                    "skip second checkpoint: rewrite failed with dirty T/P replay ({e})"
                ));
                return Ok(log);
            }
            drop(c);
            c = store::open(&cdir).map_err(|e| format!("open C after second ckp: {e}"))?;
            let rc = c.replay_check();
            let rt_ok = t.replay_check().is_ok();
            let rp_ok = p.replay_check().is_ok();
            match (rt_ok, rp_ok, rc.as_ref().ok()) {
                (true, true, None) => {
                    return Err(format!(
                        "HIGH R2: C replay_check failed after second checkpoint while T and P succeeded: {}; sequence: {}",
                        rc.err().unwrap(),
                        log.join(" | ")
                    ));
                }
                (true, true, Some(_)) => {}
                _ => {}
            }
            log.push("second checkpoint + open C".into());
            let user = user_set(&edges);
            with_log(
                &log,
                three_err(
                    &t,
                    &p,
                    &c,
                    &user,
                    fp,
                    &format!("seed {seed} after second checkpoint"),
                ),
            )?;
            if let Some(idx) = edges.iter().position(|e| e.live) {
                t.emit(Op::Retract {
                    event_id: edges[idx].id_t,
                })
                .map_err(|err| format!("T retract after 2nd ckp: {err}"))?;
                p.emit(Op::Retract {
                    event_id: edges[idx].id_p,
                })
                .map_err(|err| format!("P retract after 2nd ckp: {err}"))?;
                c.emit(Op::Retract {
                    event_id: edges[idx].id_c,
                })
                .map_err(|err| format!("C retract after 2nd ckp: {err}"))?;
                log.push(format!(
                    "Retract {} after second checkpoint",
                    edges[idx].key
                ));
                edges[idx].live = false;
                let user = user_set(&edges);
                with_log(
                    &log,
                    three_err(
                        &t,
                        &p,
                        &c,
                        &user,
                        fp,
                        &format!("seed {seed} retract after second checkpoint"),
                    ),
                )?;
            }
        }
    }
    Ok(log)
}

fn catch_seed(seed: u64) -> Result<Vec<String>, String> {
    std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| run_seed(seed))).unwrap_or_else(
        |payload| {
            let msg = if let Some(s) = payload.downcast_ref::<String>() {
                s.clone()
            } else if let Some(s) = payload.downcast_ref::<&str>() {
                (*s).to_string()
            } else {
                "panic (non-string)".into()
            };
            Err(msg)
        },
    )
}

#[test]
fn r3_property_seed_98_second_checkpoint() {
    let _g = tiny_limits();
    catch_seed(98).unwrap_or_else(|e| panic!("{e}"));
}

#[test]
fn r3_property_seed_99_mid_persist_open() {
    let _g = tiny_limits();
    catch_seed(99).unwrap_or_else(|e| panic!("{e}"));
}

#[test]
fn r3_property_random_graphs_three_arm() {
    let _g = tiny_limits();
    let mut failures: Vec<(u64, String)> = Vec::new();
    for seed in 1u64..=128 {
        match catch_seed(seed) {
            Ok(_) => {}
            Err(e) => failures.push((seed, e)),
        }
    }
    if failures.is_empty() {
        return;
    }
    let (seed, err) = &failures[0];
    let seq = catch_seed(*seed).err().unwrap_or_else(|| err.clone());
    let all: Vec<String> = failures
        .iter()
        .map(|(s, e)| format!("seed {s}: {e}"))
        .collect();
    panic!(
        "HIGH R2 property: {} of 128 seeds failed; first seed={seed}; all=[{}]; seq/err={seq}",
        failures.len(),
        all.join(" || "),
    );
}

#[test]
fn r3_tail_multipath_same_eventid_different_caused_by() {
    let _g = tiny_limits();
    let mut t = Runtime::default();
    t.emit_rule_ast(&horn_trans(), 2010, None).unwrap();
    t.emit_rule_ast(&horn_inverse(), 2010, None).unwrap();
    pad(&mut t, 8, "tail-mp");
    t.emit_tt_anchor(2015).unwrap();
    let (pdir, cdir, _) = persist_three(&t, "tail-mp");
    let mut p = store::open(&pdir).unwrap();
    let mut c = store::open(&cdir).unwrap();
    replay_ok(&c, "tail-mp open");

    let _bc_t = emit_keyed(&mut t, "B", "relatedTo", "C", 2010, None, "tm-bc");
    let _bc_p = emit_keyed(&mut p, "B", "relatedTo", "C", 2010, None, "tm-bc");
    let _bc_c = emit_keyed(&mut c, "B", "relatedTo", "C", 2010, None, "tm-bc");
    let ab_t = emit_keyed(&mut t, "A", "relatedTo", "B", 2010, None, "tm-ab");
    let ab_p = emit_keyed(&mut p, "A", "relatedTo", "B", 2010, None, "tm-ab");
    let ab_c = emit_keyed(&mut c, "A", "relatedTo", "B", 2010, None, "tm-ab");
    let _ad_t = emit_keyed(&mut t, "A", "relatedTo", "D", 2010, None, "tm-ad");
    let _ad_p = emit_keyed(&mut p, "A", "relatedTo", "D", 2010, None, "tm-ad");
    let _ad_c = emit_keyed(&mut c, "A", "relatedTo", "D", 2010, None, "tm-ad");
    let _dc_t = emit_keyed(&mut t, "D", "relatedTo", "C", 2010, None, "tm-dc");
    let _dc_p = emit_keyed(&mut p, "D", "relatedTo", "C", 2010, None, "tm-dc");
    let _dc_c = emit_keyed(&mut c, "D", "relatedTo", "C", 2010, None, "tm-dc");

    let rows_t = behavior_rows_for(&t, "A", "relatedTo", "C");
    let rows_c = behavior_rows_for(&c, "A", "relatedTo", "C");
    let dups_t = duplicate_id_causes(&rows_t);
    let dups_c = duplicate_id_causes(&rows_c);
    assert!(
        live_named(&t, "A", "relatedTo", "C"),
        "fixture must derive A-C in the tail"
    );
    if dups_t.is_empty() && dups_c.is_empty() {
        // Same-id minting is DET-02 + same ingested_at. Inverse+trans on A-B
        // still yields two supports for C-A / A-C; retract the first user path.
        assert!(
            !rows_t.is_empty() || live_count(&t, "A", "relatedTo", "C") > 0,
            "HIGH R2: tail diamond produced no A-C Behavior/fact"
        );
    } else {
        assert_eq!(
            dups_t
                .iter()
                .map(|(id, cs)| (*id, cs.len()))
                .collect::<Vec<_>>(),
            dups_c
                .iter()
                .map(|(id, cs)| (*id, cs.len()))
                .collect::<Vec<_>>(),
            "HIGH R2: duplicate EventId caused_by sets diverged T vs C; T={dups_t:?} C={dups_c:?}"
        );
    }

    t.emit(Op::Retract { event_id: ab_t }).unwrap();
    p.emit(Op::Retract { event_id: ab_p }).unwrap();
    c.emit(Op::Retract { event_id: ab_c }).unwrap();
    replay_ok(&c, "tail-mp retract first path");
    let user = HashSet::from([
        ("B".into(), "relatedTo".into(), "C".into(), 2010, None),
        ("A".into(), "relatedTo".into(), "D".into(), 2010, None),
        ("D".into(), "relatedTo".into(), "C".into(), 2010, None),
    ]);
    assert_three(
        &t,
        &p,
        &c,
        &user,
        false,
        "R3 tail multi-path retract first path A-B",
    );
}

#[test]
fn r3_first_cause_roundtrip_after_recheckpoint() {
    let _g = tiny_limits();
    let mut t = Runtime::default();
    t.emit_rule_ast(&horn_trans(), 2010, None).unwrap();
    let ab = emit_keyed(&mut t, "A", "relatedTo", "B", 2010, None, "fc-ab");
    let _bc = emit_keyed(&mut t, "B", "relatedTo", "C", 2010, None, "fc-bc");
    let ad = emit_keyed(&mut t, "A", "relatedTo", "D", 2010, None, "fc-ad");
    let _dc = emit_keyed(&mut t, "D", "relatedTo", "C", 2010, None, "fc-dc");
    pad(&mut t, 8, "fc");
    t.emit_tt_anchor(2015).unwrap();
    assert!(
        live_named(&t, "A", "relatedTo", "C"),
        "diamond must derive A-C"
    );
    let prefix_rows = behavior_rows_for(&t, "A", "relatedTo", "C");
    let dups = duplicate_id_causes(&prefix_rows);
    let first_cause = prefix_rows.first().map(|r| r.1);

    let (pdir, cdir, _) = persist_three(&t, "fc");
    let mut p = store::open(&pdir).unwrap();
    let mut c = store::open(&cdir).unwrap();
    pad(&mut t, 6, "fc-tail");
    pad(&mut p, 6, "fc-tail");
    pad(&mut c, 6, "fc-tail");
    store::persist(&c, &cdir).unwrap();
    store::checkpoint_sealed_prefix(&cdir).unwrap();
    drop(c);
    let mut c = store::open(&cdir).unwrap();
    replay_ok(&c, "first-cause after re-checkpoint");

    t.emit(Op::Retract { event_id: ab }).unwrap();
    p.emit(Op::Retract { event_id: ab }).unwrap();
    c.emit(Op::Retract { event_id: ab }).unwrap();
    replay_ok(&c, "first-cause retract first path");
    let user = HashSet::from([
        ("B".into(), "relatedTo".into(), "C".into(), 2010, None),
        ("A".into(), "relatedTo".into(), "D".into(), 2010, None),
        ("D".into(), "relatedTo".into(), "C".into(), 2010, None),
    ]);
    // Tail pads mint UUID v7 per arm; named answers only.
    assert_three(
        &t,
        &p,
        &c,
        &user,
        false,
        "R3 first-cause retract A-B after re-checkpoint",
    );
    assert_eq!(
        live_named(&c, "A", "relatedTo", "C"),
        live_named(&t, "A", "relatedTo", "C"),
        "HIGH R2: last-wins compacted_behavior_by_id dropped first cause {first_cause:?} dups={dups:?} prefix_rows={prefix_rows:?}"
    );

    t.emit(Op::Retract { event_id: ad }).unwrap();
    p.emit(Op::Retract { event_id: ad }).unwrap();
    c.emit(Op::Retract { event_id: ad }).unwrap();
    replay_ok(&c, "first-cause retract second path");
    let user = HashSet::from([("B".into(), "relatedTo".into(), "C".into(), 2010, None)]);
    assert_three(
        &t,
        &p,
        &c,
        &user,
        false,
        "R3 first-cause retract both diamond paths",
    );
}

#[test]
fn r3_rollback_user_cascade_after_checkpoint() {
    let _g = tiny_limits();
    let mut t = Runtime::default();
    t.emit_rule_ast(&ghost_from_known_by(), 2010, None).unwrap();
    pad(&mut t, 8, "rb");
    t.emit_tt_anchor(2015).unwrap();
    let (pdir, cdir, _) = persist_three(&t, "rb");
    let mut p = store::open(&pdir).unwrap();
    let mut c = store::open(&cdir).unwrap();
    replay_ok(&c, "rollback open");
    let alice_t = t.intern("Alice");
    let bob_t = t.intern("Bob");
    let knows_t = t.intern("knows");
    let alice_p = p.intern("Alice");
    let bob_p = p.intern("Bob");
    let knows_p = p.intern("knows");
    let alice_c = c.intern("Alice");
    let bob_c = c.intern("Bob");
    let knows_c = c.intern("knows");

    let snap = |rt: &Runtime| {
        (
            rt.log().len(),
            rt.fold().facts().len(),
            rt.fold().fingerprint(),
            product_live(rt),
            rt.outcome_records().len(),
        )
    };
    let before_t = snap(&t);
    let before_p = snap(&p);
    let before_c = snap(&c);

    let emit_knows = |rt: &mut Runtime, s, r, o| {
        rt.emit(Op::Assert {
            subject: s,
            relation: r,
            object: o,
            valid_from: 2010,
            valid_to: None,
            claim: None,
            delivery_key: None,
            polarity: None,
        })
    };

    let err_t =
        emit_knows(&mut t, alice_t, knows_t, bob_t).expect_err("T later-hop ghostRel must fail");
    let err_p =
        emit_knows(&mut p, alice_p, knows_p, bob_p).expect_err("P later-hop ghostRel must fail");
    let err_c =
        emit_knows(&mut c, alice_c, knows_c, bob_c).expect_err("C later-hop ghostRel must fail");
    for (who, err) in [("T", &err_t), ("P", &err_p), ("C", &err_c)] {
        assert!(
            matches!(err, RuntimeError::UnknownRelation { name } if name == "ghostRel"),
            "HIGH R2: {who} expected UnknownRelation ghostRel, got {err}"
        );
    }

    let after_t = snap(&t);
    let after_p = snap(&p);
    let after_c = snap(&c);
    assert_eq!(
        before_t, after_t,
        "HIGH R2: T rollback did not restore pre-cascade"
    );
    assert_eq!(
        before_p, after_p,
        "HIGH R2: P rollback did not restore pre-cascade"
    );
    assert_eq!(
        before_c, after_c,
        "HIGH R2: C rollback via fork_at did not restore pre-cascade"
    );
    assert_named(&c, &t, "R3 rollback C==T");
    assert_named(&c, &p, "R3 rollback C==P");
    replay_ok(&c, "rollback after failed cascade");

    let events = c.log().as_slice();
    let ck = first_checkpoint_index(events);
    let len = c.log().len();
    for n in 1..=ck {
        match c.fork_at(n) {
            Err(RuntimeError::ForkCutUnavailable { cut, log_len })
                if cut == n && log_len == len => {}
            other => panic!(
                "HIGH R1: after failed cascade, fork_at({n}) was {}",
                if other.is_ok() { "Ok" } else { "Err(other)" }
            ),
        }
    }
}

#[test]
fn r3_fork_legal_cuts_then_live_retract_three_arm() {
    let _g = tiny_limits();
    let mut t = Runtime::default();
    t.emit_rule_ast(&horn_trans(), 2010, None).unwrap();
    let ab = emit_keyed(&mut t, "A", "relatedTo", "B", 2010, None, "fk-ab");
    let _bc = emit_keyed(&mut t, "B", "relatedTo", "C", 2010, None, "fk-bc");
    let _cd = emit_keyed(&mut t, "C", "relatedTo", "D", 2010, None, "fk-cd");
    pad(&mut t, 8, "fk");
    t.emit_tt_anchor(2015).unwrap();
    pad(&mut t, 4, "fk-tail");
    let (pdir, cdir, _prefix_len) = persist_three(&t, "fk");
    let p = store::open(&pdir).unwrap();
    let c = store::open(&cdir).unwrap();
    replay_ok(&c, "fork open");
    let events = c.log().as_slice();
    let ck = first_checkpoint_index(events);
    let len = c.log().len();
    let twin_prefix_t = twin_prefix_len_for_checkpointed(&c, &t);
    let twin_prefix_p = twin_prefix_len_for_checkpointed(&c, &p);
    let tail = len - (ck + 1);
    let user_full = HashSet::from([
        ("A".into(), "relatedTo".into(), "B".into(), 2010, None),
        ("B".into(), "relatedTo".into(), "C".into(), 2010, None),
        ("C".into(), "relatedTo".into(), "D".into(), 2010, None),
    ]);
    assert_three(&t, &p, &c, &user_full, true, "R3 fork before retract");

    for i in [0usize, tail / 2, tail] {
        let mut cf = c
            .fork_at(ck + 1 + i)
            .unwrap_or_else(|e| panic!("HIGH R1: C fork_at(c+1+{i}) Err {e}"));
        let mut tf = t
            .fork_at(twin_prefix_t + i)
            .unwrap_or_else(|e| panic!("HIGH R1: T fork_at(twin_prefix_t+{i}) Err {e}"));
        let mut pf = p
            .fork_at(twin_prefix_p + i)
            .unwrap_or_else(|e| panic!("HIGH R1: P fork_at(twin_prefix_p+{i}) Err {e}"));
        replay_ok(&cf, &format!("C fork c+1+{i}"));
        cf.emit(Op::Retract { event_id: ab }).unwrap();
        tf.emit(Op::Retract { event_id: ab }).unwrap();
        pf.emit(Op::Retract { event_id: ab }).unwrap();
        replay_ok(&cf, &format!("C fork retract c+1+{i}"));
        let user = HashSet::from([
            ("B".into(), "relatedTo".into(), "C".into(), 2010, None),
            ("C".into(), "relatedTo".into(), "D".into(), 2010, None),
        ]);
        assert_three(
            &tf,
            &pf,
            &cf,
            &user,
            true,
            &format!("R3 fork_at(c+1+{i}) then retract A-B"),
        );
    }
}

/// Minimised seed-1: transitivity in the prefix, same Horn re-registered after
/// the cut, Retract then re-assert of a prefix premise. C must not mint a
/// second live derived row that T/P do not (delivery-key retry vs a second
/// live `kutha:rule` pin on the compacted fold).
#[test]
fn r3_reregister_same_horn_after_checkpoint() {
    let _g = tiny_limits();
    let mut t = Runtime::default();
    t.emit_rule_ast(&horn_trans(), 2010, None).unwrap();
    let ab = emit_keyed(&mut t, "A", "relatedTo", "B", 2010, None, "rr-ab");
    let _bc = emit_keyed(&mut t, "B", "relatedTo", "C", 2010, None, "rr-bc");
    pad(&mut t, 8, "rr");
    t.emit_tt_anchor(2015).unwrap();
    assert_eq!(live_count(&t, "A", "relatedTo", "C"), 1, "fixture A-C");
    let (pdir, cdir, _) = persist_three(&t, "rr");
    let mut p = store::open(&pdir).unwrap();
    let mut c = store::open(&cdir).unwrap();
    replay_ok(&c, "reregister open");
    let rt = t.emit_rule_ast(&horn_trans(), 2010, None);
    let rp = p.emit_rule_ast(&horn_trans(), 2010, None);
    let rc = c.emit_rule_ast(&horn_trans(), 2010, None);
    match (&rt, &rp, &rc) {
        (Ok(_), Ok(_), Ok(_)) => {}
        _ => panic!(
            "HIGH R2: re-register emit_rule_ast T={} P={} C={}",
            rt.as_ref()
                .err()
                .map(|e| e.to_string())
                .unwrap_or_else(|| "Ok".into()),
            rp.as_ref()
                .err()
                .map(|e| e.to_string())
                .unwrap_or_else(|| "Ok".into()),
            rc.as_ref()
                .err()
                .map(|e| e.to_string())
                .unwrap_or_else(|| "Ok".into()),
        ),
    }
    t.emit(Op::Retract { event_id: ab }).unwrap();
    p.emit(Op::Retract { event_id: ab }).unwrap();
    c.emit(Op::Retract { event_id: ab }).unwrap();
    replay_ok(&c, "reregister after retract A-B");
    assert_eq!(
        live_count(&c, "A", "relatedTo", "C"),
        live_count(&t, "A", "relatedTo", "C"),
        "HIGH R2: after retract, C A-C count diverged from T"
    );
    emit_keyed(&mut t, "A", "relatedTo", "B", 2010, None, "rr-ab-re");
    emit_keyed(&mut p, "A", "relatedTo", "B", 2010, None, "rr-ab-re");
    emit_keyed(&mut c, "A", "relatedTo", "B", 2010, None, "rr-ab-re");
    replay_ok(&c, "reregister after re-assert A-B");
    let n_t = live_count(&t, "A", "relatedTo", "C");
    let n_p = live_count(&p, "A", "relatedTo", "C");
    let n_c = live_count(&c, "A", "relatedTo", "C");
    assert_eq!(
        n_c, n_t,
        "HIGH R2: re-register same Horn after checkpoint; after re-assert A-B live A-C C={n_c} T={n_t} (seed-1 shrink: trans, A-B,B-C, pad, ckp, emit_rule_ast trans, Retract A-B, Assert A-B)"
    );
    assert_eq!(
        n_c, n_p,
        "HIGH R2: re-register same Horn after checkpoint; after re-assert A-B live A-C C={n_c} P={n_p}"
    );
    assert_eq!(
        n_p, n_t,
        "HIGH R2: P==T after re-register+re-assert; P={n_p} T={n_t}"
    );
}
