//! H2: harness JSONL is a tenant of the product log. Query AS OF the process.
//! Delivery facts, not legal norms. Architecture SoT remains docs/ADR/.

use kutha_runtime::{store, Runtime};
use std::path::Path;

const T_FAIL: u64 = 1500;
const T_OK: u64 = 2500;

#[test]
fn h2_harness_status_as_of_t1_differs_from_as_of_t2() {
    let mut rt = Runtime::default();
    let fixture = concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/tests/fixtures/harness_process.jsonl"
    );
    let report = kutha_runtime::ingest_harness_jsonl(&mut rt, Path::new(fixture)).unwrap();
    assert_eq!(2, report.status_facts);
    assert_eq!(Some("ok"), report.last_status.as_deref());

    let run = rt.intern("harness.run");
    let rel = rt.intern("runStatus");
    let fail = rt.intern("fail");
    let ok = rt.intern("ok");

    let old = rt.fold().as_of(T_FAIL);
    let new = rt.fold().as_of(T_OK);
    assert_ne!(old, new, "AS OF process T1 must differ from AS OF T2");
    assert!(old.contains(&(run, rel, fail)));
    assert!(!old.contains(&(run, rel, ok)));
    assert!(new.contains(&(run, rel, ok)));
    assert!(!new.contains(&(run, rel, fail)));
    rt.replay_check().unwrap();
}

#[test]
fn h2_unmapped_harness_relation_does_not_append() {
    let mut rt = Runtime::default();
    let n = rt.log().len();
    let report = kutha_runtime::ingest_harness_jsonl_str(
        &mut rt,
        r#"{"op":"assert","subject":"harness.run","relation":"high","object":"3","ingested_at":1,"valid_from":1}"#,
    )
    .unwrap();
    assert_eq!(0, report.status_facts);
    assert_eq!(n, rt.log().len());
}

#[test]
fn h2_tenant_persist_round_trip_keeps_as_of() {
    let mut rt = Runtime::default();
    let fixture = concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/tests/fixtures/harness_process.jsonl"
    );
    kutha_runtime::ingest_harness_jsonl(&mut rt, Path::new(fixture)).unwrap();
    let dir = std::env::temp_dir().join(format!("kutha-h2-{}", std::process::id()));
    store::persist(&rt, &dir).unwrap();
    let mut opened = store::open(&dir).unwrap();
    let run = opened.intern("harness.run");
    let rel = opened.intern("runStatus");
    let ok = opened.intern("ok");
    assert!(opened.fold().as_of(T_OK).contains(&(run, rel, ok)));
    opened.replay_check().unwrap();
    let _ = std::fs::remove_dir_all(&dir);
}
