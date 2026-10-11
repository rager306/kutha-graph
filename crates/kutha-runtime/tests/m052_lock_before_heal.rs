//! M052 S03 / Phase 114 — attach must not heal or write before `kutha.lock`
//! (CE whole-repo P1 #7). A losing concurrent attach is denied with no byte change.

use kutha_common::Op;
use kutha_runtime::{Runtime, store};
use std::collections::BTreeMap;
use std::fs;
use std::io::ErrorKind;
use std::path::{Path, PathBuf};

struct LimitGuard;

impl Drop for LimitGuard {
    fn drop(&mut self) {
        store::reset_test_segment_limits();
    }
}

fn tiny_limits() -> LimitGuard {
    store::set_test_segment_limits(Some(4096), Some(3));
    LimitGuard
}

fn persist_dir(label: &str) -> PathBuf {
    let nanos = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join("tmp")
        .join(format!(
            "m052-lock-before-heal-{label}-{}-{}",
            std::process::id(),
            nanos
        ));
    let _ = fs::remove_dir_all(&dir);
    fs::create_dir_all(&dir).unwrap();
    dir
}

fn seed_runtime() -> Runtime {
    let mut rt = Runtime::default();
    let subject = rt.intern("art-m052-s03");
    let related_to = rt.intern("relatedTo");
    let object = rt.intern("obj-m052-s03");
    rt.emit(Op::Assert {
        subject,
        relation: related_to,
        object,
        valid_from: 2010,
        valid_to: Some(2020),
        claim: None,
        delivery_key: None,
        polarity: None,
    })
    .unwrap();
    rt.emit_tt_anchor(2015).unwrap();
    for i in 0..8 {
        let _ = rt.intern(&format!("m052-s03-pad-{i}"));
    }
    rt
}

fn overlay_segmented(rt: &Runtime, dir: &Path) {
    store::persist(rt, dir).unwrap();
    let events = store::encoded_events(rt);
    let n = events.len();
    assert!(n >= 10);
    store::write_test_segmented(dir, &events, &[4, 4], n - 8).unwrap();
    let man: serde_json::Value =
        serde_json::from_str(&fs::read_to_string(dir.join(store::MANIFEST_REL)).unwrap()).unwrap();
    let mut keep = std::collections::HashSet::new();
    keep.insert(man["active"].as_str().unwrap().to_string());
    for s in man["sealed"].as_array().unwrap() {
        keep.insert(s["name"].as_str().unwrap().to_string());
    }
    for entry in fs::read_dir(dir).unwrap() {
        let entry = entry.unwrap();
        let name = entry.file_name().to_string_lossy().into_owned();
        if name.starts_with("seg-") && name.ends_with(".log") && !keep.contains(&name) {
            let _ = fs::remove_file(entry.path());
        }
    }
    let _ = fs::remove_file(dir.join("events.jsonl"));
    let _ = fs::remove_file(dir.join("events.wal"));
}

/// Store files except `kutha.lock` (the lock fd is held by the live writer).
fn store_payloads(dir: &Path) -> BTreeMap<String, Vec<u8>> {
    let mut out = BTreeMap::new();
    for entry in fs::read_dir(dir).unwrap() {
        let entry = entry.unwrap();
        let name = entry.file_name().to_string_lossy().into_owned();
        if name == store::STORE_LOCK_REL {
            continue;
        }
        if !entry.file_type().unwrap().is_file() {
            continue;
        }
        out.insert(name, fs::read(entry.path()).unwrap());
    }
    out
}

fn assert_writer_lock_error(err: &std::io::Error) {
    assert_eq!(
        err.kind(),
        ErrorKind::WouldBlock,
        "second writer must fail closed with WouldBlock; got {err}"
    );
    assert!(
        err.to_string().contains(store::STORE_LOCK_REL),
        "error must name the lock file; got {err}"
    );
}

fn assert_payloads_unchanged(
    before: &BTreeMap<String, Vec<u8>>,
    after: &BTreeMap<String, Vec<u8>>,
) {
    assert_eq!(
        before.keys().collect::<Vec<_>>(),
        after.keys().collect::<Vec<_>>(),
        "losing attach must not create or delete store files; before {:?} after {:?}",
        before.keys().collect::<Vec<_>>(),
        after.keys().collect::<Vec<_>>()
    );
    for (name, bytes) in before {
        assert_eq!(
            after.get(name).map(Vec::as_slice),
            Some(bytes.as_slice()),
            "losing attach must not mutate {name}"
        );
    }
}

/// Live writer holds `kutha.lock` after an interrupted rollover seal. A second
/// attach is denied (`WouldBlock`) and must not publish a healed manifest or
/// create the next segment.
#[test]
fn losing_attach_while_writer_holds_lock_does_not_heal() {
    let _limits = tiny_limits();
    let mut rt = seed_runtime();
    let dir = persist_dir("held-lock");
    overlay_segmented(&rt, &dir);
    let mut p = store::Persister::attach(&rt, &dir).unwrap();
    rt.emit_tt_anchor(2021).unwrap();
    store::append_with_failpoint(&mut p, &rt, store::AppendStep::RolloverTrailer)
        .expect_err("rollover failpoint must inject");

    let before = store_payloads(&dir);
    assert!(
        !before.is_empty(),
        "interrupted-seal store must have payload files"
    );

    let opened = store::open(&dir).expect("reader open during interrupted seal");
    let err = match store::Persister::attach(&opened, &dir) {
        Ok(_) => panic!("second attach must fail closed while the writer holds kutha.lock"),
        Err(e) => e,
    };
    assert_writer_lock_error(&err);
    assert_payloads_unchanged(&before, &store_payloads(&dir));

    drop(p);
    let _ = fs::remove_dir_all(&dir);
}

/// Once the interrupting writer drops the lock, attach may finish the seal.
#[test]
fn attach_heals_interrupted_seal_after_lock_released() {
    let _limits = tiny_limits();
    let mut rt = seed_runtime();
    let dir = persist_dir("after-drop");
    overlay_segmented(&rt, &dir);
    let committed_fp = rt.fold().fingerprint();
    let mut p = store::Persister::attach(&rt, &dir).unwrap();
    rt.emit_tt_anchor(2021).unwrap();
    store::append_with_failpoint(&mut p, &rt, store::AppendStep::RolloverTrailer)
        .expect_err("rollover failpoint must inject");
    let before_heal = store_payloads(&dir);

    drop(p);
    let opened = store::open(&dir).expect("reader open");
    assert_eq!(opened.fold().fingerprint(), committed_fp);
    let mut writer = store::Persister::attach(&opened, &dir)
        .expect("attach must finish interrupted rollover after the lock is free");
    let after_heal = store_payloads(&dir);
    assert_ne!(
        before_heal, after_heal,
        "heal under the lock must publish the sealed layout"
    );

    let mut healed = store::open(&dir).expect("reopen after attach heal");
    assert_eq!(healed.fold().fingerprint(), committed_fp);
    healed.emit_tt_anchor(2022).unwrap();
    writer.append(&healed).expect("append after heal");
    let again = store::open(&dir).expect("reopen after post-heal append");
    assert_eq!(again.fold().fingerprint(), healed.fold().fingerprint());
    let _ = fs::remove_dir_all(&dir);
}
