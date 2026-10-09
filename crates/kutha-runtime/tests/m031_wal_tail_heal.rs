//! M031 S01 / Phase 91 — jsonl-ahead-of-WAL must heal on open_for_append
//! (CE review P1/P2 #30). Open prefers covering jsonl with dropped==0; attach
//! must not stay wedged on the WAL tail check.

use kutha_common::Op;
use kutha_runtime::{Runtime, store};
use std::fs;
use std::path::PathBuf;

fn persist_dir(label: &str) -> PathBuf {
    let nanos = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join("tmp")
        .join(format!(
            "m031-wal-tail-heal-{label}-{}-{}",
            std::process::id(),
            nanos
        ));
    let _ = fs::remove_dir_all(&dir);
    fs::create_dir_all(&dir).unwrap();
    dir
}

fn seed_runtime() -> Runtime {
    let mut rt = Runtime::default();
    let s = rt.intern("m031-s");
    let r = rt.intern("relatedTo");
    let o = rt.intern("m031-o");
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
    rt
}

#[test]
fn open_for_append_heals_jsonl_ahead_of_wal() {
    let mut rt = seed_runtime();
    let dir = persist_dir("jsonl-ahead");
    store::ensure_legacy_dir(&dir).unwrap();
    store::persist(&rt, &dir).unwrap();

    let mut p = store::Persister::attach(&rt, &dir).unwrap();
    rt.emit_tt_anchor(2021).unwrap();
    let after_emit_fp = rt.fold().fingerprint();
    store::append_with_failpoint(&mut p, &rt, store::AppendStep::Jsonl)
        .expect_err("Jsonl failpoint must leave WAL short");
    drop(p);

    let (opened, report) = store::open_report(&dir).expect("open prefers covering jsonl");
    assert_eq!(
        report.dropped_uncommitted, 0,
        "complete jsonl quantum is visible; heal must not rely on dropped_uncommitted alone"
    );
    assert_eq!(opened.fold().fingerprint(), after_emit_fp);

    let (mut healed, mut writer) = store::open_for_append(&dir)
        .expect("open_for_append must rewrite WAL/jsonl then attach");
    assert_eq!(healed.fold().fingerprint(), after_emit_fp);

    healed.emit_tt_anchor(2022).unwrap();
    writer.append(&healed).expect("append after WAL-tail heal");
    let again = store::open(&dir).expect("reopen after post-heal append");
    assert_eq!(again.fold().fingerprint(), healed.fold().fingerprint());
    again.replay_check().unwrap();
    let _ = fs::remove_dir_all(&dir);
}
