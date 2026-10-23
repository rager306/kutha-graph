//! M013 S03 / Phase 24: ingest fold indexes — visit counts constant in N.

use kutha_common::Op;
use kutha_runtime::{Runtime, store};
use std::fs;

fn emit_keyed(rt: &mut Runtime, i: u32, rel: kutha_common::TermId, obj: kutha_common::TermId) {
    let s = rt.intern(&format!("s{i}"));
    rt.emit(Op::Assert {
        subject: s,
        relation: rel,
        object: obj,
        valid_from: 2010,
        valid_to: None,
        claim: None,
        delivery_key: Some(format!("dk{i}")),
        polarity: None,
    })
    .unwrap();
}

#[test]
fn green_ingest_visits_constant_in_n() {
    let mut visits = Vec::new();
    for &n in &[64u32, 128, 256] {
        let mut rt = Runtime::default();
        let rel = rt.intern("relatedTo");
        let obj = rt.intern("o");
        for i in 0..n {
            emit_keyed(&mut rt, i, rel, obj);
        }
        rt.reset_ingest_examine_count();
        // Retry first keyed assert (delivery_key path) + admit onto its claim.
        let first = rt.log().as_slice()[0].id; // may be Define; find first Assert
        let (first_id, first_key) = rt
            .log()
            .as_slice()
            .iter()
            .find_map(|e| match &e.op {
                Op::Assert {
                    delivery_key: Some(k),
                    ..
                } if k.starts_with("dk") => Some((e.id, k.clone())),
                _ => None,
            })
            .expect("keyed assert");
        let _ = first;
        let s0 = rt.intern("s0");
        rt.emit(Op::Assert {
            subject: s0,
            relation: rel,
            object: obj,
            valid_from: 2010,
            valid_to: None,
            claim: None,
            delivery_key: Some(first_key),
            polarity: None,
        })
        .unwrap();
        // Unknown-claim fail path still uses claim_known (one bump).
        let ghost = rt.intern("ghost");
        let _ = rt.emit(Op::Assert {
            subject: ghost,
            relation: rel,
            object: obj,
            valid_from: 2010,
            valid_to: None,
            claim: Some(kutha_common::EventId::nil()),
            delivery_key: None,
            polarity: None,
        });
        let _ = first_id;
        visits.push(rt.ingest_examine_count());
    }
    assert!(
        visits[2] <= visits[0] + 8,
        "ingest visits must not grow with N: {visits:?}"
    );
}

#[test]
fn open_retract_reemit_indexes_match_fingerprint() {
    let mut rt = Runtime::default();
    let rel = rt.intern("relatedTo");
    let obj = rt.intern("o");
    emit_keyed(&mut rt, 0, rel, obj);
    let id = rt
        .log()
        .as_slice()
        .iter()
        .find_map(|e| match &e.op {
            Op::Assert {
                delivery_key: Some(k),
                ..
            } if k.starts_with("dk") => Some(e.id),
            _ => None,
        })
        .expect("keyed product assert");
    let nanos = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    let dir = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join("tmp")
        .join(format!("kutha-p24-{nanos}"));
    let _ = fs::remove_dir_all(&dir);
    fs::create_dir_all(&dir).unwrap();
    store::persist(&rt, &dir).unwrap();
    let mut opened = store::open(&dir).unwrap();
    opened.emit(Op::Retract { event_id: id }).unwrap();
    emit_keyed(&mut opened, 1, rel, obj);
    let fp = opened.fold().fingerprint();
    store::persist(&opened, &dir).unwrap();
    let rebuilt = store::open(&dir).unwrap();
    assert_eq!(
        fp,
        rebuilt.fold().fingerprint(),
        "persist+open after retract+reemit must match in-memory fingerprint"
    );
    assert_eq!(
        fp,
        opened.replay_check().unwrap().fingerprint(),
        "replay_check must match in-memory fingerprint"
    );
    let mut forked = opened.fork_at(opened.log().len()).unwrap();
    emit_keyed(&mut forked, 2, rel, obj);
    assert_eq!(
        forked.fold().fingerprint(),
        forked.replay_check().unwrap().fingerprint()
    );
    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn fork_at_parent_indexes_untouched() {
    let mut parent = Runtime::default();
    let rel = parent.intern("relatedTo");
    let obj = parent.intern("o");
    emit_keyed(&mut parent, 0, rel, obj);
    let before = parent.fold().fingerprint();
    let n = parent.log().len();
    let mut child = parent.fork_at(n).unwrap();
    emit_keyed(&mut child, 1, rel, obj);
    assert_eq!(parent.fold().fingerprint(), before);
    assert_ne!(child.fold().fingerprint(), before);
}

#[test]
fn admit_allow_lookup_is_constant_in_product_n() {
    let mut examined = Vec::new();
    for &n in &[64u32, 256] {
        let mut rt = Runtime::default();
        let rel = rt.intern("relatedTo");
        let obj = rt.intern("o");
        for i in 0..n {
            let s = rt.intern(&format!("s{i}"));
            rt.emit(Op::Assert {
                subject: s,
                relation: rel,
                object: obj,
                valid_from: 2010,
                valid_to: None,
                claim: None,
                delivery_key: None,
                polarity: None,
            })
            .unwrap();
        }
        rt.fold().reset_hot_examine_count();
        let extra = rt.intern("s-extra");
        rt.emit(Op::Assert {
            subject: extra,
            relation: rel,
            object: obj,
            valid_from: 2010,
            valid_to: None,
            claim: None,
            delivery_key: None,
            polarity: None,
        })
        .unwrap();
        examined.push(rt.fold().hot_examine_count());
    }
    assert!(
        examined[1] <= examined[0] + 8,
        "admit kutha:allows lookup must not grow with product N: {examined:?}"
    );
    assert!(
        examined[1] < 32,
        "admit must not collect live product facts: {examined:?}"
    );
}
