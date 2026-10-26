//! Phase 237 / M163 S01 — vintage checkpoint lineage fails closed (b9).
//! A blob without `behaviors_exact` must fail `store::open` even when digest,
//! fact_count, and fingerprint match. A twin that keeps the mark opens.

use kutha_common::{Event, Op, RuleAst, RuleAtom, RuleTerm, sha256_hex};
use kutha_runtime::{Runtime, store};
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
    common::PersistScratch::new(&format!("b9-vintage-{label}"))
}

fn no_horn_runtime() -> Runtime {
    let mut rt = Runtime::default();
    let article = rt.intern("art-12-speed");
    let in_force = rt.intern("inForceAs");
    for i in 0..6 {
        let object = rt.intern(&format!("obj-{i}"));
        rt.emit(Op::Assert {
            subject: article,
            relation: in_force,
            object,
            valid_from: 2010,
            valid_to: Some(2020),
            claim: None,
            delivery_key: Some(format!("dk-{i}")),
            polarity: None,
        })
        .unwrap();
    }
    rt
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

fn export_events(dir: &Path) -> Vec<Event> {
    store::export_jsonl(dir).unwrap();
    let text = fs::read_to_string(dir.join("events.jsonl")).unwrap();
    text.lines()
        .filter(|l| !l.trim().is_empty())
        .map(|l| serde_json::from_str::<Event>(l).expect("jsonl Event"))
        .collect()
}

fn rewrite_on_disk_log(dir: &Path, events: &[Event]) {
    let n = events.len();
    let (sealed, active) = if n >= 2 {
        (vec![n - 1], 1)
    } else {
        (Vec::new(), n)
    };
    store::write_test_segmented(dir, events, &sealed, active).unwrap();
}

fn checkpoint_blob_digest(events: &[Event]) -> String {
    events
        .iter()
        .find_map(|e| match &e.op {
            Op::Checkpoint { blob_digest, .. } => Some(blob_digest.clone()),
            _ => None,
        })
        .expect("Op::Checkpoint")
}

fn strip_behaviors_exact(events: &mut [Event]) {
    let digest = checkpoint_blob_digest(events);
    let mut found = false;
    for e in events {
        if let Op::StoreBlob { digest: d, bytes } = &mut e.op {
            if *d == digest {
                let mut v: serde_json::Value =
                    serde_json::from_slice(bytes).expect("checkpoint Snapshot JSON");
                v.as_object_mut()
                    .expect("snapshot object")
                    .remove("behaviors_exact");
                *bytes = serde_json::to_vec(&v).expect("reserialize snapshot");
                found = true;
                break;
            }
        }
    }
    assert!(found, "checkpoint StoreBlob matching receipt digest");
}

fn recompute_blob_digest(events: &mut [Event]) {
    let old = checkpoint_blob_digest(events);
    let mut new_digest = None;
    for e in events.iter_mut() {
        if let Op::StoreBlob { digest: d, bytes } = &mut e.op {
            if *d == old {
                *d = sha256_hex(bytes);
                new_digest = Some(d.clone());
            }
        }
    }
    let new_digest = new_digest.expect("StoreBlob to rehash");
    for e in events {
        if let Op::Checkpoint { blob_digest, .. } = &mut e.op {
            *blob_digest = new_digest.clone();
        }
    }
}

#[test]
fn unmarked_checkpoint_blob_fails_open() {
    let _g = tiny_limits();
    let rt = no_horn_runtime();

    let marked = persist_dir("marked");
    store::persist(&rt, &marked).unwrap();
    store::checkpoint_sealed_prefix(&marked).unwrap();
    store::open(&marked).expect("twin that keeps the mark must open");

    let unmarked = persist_dir("unmarked");
    copy_store(&marked, &unmarked);
    let mut events = export_events(&unmarked);
    strip_behaviors_exact(&mut events);
    recompute_blob_digest(&mut events);
    rewrite_on_disk_log(&unmarked, &events);

    match store::open(&unmarked) {
        Err(_) => {}
        Ok(_) => panic!("unmarked checkpoint blob must fail open (got Ok)"),
    }
}

fn must_fail_open(dir: &Path, what: &str) {
    match store::open(dir) {
        Err(_) => {}
        Ok(_) => panic!("{what}: store::open must fail (got Ok)"),
    }
}

fn mutate_checkpoint_snapshot(events: &mut [Event], f: impl FnOnce(&mut serde_json::Value)) {
    let digest = checkpoint_blob_digest(events);
    let mut found = false;
    for e in events {
        if let Op::StoreBlob { digest: d, bytes } = &mut e.op {
            if *d == digest {
                let mut v: serde_json::Value =
                    serde_json::from_slice(bytes).expect("checkpoint Snapshot JSON");
                f(&mut v);
                *bytes = serde_json::to_vec(&v).expect("reserialize snapshot");
                found = true;
                break;
            }
        }
    }
    assert!(found, "checkpoint StoreBlob matching receipt digest");
}

fn checkpoint_blob_json(events: &[Event]) -> serde_json::Value {
    let digest = checkpoint_blob_digest(events);
    events
        .iter()
        .find_map(|e| match &e.op {
            Op::StoreBlob { digest: d, bytes } if *d == digest => {
                Some(serde_json::from_slice(bytes).expect("checkpoint Snapshot JSON"))
            }
            _ => None,
        })
        .expect("checkpoint StoreBlob")
}

fn store_tree(dir: &Path) -> Vec<(String, Vec<u8>)> {
    fn walk(root: &Path, cur: &Path, out: &mut Vec<(String, Vec<u8>)>) {
        let mut ents: Vec<_> = fs::read_dir(cur).unwrap().map(|e| e.unwrap()).collect();
        ents.sort_by_key(|e| e.file_name());
        for ent in ents {
            let name = ent.file_name();
            if name == "kutha.lock" {
                continue;
            }
            let path = ent.path();
            let rel = path
                .strip_prefix(root)
                .unwrap()
                .to_string_lossy()
                .into_owned();
            if path.is_dir() {
                walk(root, &path, out);
            } else {
                out.push((rel, fs::read(&path).unwrap()));
            }
        }
    }
    let mut out = Vec::new();
    walk(dir, dir, &mut out);
    out.sort_by(|a, b| a.0.cmp(&b.0));
    out
}

fn horn_runtime() -> Runtime {
    let mut rt = Runtime::default();
    rt.emit_allow("derive_pq", 0, None).unwrap();
    let head = RuleAtom::new(
        "relatedTo",
        RuleTerm::var(0).unwrap(),
        RuleTerm::constant("derived").unwrap(),
    )
    .unwrap();
    let prem = RuleAtom::new(
        "relatedTo",
        RuleTerm::var(0).unwrap(),
        RuleTerm::constant("true").unwrap(),
    )
    .unwrap();
    let ast = RuleAst::horn(head, vec![prem]).unwrap();
    let rule_pin = ast.hash();
    rt.emit_rule_ast(&ast, 2010, None).unwrap();
    let p = rt.intern("P");
    let rel = rt.intern("relatedTo");
    let true_ = rt.intern("true");
    let derived = rt.intern("derived");
    let asserted = rt
        .emit(Op::Assert {
            subject: p,
            relation: rel,
            object: true_,
            valid_from: 2010,
            valid_to: None,
            claim: None,
            delivery_key: None,
            polarity: None,
        })
        .unwrap();
    let cause = asserted.receipt.event_ids[0];
    rt.emit(Op::Behavior {
        name: "derive_pq".into(),
        caused_by: cause,
        rule_version: rule_pin,
        subject: p,
        relation: rel,
        object: derived,
        valid_from: 2010,
        valid_to: None,
    })
    .unwrap();
    let relation = rt.intern("inForceAs");
    for i in 0..6 {
        let subject = rt.intern(&format!("art-{i}"));
        let object = rt.intern(&format!("obj-{i}"));
        rt.emit(Op::Assert {
            subject,
            relation,
            object,
            valid_from: 2010,
            valid_to: Some(2020),
            claim: None,
            delivery_key: Some(format!("dk-{i}")),
            polarity: None,
        })
        .unwrap();
    }
    rt
}

#[test]
fn explicit_false_behaviors_exact_fails_open() {
    let _g = tiny_limits();
    let rt = no_horn_runtime();
    let marked = persist_dir("false-src");
    store::persist(&rt, &marked).unwrap();
    store::checkpoint_sealed_prefix(&marked).unwrap();

    let dir = persist_dir("false-mark");
    copy_store(&marked, &dir);
    let mut events = export_events(&dir);
    mutate_checkpoint_snapshot(&mut events, |v| {
        v.as_object_mut()
            .expect("snapshot object")
            .insert("behaviors_exact".into(), serde_json::json!(false));
    });
    recompute_blob_digest(&mut events);
    rewrite_on_disk_log(&dir, &events);
    must_fail_open(&dir, "explicit behaviors_exact:false with matching digest");
}

#[test]
fn marked_no_horn_checkpoint_opens() {
    let _g = tiny_limits();
    let rt = no_horn_runtime();
    let encoded = serde_json::to_value(rt.snapshot()).expect("Runtime::snapshot JSON");
    assert_eq!(
        encoded.get("behaviors_exact"),
        Some(&serde_json::json!(true)),
        "Runtime::snapshot always serializes behaviors_exact"
    );
    assert!(
        encoded.get("behaviors").is_none()
            || encoded["behaviors"].as_array().is_some_and(|a| a.is_empty()),
        "no-Horn Runtime::snapshot omits or empties behaviors"
    );

    let dir = persist_dir("no-horn-open");
    store::persist(&rt, &dir).unwrap();
    store::checkpoint_sealed_prefix(&dir).unwrap();
    let events = export_events(&dir);
    let blob = checkpoint_blob_json(&events);
    assert_eq!(
        blob.get("behaviors_exact"),
        Some(&serde_json::json!(true)),
        "checkpoint blob always serializes behaviors_exact"
    );
    assert!(
        blob.get("behaviors").is_none()
            || blob["behaviors"].as_array().is_some_and(|a| a.is_empty()),
        "no-Horn checkpoint blob omits or empties behaviors"
    );
    let opened = store::open(&dir).expect("marked no-Horn checkpoint must open");
    let snap = opened.snapshot();
    assert!(snap.behaviors_exact);
    assert!(snap.behaviors.is_empty());
}

#[test]
fn marked_horn_checkpoint_restores_behaviors() {
    let _g = tiny_limits();
    let rt = horn_runtime();
    let dir = persist_dir("horn-restore");
    store::persist(&rt, &dir).unwrap();
    store::checkpoint_sealed_prefix(&dir).unwrap();
    let events = export_events(&dir);
    let blob = checkpoint_blob_json(&events);
    assert_eq!(blob.get("behaviors_exact"), Some(&serde_json::json!(true)));
    let blob_behaviors = blob
        .get("behaviors")
        .and_then(|b| b.as_array())
        .expect("Horn checkpoint blob must serialize behaviors");
    assert!(
        !blob_behaviors.is_empty(),
        "Horn checkpoint blob must carry Behavior rows"
    );

    let opened = store::open(&dir).expect("marked Horn checkpoint must open");
    let snap = opened.snapshot();
    assert!(snap.behaviors_exact);
    let restored: Vec<_> = snap
        .behaviors
        .iter()
        .filter(|e| matches!(&e.op, Op::Behavior { name, .. } if name == "derive_pq"))
        .collect();
    assert_eq!(
        restored.len(),
        1,
        "open must restore compacted Horn Behavior lineage"
    );
}

#[test]
fn digest_mismatch_with_mark_true_fails_open() {
    let _g = tiny_limits();
    let rt = no_horn_runtime();
    let marked = persist_dir("digest-src");
    store::persist(&rt, &marked).unwrap();
    store::checkpoint_sealed_prefix(&marked).unwrap();

    let dir = persist_dir("digest-mismatch");
    copy_store(&marked, &dir);
    let mut events = export_events(&dir);
    mutate_checkpoint_snapshot(&mut events, |v| {
        assert_eq!(v.get("behaviors_exact"), Some(&serde_json::json!(true)));
        let offset = v
            .get("log_offset")
            .and_then(|x| x.as_u64())
            .expect("log_offset");
        v.as_object_mut()
            .expect("snapshot object")
            .insert("log_offset".into(), serde_json::json!(offset + 1));
    });
    rewrite_on_disk_log(&dir, &events);
    must_fail_open(&dir, "digest mismatch with behaviors_exact true");
}

#[test]
fn recheckpoint_does_not_launder_vintage() {
    let _g = tiny_limits();
    let rt = no_horn_runtime();
    let marked = persist_dir("launder-src");
    store::persist(&rt, &marked).unwrap();
    store::checkpoint_sealed_prefix(&marked).unwrap();

    let dir = persist_dir("launder");
    copy_store(&marked, &dir);
    let mut events = export_events(&dir);
    strip_behaviors_exact(&mut events);
    recompute_blob_digest(&mut events);
    rewrite_on_disk_log(&dir, &events);
    must_fail_open(&dir, "vintage blob before re-checkpoint");

    let before = store_tree(&dir);
    match store::checkpoint_sealed_prefix(&dir) {
        Err(_) => {}
        Ok(()) => panic!("checkpoint_sealed_prefix must not rewrite a vintage blob"),
    }
    let after = store_tree(&dir);
    assert_eq!(
        before, after,
        "failed re-checkpoint must leave store bytes unchanged (except kutha.lock)"
    );
    must_fail_open(&dir, "vintage blob after failed re-checkpoint");
}
