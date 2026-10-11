//! M056 S02 / Phase 124 — CE whole-repo #29 / #30.
//! Short orphan next-segment headers must recreate on heal; migrate verify-fail
//! must sync_dir after removing a bad manifest.

use kutha_common::Op;
use kutha_runtime::{Runtime, store};
use std::fs;
use std::io::{ErrorKind, Write};
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
            "m056-persist-heal-{label}-{}-{}",
            std::process::id(),
            nanos
        ));
    let _ = fs::remove_dir_all(&dir);
    fs::create_dir_all(&dir).unwrap();
    dir
}

fn seed_runtime() -> Runtime {
    let mut rt = Runtime::default();
    let article = rt.intern("art-m056");
    let in_force = rt.intern("inForceAs");
    let limit = rt.intern("50-kmh");
    rt.emit(Op::Assert {
        subject: article,
        relation: in_force,
        object: limit,
        valid_from: 2010,
        valid_to: Some(2020),
        claim: None,
        delivery_key: None,
        polarity: None,
    })
    .unwrap();
    rt.emit_tt_anchor(2015).unwrap();
    for i in 0..8 {
        let _ = rt.intern(&format!("m056-pad-{i}"));
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

/// #29: short orphan next segment after RolloverNewHeader must recreate, not wedge.
#[test]
fn open_for_append_recreates_short_orphan_next_header() {
    let _limits = tiny_limits();
    let mut rt = seed_runtime();
    let dir = persist_dir("short-orphan");
    overlay_segmented(&rt, &dir);
    let committed_fp = rt.fold().fingerprint();
    let mut p = store::Persister::attach(&rt, &dir).unwrap();
    rt.emit_tt_anchor(2021).unwrap();
    store::append_with_failpoint(&mut p, &rt, store::AppendStep::RolloverNewHeader)
        .expect_err("rollover failpoint must inject");

    // Crash mid-header-write shape: the *next* seg (not sealed, not manifest.active)
    // exists but is shorter than HEADER_LEN.
    let man: serde_json::Value =
        serde_json::from_str(&fs::read_to_string(dir.join(store::MANIFEST_REL)).unwrap()).unwrap();
    let active = man["active"].as_str().unwrap().to_string();
    let mut sealed = std::collections::HashSet::new();
    for s in man["sealed"].as_array().unwrap() {
        sealed.insert(s["name"].as_str().unwrap().to_string());
    }
    let mut candidates: Vec<PathBuf> = fs::read_dir(&dir)
        .unwrap()
        .filter_map(|e| e.ok())
        .filter_map(|e| {
            let name = e.file_name().to_string_lossy().into_owned();
            if name.starts_with("seg-")
                && name.ends_with(".log")
                && name != active
                && !sealed.contains(&name)
            {
                Some(e.path())
            } else {
                None
            }
        })
        .collect();
    candidates.sort();
    let orphan = candidates
        .pop()
        .expect("RolloverNewHeader must have created an unreferenced next segment");
    {
        let mut f = fs::OpenOptions::new()
            .write(true)
            .truncate(true)
            .open(&orphan)
            .unwrap();
        f.write_all(b"KUTHASEG").unwrap();
        f.sync_all().unwrap();
    }
    assert!(
        fs::metadata(&orphan).unwrap().len() < store::SEGMENT_HEADER_LEN,
        "orphan must be shorter than a full header"
    );

    drop(p);
    let (mut healed, mut writer) = store::open_for_append(&dir).expect(
        "open_for_append must recreate a short unreferenced next-segment header (#29)",
    );
    assert_eq!(healed.fold().fingerprint(), committed_fp);
    healed.emit_tt_anchor(2022).unwrap();
    writer.append(&healed).expect("append after short-orphan heal");
    let again = store::open(&dir).unwrap();
    assert_eq!(again.fold().fingerprint(), healed.fold().fingerprint());
    let _ = fs::remove_dir_all(&dir);
}

/// #30: migrate verify-fail must sync_dir after removing the bad manifest.
#[test]
fn migrate_verify_fail_syncs_dir_after_manifest_remove() {
    // Source pin: remove_file(&man) must be followed by sync_dir before Err return.
    let src = include_str!("../src/store.rs");
    let remove = src
        .find("fs::remove_file(&man)?")
        .expect("migrate unflip must remove manifest");
    let after_remove = &src[remove..];
    let sync = after_remove
        .find("sync_dir(dir)?")
        .expect("migrate unflip must sync_dir after remove (#30)");
    let ret_err = after_remove
        .find("return Err(e)")
        .expect("migrate unflip returns the verify error");
    assert!(
        sync < ret_err,
        "sync_dir must run after remove_file and before return Err"
    );

    let mut rt = Runtime::default();
    let s = rt.intern("m056-mig-s");
    let r = rt.intern("relatedTo");
    let o = rt.intern("m056-mig-o");
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
    .unwrap();
    rt.emit_tt_anchor(2015).unwrap();

    let dir = persist_dir("mig-sync");
    store::ensure_legacy_dir(&dir).unwrap();
    // ensure_legacy + persist on non-empty legacy keeps jsonl/WAL.
    store::persist(&rt, &dir).unwrap();
    assert!(dir.join("events.jsonl").is_file());

    let err = store::migrate_with_verify_fault(&dir, store::MigrateVerifyFault::FlipSealedByte)
        .expect_err("verify fault must fail");
    assert_eq!(err.kind(), ErrorKind::InvalidData);
    assert!(
        !dir.join(store::MANIFEST_REL).exists(),
        "manifest must be removed on verify fail"
    );
    assert!(dir.join("events.jsonl").exists());
    let opened = store::open(&dir).expect("legacy open after unflip");
    assert!(!opened.log().as_slice().is_empty());
    let _ = fs::remove_dir_all(&dir);
}
