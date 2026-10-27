//! Phase 239 / M165 S01 — compacted Horn `caused_by` must support the head.
//! A cited checkpoint blob can keep `behaviors_exact`, a matching digest, and
//! the same fold fingerprint while pointing `caused_by` at nil or at another
//! assert. Log `seen` is not the test: an honest prefix assert is a restored
//! fact, not a log row.

use kutha_common::{Event, EventId, Op, RuleAst, RuleAtom, RuleTerm, sha256_hex};
use kutha_runtime::{Runtime, Snapshot, store};
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
    common::PersistScratch::new(&format!("b11-cause-{label}"))
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

fn checkpoint_horn(label: &str) -> common::PersistScratch {
    let rt = horn_runtime();
    let dir = persist_dir(label);
    store::persist(&rt, &dir).unwrap();
    store::checkpoint_sealed_prefix(&dir).unwrap();
    dir
}

fn mutate_compacted_caused_by(events: &mut [Event], new_cause: EventId) -> EventId {
    let digest = checkpoint_blob_digest(events);
    let mut old_cause = None;
    for e in events {
        if let Op::StoreBlob { digest: d, bytes } = &mut e.op {
            if *d == digest {
                let mut snap: Snapshot =
                    serde_json::from_slice(bytes).expect("checkpoint Snapshot");
                assert!(
                    snap.behaviors_exact,
                    "rewrite must keep behaviors_exact true"
                );
                let b = snap
                    .behaviors
                    .get_mut(0)
                    .expect("Horn checkpoint blob must carry Behavior rows");
                let Op::Behavior { caused_by, .. } = &mut b.op else {
                    panic!("behaviors[0] must be Op::Behavior");
                };
                old_cause = Some(*caused_by);
                *caused_by = new_cause;
                *bytes = serde_json::to_vec(&snap).expect("reserialize snapshot");
                break;
            }
        }
    }
    old_cause.expect("rewrote compacted caused_by")
}

fn swapped_cause_in_fold(events: &[Event], original: EventId) -> EventId {
    let digest = checkpoint_blob_digest(events);
    let snap: Snapshot = events
        .iter()
        .find_map(|e| match &e.op {
            Op::StoreBlob { digest: d, bytes } if *d == digest => {
                Some(serde_json::from_slice(bytes).expect("checkpoint Snapshot"))
            }
            _ => None,
        })
        .expect("checkpoint StoreBlob");
    assert!(snap.behaviors_exact);
    snap.fold
        .facts()
        .iter()
        .map(|f| f.event_id)
        .find(|id| *id != original && *id != EventId::nil())
        .expect("another assert id in the fold")
}

fn fail_open_msg(dir: &Path, what: &str) -> String {
    match store::open(dir) {
        Err(e) => e.to_string(),
        Ok(_) => panic!("{what} (got Ok)"),
    }
}

#[test]
fn honest_horn_checkpoint_opens() {
    let _g = tiny_limits();
    let dir = checkpoint_horn("honest");
    store::open(&dir).expect("unmodified Horn checkpoint of the same fixture must open");
}

#[test]
fn nil_caused_by_in_cited_blob_fails_open() {
    let _g = tiny_limits();
    let dir = checkpoint_horn("nil");
    let mut events = export_events(&dir);
    let old = mutate_compacted_caused_by(&mut events, EventId::nil());
    assert_ne!(old, EventId::nil(), "fixture cause must not already be nil");
    recompute_blob_digest(&mut events);
    rewrite_on_disk_log(&dir, &events);
    let msg = fail_open_msg(&dir, "nil caused_by in cited blob must fail open");
    assert!(
        msg.starts_with("broken lineage caused_by="),
        "expected BrokenLineage, got {msg}"
    );
}

#[test]
fn swapped_caused_by_in_cited_blob_fails_open() {
    let _g = tiny_limits();
    let dir = checkpoint_horn("swap");
    let mut events = export_events(&dir);
    let original = {
        let digest = checkpoint_blob_digest(&events);
        let snap: Snapshot = events
            .iter()
            .find_map(|e| match &e.op {
                Op::StoreBlob { digest: d, bytes } if *d == digest => {
                    Some(serde_json::from_slice(bytes).expect("checkpoint Snapshot"))
                }
                _ => None,
            })
            .expect("checkpoint StoreBlob");
        match &snap.behaviors[0].op {
            Op::Behavior { caused_by, .. } => *caused_by,
            _ => panic!("behaviors[0] must be Op::Behavior"),
        }
    };
    let swapped = swapped_cause_in_fold(&events, original);
    assert_ne!(swapped, original);
    mutate_compacted_caused_by(&mut events, swapped);
    recompute_blob_digest(&mut events);
    rewrite_on_disk_log(&dir, &events);
    let msg = fail_open_msg(&dir, "swapped caused_by in cited blob must fail open");
    assert!(
        msg.starts_with("broken lineage caused_by="),
        "expected BrokenLineage, got {msg}"
    );
}
